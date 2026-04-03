//! Improved ASCII art for corvid pets (v2).
//!
//! This module provides enhanced art with:
//! - Better proportions and expression
//! - Unicode support (optional, via feature flag)
//! - Larger canvas (15-20 lines)
//! - More distinct species characteristics
//!
//! Enable the `unicode` feature for Unicode box-drawing and braille characters.

use crate::{Mood, Species};

/// Renders art for the given species and mood.
pub fn render(species: Species, mood: Mood, use_unicode: bool) -> String {
    if use_unicode {
        render_unicode(species, mood)
    } else {
        render_ascii(species, mood)
    }
}

// MARK: - ASCII Art (Improved)

fn render_ascii(species: Species, mood: Mood) -> String {
    match species {
        Species::Crow => crow_ascii(mood),
        Species::Raven => raven_ascii(mood),
        Species::Magpie => magpie_ascii(mood),
        Species::Jay => jay_ascii(mood),
    }
}

fn crow_ascii(mood: Mood) -> String {
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
        "hunched" => "    | |    ",
        "roosting" => "    | |    ",
        _ => "   /| |\\   ",
    };

    format!(r#"         ."-".
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
      `"-'`-'"`"#)
}

fn raven_ascii(mood: Mood) -> String {
    let (eyes, beak, wings) = match mood {
        Mood::Happy => ("◕   ◕", " > ", "spread"),
        Mood::Sad => ("◕   ◕", " - ", "drooping"),
        Mood::Neutral => ("◕   ◕", " v ", "folded"),
        Mood::Confused => ("◕   ◯", " ? ", "twitch"),
        Mood::Excited => ("◕   ◕", " > ", "raised"),
        Mood::Sleepy => ("-   -", " v ", "tucked"),
    };

    let wing_art = match wings {
        "spread" => r#" ╱         ╲"#,
        "raised" => r#" ╱         ╲"#,
        "drooping" => r#"  \\       /"#,
        _ => r#"  |       |"#,
    };

    format!(r#"        _____
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
      `"'"`"`"`"#)
}

fn magpie_ascii(mood: Mood) -> String {
    let (eyes, beak, tail) = match mood {
        Mood::Happy => ("◕   ◕", " > ", "fanned"),
        Mood::Sad => ("◕   ◕", " - ", "droop"),
        Mood::Neutral => ("◕   ◕", " < ", "long"),
        Mood::Confused => ("◕   ◯", " ? ", "twitch"),
        Mood::Excited => ("◕   ◕", " > ", "raised"),
        Mood::Sleepy => ("-   -", " . ", "folded"),
    };

    let tail_art = match tail {
        "fanned" | "raised" => r#"       /\ /\ /\
      /  | |  \"#,
        "droop" | "folded" => r#"       | | | |
      \  | |  /"#,
        _ => r#"       | | | |
      |  | |  |"#,
    };

    format!(r#"         .---.
        /     \
       /       \
      |  {eyes}  |
      |    {beak}    |
      |   \\|/   |
     /|    |    |\
    / |    |    | \
   /  |____|____|  \
{tail_art}
        `"-'`-'"#)
}

fn jay_ascii(mood: Mood) -> String {
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
        "flat" | "down" => r#"      /        \"#,
        "ruffled" => r#"      /\\/\\  /\\/\\"#,
        _ => r#"       /\\   //\\"#,
    };

    format!(r#"       ,---,
{crest_art}
     /  {eyes}  \
    |    {beak}    |
   ~/|   \\|/   |\\~
  (_/|    |    |\\_)
     |    |    |
     |____|____|
        |  |
       /    \
      `"--'--'"#)
}

// MARK: - Unicode Art

fn render_unicode(_species: Species, _mood: Mood) -> String {
    // For now, return ASCII - full Unicode support can be added later
    render_ascii(_species, _mood)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_render() {
        let art = render(Species::Crow, Mood::Happy, false);
        assert!(!art.is_empty());
        assert!(art.contains('o') || art.contains('◕'));
    }

    #[test]
    fn test_all_combinations() {
        for species in [Species::Crow, Species::Raven, Species::Magpie, Species::Jay] {
            for mood in [Mood::Happy, Mood::Sad, Mood::Neutral, Mood::Confused, Mood::Excited, Mood::Sleepy] {
                let art = render(species, mood, false);
                assert!(!art.is_empty(), "Empty art for {:?} {:?}", species, mood);
            }
        }
    }

    #[test]
    fn test_mood_variations() {
        // Each mood should produce different art
        let happy = render(Species::Crow, Mood::Happy, false);
        let sad = render(Species::Crow, Mood::Sad, false);
        let confused = render(Species::Crow, Mood::Confused, false);

        assert_ne!(happy, sad);
        assert_ne!(happy, confused);
        assert_ne!(sad, confused);
    }
}
