//! The visual rules that each theme must obey: contrast, opaque surfaces
//! that differ by tone steps, neutral fills, the 4 px grid and the motion
//! range.

use std::fmt;
use std::time::Duration;

use crate::color::{Rgba, contrast_ratio};
use crate::theme::Theme;
use crate::token::{ColorToken, DurationToken, LengthToken};
use crate::value::Length;

/// The minimum contrast ratio of text.
pub const TEXT_CONTRAST: f64 = 4.5;
/// The minimum contrast ratio of a mark: a status mark, a focus ring, a
/// gutter bar or a cursor.
pub const MARK_CONTRAST: f64 = 3.0;
/// The spacing grid in logical pixels.
pub const GRID: f32 = 4.0;
/// The shortest and the longest duration of a transition.
pub const MOTION_RANGE: (Duration, Duration) =
    (Duration::from_millis(120), Duration::from_millis(200));
/// The smallest and the largest alpha of a hover or selection fill, in percent.
pub const FILL_ALPHA_PERCENT: (u32, u32) = (4, 8);

const SURFACES: [ColorToken; 5] = [
    ColorToken::SurfaceSunken,
    ColorToken::SurfaceBase,
    ColorToken::SurfaceButton,
    ColorToken::SurfaceRaised,
    ColorToken::SurfaceFloating,
];
/// A surface and the surface that it must differ from, because the UI puts
/// the first one on the second one or next to it.
const TONE_STEPS: [(ColorToken, ColorToken); 6] = [
    (ColorToken::SurfaceSunken, ColorToken::SurfaceBase),
    (ColorToken::SurfaceRaised, ColorToken::SurfaceBase),
    (ColorToken::SurfaceButton, ColorToken::SurfaceSunken),
    (ColorToken::SurfaceButton, ColorToken::SurfaceBase),
    (ColorToken::SurfaceButton, ColorToken::SurfaceRaised),
    (ColorToken::SurfaceFloating, ColorToken::SurfaceBase),
];
const TEXT_LEVELS: [ColorToken; 3] = [
    ColorToken::TextPrimary,
    ColorToken::TextSecondary,
    ColorToken::TextMuted,
];
const FILLS: [ColorToken; 2] = [ColorToken::FillHover, ColorToken::FillSelected];
const MARKS: [ColorToken; 7] = [
    ColorToken::StateWorking,
    ColorToken::StateNeedsYou,
    ColorToken::StateDone,
    ColorToken::StateError,
    ColorToken::StateIdle,
    ColorToken::Accent,
    ColorToken::FocusRing,
];
const SYNTAX: [ColorToken; 6] = [
    ColorToken::SyntaxComment,
    ColorToken::SyntaxKeyword,
    ColorToken::SyntaxString,
    ColorToken::SyntaxConstant,
    ColorToken::SyntaxFunction,
    ColorToken::SyntaxType,
];
const DIFF_BACKGROUNDS: [ColorToken; 3] = [
    ColorToken::DiffAddedBackground,
    ColorToken::DiffRemovedBackground,
    ColorToken::DiffChangedBackground,
];
const DIFF_GUTTERS: [ColorToken; 3] = [
    ColorToken::DiffAddedGutter,
    ColorToken::DiffRemovedGutter,
    ColorToken::DiffChangedGutter,
];
const SPACING: [LengthToken; 12] = [
    LengthToken::Space1,
    LengthToken::Space2,
    LengthToken::Space3,
    LengthToken::Space4,
    LengthToken::DensityComfortableRowHeight,
    LengthToken::DensityComfortableControlHeight,
    LengthToken::DensityComfortablePadding,
    LengthToken::DensityComfortableTwoLineRowHeight,
    LengthToken::DensityCompactRowHeight,
    LengthToken::DensityCompactControlHeight,
    LengthToken::DensityCompactPadding,
    LengthToken::DensityCompactTwoLineRowHeight,
];

/// One pair of colours that must have a minimum contrast, with the ratio
/// that the theme gives.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContrastCheck {
    /// The text or the mark.
    pub foreground: ColorToken,
    /// The opaque colour below.
    pub background: ColorToken,
    /// A transparent colour that is painted over the background first.
    pub overlay: Option<ColorToken>,
    /// The contrast ratio between the foreground and what is below it.
    pub ratio: f64,
    /// The ratio that the rule requires.
    pub minimum: f64,
}

impl ContrastCheck {
    /// Returns `true` when the ratio is sufficient.
    pub fn passes(&self) -> bool {
        self.ratio >= self.minimum
    }
}

impl fmt::Display for ContrastCheck {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "`{}` on `{}`",
            self.foreground.name(),
            self.background.name()
        )?;
        if let Some(overlay) = self.overlay {
            write!(formatter, " with `{}`", overlay.name())?;
        }
        write!(
            formatter,
            " has a contrast of {:.2}:1, the minimum is {}:1",
            self.ratio, self.minimum
        )
    }
}

/// One rule that a theme does not obey.
#[derive(Clone, Debug, PartialEq)]
pub enum Violation {
    /// A pair of colours has too little contrast.
    Contrast(ContrastCheck),
    /// A surface or a text level is transparent.
    NotOpaque {
        /// The token.
        token: ColorToken,
    },
    /// Two surfaces that the UI puts on each other have the same colour.
    NoToneStep {
        /// The surface above.
        surface: ColorToken,
        /// The surface below.
        below: ColorToken,
    },
    /// A hover or selection fill is not a tint of `text.primary` in the
    /// permitted alpha range.
    NotTextTint {
        /// The token.
        token: ColorToken,
        /// The value of the token.
        value: Rgba,
    },
    /// A spacing length is not a multiple of the grid.
    OffGrid {
        /// The token.
        token: LengthToken,
        /// The value of the token.
        value: Length,
    },
    /// A duration is not in the motion range.
    MotionOutOfRange {
        /// The token.
        token: DurationToken,
        /// The value of the token.
        value: Duration,
    },
}

impl fmt::Display for Violation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contrast(check) => check.fmt(formatter),
            Self::NotOpaque { token } => {
                write!(
                    formatter,
                    "`{}` must be opaque. Change the colour, not the alpha",
                    token.name()
                )
            }
            Self::NoToneStep { surface, below } => write!(
                formatter,
                "`{}` is equal to `{}`. The two surfaces must differ by a tone step",
                surface.name(),
                below.name()
            ),
            Self::NotTextTint { token, value } => write!(
                formatter,
                "`{}` is {value}. It must be `text.primary` with an alpha from {}% to {}%",
                token.name(),
                FILL_ALPHA_PERCENT.0,
                FILL_ALPHA_PERCENT.1
            ),
            Self::OffGrid { token, value } => {
                write!(
                    formatter,
                    "`{}` is {value}. It must be a multiple of {GRID}px",
                    token.name()
                )
            }
            Self::MotionOutOfRange { token, value } => write!(
                formatter,
                "`{}` is {}ms. It must be from {}ms to {}ms",
                token.name(),
                value.as_millis(),
                MOTION_RANGE.0.as_millis(),
                MOTION_RANGE.1.as_millis()
            ),
        }
    }
}

/// Each contrast pair of the rules with its ratio in this theme, the pairs
/// that pass included.
pub fn contrast_checks(theme: &Theme) -> Vec<ContrastCheck> {
    let mut pairs = Vec::new();
    for surface in SURFACES {
        for overlay in [None, Some(FILLS[0]), Some(FILLS[1])] {
            pairs.extend(TEXT_LEVELS.map(|text| (text, surface, overlay, TEXT_CONTRAST)));
            pairs.extend(MARKS.map(|mark| (mark, surface, overlay, MARK_CONTRAST)));
        }
    }
    pairs.push((
        ColorToken::TextOnAccent,
        ColorToken::Accent,
        None,
        TEXT_CONTRAST,
    ));

    let terminal = ColorToken::TerminalBackground;
    pairs.push((
        ColorToken::TerminalForeground,
        terminal,
        None,
        TEXT_CONTRAST,
    ));
    pairs.push((
        ColorToken::TerminalForeground,
        terminal,
        Some(ColorToken::TerminalSelection),
        TEXT_CONTRAST,
    ));
    pairs.push((ColorToken::TerminalCursor, terminal, None, MARK_CONTRAST));

    let base = ColorToken::SurfaceBase;
    pairs.extend(SYNTAX.map(|syntax| (syntax, base, None, TEXT_CONTRAST)));
    pairs.extend(
        DIFF_BACKGROUNDS.map(|diff| (ColorToken::TextPrimary, base, Some(diff), TEXT_CONTRAST)),
    );
    pairs.extend(DIFF_GUTTERS.map(|gutter| (gutter, base, None, MARK_CONTRAST)));

    pairs
        .into_iter()
        .map(|(foreground, background, overlay, minimum)| {
            let mut below = theme.color(background);
            if let Some(overlay) = overlay {
                below = theme.color(overlay).over(below);
            }
            let ratio = contrast_ratio(theme.color(foreground).over(below), below);
            ContrastCheck {
                foreground,
                background,
                overlay,
                ratio,
                minimum,
            }
        })
        .collect()
}

/// The rules that this theme does not obey. The list is empty when the
/// theme is correct.
pub fn violations(theme: &Theme) -> Vec<Violation> {
    let mut violations = Vec::new();

    let opaque = SURFACES
        .into_iter()
        .chain(TEXT_LEVELS)
        .chain([ColorToken::TerminalBackground]);
    violations.extend(
        opaque
            .filter(|token| !theme.color(*token).is_opaque())
            .map(|token| Violation::NotOpaque { token }),
    );

    let same_tone = TONE_STEPS
        .into_iter()
        .filter(|(surface, below)| theme.color(*surface) == theme.color(*below));
    violations.extend(same_tone.map(|(surface, below)| Violation::NoToneStep { surface, below }));

    let text = theme.color(ColorToken::TextPrimary);
    for token in FILLS {
        let value = theme.color(token);
        let percent = (f64::from(value.alpha) / 255.0 * 100.0).round() as u32;
        let in_range = (FILL_ALPHA_PERCENT.0..=FILL_ALPHA_PERCENT.1).contains(&percent);
        if value.with_alpha(255) != text.with_alpha(255) || !in_range {
            violations.push(Violation::NotTextTint { token, value });
        }
    }

    for token in SPACING {
        let value = theme.length(token);
        if value.pixels() % GRID != 0.0 {
            violations.push(Violation::OffGrid { token, value });
        }
    }

    for token in DurationToken::ALL {
        let value = theme.duration(*token);
        if value < MOTION_RANGE.0 || value > MOTION_RANGE.1 {
            violations.push(Violation::MotionOutOfRange {
                token: *token,
                value,
            });
        }
    }

    let contrast = contrast_checks(theme)
        .into_iter()
        .filter(|check| !check.passes());
    violations.extend(contrast.map(Violation::Contrast));
    violations
}
