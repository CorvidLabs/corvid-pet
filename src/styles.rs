//! Minimal ASCII art style for the crow companion.
//!
//! This module provides the clean, minimal crow art with thought bubble.

/// Available art styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ArtStyle {
    /// Minimal, compact art style with thought bubble (default).
    #[default]
    Minimal,
}

impl ArtStyle {
    /// Returns ASCII art for the given mood.
    pub fn render(&self, _species: super::Species, mood: super::Mood) -> String {
        match self {
            ArtStyle::Minimal => minimal::render(mood),
        }
    }

    /// Returns the style name.
    pub fn name(&self) -> &'static str {
        match self {
            ArtStyle::Minimal => "minimal",
        }
    }
}

impl std::fmt::Display for ArtStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::str::FromStr for ArtStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "minimal" => Ok(ArtStyle::Minimal),
            _ => Err(format!("Unknown style: {}", s)),
        }
    }
}

// MARK: - Minimal Style

mod minimal {
    use crate::Mood;

    pub fn render(mood: Mood) -> String {
        let thought = match mood {
            Mood::Happy => "Caw! ✨",
            Mood::Sad => "oh no...",
            Mood::Neutral => "hmm",
            Mood::Confused => "??",
            Mood::Excited => "CAW! 🎉",
            Mood::Sleepy => "zzz...",
        };

        format!(
            r#"   .oO({thought})
      _
    <(o\
     |/(\\
      \(\\
      "^`"."#
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Mood, Species};

    #[test]
    fn test_minimal_style() {
        let style = ArtStyle::Minimal;
        let art = style.render(Species::Crow, Mood::Happy);
        assert!(!art.is_empty());
        assert!(art.contains("oO"));
    }

    #[test]
    fn test_style_name() {
        assert_eq!(ArtStyle::Minimal.name(), "minimal");
    }

    #[test]
    fn test_style_display() {
        assert_eq!(format!("{}", ArtStyle::Minimal), "minimal");
    }

    #[test]
    fn test_style_from_str() {
        assert_eq!("minimal".parse::<ArtStyle>().unwrap(), ArtStyle::Minimal);
        assert!("unknown".parse::<ArtStyle>().is_err());
    }

    #[test]
    fn test_all_moods() {
        let moods = [
            Mood::Happy,
            Mood::Sad,
            Mood::Neutral,
            Mood::Confused,
            Mood::Excited,
            Mood::Sleepy,
        ];

        for m in &moods {
            let art = ArtStyle::Minimal.render(Species::Crow, *m);
            assert!(!art.is_empty(), "Empty art for {:?}", m);
        }
    }
}
