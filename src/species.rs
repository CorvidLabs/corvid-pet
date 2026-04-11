/// Corvid species with unique personalities and characteristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Species {
    /// Clever problem solver - the default companion.
    #[default]
    Crow,
    /// Wise and ominous, but helpful.
    Raven,
    /// Shiny-obsessed collector.
    Magpie,
    /// Loud, opinionated, colorful personality.
    Jay,
}

impl Species {
    /// Returns the default name for this species.
    pub fn default_name(&self) -> String {
        match self {
            Species::Crow => "Corvin",
            Species::Raven => "Nevermore",
            Species::Magpie => "Shiny",
            Species::Jay => "Jay",
        }
        .to_string()
    }

    /// Returns the personality description for this species.
    pub fn personality(&self) -> &str {
        match self {
            Species::Crow => "clever and resourceful",
            Species::Raven => "wise and mysterious",
            Species::Magpie => "curious and shiny-obsessed",
            Species::Jay => "loud and opinionated",
        }
    }
}

impl std::fmt::Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Species::Crow => write!(f, "Crow"),
            Species::Raven => write!(f, "Raven"),
            Species::Magpie => write!(f, "Magpie"),
            Species::Jay => write!(f, "Jay"),
        }
    }
}
