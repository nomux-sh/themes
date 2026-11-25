//! The value kinds of the tokens and their syntax in a theme file.

use std::fmt;
use std::time::Duration;

use crate::color::Rgba;

/// The kind of value that a token has.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueKind {
    /// A colour: `"#rgb"`, `"#rgba"`, `"#rrggbb"` or `"#rrggbbaa"`.
    Color,
    /// A length in logical pixels: `"4px"`, `"0.5px"`.
    Length,
    /// A duration in milliseconds: `"120ms"`.
    Duration,
    /// A list of font family names, the preferred family first.
    FontFamily,
    /// A font weight: an integer from 1 to 1000.
    FontWeight,
    /// A cubic Bézier easing: `"cubic-bezier(0, 0, 0.2, 1)"`.
    Easing,
}

impl ValueKind {
    /// A short description of the syntax, for error messages and the reference.
    pub const fn syntax(self) -> &'static str {
        match self {
            Self::Color => "a string such as \"#16181c\" or \"#16181c80\"",
            Self::Length => "a string such as \"4px\" or \"0.5px\"",
            Self::Duration => "a string such as \"120ms\"",
            Self::FontFamily => "an array of font family names",
            Self::FontWeight => "an integer from 1 to 1000",
            Self::Easing => "a string such as \"cubic-bezier(0, 0, 0.2, 1)\"",
        }
    }

    /// Reads a value of this kind from a TOML value. The error is the reason
    /// why the TOML value is not correct.
    pub fn parse(self, value: &toml::Value) -> Result<Value, String> {
        match self {
            Self::Color => Rgba::parse_hex(text(self, value)?)
                .map(Value::Color)
                .map_err(|error| error.to_string()),
            Self::Length => Length::parse(text(self, value)?).map(Value::Length),
            Self::Duration => parse_duration(text(self, value)?).map(Value::Duration),
            Self::FontFamily => FontFamily::from_toml(value).map(Value::FontFamily),
            Self::FontWeight => FontWeight::from_toml(value).map(Value::FontWeight),
            Self::Easing => Easing::parse(text(self, value)?).map(Value::Easing),
        }
    }
}

fn text(kind: ValueKind, value: &toml::Value) -> Result<&str, String> {
    value
        .as_str()
        .ok_or_else(|| format!("the value must be {}", kind.syntax()))
}

/// A length in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Length(f32);

impl Length {
    /// The number of logical pixels.
    pub const fn pixels(self) -> f32 {
        self.0
    }

    fn parse(text: &str) -> Result<Self, String> {
        let number = text.strip_suffix("px").ok_or("a length ends with `px`")?;
        match plain_number(number) {
            Some(pixels) => Ok(Self(pixels as f32)),
            None => Err(format!("`{number}` is not a number of pixels")),
        }
    }
}

impl fmt::Display for Length {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}px", self.0)
    }
}

fn parse_duration(text: &str) -> Result<Duration, String> {
    let number = text.strip_suffix("ms").ok_or("a duration ends with `ms`")?;
    if number.is_empty() || !number.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(format!("`{number}` is not a whole number of milliseconds"));
    }
    number
        .parse()
        .map(Duration::from_millis)
        .map_err(|_| format!("`{number}` is too large"))
}

/// Parses digits with an optional fraction. `f64::from_str` also accepts
/// signs, exponents, `inf` and `nan`, which a theme file must not have.
fn plain_number(text: &str) -> Option<f64> {
    let (whole, fraction) = text.split_once('.').unwrap_or((text, "0"));
    let digits = |part: &str| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit());
    if !digits(whole) || !digits(fraction) {
        return None;
    }
    text.parse().ok()
}

/// A list of font family names. The client uses the first family that the
/// system has.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontFamily(Vec<String>);

impl FontFamily {
    /// The family names, the preferred family first.
    pub fn names(&self) -> &[String] {
        &self.0
    }

    fn from_toml(value: &toml::Value) -> Result<Self, String> {
        let syntax = || format!("the value must be {}", ValueKind::FontFamily.syntax());
        let names = value
            .as_array()
            .ok_or_else(syntax)?
            .iter()
            .map(|name| name.as_str().map(str::to_owned).ok_or_else(syntax))
            .collect::<Result<Vec<String>, String>>()?;
        if names.is_empty() {
            return Err("the list of font families is empty".to_owned());
        }
        if names
            .iter()
            .any(|name| name.trim().is_empty() || name.contains(['"', '\\', '\n']))
        {
            return Err(
                "a font family name is empty or has a quote, a backslash or a line break"
                    .to_owned(),
            );
        }
        Ok(Self(names))
    }
}

impl fmt::Display for FontFamily {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names: Vec<String> = self.0.iter().map(|name| format!("\"{name}\"")).collect();
        write!(formatter, "[{}]", names.join(", "))
    }
}

/// A font weight on the scale from 1 to 1000. 400 is regular, 500 is medium.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FontWeight(u16);

impl FontWeight {
    /// The weight as a number.
    pub const fn value(self) -> u16 {
        self.0
    }

    fn from_toml(value: &toml::Value) -> Result<Self, String> {
        match value.as_integer() {
            Some(weight @ 1..=1000) => Ok(Self(weight as u16)),
            Some(weight) => Err(format!("{weight} is not a font weight from 1 to 1000")),
            None => Err(format!(
                "the value must be {}",
                ValueKind::FontWeight.syntax()
            )),
        }
    }
}

impl fmt::Display for FontWeight {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// A cubic Bézier easing with the two control points `(x1, y1)` and
/// `(x2, y2)`, as in CSS.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Easing {
    /// X coordinate of the first control point, from 0 to 1.
    pub x1: f32,
    /// Y coordinate of the first control point.
    pub y1: f32,
    /// X coordinate of the second control point, from 0 to 1.
    pub x2: f32,
    /// Y coordinate of the second control point.
    pub y2: f32,
}

impl Easing {
    fn parse(text: &str) -> Result<Self, String> {
        let syntax = || format!("the value must be {}", ValueKind::Easing.syntax());
        let arguments = text
            .strip_prefix("cubic-bezier(")
            .and_then(|rest| rest.strip_suffix(')'))
            .ok_or_else(syntax)?;
        let numbers = arguments
            .split(',')
            .map(|number| {
                number
                    .trim()
                    .parse::<f32>()
                    .ok()
                    .filter(|number| number.is_finite())
            })
            .collect::<Option<Vec<f32>>>()
            .ok_or_else(syntax)?;
        let &[x1, y1, x2, y2] = numbers.as_slice() else {
            return Err("a cubic Bézier easing has four numbers".to_owned());
        };
        if !(0.0..=1.0).contains(&x1) || !(0.0..=1.0).contains(&x2) {
            return Err("the first and the third number must be from 0 to 1".to_owned());
        }
        Ok(Self { x1, y1, x2, y2 })
    }
}

impl fmt::Display for Easing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "cubic-bezier({}, {}, {}, {})",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}

/// The value of one token.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// A colour.
    Color(Rgba),
    /// A length.
    Length(Length),
    /// A duration.
    Duration(Duration),
    /// A list of font families.
    FontFamily(FontFamily),
    /// A font weight.
    FontWeight(FontWeight),
    /// An easing.
    Easing(Easing),
}

impl fmt::Display for Value {
    /// Writes the value as a theme file shows it, without the TOML quotes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Color(color) => color.fmt(formatter),
            Self::Length(length) => length.fmt(formatter),
            Self::Duration(duration) => write!(formatter, "{}ms", duration.as_millis()),
            Self::FontFamily(family) => family.fmt(formatter),
            Self::FontWeight(weight) => weight.fmt(formatter),
            Self::Easing(easing) => easing.fmt(formatter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(kind: ValueKind, toml: &str) -> Result<Value, String> {
        let table: toml::Table = format!("value = {toml}").parse().unwrap();
        kind.parse(&table["value"])
    }

    #[test]
    fn reads_each_value_kind() {
        assert_eq!(
            parse(ValueKind::Length, "\"0.5px\"").unwrap().to_string(),
            "0.5px"
        );
        assert_eq!(
            parse(ValueKind::Duration, "\"120ms\""),
            Ok(Value::Duration(Duration::from_millis(120)))
        );
        assert_eq!(
            parse(ValueKind::FontWeight, "500").unwrap().to_string(),
            "500"
        );
        assert_eq!(
            parse(ValueKind::FontFamily, "[\"Inter\", \"sans-serif\"]")
                .unwrap()
                .to_string(),
            "[\"Inter\", \"sans-serif\"]"
        );
        assert_eq!(
            parse(ValueKind::Easing, "\"cubic-bezier(0, 0, 0.2, 1)\"")
                .unwrap()
                .to_string(),
            "cubic-bezier(0, 0, 0.2, 1)"
        );
    }

    #[test]
    fn rejects_a_value_with_a_wrong_unit_sign_or_type() {
        for (kind, toml) in [
            (ValueKind::Color, "16181"),
            (ValueKind::Length, "4"),
            (ValueKind::Length, "\"4\""),
            (ValueKind::Length, "\"-4px\""),
            (ValueKind::Length, "\"infpx\""),
            (ValueKind::Length, "\"1e3px\""),
            (ValueKind::Length, "\".5px\""),
            (ValueKind::Duration, "\"0.2s\""),
            (ValueKind::Duration, "\"1.5ms\""),
            (ValueKind::FontWeight, "\"500\""),
            (ValueKind::FontWeight, "0"),
            (ValueKind::FontFamily, "\"Inter\""),
            (ValueKind::FontFamily, "[]"),
            (ValueKind::FontFamily, "[\"\"]"),
            (ValueKind::Easing, "\"ease-out\""),
            (ValueKind::Easing, "\"cubic-bezier(0, 0, 1)\""),
            (ValueKind::Easing, "\"cubic-bezier(2, 0, 0.2, 1)\""),
        ] {
            assert!(parse(kind, toml).is_err(), "{kind:?} accepted {toml}");
        }
    }
}
