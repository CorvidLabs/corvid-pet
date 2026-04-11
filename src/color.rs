//! Colored output support for corvid pets.
//!
//! This module provides ANSI color support when the `color` feature is enabled.
//! - Crow: Gray/Black with white beak

/// Applies species-appropriate ANSI colors to ASCII art.
///
/// When the `color` feature is enabled, this returns a colored string.
/// When disabled, returns the art unchanged.
#[cfg(feature = "color")]
pub fn colorize(art: &str, species: crate::Species) -> String {
    match species {
        crate::Species::Crow => colorize_crow(art),
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
