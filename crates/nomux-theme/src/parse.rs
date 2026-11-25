//! Parsing: the text of a theme file to its tables. The token names and the
//! token values are not examined here.

use std::collections::BTreeMap;
use std::fmt;

use toml::{Table, Value};

use crate::problem::Problem;

/// Whether a theme is light or dark.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Appearance {
    /// Dark text on light surfaces.
    Light,
    /// Light text on dark surfaces.
    Dark,
}

impl Appearance {
    /// The word in a theme file: `light` or `dark`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

impl fmt::Display for Appearance {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// The content of one theme file.
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeFile {
    /// `theme.name`: the name that the UI shows.
    pub name: String,
    /// `theme.appearance`.
    pub appearance: Appearance,
    /// `theme.extends`: the identifier of the base theme.
    pub extends: Option<String>,
    /// The `[tokens]` table as the file has it.
    pub tokens: BTreeMap<String, Value>,
}

/// Returns `true` when `id` can be a theme identifier: lowercase letters,
/// digits and hyphens.
pub fn is_theme_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

/// Parses the text of a theme file.
pub fn parse(text: &str) -> Result<ThemeFile, Vec<Problem>> {
    let mut root: Table = text
        .parse()
        .map_err(|error: toml::de::Error| vec![Problem::Syntax(error.to_string())])?;
    let mut problems = Vec::new();

    let theme = take_table(&mut root, "theme", &mut problems);
    let tokens = take_table(&mut root, "tokens", &mut problems);
    for key in root.keys() {
        problems.push(Problem::Structure(format!(
            "unknown table or key `{key}`. A theme file has [theme] and [tokens]"
        )));
    }

    let meta = match theme {
        Some(theme) => read_meta(theme, &mut problems),
        None => {
            problems.push(Problem::Structure(
                "the [theme] table is missing".to_owned(),
            ));
            None
        }
    };
    match meta {
        Some((name, appearance, extends)) if problems.is_empty() => Ok(ThemeFile {
            name,
            appearance,
            extends,
            tokens: tokens.unwrap_or_default().into_iter().collect(),
        }),
        _ => Err(problems),
    }
}

fn take_table(root: &mut Table, key: &str, problems: &mut Vec<Problem>) -> Option<Table> {
    match root.remove(key)? {
        Value::Table(table) => Some(table),
        _ => {
            problems.push(Problem::Structure(format!("`{key}` must be a table")));
            None
        }
    }
}

fn read_meta(
    mut theme: Table,
    problems: &mut Vec<Problem>,
) -> Option<(String, Appearance, Option<String>)> {
    let name = take_string(&mut theme, "name", problems);
    let appearance = take_string(&mut theme, "appearance", problems);
    let extends = take_string(&mut theme, "extends", problems);
    for key in theme.keys() {
        problems.push(Problem::Structure(format!(
            "unknown key `{key}` in [theme]. The keys are `name`, `appearance` and `extends`"
        )));
    }

    if name.as_deref().is_none_or(|name| name.trim().is_empty()) {
        problems.push(Problem::Structure(
            "`theme.name` is missing or empty".to_owned(),
        ));
    }
    let appearance = match appearance.as_deref() {
        Some("light") => Some(Appearance::Light),
        Some("dark") => Some(Appearance::Dark),
        _ => {
            problems.push(Problem::Structure(
                "`theme.appearance` must be \"light\" or \"dark\"".to_owned(),
            ));
            None
        }
    };
    if extends
        .as_deref()
        .is_some_and(|extends| !is_theme_id(extends))
    {
        problems.push(Problem::Structure(
            "`theme.extends` must be a theme identifier: lowercase letters, digits and hyphens"
                .to_owned(),
        ));
    }
    Some((name?, appearance?, extends))
}

fn take_string(table: &mut Table, key: &str, problems: &mut Vec<Problem>) -> Option<String> {
    match table.remove(key)? {
        Value::String(text) => Some(text),
        _ => {
            problems.push(Problem::Structure(format!(
                "`theme.{key}` must be a string"
            )));
            None
        }
    }
}
