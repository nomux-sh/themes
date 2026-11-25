//! The theme loader of [nomux](https://github.com/nomux-sh).
//!
//! The nomux UI reads one set of semantic design tokens ([`token`]). A theme
//! is a TOML file that gives values to tokens. A token that a theme does not
//! set comes from the base theme that it `extends`.
//!
//! ```no_run
//! use std::path::Path;
//!
//! use nomux_theme::{ColorToken, TextStyle, ThemeSet, contrast_ratio, rules};
//!
//! let themes = ThemeSet::load_dir(Path::new("themes"))?;
//! let theme = themes.resolve("nomux-dark")?;
//!
//! let text = theme.color(ColorToken::TextMuted);
//! let surface = theme.color(ColorToken::SurfaceBase);
//! assert!(contrast_ratio(text, surface) >= rules::TEXT_CONTRAST);
//!
//! let title = theme.font(TextStyle::Title);
//! println!("{} at {}", title.family, title.size);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! The parts of the crate:
//!
//! - [`token`]: the token set.
//! - [`parse`], [`validate`], [`ThemeSet`]: a theme file to a resolved [`Theme`].
//! - [`Theme`]: typed access to the values.
//! - [`Rgba`], [`contrast_ratio`]: the colour maths of WCAG 2.x.
//! - [`rules`]: the visual rules that each theme must obey.
//! - [`ItermScheme`]: import of an iTerm colour scheme.

// Compiles and runs the examples of the README as documentation tests.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeExamples;

mod color;
mod iterm;
pub mod parse;
mod problem;
mod resolve;
pub mod rules;
mod theme;
pub mod token;
pub mod validate;
mod value;

pub use color::{ColorSyntaxError, Rgba, contrast_ratio};
pub use iterm::{ImportError, ItermScheme};
pub use parse::Appearance;
pub use problem::{LoadError, Problem, ThemeError};
pub use resolve::ThemeSet;
pub use theme::{Density, DensityMetrics, Entry, FontSpec, TextStyle, Theme};
pub use token::{
    ColorToken, DurationToken, EasingToken, FontFamilyToken, FontWeightToken, LengthToken,
    TokenSpec,
};
pub use value::{Easing, FontFamily, FontWeight, Length, Value, ValueKind};
