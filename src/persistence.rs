//! Persistence support for corvid pets.
//!
//! This module provides save/load functionality for pet state,
//! allowing pets to persist across sessions.
//!
//! Requires the `persistence` feature to be enabled.

use crate::{Mood, Pet, Species};

#[cfg(feature = "persistence")]
use std::path::PathBuf;

/// Serializable simulation data.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct SimStateData {
    pub hunger: f32,
    pub energy: f32,
    pub happiness: f32,
    pub health: f32,
    pub stage: String,
    pub personality: String,
    pub age_secs: f64,
    pub interaction_count: u64,
    pub born_at: u64,
    pub last_tick: u64,
    pub cooldowns: Vec<(String, u64)>,
}

/// Serializable color scheme data.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorSchemeData {
    pub body: String,
    pub bubble: String,
}

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
    /// Optional custom color scheme.
    #[cfg_attr(feature = "persistence", serde(default))]
    pub color_scheme: Option<ColorSchemeData>,
    /// Optional simulation state.
    #[cfg_attr(feature = "persistence", serde(default))]
    pub sim: Option<SimStateData>,
}

impl PetState {
    /// Creates a state from a pet.
    pub fn from_pet(pet: &Pet) -> Self {
        let sim = pet.sim().map(|s| SimStateData {
            hunger: s.stats.hunger,
            energy: s.stats.energy,
            happiness: s.stats.happiness,
            health: s.stats.health,
            stage: s.stage.to_string(),
            personality: s.personality.to_string(),
            age_secs: s.age_secs,
            interaction_count: s.interaction_count,
            born_at: s.born_at,
            last_tick: s.last_tick,
            cooldowns: s
                .cooldowns
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
        });

        let color_scheme = pet.color_scheme.as_ref().map(|cs| ColorSchemeData {
            body: cs.body.to_string(),
            bubble: cs.bubble.to_string(),
        });

        Self {
            name: pet.name().to_string(),
            species: pet.species().to_string(),
            mood: pet.mood().to_string(),
            interaction_count: pet.sim().map_or(0, |s| s.interaction_count),
            last_saved: None,
            color_scheme,
            sim,
        }
    }

    /// Converts this state back into a pet.
    pub fn to_pet(&self) -> Pet {
        let species: Species = self.species.parse().unwrap_or_default();

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

        // Restore simulation state if present.
        if let Some(sim_data) = &self.sim {
            use crate::life_stage::LifeStage;
            use crate::needs::Need;
            use crate::personality::Personality;
            use crate::sim::SimState;
            use crate::stats::Stats;
            use std::collections::HashMap;

            let personality = match sim_data.personality.as_str() {
                "Shy" => Personality::Shy,
                "Mischievous" => Personality::Mischievous,
                "Stoic" => Personality::Stoic,
                "Affectionate" => Personality::Affectionate,
                "Greedy" => Personality::Greedy,
                _ => Personality::Curious,
            };

            let stage = match sim_data.stage.as_str() {
                "Hatchling" => LifeStage::Hatchling,
                "Fledgling" => LifeStage::Fledgling,
                "Adult" => LifeStage::Adult,
                "Elder" => LifeStage::Elder,
                _ => LifeStage::Egg,
            };

            let mut cooldowns = HashMap::new();
            for (k, v) in &sim_data.cooldowns {
                let need = match k.as_str() {
                    "Feed" => Need::Feed,
                    "Play" => Need::Play,
                    "Rest" => Need::Rest,
                    "Clean" => Need::Clean,
                    "Pet" => Need::Pet,
                    _ => continue,
                };
                cooldowns.insert(need, *v);
            }

            let sim = SimState {
                stats: Stats {
                    hunger: sim_data.hunger,
                    energy: sim_data.energy,
                    happiness: sim_data.happiness,
                    health: sim_data.health,
                },
                stage,
                personality,
                age_secs: sim_data.age_secs,
                interaction_count: sim_data.interaction_count,
                last_tick: sim_data.last_tick,
                cooldowns,
                born_at: sim_data.born_at,
            };

            pet.sim_state = Some(sim);
        }

        // Restore color scheme if present.
        if let Some(ref colors) = self.color_scheme
            && let (Ok(body), Ok(bubble)) = (
                colors.body.parse::<crate::color::PetColor>(),
                colors.bubble.parse::<crate::color::PetColor>(),
            )
        {
            pet.color_scheme = Some(crate::color::ColorScheme::new(body, bubble));
        }

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
            color_scheme: None,
            sim: None,
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
            name: "Corvin".to_string(),
            species: "Crow".to_string(),
            mood: "Happy".to_string(),
            interaction_count: 0,
            last_saved: None,
            color_scheme: None,
            sim: None,
        };

        let pet = state.to_pet();
        assert_eq!(pet.name(), "Corvin");
        assert_eq!(pet.species(), Species::Crow);
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
        for species in [Species::Crow, Species::Magpie, Species::Raven, Species::Jackdaw] {
            for mood in [
                Mood::Happy,
                Mood::Sad,
                Mood::Neutral,
                Mood::Confused,
                Mood::Excited,
                Mood::Sleepy,
            ] {
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
