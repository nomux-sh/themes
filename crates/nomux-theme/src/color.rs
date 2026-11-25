//! Colours and the colour maths of the contrast rules.

use std::fmt;

/// A colour in the sRGB colour space with straight (not premultiplied) alpha.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rgba {
    /// Red channel.
    pub red: u8,
    /// Green channel.
    pub green: u8,
    /// Blue channel.
    pub blue: u8,
    /// Alpha channel: 0 is transparent, 255 is opaque.
    pub alpha: u8,
}

/// The reason why a text is not a colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSyntaxError {
    /// The text does not start with `#`.
    MissingHash,
    /// The number of hexadecimal digits is not 3, 4, 6 or 8.
    Length(usize),
    /// A character after `#` is not a hexadecimal digit.
    Digit(char),
}

impl fmt::Display for ColorSyntaxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHash => formatter.write_str("a colour starts with `#`"),
            Self::Length(length) => {
                write!(
                    formatter,
                    "a colour has 3, 4, 6 or 8 hexadecimal digits, not {length}"
                )
            }
            Self::Digit(digit) => write!(formatter, "`{digit}` is not a hexadecimal digit"),
        }
    }
}

impl std::error::Error for ColorSyntaxError {}

impl Rgba {
    /// Makes an opaque colour.
    pub const fn opaque(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    /// Parses `#rgb`, `#rgba`, `#rrggbb` or `#rrggbbaa`. The digits can be
    /// uppercase or lowercase.
    pub fn parse_hex(text: &str) -> Result<Self, ColorSyntaxError> {
        let digits = text
            .strip_prefix('#')
            .ok_or(ColorSyntaxError::MissingHash)?;
        let nibbles = digits
            .chars()
            .map(|digit| {
                digit
                    .to_digit(16)
                    .map(|value| value as u8)
                    .ok_or(ColorSyntaxError::Digit(digit))
            })
            .collect::<Result<Vec<u8>, _>>()?;
        let channels: Vec<u8> = match nibbles.len() {
            3 | 4 => nibbles.iter().map(|nibble| nibble * 17).collect(),
            6 | 8 => nibbles
                .chunks(2)
                .map(|pair| pair[0] * 16 + pair[1])
                .collect(),
            length => return Err(ColorSyntaxError::Length(length)),
        };
        Ok(Self {
            red: channels[0],
            green: channels[1],
            blue: channels[2],
            alpha: channels.get(3).copied().unwrap_or(255),
        })
    }

    /// Returns `true` when the colour has no transparency.
    pub const fn is_opaque(self) -> bool {
        self.alpha == 255
    }

    /// Returns the same colour with a different alpha channel.
    pub const fn with_alpha(self, alpha: u8) -> Self {
        Self { alpha, ..self }
    }

    /// Paints this colour over `background` (the "source over" operator).
    pub fn over(self, background: Self) -> Self {
        let source_alpha = f64::from(self.alpha) / 255.0;
        let background_alpha = f64::from(background.alpha) / 255.0 * (1.0 - source_alpha);
        let alpha = source_alpha + background_alpha;
        if alpha == 0.0 {
            return Self {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 0,
            };
        }
        let blend = |source: u8, background: u8| {
            let value = (f64::from(source) * source_alpha
                + f64::from(background) * background_alpha)
                / alpha;
            value.round() as u8
        };
        Self {
            red: blend(self.red, background.red),
            green: blend(self.green, background.green),
            blue: blend(self.blue, background.blue),
            alpha: (alpha * 255.0).round() as u8,
        }
    }

    /// The relative luminance of WCAG 2.x: 0 is black and 1 is white. The
    /// alpha channel is ignored. Paint a transparent colour [`over`](Self::over)
    /// its background first.
    pub fn relative_luminance(self) -> f64 {
        let linear = |channel: u8| {
            let value = f64::from(channel) / 255.0;
            if value <= 0.03928 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * linear(self.red) + 0.7152 * linear(self.green) + 0.0722 * linear(self.blue)
    }
}

impl fmt::Display for Rgba {
    /// Writes `#rrggbb`, or `#rrggbbaa` when the colour is not opaque.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "#{:02x}{:02x}{:02x}",
            self.red, self.green, self.blue
        )?;
        if !self.is_opaque() {
            write!(formatter, "{:02x}", self.alpha)?;
        }
        Ok(())
    }
}

/// The contrast ratio of WCAG 2.x between two opaque colours: from 1 (the
/// same luminance) to 21 (black and white). The sequence of the arguments
/// does not change the result.
///
/// The alpha channels are ignored. To get the ratio of a transparent colour,
/// paint it [`over`](Rgba::over) its background first.
pub fn contrast_ratio(first: Rgba, second: Rgba) -> f64 {
    let first = first.relative_luminance();
    let second = second.relative_luminance();
    (first.max(second) + 0.05) / (first.min(second) + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_each_length_of_a_hexadecimal_colour() {
        assert_eq!(Rgba::parse_hex("#fff"), Ok(Rgba::opaque(255, 255, 255)));
        assert_eq!(
            Rgba::parse_hex("#1a2B3c"),
            Ok(Rgba::opaque(0x1a, 0x2b, 0x3c))
        );
        assert_eq!(
            Rgba::parse_hex("#f008"),
            Ok(Rgba {
                red: 255,
                green: 0,
                blue: 0,
                alpha: 0x88
            })
        );
        assert_eq!(
            Rgba::parse_hex("#0000000f"),
            Ok(Rgba {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 15
            })
        );
    }

    #[test]
    fn rejects_text_that_is_not_a_colour() {
        assert_eq!(
            Rgba::parse_hex("16181c"),
            Err(ColorSyntaxError::MissingHash)
        );
        assert_eq!(Rgba::parse_hex(""), Err(ColorSyntaxError::MissingHash));
        assert_eq!(Rgba::parse_hex("#"), Err(ColorSyntaxError::Length(0)));
        assert_eq!(Rgba::parse_hex("#12345"), Err(ColorSyntaxError::Length(5)));
        assert_eq!(
            Rgba::parse_hex("#16181g"),
            Err(ColorSyntaxError::Digit('g'))
        );
        assert_eq!(Rgba::parse_hex("#ééé"), Err(ColorSyntaxError::Digit('é')));
        assert_eq!(Rgba::parse_hex("# fff"), Err(ColorSyntaxError::Digit(' ')));
    }

    #[test]
    fn writes_the_alpha_channel_only_when_the_colour_is_transparent() {
        assert_eq!(Rgba::opaque(0x16, 0x18, 0x1c).to_string(), "#16181c");
        assert_eq!(
            Rgba::opaque(0, 0, 0).with_alpha(0x59).to_string(),
            "#00000059"
        );
    }

    fn ratio(first: &str, second: &str) -> f64 {
        contrast_ratio(
            Rgba::parse_hex(first).unwrap(),
            Rgba::parse_hex(second).unwrap(),
        )
    }

    // The reference values are the published results of the WCAG 2.x formula.
    // #767676 is the lightest grey that passes 4.5:1 on white, and #777777 is
    // the first grey that fails.
    #[test]
    fn contrast_ratio_agrees_with_known_wcag_values() {
        assert!((ratio("#000000", "#ffffff") - 21.0).abs() < 1e-9);
        assert!((ratio("#ffffff", "#ffffff") - 1.0).abs() < 1e-9);
        assert!((ratio("#767676", "#ffffff") - 4.54).abs() < 0.005);
        assert!((ratio("#777777", "#ffffff") - 4.48).abs() < 0.005);
        assert!((ratio("#ff0000", "#ffffff") - 4.00).abs() < 0.005);
        assert!((ratio("#0000ff", "#ffffff") - 8.59).abs() < 0.005);
        assert_eq!(ratio("#16181c", "#9aa1ab"), ratio("#9aa1ab", "#16181c"));
    }

    #[test]
    fn paints_a_transparent_colour_over_its_background() {
        let white = Rgba::opaque(255, 255, 255);
        let black = Rgba::opaque(0, 0, 0);
        assert_eq!(white.with_alpha(0).over(black), black);
        assert_eq!(white.over(black), white);
        assert_eq!(
            white.with_alpha(128).over(black),
            Rgba::opaque(128, 128, 128)
        );
        assert_eq!(white.with_alpha(0).over(black.with_alpha(0)).alpha, 0);
    }
}
