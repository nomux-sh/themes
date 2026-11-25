//! Import of an iTerm colour scheme (`.itermcolors`, an XML property list)
//! into the terminal tokens of a new theme.

use std::fmt;

use roxmltree::{Document, Node, ParsingOptions};

use crate::color::Rgba;
use crate::parse::Appearance;
use crate::token::ColorToken;

/// The iTerm keys that a scheme must have, with the token of each key.
const REQUIRED: [(&str, ColorToken); 18] = [
    ("Background Color", ColorToken::TerminalBackground),
    ("Foreground Color", ColorToken::TerminalForeground),
    ("Ansi 0 Color", ColorToken::TerminalAnsiBlack),
    ("Ansi 1 Color", ColorToken::TerminalAnsiRed),
    ("Ansi 2 Color", ColorToken::TerminalAnsiGreen),
    ("Ansi 3 Color", ColorToken::TerminalAnsiYellow),
    ("Ansi 4 Color", ColorToken::TerminalAnsiBlue),
    ("Ansi 5 Color", ColorToken::TerminalAnsiMagenta),
    ("Ansi 6 Color", ColorToken::TerminalAnsiCyan),
    ("Ansi 7 Color", ColorToken::TerminalAnsiWhite),
    ("Ansi 8 Color", ColorToken::TerminalAnsiBrightBlack),
    ("Ansi 9 Color", ColorToken::TerminalAnsiBrightRed),
    ("Ansi 10 Color", ColorToken::TerminalAnsiBrightGreen),
    ("Ansi 11 Color", ColorToken::TerminalAnsiBrightYellow),
    ("Ansi 12 Color", ColorToken::TerminalAnsiBrightBlue),
    ("Ansi 13 Color", ColorToken::TerminalAnsiBrightMagenta),
    ("Ansi 14 Color", ColorToken::TerminalAnsiBrightCyan),
    ("Ansi 15 Color", ColorToken::TerminalAnsiBrightWhite),
];

/// The iTerm keys that a scheme can omit. The base theme gives the token
/// when the key is missing.
const OPTIONAL: [(&str, ColorToken); 2] = [
    ("Cursor Color", ColorToken::TerminalCursor),
    ("Selection Color", ColorToken::TerminalSelection),
];

/// The reason why a file is not an iTerm colour scheme that nomux can import.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportError {
    /// The file is not correct XML.
    Xml(String),
    /// The XML is not a property list with a dictionary at the top.
    NotPropertyList,
    /// The scheme does not have a colour that a terminal needs.
    MissingColor {
        /// The iTerm key.
        key: &'static str,
    },
    /// A colour of the scheme is not a dictionary with correct components.
    BadColor {
        /// The iTerm key.
        key: &'static str,
        /// Why the colour is not correct.
        reason: String,
    },
}

impl fmt::Display for ImportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Xml(message) => write!(formatter, "the file is not correct XML: {message}"),
            Self::NotPropertyList => {
                formatter.write_str("the file is not an XML property list with a dictionary")
            }
            Self::MissingColor { key } => write!(formatter, "the scheme has no `{key}`"),
            Self::BadColor { key, reason } => write!(formatter, "`{key}`: {reason}"),
        }
    }
}

impl std::error::Error for ImportError {}

/// The terminal colours of an iTerm colour scheme.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItermScheme {
    colors: Vec<(ColorToken, Rgba)>,
}

impl ItermScheme {
    /// Reads the text of an `.itermcolors` file.
    ///
    /// The components are read as sRGB. A scheme in the Display P3 colour
    /// space is not converted, thus its colours are a little less saturated
    /// than in iTerm.
    pub fn parse(text: &str) -> Result<Self, ImportError> {
        let options = ParsingOptions {
            allow_dtd: true,
            ..ParsingOptions::default()
        };
        let document = Document::parse_with_options(text, options)
            .map_err(|error| ImportError::Xml(error.to_string()))?;
        let root = document.root_element();
        let scheme = root.children().find(|node| node.has_tag_name("dict"));
        let Some(scheme) = scheme.filter(|_| root.has_tag_name("plist")) else {
            return Err(ImportError::NotPropertyList);
        };

        let mut colors = Vec::new();
        for (key, token) in REQUIRED {
            let entry = entry(scheme, key).ok_or(ImportError::MissingColor { key })?;
            colors.push((token, color(entry, key)?));
        }
        for (key, token) in OPTIONAL {
            if let Some(entry) = entry(scheme, key) {
                colors.push((token, color(entry, key)?));
            }
        }
        colors.sort_by_key(|(token, _)| *token);
        Ok(Self { colors })
    }

    /// The terminal tokens that the scheme sets, with their colours.
    pub fn colors(&self) -> &[(ColorToken, Rgba)] {
        &self.colors
    }

    /// Dark when the background is darker than the text.
    pub fn appearance(&self) -> Appearance {
        let luminance = |token| {
            let color = self
                .colors
                .iter()
                .find(|(candidate, _)| *candidate == token);
            color.map_or(0.0, |(_, color)| color.relative_luminance())
        };
        if luminance(ColorToken::TerminalBackground) <= luminance(ColorToken::TerminalForeground) {
            Appearance::Dark
        } else {
            Appearance::Light
        }
    }

    /// Writes a theme file that extends the base theme of the same
    /// appearance and sets the terminal tokens.
    pub fn to_theme_file(&self, name: &str) -> String {
        let appearance = self.appearance();
        let mut file = format!(
            "[theme]\nname = {}\nappearance = \"{appearance}\"\nextends = \"base-{appearance}\"\n\n[tokens]\n",
            toml_string(name)
        );
        for (token, color) in &self.colors {
            file.push_str(&format!("\"{}\" = \"{color}\"\n", token.name()));
        }
        file
    }
}

/// Writes a text as a TOML basic string.
fn toml_string(text: &str) -> String {
    let mut string = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => string.push_str("\\\""),
            '\\' => string.push_str("\\\\"),
            control if control.is_control() => {
                string.push_str(&format!("\\u{:04x}", control as u32))
            }
            other => string.push(other),
        }
    }
    string.push('"');
    string
}

/// The value of one key of a property list dictionary.
fn entry<'a, 'input>(dictionary: Node<'a, 'input>, key: &str) -> Option<Node<'a, 'input>> {
    let mut children = dictionary.children().filter(Node::is_element);
    while let Some(child) = children.next() {
        let value = children.next()?;
        if child.has_tag_name("key") && child.text() == Some(key) {
            return Some(value);
        }
    }
    None
}

fn color(dictionary: Node, key: &'static str) -> Result<Rgba, ImportError> {
    let bad = |reason: String| ImportError::BadColor { key, reason };
    if !dictionary.has_tag_name("dict") {
        return Err(bad("the value is not a dictionary".to_owned()));
    }
    let component = |name: &str| -> Result<Option<u8>, ImportError> {
        let Some(node) = entry(dictionary, name) else {
            return Ok(None);
        };
        let number = node
            .text()
            .and_then(|text| text.trim().parse::<f64>().ok())
            .filter(|number| number.is_finite());
        let number = number.ok_or_else(|| bad(format!("`{name}` is not a number")))?;
        Ok(Some((number.clamp(0.0, 1.0) * 255.0).round() as u8))
    };
    let required = |name: &str| component(name)?.ok_or_else(|| bad(format!("`{name}` is missing")));
    Ok(Rgba {
        red: required("Red Component")?,
        green: required("Green Component")?,
        blue: required("Blue Component")?,
        alpha: component("Alpha Component")?.unwrap_or(255),
    })
}
