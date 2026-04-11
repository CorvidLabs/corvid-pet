/// Emotional states for the corvid companion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Mood {
    /// All specs passing, feeling good.
    Happy,
    /// Errors found, feeling down.
    Sad,
    /// Just hanging out, default state.
    #[default]
    Neutral,
    /// Validation warnings, puzzled.
    Confused,
    /// New spec generated, energized.
    Excited,
    /// Idle, resting.
    Sleepy,
}

impl std::fmt::Display for Mood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mood::Happy => write!(f, "Happy"),
            Mood::Sad => write!(f, "Sad"),
            Mood::Neutral => write!(f, "Neutral"),
            Mood::Confused => write!(f, "Confused"),
            Mood::Excited => write!(f, "Excited"),
            Mood::Sleepy => write!(f, "Sleepy"),
        }
    }
}

/// Returns ASCII art for the given species and mood.
/// Art is designed to be animatable (eyes, beak, body position).
pub fn ascii_art(species: super::Species, mood: Mood) -> String {
    match species {
        super::Species::Crow => crow_art(mood),
    }
}

/// Returns open-eye variant of ASCII art for animations.
pub fn ascii_art_open_eyes(species: super::Species, mood: Mood) -> String {
    match species {
        super::Species::Crow => crow_art_open(mood),
    }
}

/// Returns closed-eye variant of ASCII art for animations.
pub fn ascii_art_closed_eyes(species: super::Species, mood: Mood) -> String {
    match species {
        super::Species::Crow => crow_art_closed(mood),
    }
}

// MARK: - Crow ASCII Art

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

fn crow_art_open(_mood: Mood) -> String {
    // For crow, open eyes are the default
    crow_art(Mood::Neutral)
}

fn crow_art_closed(_mood: Mood) -> String {
    r#"        .-.
       /   \
      |-   -|
      |  <  |
      | \|/ |
     /|  `-'  |\
    / |       | \
   |  |_______|  |
   |__|       |__|"#
        .to_string()
}
