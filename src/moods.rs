/// Emotional states for the corvid companion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mood {
    /// All specs passing, feeling good.
    Happy,
    /// Errors found, feeling down.
    Sad,
    /// Just hanging out, default state.
    Neutral,
    /// Validation warnings, puzzled.
    Confused,
    /// New spec generated, energized.
    Excited,
    /// Idle, resting.
    Sleepy,
}

impl Default for Mood {
    fn default() -> Self {
        Mood::Neutral
    }
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
        super::Species::Raven => raven_art(mood),
        super::Species::Magpie => magpie_art(mood),
        super::Species::Jay => jay_art(mood),
    }
}

/// Returns open-eye variant of ASCII art for animations.
pub fn ascii_art_open_eyes(species: super::Species, mood: Mood) -> String {
    match species {
        super::Species::Crow => crow_art_open(mood),
        super::Species::Raven => raven_art_open(mood),
        super::Species::Magpie => magpie_art_open(mood),
        super::Species::Jay => jay_art_open(mood),
    }
}

/// Returns closed-eye variant of ASCII art for animations.
pub fn ascii_art_closed_eyes(species: super::Species, mood: Mood) -> String {
    match species {
        super::Species::Crow => crow_art_closed(mood),
        super::Species::Raven => raven_art_closed(mood),
        super::Species::Magpie => magpie_art_closed(mood),
        super::Species::Jay => jay_art_closed(mood),
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

// MARK: - Raven ASCII Art

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

fn raven_art_open(_mood: Mood) -> String {
    raven_art(Mood::Neutral)
}

fn raven_art_closed(_mood: Mood) -> String {
    r#"         ___
        /   \
       / - - \
      |   v   |
      |  \|/  |
     /|   `-'   |\
    / |         | \
   /  |_________|  \
  (__/         \__)"#
        .to_string()
}

// MARK: - Magpie ASCII Art

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

fn magpie_art_open(_mood: Mood) -> String {
    magpie_art(Mood::Neutral)
}

fn magpie_art_closed(_mood: Mood) -> String {
    r#"        .---.
       / - - \
      |   <   |
      |  \|/  |
     /|   `-'   |\
   (*|         |*)
    --|_______|--
       |     |
      /       \
     `-------'"#
        .to_string()
}

// MARK: - Jay ASCII Art

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

fn jay_art_open(_mood: Mood) -> String {
    jay_art(Mood::Neutral)
}

fn jay_art_closed(_mood: Mood) -> String {
    r#"       ,---.
      /\  //\
     / -  - \
    |   <    |
   ~/|  \|/  |\~
  (_/|   `-'   |\_)
     |         |
     |_________|
        |   |
       /     \
      `-------'"#
        .to_string()
}
