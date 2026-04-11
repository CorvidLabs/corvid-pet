//! Life stage progression for corvid pets.
//!
//! Pets progress through stages from Egg to Elder, each with
//! different stat modifiers and behaviors.

/// Duration constants in seconds.
const EGG_DURATION: u64 = 300; // 5 minutes
const HATCHLING_DURATION: u64 = 1800; // 30 minutes
const FLEDGLING_DURATION: u64 = 7200; // 2 hours
const ELDER_THRESHOLD: u64 = 86400; // 24 hours total age

/// Life stages a pet progresses through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LifeStage {
    /// Waiting to hatch. No stat decay, no interactions.
    #[default]
    Egg,
    /// Tiny and hungry. 1.5x hunger decay.
    Hatchling,
    /// Learning to fly. 1.2x energy decay.
    Fledgling,
    /// Full-grown. Normal rates.
    Adult,
    /// Wise and slow. 0.7x all decay.
    Elder,
}

impl LifeStage {
    /// Returns the next stage, or None if already Elder.
    pub fn next(&self) -> Option<LifeStage> {
        match self {
            LifeStage::Egg => Some(LifeStage::Hatchling),
            LifeStage::Hatchling => Some(LifeStage::Fledgling),
            LifeStage::Fledgling => Some(LifeStage::Adult),
            LifeStage::Adult => Some(LifeStage::Elder),
            LifeStage::Elder => None,
        }
    }

    /// Duration in seconds before transitioning to the next stage.
    /// None for indefinite stages (Adult, Elder).
    pub fn duration_secs(&self) -> Option<u64> {
        match self {
            LifeStage::Egg => Some(EGG_DURATION),
            LifeStage::Hatchling => Some(HATCHLING_DURATION),
            LifeStage::Fledgling => Some(FLEDGLING_DURATION),
            LifeStage::Adult => None,
            LifeStage::Elder => None,
        }
    }

    /// Hunger decay multiplier for this stage.
    pub fn hunger_multiplier(&self) -> f32 {
        match self {
            LifeStage::Egg => 0.0,
            LifeStage::Hatchling => 1.5,
            LifeStage::Fledgling => 1.0,
            LifeStage::Adult => 1.0,
            LifeStage::Elder => 0.7,
        }
    }

    /// Energy decay multiplier for this stage.
    pub fn energy_multiplier(&self) -> f32 {
        match self {
            LifeStage::Egg => 0.0,
            LifeStage::Hatchling => 1.0,
            LifeStage::Fledgling => 1.2,
            LifeStage::Adult => 1.0,
            LifeStage::Elder => 0.7,
        }
    }

    /// Happiness decay multiplier for this stage.
    pub fn happiness_multiplier(&self) -> f32 {
        match self {
            LifeStage::Egg => 0.0,
            LifeStage::Hatchling => 1.0,
            LifeStage::Fledgling => 1.0,
            LifeStage::Adult => 1.0,
            LifeStage::Elder => 0.7,
        }
    }

    /// Whether the pet can accept interactions in this stage.
    pub fn can_interact(&self) -> bool {
        !matches!(self, LifeStage::Egg)
    }

    /// Determines the correct stage for a given total age in seconds.
    pub fn for_age(age_secs: f64) -> LifeStage {
        let age = age_secs as u64;
        if age < EGG_DURATION {
            LifeStage::Egg
        } else if age < EGG_DURATION + HATCHLING_DURATION {
            LifeStage::Hatchling
        } else if age < EGG_DURATION + HATCHLING_DURATION + FLEDGLING_DURATION {
            LifeStage::Fledgling
        } else if age < ELDER_THRESHOLD {
            LifeStage::Adult
        } else {
            LifeStage::Elder
        }
    }

    /// Returns progress (0.0-1.0) through the current stage.
    /// Returns 1.0 for indefinite stages.
    pub fn progress(&self, age_secs: f64) -> f32 {
        let age = age_secs as u64;
        match self {
            LifeStage::Egg => (age as f32 / EGG_DURATION as f32).min(1.0),
            LifeStage::Hatchling => {
                let stage_age = age.saturating_sub(EGG_DURATION);
                (stage_age as f32 / HATCHLING_DURATION as f32).min(1.0)
            }
            LifeStage::Fledgling => {
                let stage_age = age.saturating_sub(EGG_DURATION + HATCHLING_DURATION);
                (stage_age as f32 / FLEDGLING_DURATION as f32).min(1.0)
            }
            LifeStage::Adult | LifeStage::Elder => 1.0,
        }
    }
}

impl std::fmt::Display for LifeStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LifeStage::Egg => write!(f, "Egg"),
            LifeStage::Hatchling => write!(f, "Hatchling"),
            LifeStage::Fledgling => write!(f, "Fledgling"),
            LifeStage::Adult => write!(f, "Adult"),
            LifeStage::Elder => write!(f, "Elder"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_progression() {
        assert_eq!(LifeStage::Egg.next(), Some(LifeStage::Hatchling));
        assert_eq!(LifeStage::Hatchling.next(), Some(LifeStage::Fledgling));
        assert_eq!(LifeStage::Fledgling.next(), Some(LifeStage::Adult));
        assert_eq!(LifeStage::Adult.next(), Some(LifeStage::Elder));
        assert_eq!(LifeStage::Elder.next(), None);
    }

    #[test]
    fn test_egg_no_decay() {
        assert_eq!(LifeStage::Egg.hunger_multiplier(), 0.0);
        assert_eq!(LifeStage::Egg.energy_multiplier(), 0.0);
        assert_eq!(LifeStage::Egg.happiness_multiplier(), 0.0);
    }

    #[test]
    fn test_egg_no_interaction() {
        assert!(!LifeStage::Egg.can_interact());
        assert!(LifeStage::Hatchling.can_interact());
        assert!(LifeStage::Adult.can_interact());
    }

    #[test]
    fn test_for_age() {
        assert_eq!(LifeStage::for_age(0.0), LifeStage::Egg);
        assert_eq!(LifeStage::for_age(299.0), LifeStage::Egg);
        assert_eq!(LifeStage::for_age(300.0), LifeStage::Hatchling);
        assert_eq!(LifeStage::for_age(2099.0), LifeStage::Hatchling);
        assert_eq!(LifeStage::for_age(2100.0), LifeStage::Fledgling);
        assert_eq!(LifeStage::for_age(9299.0), LifeStage::Fledgling);
        assert_eq!(LifeStage::for_age(9300.0), LifeStage::Adult);
        assert_eq!(LifeStage::for_age(86399.0), LifeStage::Adult);
        assert_eq!(LifeStage::for_age(86400.0), LifeStage::Elder);
    }

    #[test]
    fn test_elder_slower_decay() {
        assert!(LifeStage::Elder.hunger_multiplier() < 1.0);
        assert!(LifeStage::Elder.energy_multiplier() < 1.0);
        assert!(LifeStage::Elder.happiness_multiplier() < 1.0);
    }

    #[test]
    fn test_hatchling_hungry() {
        assert!(LifeStage::Hatchling.hunger_multiplier() > 1.0);
    }

    #[test]
    fn test_fledgling_energy_drain() {
        assert!(LifeStage::Fledgling.energy_multiplier() > 1.0);
    }

    #[test]
    fn test_progress() {
        assert!((LifeStage::Egg.progress(150.0) - 0.5).abs() < 0.01);
        assert_eq!(LifeStage::Adult.progress(50000.0), 1.0);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", LifeStage::Egg), "Egg");
        assert_eq!(format!("{}", LifeStage::Elder), "Elder");
    }
}
