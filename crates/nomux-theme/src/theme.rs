//! A resolved theme: one value for each token, with typed access.

use std::time::Duration;

use crate::color::Rgba;
use crate::parse::Appearance;
use crate::token::{
    self, ColorToken, DurationToken, EasingToken, FontFamilyToken, FontWeightToken, LengthToken,
    TokenSpec,
};
use crate::value::{Easing, FontFamily, FontWeight, Length, Value};

/// One step of the type scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextStyle {
    /// Titles: UI font.
    Title,
    /// Rows and body text: UI font.
    Row,
    /// Metadata: UI font.
    Meta,
    /// Code and terminals: monospace font.
    Code,
}

/// The font of one step of the type scale.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontSpec<'theme> {
    /// The font families, the preferred family first.
    pub family: &'theme FontFamily,
    /// The font size.
    pub size: Length,
    /// The font weight.
    pub weight: FontWeight,
}

/// The density of the UI. The user selects it; a theme gives the metrics of
/// the two densities.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Density {
    /// More space around rows and controls.
    Comfortable,
    /// More rows on the screen.
    Compact,
}

impl Density {
    /// The two densities.
    pub const ALL: [Self; 2] = [Self::Comfortable, Self::Compact];

    /// The word in the configuration and in the token names.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Comfortable => "comfortable",
            Self::Compact => "compact",
        }
    }
}

/// The metrics of one density.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DensityMetrics {
    /// The height of a row.
    pub row_height: Length,
    /// The height of a control.
    pub control_height: Length,
    /// The horizontal padding of rows, Tabs and controls.
    pub padding: Length,
}

/// One token of a resolved theme.
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    /// The description of the token.
    pub spec: &'static TokenSpec,
    /// The value of the token.
    pub value: Value,
    /// The identifier of the theme that sets the value: the theme itself or
    /// one of its base themes.
    pub origin: String,
}

/// A theme after the resolution of `extends`: each token of the token set
/// has a value.
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    id: String,
    name: String,
    appearance: Appearance,
    entries: Vec<Entry>,
    colors: Vec<Rgba>,
    lengths: Vec<Length>,
    durations: Vec<Duration>,
    font_families: Vec<FontFamily>,
    font_weights: Vec<FontWeight>,
    easings: Vec<Easing>,
}

impl Theme {
    /// Makes a theme from one entry for each token, in the sequence of
    /// [`token::all`]. The resolution guarantees the sequence and the kinds.
    pub(crate) fn from_entries(
        id: String,
        name: String,
        appearance: Appearance,
        entries: Vec<Entry>,
    ) -> Self {
        debug_assert!(entries.iter().map(|entry| entry.spec).eq(token::all()));
        let mut theme = Self {
            id,
            name,
            appearance,
            colors: Vec::new(),
            lengths: Vec::new(),
            durations: Vec::new(),
            font_families: Vec::new(),
            font_weights: Vec::new(),
            easings: Vec::new(),
            entries: Vec::new(),
        };
        for entry in &entries {
            match &entry.value {
                Value::Color(color) => theme.colors.push(*color),
                Value::Length(length) => theme.lengths.push(*length),
                Value::Duration(duration) => theme.durations.push(*duration),
                Value::FontFamily(family) => theme.font_families.push(family.clone()),
                Value::FontWeight(weight) => theme.font_weights.push(*weight),
                Value::Easing(easing) => theme.easings.push(*easing),
            }
        }
        theme.entries = entries;
        theme
    }

    /// The identifier: the file name without `.toml`.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The name that the UI shows.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whether the theme is light or dark.
    pub fn appearance(&self) -> Appearance {
        self.appearance
    }

    /// All tokens with their values, in the sequence of the token reference.
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// The value of a colour token.
    pub fn color(&self, token: ColorToken) -> Rgba {
        self.colors[token as usize]
    }

    /// The value of a length token.
    pub fn length(&self, token: LengthToken) -> Length {
        self.lengths[token as usize]
    }

    /// The value of a duration token.
    pub fn duration(&self, token: DurationToken) -> Duration {
        self.durations[token as usize]
    }

    /// The value of a font family token.
    pub fn font_family(&self, token: FontFamilyToken) -> &FontFamily {
        &self.font_families[token as usize]
    }

    /// The value of a font weight token.
    pub fn font_weight(&self, token: FontWeightToken) -> FontWeight {
        self.font_weights[token as usize]
    }

    /// The value of an easing token.
    pub fn easing(&self, token: EasingToken) -> Easing {
        self.easings[token as usize]
    }

    /// The font of one step of the type scale.
    pub fn font(&self, style: TextStyle) -> FontSpec<'_> {
        let (family, size, weight) = match style {
            TextStyle::Title => (
                FontFamilyToken::Ui,
                LengthToken::FontTitleSize,
                FontWeightToken::FontTitleWeight,
            ),
            TextStyle::Row => (
                FontFamilyToken::Ui,
                LengthToken::FontRowSize,
                FontWeightToken::FontRowWeight,
            ),
            TextStyle::Meta => (
                FontFamilyToken::Ui,
                LengthToken::FontMetaSize,
                FontWeightToken::FontMetaWeight,
            ),
            TextStyle::Code => (
                FontFamilyToken::Mono,
                LengthToken::FontCodeSize,
                FontWeightToken::FontCodeWeight,
            ),
        };
        FontSpec {
            family: self.font_family(family),
            size: self.length(size),
            weight: self.font_weight(weight),
        }
    }

    /// The metrics of one density.
    pub fn density(&self, density: Density) -> DensityMetrics {
        let (row_height, control_height, padding) = match density {
            Density::Comfortable => (
                LengthToken::DensityComfortableRowHeight,
                LengthToken::DensityComfortableControlHeight,
                LengthToken::DensityComfortablePadding,
            ),
            Density::Compact => (
                LengthToken::DensityCompactRowHeight,
                LengthToken::DensityCompactControlHeight,
                LengthToken::DensityCompactPadding,
            ),
        };
        DensityMetrics {
            row_height: self.length(row_height),
            control_height: self.length(control_height),
            padding: self.length(padding),
        }
    }
}
