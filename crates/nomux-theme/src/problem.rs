//! The problems that a theme file can have.

use std::fmt;
use std::path::PathBuf;

/// One problem of a theme.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Problem {
    /// The file is not correct TOML.
    Syntax(String),
    /// A table or a key is missing, is not known or has the wrong type.
    Structure(String),
    /// A token name is not in quotes, thus TOML reads it as a table.
    UnquotedTokenName {
        /// The first part of the name, which TOML reads as a table name.
        key: String,
    },
    /// The token set does not have a token with this name.
    UnknownToken {
        /// The name in the file.
        name: String,
        /// The token name that is nearest to `name`, when one is near.
        nearest: Option<&'static str>,
    },
    /// The value of a token does not have the syntax of its kind.
    BadValue {
        /// The name of the token.
        token: &'static str,
        /// Why the value is not correct.
        reason: String,
    },
    /// No theme has this identifier.
    UnknownTheme,
    /// `extends` names a theme that is not available.
    UnknownBase {
        /// The value of `extends`.
        extends: String,
    },
    /// The `extends` chain comes back to a theme that it already has.
    Cycle {
        /// The identifiers of the chain. The last one is equal to an earlier one.
        chain: Vec<String>,
    },
    /// The appearance of the theme is not the appearance of its base theme.
    AppearanceMismatch {
        /// The identifier of the base theme.
        base: String,
    },
    /// The theme and its base themes do not give a value to a token.
    MissingToken {
        /// The name of the token.
        name: &'static str,
    },
}

impl fmt::Display for Problem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(message) => write!(formatter, "the file is not correct TOML: {message}"),
            Self::Structure(message) => formatter.write_str(message),
            Self::UnquotedTokenName { key } => write!(
                formatter,
                "`{key}` in [tokens] is a table. Put each token name in quotes, for example \"surface.base\""
            ),
            Self::UnknownToken {
                name,
                nearest: Some(nearest),
            } => {
                write!(
                    formatter,
                    "unknown token `{name}`. Did you mean `{nearest}`?"
                )
            }
            Self::UnknownToken {
                name,
                nearest: None,
            } => write!(formatter, "unknown token `{name}`"),
            Self::BadValue { token, reason } => write!(formatter, "token `{token}`: {reason}"),
            Self::UnknownTheme => formatter.write_str("no theme has this identifier"),
            Self::UnknownBase { extends } => {
                write!(formatter, "the base theme `{extends}` is not available")
            }
            Self::Cycle { chain } => {
                write!(formatter, "`extends` makes a cycle: {}", chain.join(" → "))
            }
            Self::AppearanceMismatch { base } => {
                write!(
                    formatter,
                    "the appearance is not the appearance of the base theme `{base}`"
                )
            }
            Self::MissingToken { name } => {
                write!(
                    formatter,
                    "no value for token `{name}`. Set it, or extend a complete base theme"
                )
            }
        }
    }
}

/// The problems of one theme.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeError {
    /// The identifier of the theme: the file name without `.toml`.
    pub theme: String,
    /// The problems. The list is not empty.
    pub problems: Vec<Problem>,
}

impl fmt::Display for ThemeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, problem) in self.problems.iter().enumerate() {
            if index > 0 {
                formatter.write_str("\n")?;
            }
            write!(formatter, "{}: {problem}", self.theme)?;
        }
        Ok(())
    }
}

impl std::error::Error for ThemeError {}

/// The error of [`ThemeSet::load_dir`](crate::ThemeSet::load_dir).
#[derive(Debug)]
pub enum LoadError {
    /// The directory or a file in it cannot be read.
    Io {
        /// The path that the read was for.
        path: PathBuf,
        /// The error of the operating system.
        source: std::io::Error,
    },
    /// One or more theme files have problems.
    Themes(Vec<ThemeError>),
}

impl fmt::Display for LoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(formatter, "cannot read {}: {source}", path.display())
            }
            Self::Themes(errors) => {
                let lines: Vec<String> = errors.iter().map(ThemeError::to_string).collect();
                formatter.write_str(&lines.join("\n"))
            }
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Themes(_) => None,
        }
    }
}
