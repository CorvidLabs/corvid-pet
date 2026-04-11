//! Colored output support for corvid pets.
//!
//! This module provides ANSI color support when the `color` feature is enabled.
//! Each species has distinctive colors:
//! - Crow: Gray/Black with white beak
//! - Raven: Purple/Black with silver accents
//! - Magpie: Black/White with blue-green iridescence
//! - Jay: Blue/Crest colors

/// Applies species-appropriate ANSI colors to ASCII art.
///
/// When the `color` feature is enabled, this returns a colored string.
/// When disabled, returns the art unchanged.
#[cfg(feature = "color")]
pub fn colorize(art: &str, species: crate::Species) -> String {
    match species {
        crate::Species::Crow => colorize_crow(art),
        crate::Species::Raven => colorize_raven(art),
        crate::Species::Magpie => colorize_magpie(art),
        crate::Species::Jay => colorize_jay(art),
    }
}

/// No-op when color feature is disabled.
#[cfg(not(feature = "color"))]
pub fn colorize(art: &str, _species: crate::Species) -> String {
    art.to_string()
}

#[cfg(feature = "color")]
fn colorize_crow(art: &str) -> String {
    use colored::Colorize as _;

    // Crows are primarily black/dark gray with a subtle sheen
    art.lines()
        .map(|line| {
            if line.contains('o')
                && (line.contains("|") || line.contains("/") || line.contains("\\"))
            {
                // Eye line - highlight eyes
                line.replace("o", &"o".bright_white().to_string())
                    .replace("|", &"|".bright_black().to_string())
            } else if line.contains('\"') || line.contains("Caw") {
                // Speech bubble - subtle gray
                line.bright_black().to_string()
            } else {
                // Body - dark with slight variation
                line.bright_black().to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(feature = "color")]
fn colorize_raven(art: &str) -> String {
    use colored::Colorize as _;

    // Ravens have a purple/blue iridescence
    art.lines()
        .map(|line| {
            if line.contains('o') && line.contains('|') {
                // Eye line - white eyes
                line.replace("o", &"o".bright_white().to_string())
                    .replace("|", &"|".magenta().to_string())
            } else if line.contains("Quoth") || line.contains("Nevermore") {
                // Speech - purple tinted
                line.magenta().to_string()
            } else {
                // Body - dark purple/black
                line.bright_black().to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(feature = "color")]
fn colorize_magpie(art: &str) -> String {
    use colored::Colorize as _;

    // Magpies are black and white with iridescent wings
    art.lines()
        .map(|line| {
            if line.contains("*") || line.contains(".") && line.contains("---") {
                // Wing/tail area - blue-green iridescence
                line.cyan().to_string()
            } else if line.contains('o') && line.contains("|") {
                // Eyes - bright
                line.replace("o", &"o".bright_white().to_string())
                    .replace("|", &"|".white().to_string())
            } else if line.contains("shiny") || line.contains("Shiny") {
                // Speech with "shiny" - highlight
                line.replace("shiny", &"shiny".bright_cyan().to_string())
                    .replace("Shiny", &"Shiny".bright_cyan().to_string())
            } else {
                // Body - alternating black/white effect
                line.white().to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(feature = "color")]
fn colorize_jay(art: &str) -> String {
    use colored::Colorize as _;

    // Jays are blue with white/gray
    art.lines()
        .enumerate()
        .map(|(i, line)| {
            if i == 1 && line.contains("/") {
                // Crest area - bright blue
                line.bright_blue().to_string()
            } else if line.contains('o') && line.contains("|") {
                // Eyes
                line.replace("o", &"o".bright_white().to_string())
                    .replace("|", &"|".blue().to_string())
            } else if line.contains("HEY") {
                // Loud speech - bright
                line.bright_blue().bold().to_string()
            } else {
                // Body - blue
                line.blue().to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
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
        // Should have same number of lines
        assert_eq!(result.lines().count(), 3);
    }
}
