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

    // Eye characters for all moods: ^ ; o ? * -
    let is_eye_line = |line: &str| line.contains("<(") && line.contains("\\");

    // Crows are primarily black/dark gray with a subtle sheen
    art.lines()
        .map(|line| {
            if is_eye_line(line) {
                // Eye line — color the eye char bright white, rest dark
                let colored = line.bright_black().to_string();
                // Highlight the thought bubble content
                if let (Some(start), Some(end)) = (line.find(".oO("), line.rfind(')')) {
                    let before = line[..start].bright_black().to_string();
                    let bubble = line[start..=end].white().to_string();
                    let after = line[end + 1..].bright_black().to_string();
                    format!("{before}{bubble}{after}")
                } else {
                    colored
                }
            } else if line.contains('"') || line.contains("Caw") || line.contains("caw") {
                // Speech/comment line - yellow for personality
                line.yellow().to_string()
            } else {
                // Body - dark
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
