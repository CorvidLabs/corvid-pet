//! Pet vital statistics that decay over time.
//!
//! Stats drive the pet's mood and behavior. They decay at fixed rates
//! modified by life stage and personality, and recover through interactions.

use crate::Mood;

/// Base decay rates per second.
const HUNGER_DECAY_PER_SEC: f32 = 1.0 / 60.0;
const ENERGY_DECAY_PER_SEC: f32 = 0.5 / 60.0;
const HAPPINESS_DECAY_PER_SEC: f32 = 0.3 / 60.0;
const HEALTH_DECAY_PER_SEC: f32 = 0.2 / 60.0;

/// Maximum elapsed seconds to apply in a single tick (24 hours).
const MAX_TICK_SECS: f64 = 86400.0;

/// Vital statistics for a pet.
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    /// How full the pet is. 0 = starving, 100 = stuffed.
    pub hunger: f32,
    /// How rested the pet is. 0 = exhausted, 100 = energized.
    pub energy: f32,
    /// How content the pet is. 0 = miserable, 100 = elated.
    pub happiness: f32,
    /// Overall wellness. Decays when hunger or energy are critically low.
    pub health: f32,
}

impl Stats {
    /// Creates stats with all values at maximum.
    pub fn new() -> Self {
        Self {
            hunger: 100.0,
            energy: 100.0,
            happiness: 100.0,
            health: 100.0,
        }
    }

    /// Applies time-based stat decay.
    ///
    /// `elapsed_secs` is capped at 24 hours to prevent death from forgetting.
    /// `hunger_mult`, `energy_mult`, `happiness_mult` are stage/personality modifiers.
    pub fn tick(
        &mut self,
        elapsed_secs: f64,
        hunger_mult: f32,
        energy_mult: f32,
        happiness_mult: f32,
    ) {
        let dt = elapsed_secs.min(MAX_TICK_SECS) as f32;

        self.hunger -= HUNGER_DECAY_PER_SEC * dt * hunger_mult;
        self.energy -= ENERGY_DECAY_PER_SEC * dt * energy_mult;
        self.happiness -= HAPPINESS_DECAY_PER_SEC * dt * happiness_mult;

        // Health decays when hunger or energy are critically low.
        if self.hunger < 20.0 || self.energy < 10.0 {
            self.health -= HEALTH_DECAY_PER_SEC * dt;
        }

        self.clamp();
    }

    /// Clamps all stats to the valid range.
    pub fn clamp(&mut self) {
        self.hunger = self.hunger.clamp(0.0, 100.0);
        self.energy = self.energy.clamp(0.0, 100.0);
        self.happiness = self.happiness.clamp(0.0, 100.0);
        self.health = self.health.clamp(0.0, 100.0);
    }

    /// Weighted average of all stats.
    /// Health 40%, happiness 25%, hunger 20%, energy 15%.
    pub fn overall(&self) -> f32 {
        self.health * 0.40 + self.happiness * 0.25 + self.hunger * 0.20 + self.energy * 0.15
    }

    /// Returns which needs are critical (stat below 20.0).
    pub fn critical_needs(&self) -> Vec<crate::needs::Need> {
        use crate::needs::Need;
        let mut needs = Vec::new();
        if self.hunger < 20.0 {
            needs.push(Need::Feed);
        }
        if self.energy < 20.0 {
            needs.push(Need::Rest);
        }
        if self.happiness < 20.0 {
            needs.push(Need::Play);
        }
        if self.health < 20.0 {
            needs.push(Need::Clean);
        }
        needs
    }

    /// Derives the dominant mood from current stat levels.
    pub fn dominant_mood(&self) -> Mood {
        // Priority order: critical hunger > low energy > high happiness > low happiness > low health
        if self.hunger < 20.0 {
            Mood::Sad
        } else if self.energy < 15.0 {
            Mood::Sleepy
        } else if self.happiness > 80.0 {
            Mood::Happy
        } else if self.happiness < 20.0 {
            Mood::Sad
        } else if self.health < 30.0 {
            Mood::Confused
        } else {
            Mood::Neutral
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stats_are_full() {
        let s = Stats::new();
        assert_eq!(s.hunger, 100.0);
        assert_eq!(s.energy, 100.0);
        assert_eq!(s.happiness, 100.0);
        assert_eq!(s.health, 100.0);
    }

    #[test]
    fn test_tick_decays_stats() {
        let mut s = Stats::new();
        s.tick(60.0, 1.0, 1.0, 1.0); // 1 minute
        assert!((s.hunger - 99.0).abs() < 0.01);
        assert!((s.energy - 99.5).abs() < 0.01);
        assert!((s.happiness - 99.7).abs() < 0.01);
        assert_eq!(s.health, 100.0); // No health decay when stats are high
    }

    #[test]
    fn test_health_decays_when_hungry() {
        let mut s = Stats::new();
        s.hunger = 10.0;
        s.tick(60.0, 1.0, 1.0, 1.0);
        assert!(s.health < 100.0);
    }

    #[test]
    fn test_health_decays_when_exhausted() {
        let mut s = Stats::new();
        s.energy = 5.0;
        s.tick(60.0, 1.0, 1.0, 1.0);
        assert!(s.health < 100.0);
    }

    #[test]
    fn test_tick_caps_at_24_hours() {
        let mut s1 = Stats::new();
        let mut s2 = Stats::new();
        s1.tick(86400.0, 1.0, 1.0, 1.0); // Exactly 24h
        s2.tick(200000.0, 1.0, 1.0, 1.0); // Way more than 24h
        assert_eq!(s1.hunger, s2.hunger);
        assert_eq!(s1.energy, s2.energy);
    }

    #[test]
    fn test_stats_clamp_to_zero() {
        let mut s = Stats::new();
        s.tick(86400.0, 1.0, 1.0, 1.0);
        assert!(s.hunger >= 0.0);
        assert!(s.energy >= 0.0);
        assert!(s.happiness >= 0.0);
        assert!(s.health >= 0.0);
    }

    #[test]
    fn test_overall_weighted_average() {
        let s = Stats {
            hunger: 100.0,
            energy: 100.0,
            happiness: 100.0,
            health: 100.0,
        };
        assert!((s.overall() - 100.0).abs() < 0.01);

        let s2 = Stats {
            hunger: 0.0,
            energy: 0.0,
            happiness: 0.0,
            health: 0.0,
        };
        assert!((s2.overall()).abs() < 0.01);
    }

    #[test]
    fn test_dominant_mood_hungry() {
        let s = Stats {
            hunger: 10.0,
            energy: 100.0,
            happiness: 100.0,
            health: 100.0,
        };
        assert_eq!(s.dominant_mood(), Mood::Sad);
    }

    #[test]
    fn test_dominant_mood_sleepy() {
        let s = Stats {
            hunger: 50.0,
            energy: 10.0,
            happiness: 50.0,
            health: 100.0,
        };
        assert_eq!(s.dominant_mood(), Mood::Sleepy);
    }

    #[test]
    fn test_dominant_mood_happy() {
        let s = Stats {
            hunger: 80.0,
            energy: 80.0,
            happiness: 90.0,
            health: 100.0,
        };
        assert_eq!(s.dominant_mood(), Mood::Happy);
    }

    #[test]
    fn test_dominant_mood_neutral() {
        let s = Stats {
            hunger: 50.0,
            energy: 50.0,
            happiness: 50.0,
            health: 50.0,
        };
        assert_eq!(s.dominant_mood(), Mood::Neutral);
    }

    #[test]
    fn test_multipliers_affect_decay() {
        let mut s1 = Stats::new();
        let mut s2 = Stats::new();
        s1.tick(60.0, 1.0, 1.0, 1.0);
        s2.tick(60.0, 2.0, 1.0, 1.0);
        assert!(s2.hunger < s1.hunger); // 2x hunger decay
    }
}
