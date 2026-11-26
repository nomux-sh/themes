//! The `preview` command: one static HTML page that shows each theme. The
//! page has no script and loads nothing from the network.

mod glyph;
mod sidebar;
mod swatches;

use std::fs;
use std::path::Path;

use nomux_theme::{Density, Theme, Value, rules};

use crate::check;

const STYLE: &str = include_str!("preview/style.css");
const SIDEBAR_STYLE: &str = include_str!("preview/sidebar.css");
const WORKSPACE: &str = include_str!("preview/workspace.html");
const DETAILS: &str = include_str!("preview/details.html");

pub fn run(directory: &Path, output: &Path) -> Result<(), String> {
    let mut themes = check::load(directory)?;
    themes.sort_by_key(|theme| (!theme.id().starts_with("nomux"), theme.id().to_owned()));
    fs::write(output, render(&themes))
        .map_err(|error| format!("cannot write {}: {error}", output.display()))
}

fn render(themes: &[Theme]) -> String {
    let links: String = themes
        .iter()
        .map(|theme| format!("<a href=\"#{}\">{}</a>", theme.id(), escape(theme.name())))
        .collect();
    let sections: String = themes.iter().map(section).collect();
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>nomux themes</title>\n<style>\n{STYLE}{SIDEBAR_STYLE}</style>\n</head>\n<body>\n\
         <header class=\"page-header\">\n<h1>nomux themes</h1>\n\
         <p>Each section shows one theme: mock-ups of the nomux UI that use only the tokens of the theme, \
         then each token with its value and its contrast ratio. The command <code>nomux-theme preview</code> \
         writes this page from the theme files. The fonts are the fonts of this system when the font \
         families of the theme are not installed.</p>\n<nav>{links}</nav>\n</header>\n\
         {sections}</body>\n</html>\n"
    )
}

fn section(theme: &Theme) -> String {
    let checks = rules::contrast_checks(theme);
    let passed = checks.iter().filter(|check| check.passes()).count();
    let violations = rules::violations(theme);
    let state = if violations.is_empty() {
        "done"
    } else {
        "error"
    };
    let origins = origins(theme);
    let violation_list = if violations.is_empty() {
        String::new()
    } else {
        let items: String = violations
            .iter()
            .map(|violation| format!("<li>{}</li>", escape(&violation.to_string())))
            .collect();
        format!("<ul class=\"violations\">{items}</ul>\n")
    };
    let workspace = glyph::fill_template(
        &WORKSPACE
            .replace("{rail}", &sidebar::rail())
            .replace("{sidebar}", &sidebar::sidebar(sidebar::Frame::Rest)),
    );
    let workspaces: String = Density::ALL
        .map(|density| {
            let name = density.as_str();
            format!(
                "<figure class=\"{name}\">\n{workspace}<figcaption class=\"caption\">Workspace, {name} density: \
                 project rail, sidebar, tab bar with agent Tabs and shell chips, team roster, \
                 terminal with the 16 ANSI colours</figcaption>\n</figure>\n"
            )
        })
        .concat();

    format!(
        "<section class=\"theme\" id=\"{id}\" style=\"{variables}\">\n\
         <header>\n<h2>{name}</h2>\n<p class=\"meta\">{id} · {appearance} · {origins}</p>\n</header>\n\
         <p class=\"summary\"><span class=\"mark {state}\"></span>{passed} of {total} contrast pairs pass · \
         {count} rule violations</p>\n{violation_list}\
         <div class=\"figures wide\">\n{workspaces}</div>\n{sidebar}{groups}<div class=\"figures\">\n{DETAILS}</div>\n{table}</section>\n",
        id = theme.id(),
        name = escape(theme.name()),
        appearance = theme.appearance(),
        variables = variables(theme),
        total = checks.len(),
        count = violations.len(),
        table = swatches::table(theme, &checks),
        sidebar = sidebar::section(theme.name()),
        groups = if theme.id().starts_with("nomux") {
            sidebar::groups_section(theme.name())
        } else {
            String::new()
        },
    )
}

/// The themes that give the values, for example "40 tokens from nomux-dark,
/// 43 from base-dark".
fn origins(theme: &Theme) -> String {
    let mut counts: Vec<(&str, usize)> = Vec::new();
    for entry in theme.entries() {
        match counts
            .iter_mut()
            .find(|(origin, _)| *origin == entry.origin)
        {
            Some((_, count)) => *count += 1,
            None => counts.push((&entry.origin, 1)),
        }
    }
    counts.sort_by_key(|(origin, _)| *origin != theme.id());
    let parts: Vec<String> = counts
        .iter()
        .map(|(origin, count)| format!("{count} from {origin}"))
        .collect();
    format!("tokens: {}", parts.join(", "))
}

/// The tokens of a theme as CSS custom properties: `surface.base` becomes
/// `--surface-base`.
fn variables(theme: &Theme) -> String {
    theme
        .entries()
        .iter()
        .map(|entry| {
            format!(
                "--{}: {};",
                entry.spec.name.replace('.', "-"),
                escape(&css_value(&entry.value))
            )
        })
        .collect()
}

fn css_value(value: &Value) -> String {
    const GENERIC_FAMILIES: [&str; 6] = [
        "serif",
        "sans-serif",
        "monospace",
        "system-ui",
        "ui-monospace",
        "ui-sans-serif",
    ];
    match value {
        Value::FontFamily(family) => {
            let names = family.names().iter().map(|name| {
                if GENERIC_FAMILIES.contains(&name.as_str()) {
                    name.clone()
                } else {
                    format!("\"{name}\"")
                }
            });
            names.collect::<Vec<String>>().join(", ")
        }
        other => other.to_string(),
    }
}

pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
