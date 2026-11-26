//! The themes of the `themes/` directory, and the rules with a theme that
//! breaks them.

use std::path::Path;

use nomux_theme::rules::{self, Violation};
use nomux_theme::{ColorToken, DurationToken, LengthToken, ThemeSet};

fn shipped() -> ThemeSet {
    ThemeSet::load_dir(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../themes")).unwrap()
}

#[test]
fn each_shipped_theme_resolves_and_obeys_the_rules() {
    let themes = shipped().resolve_all().unwrap();
    for id in ["base-dark", "base-light", "nomux-dark", "nomux-light"] {
        assert!(
            themes.iter().any(|theme| theme.id() == id),
            "{id} is missing"
        );
    }
    for theme in &themes {
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
    let mut themes = shipped();
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
