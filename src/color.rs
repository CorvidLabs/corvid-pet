//! Colored output support for corvid pets.
//!
//! This module provides ANSI color support when the `color` feature is enabled.
//! Users can customize colors via `ColorScheme`, or use species defaults.

/// A named color for the pet's ASCII art.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PetColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl PetColor {
    /// All available colors.
    pub const ALL: &[PetColor] = &[
        PetColor::Black,
        PetColor::Red,
        PetColor::Green,
        PetColor::Yellow,
        PetColor::Blue,
        PetColor::Magenta,
        PetColor::Cyan,
        PetColor::White,
        PetColor::BrightBlack,
        PetColor::BrightRed,
        PetColor::BrightGreen,
        PetColor::BrightYellow,
        PetColor::BrightBlue,
        PetColor::BrightMagenta,
        PetColor::BrightCyan,
        PetColor::BrightWhite,
    ];

    /// Returns a random color.
    pub fn random() -> Self {
        use rand::seq::SliceRandom;
        *Self::ALL.choose(&mut rand::thread_rng()).unwrap()
    }
}

impl std::fmt::Display for PetColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PetColor::Black => write!(f, "black"),
            PetColor::Red => write!(f, "red"),
            PetColor::Green => write!(f, "green"),
            PetColor::Yellow => write!(f, "yellow"),
            PetColor::Blue => write!(f, "blue"),
            PetColor::Magenta => write!(f, "magenta"),
            PetColor::Cyan => write!(f, "cyan"),
            PetColor::White => write!(f, "white"),
            PetColor::BrightBlack => write!(f, "bright-black"),
            PetColor::BrightRed => write!(f, "bright-red"),
            PetColor::BrightGreen => write!(f, "bright-green"),
            PetColor::BrightYellow => write!(f, "bright-yellow"),
            PetColor::BrightBlue => write!(f, "bright-blue"),
            PetColor::BrightMagenta => write!(f, "bright-magenta"),
            PetColor::BrightCyan => write!(f, "bright-cyan"),
            PetColor::BrightWhite => write!(f, "bright-white"),
        }
    }
}

impl std::str::FromStr for PetColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "black" => Ok(PetColor::Black),
            "red" => Ok(PetColor::Red),
            "green" => Ok(PetColor::Green),
            "yellow" => Ok(PetColor::Yellow),
            "blue" => Ok(PetColor::Blue),
            "magenta" | "purple" => Ok(PetColor::Magenta),
            "cyan" => Ok(PetColor::Cyan),
            "white" => Ok(PetColor::White),
            "bright-black" | "bright_black" | "gray" | "grey" => Ok(PetColor::BrightBlack),
            "bright-red" | "bright_red" => Ok(PetColor::BrightRed),
            "bright-green" | "bright_green" => Ok(PetColor::BrightGreen),
            "bright-yellow" | "bright_yellow" => Ok(PetColor::BrightYellow),
            "bright-blue" | "bright_blue" => Ok(PetColor::BrightBlue),
            "bright-magenta" | "bright_magenta" | "bright-purple" | "bright_purple" => {
                Ok(PetColor::BrightMagenta)
            }
            "bright-cyan" | "bright_cyan" => Ok(PetColor::BrightCyan),
            "bright-white" | "bright_white" => Ok(PetColor::BrightWhite),
            _ => Err(format!("Unknown color: {}", s)),
        }
    }
}

/// User-configurable color scheme for a pet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorScheme {
    /// Color for the bird's body.
    pub body: PetColor,
    /// Color for the thought bubble text.
    pub bubble: PetColor,
}

impl ColorScheme {
    /// Creates a new color scheme.
    pub fn new(body: PetColor, bubble: PetColor) -> Self {
        Self { body, bubble }
    }

    /// Returns the default color scheme for a species.
    pub fn default_for(_species: crate::Species) -> Self {
        // Crow default: blue body, cyan bubble
        Self {
            body: PetColor::Blue,
            bubble: PetColor::Cyan,
        }
    }

    /// Returns a random color scheme.
    pub fn random() -> Self {
        Self {
            body: PetColor::random(),
            bubble: PetColor::random(),
        }
    }
}

/// Applies ANSI colors to ASCII art using the given color scheme.
///
/// When the `color` feature is enabled, this returns a colored string.
/// When disabled, returns the art unchanged.
#[cfg(feature = "color")]
pub fn colorize(art: &str, species: crate::Species) -> String {
    colorize_with_scheme(art, &ColorScheme::default_for(species))
}

/// No-op when color feature is disabled.
#[cfg(not(feature = "color"))]
pub fn colorize(art: &str, _species: crate::Species) -> String {
    art.to_string()
}

/// Applies ANSI colors using a custom color scheme.
#[cfg(feature = "color")]
pub fn colorize_with_scheme(art: &str, scheme: &ColorScheme) -> String {
    art.lines()
        .map(|line| {
            if let (Some(start), Some(end)) = (line.find(".oO("), line.rfind(')')) {
                let bird_part = apply_color(&line[..start], scheme.body);
                let bubble = apply_color(&line[start..=end], scheme.bubble);
                let after = apply_color(&line[end + 1..], scheme.body);
                format!("{bird_part}{bubble}{after}")
            } else {
                apply_color(line, scheme.body).to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// No-op when color feature is disabled.
#[cfg(not(feature = "color"))]
pub fn colorize_with_scheme(art: &str, _scheme: &ColorScheme) -> String {
    art.to_string()
}

#[cfg(feature = "color")]
fn apply_color(text: &str, color: PetColor) -> colored::ColoredString {
    use colored::Colorize as _;

    match color {
        PetColor::Black => text.black(),
        PetColor::Red => text.red(),
        PetColor::Green => text.green(),
        PetColor::Yellow => text.yellow(),
        PetColor::Blue => text.blue(),
        PetColor::Magenta => text.magenta(),
        PetColor::Cyan => text.cyan(),
        PetColor::White => text.white(),
        PetColor::BrightBlack => text.bright_black(),
        PetColor::BrightRed => text.bright_red(),
        PetColor::BrightGreen => text.bright_green(),
        PetColor::BrightYellow => text.bright_yellow(),
        PetColor::BrightBlue => text.bright_blue(),
        PetColor::BrightMagenta => text.bright_magenta(),
        PetColor::BrightCyan => text.bright_cyan(),
        PetColor::BrightWhite => text.bright_white(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Species;

    #[test]
    fn test_colorize_returns_string() {
        let art = "test art";
        let result = colorize(art, Species::Crow);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_colorize_preserves_structure() {
        let art = "line1\nline2\nline3";
        let result = colorize(art, Species::Crow);
        assert_eq!(result.lines().count(), 3);
    }

    #[test]
    fn test_color_scheme_default() {
        let scheme = ColorScheme::default_for(Species::Crow);
        assert_eq!(scheme.body, PetColor::Blue);
        assert_eq!(scheme.bubble, PetColor::Cyan);
    }

    #[test]
    fn test_color_scheme_random() {
        let scheme = ColorScheme::random();
        // Just verify it doesn't panic and produces valid colors
        assert!(PetColor::ALL.contains(&scheme.body));
        assert!(PetColor::ALL.contains(&scheme.bubble));
    }

    #[test]
    fn test_pet_color_parse() {
        assert_eq!("red".parse::<PetColor>().unwrap(), PetColor::Red);
        assert_eq!("purple".parse::<PetColor>().unwrap(), PetColor::Magenta);
        assert_eq!("gray".parse::<PetColor>().unwrap(), PetColor::BrightBlack);
        assert!("invalid".parse::<PetColor>().is_err());
    }

    #[test]
    fn test_pet_color_display() {
        assert_eq!(format!("{}", PetColor::Red), "red");
        assert_eq!(format!("{}", PetColor::BrightBlue), "bright-blue");
    }

    #[test]
    fn test_colorize_with_scheme_preserves_structure() {
        let art = "line1\nline2\nline3";
        let scheme = ColorScheme::new(PetColor::Red, PetColor::Green);
        let result = colorize_with_scheme(art, &scheme);
        assert_eq!(result.lines().count(), 3);
    }
}
