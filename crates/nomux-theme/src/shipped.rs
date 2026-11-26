//! The themes that the repository ships, embedded in the crate. The feature
//! `shipped` (on by default) adds this module.
//!
//! ```
//! use nomux_theme::{Appearance, ColorToken, shipped};
//!
//! let theme = shipped::theme("nomux-dark").unwrap();
//! assert_eq!(theme.appearance(), Appearance::Dark);
//! let surface = theme.color(ColorToken::SurfaceBase);
//!
//! let light: Vec<&str> = shipped::themes()
//!     .iter()
//!     .filter(|theme| theme.appearance() == Appearance::Light)
//!     .map(|theme| theme.id())
//!     .collect();
//! assert!(light.contains(&"nomux-light"));
//! # let _ = surface;
//! ```

use std::sync::OnceLock;

use crate::{Theme, ThemeSet};

/// The identifier and the text of each theme file in the `themes/` directory
/// of the crate, sorted by identifier. To add themes of a user that extend a
/// shipped theme, insert these files and the files of the user into one
/// [`ThemeSet`].
pub const FILES: [(&str, &str); 7] = [
    ("base-dark", include_str!("../themes/base-dark.toml")),
    ("base-light", include_str!("../themes/base-light.toml")),
    (
        "catppuccin-mocha",
        include_str!("../themes/catppuccin-mocha.toml"),
    ),
    ("gruvbox-dark", include_str!("../themes/gruvbox-dark.toml")),
    ("nomux-dark", include_str!("../themes/nomux-dark.toml")),
    ("nomux-light", include_str!("../themes/nomux-light.toml")),
    ("tokyo-night", include_str!("../themes/tokyo-night.toml")),
];

/// Each shipped theme with `extends` resolved, sorted by identifier. The
/// first call parses and resolves the files. The other calls return the same
/// themes.
pub fn themes() -> &'static [Theme] {
    static THEMES: OnceLock<Vec<Theme>> = OnceLock::new();
    THEMES.get_or_init(|| {
        let mut set = ThemeSet::new();
        for (id, text) in FILES {
            set.insert(id, text)
                .expect("the test of the shipped themes parses each file");
        }
        set.resolve_all()
            .expect("the test of the shipped themes resolves each file")
    })
}

/// The shipped theme with this identifier.
pub fn theme(id: &str) -> Option<&'static Theme> {
    themes().iter().find(|theme| theme.id() == id)
}
