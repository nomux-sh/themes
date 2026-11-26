//! The iTerm import with a file in the format that iTerm exports.

use nomux_theme::{Appearance, ColorToken, ImportError, ItermScheme, Rgba, ThemeSet};

const SAMPLE: &str = include_str!("fixtures/sample.itermcolors");

#[test]
fn an_iterm_scheme_sets_the_terminal_tokens_of_a_new_theme() {
    let scheme = ItermScheme::parse(SAMPLE).unwrap();
    let color = |token| {
        scheme
            .colors()
            .iter()
            .find(|(candidate, _)| *candidate == token)
            .map(|(_, color)| *color)
    };

    assert_eq!(
        color(ColorToken::TerminalBackground),
        Some(Rgba::opaque(0x11, 0x13, 0x16))
    );
    assert_eq!(
        color(ColorToken::TerminalAnsiRed),
        Some(Rgba::opaque(0xe5, 0x53, 0x4b))
    );
    assert_eq!(
        color(ColorToken::TerminalAnsiBrightWhite),
        Some(Rgba::opaque(0xf0, 0xf2, 0xf4))
    );
    assert_eq!(
        color(ColorToken::TerminalCursor),
        Some(Rgba::opaque(0xe6, 0xe8, 0xeb))
    );
    // The sample has no `Selection Color`, and `Bold Color` has no token.
    assert_eq!(color(ColorToken::TerminalSelection), None);
    assert_eq!(scheme.colors().len(), 19);
    assert_eq!(scheme.appearance(), Appearance::Dark);
}

#[test]
fn the_theme_file_of_an_import_is_a_theme_that_loads() {
    let file = ItermScheme::parse(SAMPLE)
        .unwrap()
        .to_theme_file("My \"Scheme\"");

    let mut themes = ThemeSet::new();
    themes
        .insert("base-dark", include_str!("../themes/base-dark.toml"))
        .unwrap();
    themes.insert("imported", &file).unwrap();
    let theme = themes.resolve("imported").unwrap();
    let base = themes.resolve("base-dark").unwrap();

    assert_eq!(theme.name(), "My \"Scheme\"");
    assert_eq!(
        theme.color(ColorToken::TerminalAnsiBlue),
        Rgba::opaque(0x5b, 0x9c, 0xf5)
    );
    assert_eq!(
        theme.color(ColorToken::TerminalSelection),
        base.color(ColorToken::TerminalSelection)
    );
    assert_eq!(
        theme.color(ColorToken::SurfaceBase),
        base.color(ColorToken::SurfaceBase)
    );
}

#[test]
fn a_file_that_is_not_a_complete_scheme_is_an_error() {
    assert!(matches!(
        ItermScheme::parse("not xml"),
        Err(ImportError::Xml(_))
    ));
    assert_eq!(
        ItermScheme::parse("<html><dict/></html>"),
        Err(ImportError::NotPropertyList)
    );
    assert_eq!(
        ItermScheme::parse(&SAMPLE.replace("Ansi 9 Color", "Ansi 99 Color")),
        Err(ImportError::MissingColor {
            key: "Ansi 9 Color"
        })
    );
    let broken = SAMPLE.replacen("<real>1</real>", "<real>opaque</real>", 1);
    assert_eq!(
        ItermScheme::parse(&broken),
        Err(ImportError::BadColor {
            key: "Ansi 0 Color",
            reason: "`Alpha Component` is not a number".to_owned()
        })
    );
}
