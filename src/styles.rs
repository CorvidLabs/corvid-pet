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

// MARK: - Detailed Style (improved v2)

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
        let (eyes, beak, posture) = match mood {
            Mood::Happy => ("o   o", " ^ ", "alert"),
            Mood::Sad => ("o   o", " - ", "hunched"),
            Mood::Neutral => ("o   o", " < ", "perched"),
            Mood::Confused => ("o   O", " ? ", "tilted"),
            Mood::Excited => ("o   o", " > ", "wings"),
            Mood::Sleepy => ("-   -", " . ", "roosting"),
        };

        let wings = match posture {
            "wings" => "  \\   /  ",
            _ => "   | |   ",
        };

        let body = match posture {
            "hunched" | "roosting" => "    | |    ",
            _ => "   /| |\\   ",
        };

        format!(
            r#"         ."-".
        /     \
       /       \
      | {eyes} |
      |   {beak}   |
      |  \\|/  |
     /|   |   |\
    / |   |   | \
   {wings}
      |_______|
      /     \
{body}
      `"-'`-'"`"#
        )
    }

    fn raven_art(mood: Mood) -> String {
        let (eyes, beak, wings) = match mood {
            Mood::Happy => ("◕   ◕", " > ", "spread"),
            Mood::Sad => ("◕   ◕", " - ", "drooping"),
            Mood::Neutral => ("◕   ◕", " v ", "folded"),
            Mood::Confused => ("◕   ◯", " ? ", "twitch"),
            Mood::Excited => ("◕   ◕", " > ", "raised"),
            Mood::Sleepy => ("-   -", " v ", "tucked"),
        };

        let wing_art = match wings {
            "spread" | "raised" => r#" ╱         ╲"#,
            "drooping" => r#"  \\       /"#,
            _ => r#"  |       |"#,
        };

        format!(
            r#"        _____
       /     \
      /       \
     |  {eyes}  |
     |    {beak}    |
    /|   \\|/   |\
   / |    |    | \
{wing_art}
    \ |    |    | /
     |___|_|___|
        |   |
       /     \
      `"'"`"`"`"#
        )
    }

    fn magpie_art(mood: Mood) -> String {
        let (eyes, beak, tail) = match mood {
            Mood::Happy => ("◕   ◕", " > ", "fanned"),
            Mood::Sad => ("◕   ◕", " - ", "droop"),
            Mood::Neutral => ("◕   ◕", " < ", "long"),
            Mood::Confused => ("◕   ◯", " ? ", "twitch"),
            Mood::Excited => ("◕   ◕", " > ", "raised"),
            Mood::Sleepy => ("-   -", " . ", "folded"),
        };

        let tail_art = match tail {
            "fanned" | "raised" => {
                r#"       /\ /\ /\
      /  | |  \"#
            }
            "droop" | "folded" => {
                r#"       | | | |
      \  | |  /"#
            }
            _ => {
                r#"       | | | |
      |  | |  |"#
            }
        };

        format!(
            r#"         .---.
        /     \
       /       \
      |  {eyes}  |
      |    {beak}    |
      |   \\|/   |
     /|    |    |\
    / |    |    | \
   /  |____|____|  \
{tail_art}
        `"-'`-'"#
        )
    }

    fn jay_art(mood: Mood) -> String {
        let (eyes, beak, crest) = match mood {
            Mood::Happy => ("◕   ◕", " > ", "up"),
            Mood::Sad => ("◕   ◕", " - ", "flat"),
            Mood::Neutral => ("◕   ◕", " < ", "slight"),
            Mood::Confused => ("◕   ◯", " ? ", "ruffled"),
            Mood::Excited => ("◕   ◕", " > ", "raised"),
            Mood::Sleepy => ("-   -", " . ", "down"),
        };

        let crest_art = match crest {
            "up" | "raised" => r#"       /\\   //\\"#,
            "flat" | "down" => r#"      /        \\"#,
            "ruffled" => r#"      /\\/\\  /\\/\\\\"#,
            _ => r#"       /\\   //\\"#,
        };

        format!(
            r#"       ,---,
{crest_art}
     /  {eyes}  \
    |    {beak}    |
   ~/|   \\|/   |\\~
  (_/|    |    |\\_)
     |    |    |
     |____|____|
        |  |
       /    \
      `"--'--'"#
        )
    }
}

// MARK: - Minimal Style

mod minimal {
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
     |/(\
      \(\\
      "^`\"."#
        )
    }

    fn raven_art(_mood: Mood) -> String {
        r#"    ___        ___
   /   \      /   \
  | o o |    | o o |
   \___/      \___/
    /  \      /  \
   `---'      `---'"
            Raven"#
            .to_string()
    }

    fn magpie_art(_mood: Mood) -> String {
        r#"    .-.
   /o o\
   \ - /
    |-|
   /| |\
  (_| |_)"
            Magpie"#
            .to_string()
    }

    fn jay_art(_mood: Mood) -> String {
        r#"    ,-.
   /o o\
  / \^/ \
   \ | /
    |=|
   /   \
  `-----'"
            Jay"#
            .to_string()
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
        assert!(art.contains(".\"-.") || art.contains("o   o"));
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
