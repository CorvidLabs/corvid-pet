//! Art styles for corvid companions.
//!
//! Two built-in styles:
//! - **Minimal**: Compact ~6-line silhouettes with thought bubbles (default)
//! - **Detailed**: Larger ~12-line species-specific art from art_v2

/// Available art styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ArtStyle {
    /// Minimal, compact art style with thought bubble (default).
    #[default]
    Minimal,
    /// Detailed, larger species-differentiated art from art_v2.
    Detailed,
}

impl ArtStyle {
    /// Returns ASCII art for the given species and mood.
    pub fn render(&self, species: super::Species, mood: super::Mood) -> String {
        match self {
            ArtStyle::Minimal => minimal::render(species, mood),
            ArtStyle::Detailed => crate::art_v2::render(species, mood, false),
        }
    }

    /// Returns the style name.
    pub fn name(&self) -> &'static str {
        match self {
            ArtStyle::Minimal => "minimal",
            ArtStyle::Detailed => "detailed",
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
            "detailed" => Ok(ArtStyle::Detailed),
            _ => Err(format!("Unknown style: {}", s)),
        }
    }
}

// MARK: - Minimal Style

mod minimal {
    use crate::{Mood, Species};

    pub fn render(species: Species, mood: Mood) -> String {
        let thought = thought_bubble(mood);
        let body = species_art(species, mood);
        let bubble = format!(".oO({thought})");
        // Place thought bubble beside the bird's head (line 1), not above
        let lines: Vec<&str> = body.lines().collect();
        if lines.len() > 1 {
            let head_line = lines[1];
            let padded = format!("{:<10}{}", head_line, bubble);
            let owned_lines: Vec<String> = lines
                .iter()
                .enumerate()
                .map(|(i, l)| if i == 1 { padded.clone() } else { l.to_string() })
                .collect();
            owned_lines.join("\n")
        } else {
            format!("{body}  {bubble}")
        }
    }

    fn thought_bubble(mood: Mood) -> &'static str {
        match mood {
            Mood::Happy => "Caw! ^v^",
            Mood::Sad => "oh no...",
            Mood::Neutral => "hmm",
            Mood::Confused => "??",
            Mood::Excited => "CAW CAW!",
            Mood::Sleepy => "zzz...",
        }
    }

    fn species_art(species: Species, mood: Mood) -> String {
        match species {
            Species::Crow => crow(mood),
            Species::Raven => raven(mood),
            Species::Magpie => magpie(mood),
            Species::Jay => jay(mood),
        }
    }

    fn crow(mood: Mood) -> String {
        let (eye_l, eye_r) = eyes(mood);
        format!(
            r#"      _
    <({eye_l}\
     |/({eye_r}\
      \(\\
      "^`"."#
        )
    }

    fn raven(mood: Mood) -> String {
        let (eye_l, eye_r) = eyes(mood);
        format!(
            r#"     __
    ({eye_l} >
    ({eye_r}/|
     ||/ |
     |/_/
    "`""#
        )
    }

    fn magpie(mood: Mood) -> String {
        let (eye_l, eye_r) = eyes(mood);
        format!(
            r#"      _
    *({eye_l}\
    *({eye_r}/
     |/  \~~~~
     |__/
    "`""#
        )
    }

    fn jay(mood: Mood) -> String {
        let (eye_l, eye_r) = eyes(mood);
        format!(
            r#"    /\/\
    ({eye_l} >
    ({eye_r}|~~
     ||  |
     |/__\
    "`""#
        )
    }

    fn eyes(mood: Mood) -> (&'static str, &'static str) {
        match mood {
            Mood::Happy => ("^", "^"),
            Mood::Sad => ("o", "o"),
            Mood::Neutral => ("o", "o"),
            Mood::Confused => ("o", "O"),
            Mood::Excited => ("*", "*"),
            Mood::Sleepy => ("-", "-"),
        }
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
    fn test_detailed_style() {
        let style = ArtStyle::Detailed;
        let art = style.render(Species::Crow, Mood::Happy);
        assert!(!art.is_empty());
    }

    #[test]
    fn test_species_differ_minimal() {
        let crow = ArtStyle::Minimal.render(Species::Crow, Mood::Neutral);
        let raven = ArtStyle::Minimal.render(Species::Raven, Mood::Neutral);
        let magpie = ArtStyle::Minimal.render(Species::Magpie, Mood::Neutral);
        let jay = ArtStyle::Minimal.render(Species::Jay, Mood::Neutral);
        assert_ne!(crow, raven);
        assert_ne!(crow, magpie);
        assert_ne!(crow, jay);
        assert_ne!(raven, magpie);
    }

    #[test]
    fn test_style_name() {
        assert_eq!(ArtStyle::Minimal.name(), "minimal");
        assert_eq!(ArtStyle::Detailed.name(), "detailed");
    }

    #[test]
    fn test_style_display() {
        assert_eq!(format!("{}", ArtStyle::Minimal), "minimal");
        assert_eq!(format!("{}", ArtStyle::Detailed), "detailed");
    }

    #[test]
    fn test_style_from_str() {
        assert_eq!("minimal".parse::<ArtStyle>().unwrap(), ArtStyle::Minimal);
        assert_eq!("detailed".parse::<ArtStyle>().unwrap(), ArtStyle::Detailed);
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
