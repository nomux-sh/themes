//! The token table of the preview page: each token with its value, a sample
//! and its contrast ratio.

use nomux_theme::rules::ContrastCheck;
use nomux_theme::{ColorToken, Entry, Rgba, Theme, Value, contrast_ratio};

use super::escape;
use crate::reference::kind_name;

pub fn table(theme: &Theme, checks: &[ContrastCheck]) -> String {
    let rows: String = theme
        .entries()
        .iter()
        .map(|entry| row(theme, entry, checks))
        .collect();
    format!(
        "<details open>\n<summary>Tokens ({count})</summary>\n<table>\n\
         <thead><tr><th>Token</th><th>Sample</th><th>Value</th><th>Kind</th><th>From</th><th>Contrast</th></tr></thead>\n\
         <tbody>\n{rows}</tbody>\n</table>\n</details>\n",
        count = theme.entries().len()
    )
}

fn row(theme: &Theme, entry: &Entry, checks: &[ContrastCheck]) -> String {
    let name = entry.spec.name;
    let variable = format!("var(--{})", name.replace('.', "-"));
    let (sample, contrast) = match &entry.value {
        Value::Color(color) => (
            format!("<span class=\"swatch\" style=\"background: {variable}\"></span>"),
            contrast(theme, name, *color, checks),
        ),
        Value::Length(_) => (
            format!("<span class=\"bar\" style=\"width: {variable}\"></span>"),
            String::new(),
        ),
        Value::FontFamily(_) => (
            format!("<span style=\"font-family: {variable}\">Ag 0123</span>"),
            String::new(),
        ),
        Value::FontWeight(_) => (
            format!("<span style=\"font-weight: {variable}\">Ag 0123</span>"),
            String::new(),
        ),
        Value::Duration(_) | Value::Easing(_) => (String::new(), String::new()),
    };
    format!(
        "<tr><td class=\"mono\">{name}</td><td>{sample}</td><td class=\"mono\">{value}</td><td class=\"meta\">{kind}</td>\
         <td class=\"meta\">{origin}</td><td class=\"contrast\">{contrast}</td></tr>\n",
        value = escape(&entry.value.to_string()),
        kind = kind_name(entry.spec.kind),
        origin = entry.origin,
    )
}

/// A colour that the contrast rules examine shows its lowest ratio and the
/// result. Each other colour shows its ratio on `surface.base`.
fn contrast(theme: &Theme, name: &str, color: Rgba, checks: &[ContrastCheck]) -> String {
    let lowest = checks
        .iter()
        .filter(|check| check.foreground.name() == name)
        .min_by(|first, second| {
            (first.ratio - first.minimum).total_cmp(&(second.ratio - second.minimum))
        });
    let Some(check) = lowest else {
        let base = theme.color(ColorToken::SurfaceBase);
        return format!(
            "<span class=\"meta\">{:.2}:1 on surface.base</span>",
            contrast_ratio(color.over(base), base)
        );
    };
    let (state, word) = if check.passes() {
        ("done", "pass")
    } else {
        ("error", "fail")
    };
    let overlay = check
        .overlay
        .map(|overlay| format!(" with {}", overlay.name()))
        .unwrap_or_default();
    format!(
        "<span class=\"result\"><span class=\"mark {state}\"></span>{:.2}:1 {word}</span> \
         <span class=\"meta\">lowest, on {}{overlay} · minimum {}:1</span>",
        check.ratio,
        check.background.name(),
        check.minimum
    )
}
