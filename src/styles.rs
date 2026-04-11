//! Customizable ASCII art styles for corvid pets.
//!
//! This module provides multiple art templates and allows users to
//! define custom styles for their pets.

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
    /// Returns ASCII art for the given species, mood, and style.
    pub fn render(&self, species: super::Species, mood: super::Mood) -> String {
        match self {
            ArtStyle::Detailed => detailed::render(species, mood),
            ArtStyle::Minimal => minimal::render(species, mood),
            ArtStyle::Blocky => blocky::render(species, mood),
            ArtStyle::Emoji => emoji::render(species, mood),
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

// MARK: - Detailed Style (original)

mod detailed {
    use crate::{Mood, Species};

    pub fn render(species: Species, mood: Mood) -> String {
        match species {
            Species::Crow => crow_art(mood),
            Species::Raven => raven_art(mood),
            Species::Magpie => magpie_art(mood),
            Species::Jay => jay_art(mood),
        }
    }

    fn crow_art(mood: Mood) -> String {
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

    fn raven_art(mood: Mood) -> String {
        match mood {
            Mood::Happy => r#"         ___
        /   \
       / o o \
      |   >   |
      |  \|/  |
     /|   `-'   |\
    / |         | \
   /  |_________|  \
  (__/         \__)
   "Quoth: well done.""#
                .to_string(),
            Mood::Sad => r#"         ___
        /   \
       / o o \
      |   -   |
      |  \|/  |
     /|   `-'   |\
    / |         | \
   /  |_________|  \
  (__/         \__)
   "Nevermore... errors.""#
                .to_string(),
            Mood::Neutral => r#"         ___
        /   \
       / o o \
      |   v   |
      |  \|/  |
     /|   `-'   |\
    / |         | \
   /  |_________|  \
  (__/         \__)
   "Quoth?""#
                .to_string(),
            Mood::Confused => r#"         ___
        /   \
       / o O \
      |   ?   |
      |  \|/  |
     /|   `-'   |\
    / |         | \
   /  |_________|  \
  (__/         \__)
   "Quoth the... what?""#
                .to_string(),
            Mood::Excited => r#"         ___
        /   \
       / o o \
      |   >   |
      |  \|/  |
     /|   `-'   |\
    / |         | \
   /  |_________|  \
  (__/         \__)
   "Quoth: excellent news!""#
                .to_string(),
            Mood::Sleepy => r#"         ___
        /   \
       / - - \
      |   v   |
      |  \|/  |
     /|   `-'   |\
    / |         | \
   /  |_________|  \
  (__/         \__)
   "Nevermore... zzz...""#
                .to_string(),
        }
    }

    fn magpie_art(mood: Mood) -> String {
        match mood {
            Mood::Happy => r#"        .---.
       / o o \
      |   ^   |
      |  \|/  |
     /|   `-'   |\
   (*|         |*)
    --|_______|--
       |     |
      /       \
     `-------'
   "Ooh! Shiny specs!""#
                .to_string(),
            Mood::Sad => r#"        .---.
       / o o \
      |   -   |
      |  \|/  |
     /|   `-'   |\
   (*|         |*)
    --|_______|--
       |     |
      /       \
     `-------'
   "No shiny things found...""#
                .to_string(),
            Mood::Neutral => r#"        .---.
       / o o \
      |   <   |
      |  \|/  |
     /|   `-'   |\
   (*|         |*)
    --|_______|--
       |     |
      /       \
     `-------'
   "Looking for shiny...""#
                .to_string(),
            Mood::Confused => r#"        .---.
       / o O \
      |   ?   |
      |  \|/  |
     /|   `-'   |\
   (*|         |*)
    --|_______|--
       |     |
      /       \
     `-------'
   "Is this shiny?""#
                .to_string(),
            Mood::Excited => r#"        .---.
       / o o \
      |   >   |
      |  \|/  |
     /|   `-'   |\
   (*|         |*)
    --|_______|--
       |     |
      /       \
     `-------'
   "SO MANY SHINY SPECS!""#
                .to_string(),
            Mood::Sleepy => r#"        .---.
       / - - \
      |   .   |
      |  \|/  |
     /|   `-'   |\
   (*|         |*)
    --|_______|--
       |     |
      /       \
     `-------'
   "Zzz... shiny...""#
                .to_string(),
        }
    }

    fn jay_art(mood: Mood) -> String {
        match mood {
            Mood::Happy => r#"       ,---.
      /\  //\
     / o  o \
    |   ^    |
   ~/|  \|/  |\~
  (_/|   `-'   |\_)
     |         |
     |_________|
        |   |
       /     \
      `-------'
  "HEY! GREAT JOB!""#
                .to_string(),
            Mood::Sad => r#"       ,---.
      /\  //\
     / o  o \
    |   -    |
   ~/|  \|/  |\~
  (_/|   `-'   |\_)
     |         |
     |_________|
        |   |
       /     \
      `-------'
  "HEY... IT'S BROKEN...""#
                .to_string(),
            Mood::Neutral => r#"       ,---.
      /\  //\
     / o  o \
    |   <    |
   ~/|  \|/  |\~
  (_/|   `-'   |\_)
     |         |
     |_________|
        |   |
       /     \
      `-------'
  "HEY! LISTEN!""#
                .to_string(),
            Mood::Confused => r#"       ,---.
      /\  //\
     / o  O \
    |   ?    |
   ~/|  \|/  |\~
  (_/|   `-'   |\_)
     |         |
     |_________|
        |   |
       /     \
      `-------'
  "HEY! WHAT'S THIS?!""#
                .to_string(),
            Mood::Excited => r#"       ,---.
      /\  //\
     / o  o \
    |   >    |
   ~/|  \|/  |\~
  (_/|   `-'   |\_)
     |         |
     |_________|
        |   |
       /     \
      `-------'
  "HEY! HEY! NEW SPEC!""#
                .to_string(),
            Mood::Sleepy => r#"       ,---.
      /\  //\
     / -  - \
    |   .    |
   ~/|  \|/  |\~
  (_/|   `-'   |\_)
     |         |
     |_________|
        |   |
       /     \
      `-------'
  "Zzz... hey...""#
                .to_string(),
        }
    }
}

// MARK: - Minimal Style

mod minimal {
    use crate::Mood;

    pub fn render(_species: crate::Species, mood: Mood) -> String {
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
    use crate::{Mood, Species};

    pub fn render(species: Species, mood: Mood) -> String {
        match species {
            Species::Crow => crow_art(mood),
            Species::Raven => raven_art(mood),
            Species::Magpie => magpie_art(mood),
            Species::Jay => jay_art(mood),
        }
    }

    fn crow_art(mood: Mood) -> String {
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

    fn raven_art(_mood: Mood) -> String {
        r#"┌───────────┐
│  ╱╲     ╱╲  │
│ │ ◕◕ │ │ ◕◕ │ │
│  ╲╱_____╲╱  │
│     │ │     │
└─────┴─┴─────┘
   "Quoth""#
            .to_string()
    }

    fn magpie_art(_mood: Mood) -> String {
        r#"╔═══════════╗
║  ╱ ◕◕ ╲   ║
║ │  ▽   │  ║
║  ╲_____╱   ║
║    │ │    ║
╚════╧═╧════╝
  "Shiny!""#
            .to_string()
    }

    fn jay_art(_mood: Mood) -> String {
        r#"╭───────────╮
│ ╱╲     ╱╲ │
││ ◕◕ │ ◕◕ ││
│ ╲  ▽▽▽  ╱ │
│   │   │   │
╰───┴───┴───╯
 "HEY!""#
            .to_string()
    }
}

// MARK: - Emoji Style

mod emoji {
    use crate::{Mood, Species};

    pub fn render(species: Species, mood: Mood) -> String {
        let emoji = match (species, mood) {
            (Species::Crow, Mood::Happy) => "🐦‍⬛ ✨",
            (Species::Crow, Mood::Sad) => "🐦‍⬛ 😢",
            (Species::Crow, Mood::Confused) => "🐦‍⬛ ❓",
            (Species::Crow, Mood::Excited) => "🐦‍⬛ 🎉",
            (Species::Crow, Mood::Sleepy) => "🐦‍⬛ 💤",
            (Species::Crow, _) => "🐦‍⬛",

            (Species::Raven, Mood::Happy) => "🐦‍⬛ ✨",
            (Species::Raven, Mood::Sad) => "🐦‍⬛ 🌧️",
            (Species::Raven, Mood::Confused) => "🐦‍⬛ ❓",
            (Species::Raven, Mood::Excited) => "🐦‍⬛ 🎊",
            (Species::Raven, Mood::Sleepy) => "🐦‍⬛ 🌙",
            (Species::Raven, _) => "🐦‍⬛",

            (Species::Magpie, Mood::Happy) => "🐦 ✨",
            (Species::Magpie, Mood::Sad) => "🐦 😔",
            (Species::Magpie, Mood::Confused) => "🐦 ❓",
            (Species::Magpie, Mood::Excited) => "🐦 💎",
            (Species::Magpie, Mood::Sleepy) => "🐦 💤",
            (Species::Magpie, _) => "🐦",

            (Species::Jay, Mood::Happy) => "🐦 🔵",
            (Species::Jay, Mood::Sad) => "🐦 😢",
            (Species::Jay, Mood::Confused) => "🐦 ❓",
            (Species::Jay, Mood::Excited) => "🐦 🎉",
            (Species::Jay, Mood::Sleepy) => "🐦 💤",
            (Species::Jay, _) => "🐦",
        };

        format!("{}", emoji)
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
        assert!(art.contains(".-.") || art.contains("Crow"));
    }

    #[test]
    fn test_minimal_style() {
        let style = ArtStyle::Minimal;
        let art = style.render(Species::Crow, Mood::Happy);
        assert!(!art.is_empty());
    }

    #[test]
    fn test_blocky_style() {
        let style = ArtStyle::Blocky;
        let art = style.render(Species::Crow, Mood::Happy);
        assert!(!art.is_empty());
    }

    #[test]
    fn test_emoji_style() {
        let style = ArtStyle::Emoji;
        let art = style.render(Species::Crow, Mood::Happy);
        assert!(!art.is_empty());
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
    fn test_all_styles_all_species() {
        let styles = [
            ArtStyle::Detailed,
            ArtStyle::Minimal,
            ArtStyle::Blocky,
            ArtStyle::Emoji,
        ];
        let species = [Species::Crow, Species::Raven, Species::Magpie, Species::Jay];
        let moods = [
            Mood::Happy,
            Mood::Sad,
            Mood::Neutral,
            Mood::Confused,
            Mood::Excited,
            Mood::Sleepy,
        ];

        for style in &styles {
            for s in &species {
                for m in &moods {
                    let art = style.render(*s, *m);
                    assert!(!art.is_empty(), "Empty art for {:?} {:?} {:?}", style, s, m);
                }
            }
        }
    }
}
