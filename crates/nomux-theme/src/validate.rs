//! Validation: the token names and the token values of a theme file against
//! the token set.

use std::collections::BTreeMap;

use crate::parse::ThemeFile;
use crate::problem::Problem;
use crate::token;
use crate::value::Value;

/// The typed token values of one theme file, by token name.
pub type TokenValues = BTreeMap<&'static str, Value>;

/// Examines each entry of `[tokens]`. Returns the typed values, or all
/// problems of the table.
pub fn validate(file: &ThemeFile) -> Result<TokenValues, Vec<Problem>> {
    let mut values = TokenValues::new();
    let mut problems = Vec::new();
    for (name, raw) in &file.tokens {
        let Some(spec) = token::find(name) else {
            problems.push(unknown_name(name, raw));
            continue;
        };
        match spec.kind.parse(raw) {
            Ok(value) => {
                values.insert(spec.name, value);
            }
            Err(reason) => problems.push(Problem::BadValue {
                token: spec.name,
                reason,
            }),
        }
    }
    if problems.is_empty() {
        Ok(values)
    } else {
        Err(problems)
    }
}

fn unknown_name(name: &str, raw: &toml::Value) -> Problem {
    let is_group = token::all().any(|spec| spec.group() == name && spec.name != name);
    if raw.is_table() && is_group {
        return Problem::UnquotedTokenName {
            key: name.to_owned(),
        };
    }
    Problem::UnknownToken {
        name: name.to_owned(),
        nearest: nearest_token(name),
    }
}

/// The token name with the smallest edit distance to `name`. A name that is
/// too different from all token names gets no hint.
fn nearest_token(name: &str) -> Option<&'static str> {
    let limit = (name.chars().count() / 3).max(2);
    token::all()
        .map(|spec| (edit_distance(name, spec.name), spec.name))
        .filter(|(distance, _)| *distance <= limit)
        .min_by_key(|(distance, _)| *distance)
        .map(|(_, nearest)| nearest)
}

/// The Levenshtein distance between two texts.
fn edit_distance(first: &str, second: &str) -> usize {
    let second: Vec<char> = second.chars().collect();
    let mut row: Vec<usize> = (0..=second.len()).collect();
    for (index, first_char) in first.chars().enumerate() {
        let mut diagonal = row[0];
        row[0] = index + 1;
        for (column, second_char) in second.iter().enumerate() {
            let substitution = diagonal + usize::from(first_char != *second_char);
            diagonal = row[column + 1];
            row[column + 1] = substitution.min(row[column] + 1).min(diagonal + 1);
        }
    }
    row[second.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    fn problems(tokens: &str) -> Vec<Problem> {
        let text = format!("[theme]\nname = \"Test\"\nappearance = \"dark\"\n[tokens]\n{tokens}");
        validate(&parse(&text).unwrap()).unwrap_err()
    }

    #[test]
    fn an_unknown_token_gets_the_nearest_name_as_a_hint() {
        assert_eq!(
            problems("\"surface.bse\" = \"#16181c\""),
            [Problem::UnknownToken {
                name: "surface.bse".to_owned(),
                nearest: Some("surface.base")
            }]
        );
        assert_eq!(
            problems("\"state.needs_you\" = \"#e5b445\""),
            [Problem::UnknownToken {
                name: "state.needs_you".to_owned(),
                nearest: Some("state.needs-you")
            }]
        );
        assert_eq!(
            problems("\"window.opacity\" = \"#e5b445\""),
            [Problem::UnknownToken {
                name: "window.opacity".to_owned(),
                nearest: None
            }]
        );
    }

    #[test]
    fn a_bad_colour_names_its_token() {
        assert_eq!(
            problems("\"text.muted\" = \"#9aa1a\""),
            [Problem::BadValue {
                token: "text.muted",
                reason: "a colour has 3, 4, 6 or 8 hexadecimal digits, not 5".to_owned()
            }]
        );
    }

    #[test]
    fn a_token_name_without_quotes_is_reported_as_such() {
        assert_eq!(
            problems("surface.base = \"#16181c\""),
            [Problem::UnquotedTokenName {
                key: "surface".to_owned()
            }]
        );
    }

    #[test]
    fn all_problems_of_a_file_are_reported_together() {
        assert_eq!(
            problems("\"accnt\" = \"#fff\"\n\"motion.fast\" = 120\n\"space.1\" = \"4pt\"").len(),
            3
        );
    }
}
