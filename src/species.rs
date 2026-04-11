/// Corvid species with unique personalities and characteristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Species {
    /// Clever problem solver - the default companion.
    #[default]
    Crow,
}

impl Species {
    /// Returns the default name for this species.
    pub fn default_name(&self) -> String {
        match self {
            Species::Crow => "Corvin",
        }
        .to_string()
    }

    /// Returns the personality description for this species.
    pub fn personality(&self) -> &str {
        match self {
            Species::Crow => "clever and resourceful",
        }
    }
}

impl std::fmt::Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Species::Crow => write!(f, "Crow"),
        }
    }
}
