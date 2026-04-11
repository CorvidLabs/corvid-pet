//! Persistence support for corvid pets.
//!
//! This module provides save/load functionality for pet state,
//! allowing pets to persist across sessions.
//!
//! Requires the `persistence` feature to be enabled.

use crate::{Mood, Pet, Species};

#[cfg(feature = "persistence")]
use std::path::PathBuf;

/// Serializable state of a pet.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct PetState {
    /// The pet's name.
    pub name: String,
    /// The species identifier.
    pub species: String,
    /// The current mood.
    pub mood: String,
    /// Number of times the pet has reacted to events.
    #[cfg_attr(feature = "persistence", serde(default))]
    pub interaction_count: u64,
    /// Timestamp of last save.
    #[cfg_attr(feature = "persistence", serde(default))]
    pub last_saved: Option<u64>,
}

impl PetState {
    /// Creates a state from a pet.
    pub fn from_pet(pet: &Pet) -> Self {
        Self {
            name: pet.name().to_string(),
            species: pet.species().to_string(),
            mood: pet.mood().to_string(),
            interaction_count: 0,
            last_saved: None,
        }
    }

    /// Converts this state back into a pet.
    pub fn to_pet(&self) -> Pet {
        let species = match self.species.as_str() {
            "Raven" => Species::Raven,
            "Magpie" => Species::Magpie,
            "Jay" => Species::Jay,
            _ => Species::Crow,
        };

        let mut pet = Pet::new(self.name.clone(), species);

        let mood = match self.mood.as_str() {
            "Happy" => Mood::Happy,
            "Sad" => Mood::Sad,
            "Confused" => Mood::Confused,
            "Excited" => Mood::Excited,
            "Sleepy" => Mood::Sleepy,
            _ => Mood::Neutral,
        };
        pet.set_mood(mood);

        pet
    }
}

impl Default for PetState {
    fn default() -> Self {
        Self {
            name: String::new(),
            species: "Crow".to_string(),
            mood: "Neutral".to_string(),
            interaction_count: 0,
            last_saved: None,
        }
    }
}

/// Gets the default storage directory for pet data.
#[cfg(feature = "persistence")]
pub fn data_dir() -> Option<PathBuf> {
    dirs::data_local_dir().map(|d| d.join("corvid-pet"))
}

/// Saves a pet state to disk.
#[cfg(feature = "persistence")]
pub fn save_pet(state: &PetState, id: &str) -> Result<(), PersistenceError> {
    use std::fs;
    use std::io::Write;

    let dir = data_dir().ok_or(PersistenceError::NoDataDir)?;
    fs::create_dir_all(&dir)?;

    let path = dir.join(format!("{}.json", id));
    let json = serde_json::to_string_pretty(state)?;

    let mut file = fs::File::create(&path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

/// Loads a pet state from disk.
#[cfg(feature = "persistence")]
pub fn load_pet(id: &str) -> Result<PetState, PersistenceError> {
    use std::fs;

    let dir = data_dir().ok_or(PersistenceError::NoDataDir)?;
    let path = dir.join(format!("{}.json", id));

    let json = fs::read_to_string(&path)?;
    let state: PetState = serde_json::from_str(&json)?;

    Ok(state)
}

/// Lists all saved pet IDs.
#[cfg(feature = "persistence")]
pub fn list_pets() -> Result<Vec<String>, PersistenceError> {
    use std::fs;

    let dir = data_dir().ok_or(PersistenceError::NoDataDir)?;
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut pets = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str()
            && let Some(stripped) = name.strip_suffix(".json")
        {
            pets.push(stripped.to_string());
        }
    }

    pets.sort();
    Ok(pets)
}

/// Deletes a saved pet.
#[cfg(feature = "persistence")]
pub fn delete_pet(id: &str) -> Result<(), PersistenceError> {
    use std::fs;

    let dir = data_dir().ok_or(PersistenceError::NoDataDir)?;
    let path = dir.join(format!("{}.json", id));
    fs::remove_file(&path)?;

    Ok(())
}

/// Error type for persistence operations.
#[derive(Debug)]
pub enum PersistenceError {
    /// Could not determine data directory.
    NoDataDir,
    /// IO error.
    #[cfg(feature = "persistence")]
    Io(std::io::Error),
    /// Serialization error.
    #[cfg(feature = "persistence")]
    Serde(serde_json::Error),
}

#[cfg(feature = "persistence")]
impl From<std::io::Error> for PersistenceError {
    fn from(err: std::io::Error) -> Self {
        PersistenceError::Io(err)
    }
}

#[cfg(feature = "persistence")]
impl From<serde_json::Error> for PersistenceError {
    fn from(err: serde_json::Error) -> Self {
        PersistenceError::Serde(err)
    }
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistenceError::NoDataDir => write!(f, "could not determine data directory"),
            #[cfg(feature = "persistence")]
            PersistenceError::Io(e) => write!(f, "io error: {}", e),
            #[cfg(feature = "persistence")]
            PersistenceError::Serde(e) => write!(f, "serialization error: {}", e),
        }
    }
}

impl std::error::Error for PersistenceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_from_pet() {
        let pet = Pet::new("Test".to_string(), Species::Crow);
        let state = PetState::from_pet(&pet);

        assert_eq!(state.name, "Test");
        assert_eq!(state.species, "Crow");
        assert_eq!(state.mood, "Neutral");
    }

    #[test]
    fn test_state_to_pet() {
        let state = PetState {
            name: "Raven".to_string(),
            species: "Raven".to_string(),
            mood: "Happy".to_string(),
            interaction_count: 0,
            last_saved: None,
        };

        let pet = state.to_pet();
        assert_eq!(pet.name(), "Raven");
        assert_eq!(pet.species(), Species::Raven);
        assert_eq!(pet.mood(), Mood::Happy);
    }

    #[test]
    fn test_round_trip() {
        let pet = Pet::new("Corvin".to_string(), Species::Crow);
        let state = PetState::from_pet(&pet);
        let pet2 = state.to_pet();

        assert_eq!(pet.name(), pet2.name());
        assert_eq!(pet.species(), pet2.species());
        assert_eq!(pet.mood(), pet2.mood());
    }

    #[test]
    fn test_all_species_round_trip() {
        for species in [Species::Crow, Species::Raven, Species::Magpie, Species::Jay] {
            for mood in [Mood::Happy, Mood::Sad, Mood::Neutral, Mood::Confused, Mood::Excited, Mood::Sleepy] {
                let mut pet = Pet::new("Test".to_string(), species);
                pet.set_mood(mood);

                let state = PetState::from_pet(&pet);
                let pet2 = state.to_pet();

                assert_eq!(pet.species(), pet2.species());
                assert_eq!(pet.mood(), pet2.mood());
            }
        }
    }
}
