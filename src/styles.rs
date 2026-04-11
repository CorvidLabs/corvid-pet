//! Customizable ASCII art styles for corvid pets.
//!
//! This module provides multiple art templates for the crow companion.

/// Available art styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ArtStyle {
    /// Detailed, larger art with shading.
    Detailed,
    /// Minimal, compact art style (default).
    #[default]
    Minimal,
    /// Blocky, geometric style using Unicode box characters.
    Blocky,
    /// Simple emoji-style art.
    Emoji,
}

impl ArtStyle {
    /// Returns ASCII art for the given mood and style.
    pub fn render(&self, _species: super::Species, mood: super::Mood) -> String {
        match self {
            ArtStyle::Detailed => detailed::render(mood),
            ArtStyle::Minimal => minimal::render(mood),
            ArtStyle::Blocky => blocky::render(mood),
            ArtStyle::Emoji => emoji::render(mood),
        }
    }

    /// Returns the style name.
    pub fn name(&self) -> &'static str {
        match self {
            ArtStyle::Detailed => "detailed",
            ArtStyle::Minimal => "minimal",
            ArtStyle::Blocky => "blocky",
            ArtStyle::Emoji => "emoji",
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
            "detailed" => Ok(ArtStyle::Detailed),
            "minimal" => Ok(ArtStyle::Minimal),
            "blocky" => Ok(ArtStyle::Blocky),
            "emoji" => Ok(ArtStyle::Emoji),
            _ => Err(format!("Unknown style: {}", s)),
        }
    }
}

// MARK: - Detailed Style

mod detailed {
    use crate::Mood;

    pub fn render(mood: Mood) -> String {
        match mood {
            Mood::Happy => r#"        .-.
       /   \
      |o   o|
      |  ^  |
      | \|/ |
     /|  `-'  |\
    / |       | \
   |  |_______|  |
   |__|       |__|
   "Caw! Looking good!""#
                .to_string(),
            Mood::Sad => r#"        .-.
       /   \
      |o   o|
      |  -  |
      | \|/ |
     /|  `-'  |\
    / |       | \
   |  |_______|  |
   |__|       |__|
   "Caw... something's wrong...""#
                .to_string(),
            Mood::Neutral => r#"        .-.
       /   \
      |o   o|
      |  <  |
      | \|/ |
     /|  `-'  |\
    / |       | \
   |  |_______|  |
   |__|       |__|
   "Caw?""#
                .to_string(),
            Mood::Confused => r#"        .-.
       /   \
      |o   O|
      |  ?  |
      | \|/ |
     /|  `-'  |\
    / |       | \
   |  |_______|  |
   |__|       |__|
   "Caw??""#
                .to_string(),
            Mood::Excited => r#"        .-.
       /   \
      |o   o|
      |  ^  |
      | \|/ |
     /|  `-'  |\
    / |       | \
   |  |_______|  |
   |__|       |__|
   "CAW! CAW! New spec!""#
                .to_string(),
            Mood::Sleepy => r#"        .-.
       /   \
      |-   -|
      |  .  |
      | \|/ |
     /|  `-'  |\
    / |       | \
   |  |_______|  |
   |__|       |__|
   "Zzz... caw...""#
                .to_string(),
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

// MARK: - Blocky Style

mod blocky {
    use crate::Mood;

    pub fn render(mood: Mood) -> String {
        let eyes = match mood {
            Mood::Happy | Mood::Excited => "◕ ◕",
            Mood::Sad => "◡ ◡",
            Mood::Confused => "◕ ◯",
            Mood::Sleepy => "- -",
            _ => "○ ○",
        };

        format!(
            r#"┌─────────┐
│  ╱   ╲  │
│ {eyes} │
│  ╲___╱  │
│    │    │
└────┴────┘
   "Caw!""#
        )
    }
}

// MARK: - Emoji Style

mod emoji {
    use crate::Mood;

    pub fn render(mood: Mood) -> String {
        match mood {
            Mood::Happy => "🐦‍⬛ ✨".to_string(),
            Mood::Sad => "🐦‍⬛ 😢".to_string(),
            Mood::Confused => "🐦‍⬛ ❓".to_string(),
            Mood::Excited => "🐦‍⬛ 🎉".to_string(),
            Mood::Sleepy => "🐦‍⬛ 💤".to_string(),
            Mood::Neutral => "🐦‍⬛".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Mood, Species};

    #[test]
    fn test_detailed_style() {
        let style = ArtStyle::Detailed;
        let art = style.render(Species::Crow, Mood::Happy);
        assert!(!art.is_empty());
        assert!(art.contains(".-."));
    }

    #[test]
    fn test_minimal_style() {
        let style = ArtStyle::Minimal;
        let art = style.render(Species::Crow, Mood::Happy);
        assert!(!art.is_empty());
        assert!(art.contains("oO"));
    }

    #[test]
    fn test_blocky_style() {
        let style = ArtStyle::Blocky;
        let art = style.render(Species::Crow, Mood::Happy);
        assert!(!art.is_empty());
        assert!(art.contains("◕"));
    }

    #[test]
    fn test_emoji_style() {
        let style = ArtStyle::Emoji;
        let art = style.render(Species::Crow, Mood::Happy);
        assert!(!art.is_empty());
        assert!(art.contains("✨"));
    }

    #[test]
    fn test_style_name() {
        assert_eq!(ArtStyle::Detailed.name(), "detailed");
        assert_eq!(ArtStyle::Minimal.name(), "minimal");
        assert_eq!(ArtStyle::Blocky.name(), "blocky");
        assert_eq!(ArtStyle::Emoji.name(), "emoji");
    }

    #[test]
    fn test_style_display() {
        assert_eq!(format!("{}", ArtStyle::Detailed), "detailed");
    }

    #[test]
    fn test_style_from_str() {
        assert_eq!("detailed".parse::<ArtStyle>().unwrap(), ArtStyle::Detailed);
        assert_eq!("minimal".parse::<ArtStyle>().unwrap(), ArtStyle::Minimal);
        assert_eq!("blocky".parse::<ArtStyle>().unwrap(), ArtStyle::Blocky);
        assert_eq!("emoji".parse::<ArtStyle>().unwrap(), ArtStyle::Emoji);
        assert!("unknown".parse::<ArtStyle>().is_err());
    }

    #[test]
    fn test_all_styles_all_moods() {
        let styles = [
            ArtStyle::Detailed,
            ArtStyle::Minimal,
            ArtStyle::Blocky,
            ArtStyle::Emoji,
        ];
        let moods = [
            Mood::Happy,
            Mood::Sad,
            Mood::Neutral,
            Mood::Confused,
            Mood::Excited,
            Mood::Sleepy,
        ];

        for style in &styles {
            for m in &moods {
                let art = style.render(Species::Crow, *m);
                assert!(!art.is_empty(), "Empty art for {:?} {:?}", style, m);
            }
        }
    }
}
