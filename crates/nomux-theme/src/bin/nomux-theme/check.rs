//! The `check` command.

use std::path::Path;

use nomux_theme::{Theme, ThemeSet, rules};

/// Loads and resolves each theme of a directory. The error has one line for
/// each problem.
pub fn load(directory: &Path) -> Result<Vec<Theme>, String> {
    let set = ThemeSet::load_dir(directory).map_err(|error| error.to_string())?;
    let themes = set.resolve_all().map_err(|errors| {
        let lines: Vec<String> = errors.iter().map(ToString::to_string).collect();
        lines.join("\n")
    })?;
    if themes.is_empty() {
        return Err(format!("{} has no theme files", directory.display()));
    }
    Ok(themes)
}

pub fn run(directory: &Path) -> Result<(), String> {
    let themes = load(directory)?;
    let mut failures = Vec::new();
    for theme in &themes {
        let violations = rules::violations(theme);
        if violations.is_empty() {
            println!(
                "ok    {} ({} contrast pairs)",
                theme.id(),
                rules::contrast_checks(theme).len()
            );
        }
        failures.extend(
            violations
                .iter()
                .map(|violation| format!("{}: {violation}", theme.id())),
        );
    }
    if failures.is_empty() {
        return Ok(());
    }
    Err(failures.join("\n"))
}
