//! Resolution: a set of theme files to complete themes. A token that a theme
//! does not set comes from its base theme.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::parse::{self, Appearance, is_theme_id};
use crate::problem::{LoadError, Problem, ThemeError};
use crate::theme::{Entry, Theme};
use crate::token;
use crate::validate::{self, TokenValues};

struct Source {
    name: String,
    appearance: Appearance,
    extends: Option<String>,
    values: TokenValues,
}

/// The theme files that are available, by identifier. `extends` refers to an
/// identifier of the same set.
#[derive(Default)]
pub struct ThemeSet {
    sources: BTreeMap<String, Source>,
}

impl ThemeSet {
    /// Makes an empty set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads each `.toml` file of a directory. The identifier of a theme is
    /// its file name without `.toml`.
    pub fn load_dir(directory: &Path) -> Result<Self, LoadError> {
        let io = |path: &Path, source| LoadError::Io {
            path: path.to_owned(),
            source,
        };
        let mut paths = Vec::new();
        for entry in fs::read_dir(directory).map_err(|source| io(directory, source))? {
            let path = entry.map_err(|source| io(directory, source))?.path();
            if path
                .extension()
                .is_some_and(|extension| extension == "toml")
            {
                paths.push(path);
            }
        }
        paths.sort();

        let mut set = Self::new();
        let mut errors = Vec::new();
        for path in paths {
            let id = path.file_stem().unwrap_or_default().to_string_lossy();
            let text = fs::read_to_string(&path).map_err(|source| io(&path, source))?;
            if let Err(error) = set.insert(&id, &text) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(set)
        } else {
            Err(LoadError::Themes(errors))
        }
    }

    /// Parses and validates the text of one theme file and adds it to the
    /// set. A theme with the same identifier is replaced.
    pub fn insert(&mut self, id: &str, text: &str) -> Result<(), ThemeError> {
        let error = |problems| ThemeError {
            theme: id.to_owned(),
            problems,
        };
        if !is_theme_id(id) {
            return Err(error(vec![Problem::Structure(
                "the file name must have only lowercase letters, digits and hyphens before `.toml`"
                    .to_owned(),
            )]));
        }
        let file = parse::parse(text).map_err(error)?;
        let values = validate::validate(&file).map_err(error)?;
        let source = Source {
            name: file.name,
            appearance: file.appearance,
            extends: file.extends,
            values,
        };
        self.sources.insert(id.to_owned(), source);
        Ok(())
    }

    /// The identifiers of the set, sorted.
    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.sources.keys().map(String::as_str)
    }

    /// Resolves one theme: follows `extends` and gives each token a value.
    pub fn resolve(&self, id: &str) -> Result<Theme, ThemeError> {
        let error = |problem| ThemeError {
            theme: id.to_owned(),
            problems: vec![problem],
        };
        let chain = self.chain(id).map_err(error)?;
        let (_, theme) = chain[0];

        let mut entries = Vec::new();
        let mut missing = Vec::new();
        for spec in token::all() {
            let found = chain
                .iter()
                .find_map(|(origin, source)| Some((*origin, source.values.get(spec.name)?)));
            match found {
                Some((origin, value)) => entries.push(Entry {
                    spec,
                    value: value.clone(),
                    origin: origin.to_owned(),
                }),
                None => missing.push(Problem::MissingToken { name: spec.name }),
            }
        }
        if !missing.is_empty() {
            return Err(ThemeError {
                theme: id.to_owned(),
                problems: missing,
            });
        }
        Ok(Theme::from_entries(
            id.to_owned(),
            theme.name.clone(),
            theme.appearance,
            entries,
        ))
    }

    /// Resolves each theme of the set. Returns all themes, or the problems
    /// of all themes that have one.
    pub fn resolve_all(&self) -> Result<Vec<Theme>, Vec<ThemeError>> {
        let mut themes = Vec::new();
        let mut errors = Vec::new();
        for id in self.ids() {
            match self.resolve(id) {
                Ok(theme) => themes.push(theme),
                Err(error) => errors.push(error),
            }
        }
        if errors.is_empty() {
            Ok(themes)
        } else {
            Err(errors)
        }
    }

    /// The theme and its base themes, the theme first.
    fn chain<'set>(&'set self, id: &'set str) -> Result<Vec<(&'set str, &'set Source)>, Problem> {
        let theme = self.sources.get(id).ok_or(Problem::UnknownTheme)?;
        let mut chain = vec![(id, theme)];
        let mut current = theme;
        while let Some(extends) = current.extends.as_deref() {
            if chain.iter().any(|(seen, _)| *seen == extends) {
                let mut ids: Vec<String> =
                    chain.iter().map(|(seen, _)| (*seen).to_owned()).collect();
                ids.push(extends.to_owned());
                return Err(Problem::Cycle { chain: ids });
            }
            let base = self
                .sources
                .get(extends)
                .ok_or_else(|| Problem::UnknownBase {
                    extends: extends.to_owned(),
                })?;
            if base.appearance != theme.appearance {
                return Err(Problem::AppearanceMismatch {
                    base: extends.to_owned(),
                });
            }
            chain.push((extends, base));
            current = base;
        }
        Ok(chain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Rgba;
    use crate::token::ColorToken;
    use crate::value::ValueKind;

    fn file(appearance: &str, extends: Option<&str>, tokens: &str) -> String {
        let extends = extends
            .map(|base| format!("extends = \"{base}\"\n"))
            .unwrap_or_default();
        format!(
            "[theme]\nname = \"Test\"\nappearance = \"{appearance}\"\n{extends}[tokens]\n{tokens}"
        )
    }

    /// A theme that sets each token, as a base theme does.
    fn set_with_base() -> ThemeSet {
        let tokens: String = token::all()
            .map(|spec| {
                let value = match spec.kind {
                    ValueKind::Color => "\"#000000\"",
                    ValueKind::Length => "\"4px\"",
                    ValueKind::Duration => "\"120ms\"",
                    ValueKind::FontFamily => "[\"sans-serif\"]",
                    ValueKind::FontWeight => "400",
                    ValueKind::Easing => "\"cubic-bezier(0, 0, 0.2, 1)\"",
                };
                format!("\"{}\" = {value}\n", spec.name)
            })
            .collect();
        let mut set = ThemeSet::new();
        set.insert("base-dark", &file("dark", None, &tokens))
            .unwrap();
        set
    }

    fn problems(set: &ThemeSet, id: &str) -> Vec<Problem> {
        set.resolve(id).unwrap_err().problems
    }

    #[test]
    fn a_missing_token_comes_from_the_nearest_base_theme() {
        let mut set = set_with_base();
        set.insert(
            "middle",
            &file(
                "dark",
                Some("base-dark"),
                "\"accent\" = \"#111111\"\n\"state.done\" = \"#222222\"",
            ),
        )
        .unwrap();
        set.insert(
            "top",
            &file("dark", Some("middle"), "\"accent\" = \"#333333\""),
        )
        .unwrap();

        let top = set.resolve("top").unwrap();
        let base = set.resolve("base-dark").unwrap();
        assert_eq!(
            top.color(ColorToken::Accent),
            Rgba::opaque(0x33, 0x33, 0x33)
        );
        assert_eq!(
            top.color(ColorToken::StateDone),
            Rgba::opaque(0x22, 0x22, 0x22)
        );
        assert_eq!(
            top.color(ColorToken::SurfaceBase),
            base.color(ColorToken::SurfaceBase)
        );

        let origin = |name: &str| {
            top.entries()
                .iter()
                .find(|entry| entry.spec.name == name)
                .unwrap()
                .origin
                .clone()
        };
        assert_eq!(origin("accent"), "top");
        assert_eq!(origin("state.done"), "middle");
        assert_eq!(origin("surface.base"), "base-dark");
    }

    #[test]
    fn a_cycle_is_an_error_that_shows_the_chain() {
        let mut set = ThemeSet::new();
        set.insert("a", &file("dark", Some("b"), "")).unwrap();
        set.insert("b", &file("dark", Some("c"), "")).unwrap();
        set.insert("c", &file("dark", Some("a"), "")).unwrap();
        set.insert("self", &file("dark", Some("self"), "")).unwrap();

        let chain = |ids: &[&str]| {
            vec![Problem::Cycle {
                chain: ids.iter().map(|id| (*id).to_owned()).collect(),
            }]
        };
        assert_eq!(problems(&set, "a"), chain(&["a", "b", "c", "a"]));
        assert_eq!(problems(&set, "self"), chain(&["self", "self"]));
    }

    #[test]
    fn a_base_theme_that_is_not_available_is_an_error() {
        let mut set = ThemeSet::new();
        set.insert("orphan", &file("dark", Some("base-dark"), ""))
            .unwrap();
        assert_eq!(
            problems(&set, "orphan"),
            [Problem::UnknownBase {
                extends: "base-dark".to_owned()
            }]
        );
        assert_eq!(problems(&set, "absent"), [Problem::UnknownTheme]);
    }

    #[test]
    fn a_theme_without_a_base_must_set_each_token() {
        let mut set = ThemeSet::new();
        set.insert("alone", &file("dark", None, "\"accent\" = \"#111111\""))
            .unwrap();
        let problems = problems(&set, "alone");
        assert_eq!(problems.len(), token::all().count() - 1);
        assert!(problems.contains(&Problem::MissingToken {
            name: "surface.base"
        }));
    }

    #[test]
    fn a_light_theme_cannot_extend_a_dark_theme() {
        let mut set = set_with_base();
        set.insert("light", &file("light", Some("base-dark"), ""))
            .unwrap();
        assert_eq!(
            problems(&set, "light"),
            [Problem::AppearanceMismatch {
                base: "base-dark".to_owned()
            }]
        );
    }
}
