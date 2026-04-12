//! Art styles for corvid companions.
//!
//! Built-in style:
//! - **Minimal**: Compact ~6-line silhouettes with thought bubbles (default)

/// Available art styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ArtStyle {
    /// Minimal, compact art style with thought bubble (default).
    #[default]
    Minimal,
}

impl ArtStyle {
    /// Returns ASCII art for the given species and mood.
    pub fn render(&self, species: super::Species, mood: super::Mood) -> String {
        match self {
            ArtStyle::Minimal => minimal::render(species, mood),
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
                .map(|(i, l)| {
                    if i == 1 {
                        padded.clone()
                    } else {
                        l.to_string()
                    }
                })
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
            Species::Magpie => magpie(mood),
            Species::Raven => raven(mood),
            Species::Jackdaw => jackdaw(mood),
        }
    }

    fn crow(mood: Mood) -> String {
        let eye = eye(mood);
        format!(
            r#"      _
    <({eye}\
     |/(\
      \(\\
      " "\\"#
        )
    }

    fn magpie(mood: Mood) -> String {
        let eye = eye(mood);
        format!(
            r#"     __
    ({eye}>>
    /|  \~
   / |   \
  ~  ~~~~~"#
        )
    }

    fn raven(mood: Mood) -> String {
        let eye = eye(mood);
        format!(
            r#"       _
     /{eye})\
    | /  |
    |/  /
    /  /
   /__/"#
        )
    }

    fn jackdaw(mood: Mood) -> String {
        let eye = eye(mood);
        format!(
            r#"    _
  <({eye}\
   |(|
    \|\\
    ~ ~~"#
        )
    }

    fn eye(mood: Mood) -> &'static str {
        match mood {
            Mood::Happy => "^",
            Mood::Sad => ";",
            Mood::Neutral => "o",
            Mood::Confused => "?",
            Mood::Excited => "*",
            Mood::Sleepy => "-",
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
        let species_list = [Species::Crow, Species::Magpie, Species::Raven, Species::Jackdaw];

        for s in &species_list {
            for m in &moods {
                let art = ArtStyle::Minimal.render(*s, *m);
                assert!(!art.is_empty(), "Empty art for {:?} {:?}", s, m);
            }
        }
    }
}
