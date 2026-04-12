/// Corvid species with unique personalities and characteristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Species {
    /// Clever problem solver - the default companion.
    #[default]
    Crow,
    /// Bold collector with an eye for shiny things.
    Magpie,
    /// Wise and mysterious, the largest corvid.
    Raven,
    /// Social and playful, thrives in groups.
    Jackdaw,
}

impl Species {
    /// Returns the default name for this species.
    pub fn default_name(&self) -> String {
        match self {
            Species::Crow => "Corvin",
            Species::Magpie => "Maggie",
            Species::Raven => "Munin",
            Species::Jackdaw => "Daw",
        }
        .to_string()
    }

    /// Returns the personality description for this species.
    pub fn personality(&self) -> &str {
        match self {
            Species::Crow => "clever and resourceful",
            Species::Magpie => "bold and acquisitive",
            Species::Raven => "wise and mysterious",
            Species::Jackdaw => "social and playful",
        }
    }
}

impl std::fmt::Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Species::Crow => write!(f, "Crow"),
            Species::Magpie => write!(f, "Magpie"),
            Species::Raven => write!(f, "Raven"),
            Species::Jackdaw => write!(f, "Jackdaw"),
        }
    }
}

impl std::str::FromStr for Species {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "crow" => Ok(Species::Crow),
            "magpie" => Ok(Species::Magpie),
            "raven" => Ok(Species::Raven),
            "jackdaw" => Ok(Species::Jackdaw),
            _ => Err(format!("Unknown species: {}. Choose from: crow, magpie, raven, jackdaw", s)),
        }
    }
}
