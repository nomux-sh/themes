//! The themes that the crate ships, and the rules with a theme that breaks
//! them.
#![cfg(feature = "shipped")]

use std::fs;
use std::path::Path;

use nomux_theme::rules::{self, Violation};
use nomux_theme::{Appearance, ColorToken, DurationToken, LengthToken, Theme, ThemeSet, shipped};

fn shipped_set() -> ThemeSet {
    let mut set = ThemeSet::new();
    for (id, text) in shipped::FILES {
        set.insert(id, text).unwrap();
    }
    set
}

#[test]
fn the_embedded_files_are_the_files_of_the_themes_directory() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("themes");
    let mut on_disk: Vec<String> = fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "toml")
        })
        .map(|path| path.file_stem().unwrap().to_string_lossy().into_owned())
        .collect();
    on_disk.sort();
    let embedded: Vec<&str> = shipped::FILES.iter().map(|(id, _)| *id).collect();
    assert_eq!(on_disk, embedded);
}

#[test]
fn each_shipped_theme_resolves_and_obeys_the_rules() {
    let ids: Vec<&str> = shipped::themes().iter().map(Theme::id).collect();
    let embedded: Vec<&str> = shipped::FILES.iter().map(|(id, _)| *id).collect();
    assert_eq!(ids, embedded);
    for (id, appearance) in [
        ("base-dark", Appearance::Dark),
        ("base-light", Appearance::Light),
        ("nomux-dark", Appearance::Dark),
        ("nomux-light", Appearance::Light),
    ] {
        assert_eq!(
            shipped::theme(id).map(Theme::appearance),
            Some(appearance),
            "{id}"
        );
    }
    assert!(shipped::theme("missing").is_none());
    for theme in shipped::themes() {
        let violations: Vec<String> = rules::violations(theme)
            .iter()
            .map(ToString::to_string)
            .collect();
        assert!(
            violations.is_empty(),
            "{}:\n{}",
            theme.id(),
            violations.join("\n")
        );
    }
}

#[test]
fn a_theme_that_breaks_the_rules_gets_one_violation_for_each_rule() {
    let mut themes = shipped_set();
    themes
        .insert(
            "broken",
            r##"
            [theme]
            name = "Broken"
            appearance = "dark"
            extends = "nomux-dark"

            [tokens]
            "text.muted" = "#6b727c"
            "state.idle" = "#3a3f47"
            "surface.raised" = "#1c1f24cc"
            "surface.button" = "#16181c"
            "fill.hover" = "#6b8afd0d"
            "fill.selected" = "#e6e8eb40"
            "fill.level" = "#6b8afd03"
            "space.2" = "6px"
            "density.compact.two-line-row-height" = "34px"
            "motion.slow" = "400ms"
            "motion.indicator" = "200ms"
            "##,
        )
        .unwrap();
    let violations = rules::violations(&themes.resolve("broken").unwrap());

    let contrast_of = |token| {
        violations.iter().any(|violation| matches!(violation, Violation::Contrast(check) if check.foreground == token))
    };
    assert!(contrast_of(ColorToken::TextMuted));
    assert!(contrast_of(ColorToken::StateIdle));
    assert!(!contrast_of(ColorToken::TextPrimary));
    assert!(violations.contains(&Violation::NotOpaque {
        token: ColorToken::SurfaceRaised
    }));
    assert!(violations.contains(&Violation::NoToneStep {
        surface: ColorToken::SurfaceButton,
        below: ColorToken::SurfaceBase
    }));
    for fill in [
        ColorToken::FillHover,
        ColorToken::FillSelected,
        ColorToken::FillLevel,
    ] {
        assert!(violations.iter().any(
            |violation| matches!(violation, Violation::NotTextTint { token, .. } if *token == fill)
        ));
    }
    assert!(
        violations
            .iter()
            .any(|violation| matches!(violation, Violation::LevelStep { level: 1, .. }))
    );
    assert!(violations.iter().any(|violation| matches!(
        violation,
        Violation::OffGrid {
            token: LengthToken::Space2,
            ..
        }
    )));
    assert!(violations.iter().any(|violation| matches!(
        violation,
        Violation::OffGrid {
            token: LengthToken::DensityCompactTwoLineRowHeight,
            ..
        }
    )));
    assert!(violations.iter().any(|violation| matches!(
        violation,
        Violation::MotionOutOfRange {
            token: DurationToken::MotionSlow,
            ..
        }
    )));
    assert!(violations.iter().any(|violation| matches!(
        violation,
        Violation::MotionOutOfRange {
            token: DurationToken::MotionIndicator,
            ..
        }
    )));
}
