//! Simulation state that ties stats, life stages, personality, and needs together.
//!
//! `SimState` is the central simulation coordinator. It tracks time, applies
//! decay, handles stage transitions, manages cooldowns, and processes interactions.

use std::collections::HashMap;

use crate::life_stage::LifeStage;
use crate::needs::{InteractionResult, Need};
use crate::personality::Personality;
use crate::stats::Stats;

/// Complete simulation state for a living pet.
#[derive(Debug, Clone)]
pub struct SimState {
    /// Current vital statistics.
    pub stats: Stats,
    /// Current life stage.
    pub stage: LifeStage,
    /// Personality trait.
    pub personality: Personality,
    /// Total age in seconds.
    pub age_secs: f64,
    /// Total number of interactions performed.
    pub interaction_count: u64,
    /// Unix timestamp of last tick.
    pub last_tick: u64,
    /// Cooldowns: need -> unix timestamp when available again.
    pub cooldowns: HashMap<Need, u64>,
    /// Unix timestamp of creation.
    pub born_at: u64,
}

impl SimState {
    /// Creates a new simulation state with the given personality.
    /// `now_secs` is the current unix timestamp.
    pub fn new(personality: Personality, now_secs: u64) -> Self {
        Self {
            stats: Stats::new(),
            stage: LifeStage::Egg,
            personality,
            age_secs: 0.0,
            interaction_count: 0,
            last_tick: now_secs,
            cooldowns: HashMap::new(),
            born_at: now_secs,
        }
    }

    /// Advances the simulation to the given timestamp.
    ///
    /// Applies stat decay based on elapsed time, checks for stage transitions,
    /// and updates the mood accordingly. Idempotent for the same timestamp.
    pub fn tick(&mut self, now_secs: u64) {
        if now_secs <= self.last_tick {
            return;
        }

        let elapsed = (now_secs - self.last_tick) as f64;
        self.age_secs += elapsed;
        self.last_tick = now_secs;

        // Check for stage transition.
        let new_stage = LifeStage::for_age(self.age_secs);
        if new_stage != self.stage {
            self.stage = new_stage;
        }

        // Apply stat decay with stage and personality modifiers.
        if self.stage != LifeStage::Egg {
            let hunger_mult = self.stage.hunger_multiplier();
            let energy_mult = self.stage.energy_multiplier();
            let happiness_mult = self.stage.happiness_multiplier();

            // Apply base decay.
            self.stats.tick(elapsed, hunger_mult, energy_mult, happiness_mult);

            // Apply personality additive modifiers (already per-second).
            let dt = elapsed.min(86400.0) as f32;
            self.stats.hunger -= self.personality.hunger_modifier() * dt;
            self.stats.energy -= self.personality.energy_modifier() * dt;
            self.stats.happiness -= self.personality.happiness_modifier() * dt;
            self.stats.clamp();
        }
    }

    /// Attempts an interaction. Returns the result.
    pub fn interact(&mut self, need: Need, now_secs: u64) -> InteractionResult {
        // Advance time first.
        self.tick(now_secs);

        // Check if still an egg.
        if !self.stage.can_interact() {
            return InteractionResult {
                success: false,
                message: "Still an egg... be patient!".to_string(),
                hunger_delta: 0.0,
                energy_delta: 0.0,
                happiness_delta: 0.0,
                health_delta: 0.0,
                stage_changed: false,
            };
        }

        // Check cooldown.
        if let Some(&available_at) = self.cooldowns.get(&need)
            && now_secs < available_at
        {
            return InteractionResult {
                success: false,
                message: need.cooldown_message().to_string(),
                hunger_delta: 0.0,
                energy_delta: 0.0,
                happiness_delta: 0.0,
                health_delta: 0.0,
                stage_changed: false,
            };
        }

        // Apply the interaction.
        let old_stage = self.stage;
        let (dh, de, dhap, dheal) = need.apply(&mut self.stats, &self.personality, &self.stage);

        // Set cooldown.
        self.cooldowns.insert(need, now_secs + need.cooldown_secs());
        self.interaction_count += 1;

        // Check if stage changed after interaction.
        let stage_changed = self.stage != old_stage;

        InteractionResult {
            success: true,
            message: need.success_message(&self.personality),
            hunger_delta: dh,
            energy_delta: de,
            happiness_delta: dhap,
            health_delta: dheal,
            stage_changed,
        }
    }

    /// Checks if an interaction is available (not on cooldown and not an egg).
    pub fn can_interact(&self, need: Need, now_secs: u64) -> bool {
        if !self.stage.can_interact() {
            return false;
        }
        match self.cooldowns.get(&need) {
            Some(&available_at) => now_secs >= available_at,
            None => true,
        }
    }

    /// Returns progress (0.0-1.0) through the current life stage.
    pub fn stage_progress(&self) -> f32 {
        self.stage.progress(self.age_secs)
    }

    /// Whether the pet is "alive" (health > 0).
    pub fn is_alive(&self) -> bool {
        self.health() > 0.0
    }

    /// Shortcut to current health.
    pub fn health(&self) -> f32 {
        self.stats.health
    }

    /// One-line status summary.
    pub fn status_summary(&self) -> String {
        format!(
            "{} {} | H:{:.0} E:{:.0} J:{:.0} HP:{:.0} | {}",
            self.stage,
            self.personality,
            self.stats.hunger,
            self.stats.energy,
            self.stats.happiness,
            self.stats.health,
            self.age_display(),
        )
    }

    /// Human-readable age display.
    pub fn age_display(&self) -> String {
        let total_secs = self.age_secs as u64;
        if total_secs < 60 {
            format!("{}s old", total_secs)
        } else if total_secs < 3600 {
            format!("{}m old", total_secs / 60)
        } else if total_secs < 86400 {
            let h = total_secs / 3600;
            let m = (total_secs % 3600) / 60;
            format!("{}h {}m old", h, m)
        } else {
            let d = total_secs / 86400;
            let h = (total_secs % 86400) / 3600;
            format!("{}d {}h old", d, h)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now() -> u64 {
        1000000 // Arbitrary fixed timestamp for tests.
    }

    #[test]
    fn test_new_sim() {
        let sim = SimState::new(Personality::Curious, now());
        assert_eq!(sim.stage, LifeStage::Egg);
        assert_eq!(sim.stats.hunger, 100.0);
        assert_eq!(sim.age_secs, 0.0);
        assert_eq!(sim.born_at, now());
    }

    #[test]
    fn test_tick_advances_age() {
        let mut sim = SimState::new(Personality::Curious, now());
        sim.tick(now() + 60);
        assert!((sim.age_secs - 60.0).abs() < 0.01);
    }

    #[test]
    fn test_tick_idempotent() {
        let mut sim = SimState::new(Personality::Curious, now());
        sim.tick(now() + 60);
        let age1 = sim.age_secs;
        sim.tick(now() + 60); // Same timestamp
        assert_eq!(sim.age_secs, age1);
    }

    #[test]
    fn test_tick_past_timestamp_noop() {
        let mut sim = SimState::new(Personality::Curious, now());
        sim.tick(now() + 60);
        let age1 = sim.age_secs;
        sim.tick(now() + 30); // Past timestamp
        assert_eq!(sim.age_secs, age1);
    }

    #[test]
    fn test_egg_no_decay() {
        let mut sim = SimState::new(Personality::Curious, now());
        sim.tick(now() + 200); // Still in egg stage (< 300s)
        assert_eq!(sim.stats.hunger, 100.0);
        assert_eq!(sim.stats.energy, 100.0);
    }

    #[test]
    fn test_egg_hatches() {
        let mut sim = SimState::new(Personality::Curious, now());
        sim.tick(now() + 300);
        assert_eq!(sim.stage, LifeStage::Hatchling);
    }

    #[test]
    fn test_decay_after_hatch() {
        let mut sim = SimState::new(Personality::Curious, now());
        sim.tick(now() + 360); // 60s past hatch
        assert!(sim.stats.hunger < 100.0);
    }

    #[test]
    fn test_cant_interact_as_egg() {
        let mut sim = SimState::new(Personality::Curious, now());
        let result = sim.interact(Need::Feed, now() + 10);
        assert!(!result.success);
        assert!(result.message.contains("egg"));
    }

    #[test]
    fn test_feed_interaction() {
        let mut sim = SimState::new(Personality::Curious, now());
        sim.tick(now() + 400); // Past egg stage
        sim.stats.hunger = 50.0;
        let result = sim.interact(Need::Feed, now() + 401);
        assert!(result.success);
        assert!(sim.stats.hunger > 50.0);
    }

    #[test]
    fn test_cooldown_enforced() {
        let mut sim = SimState::new(Personality::Curious, now());
        sim.tick(now() + 400);
        sim.stats.hunger = 50.0;

        let r1 = sim.interact(Need::Feed, now() + 401);
        assert!(r1.success);

        let r2 = sim.interact(Need::Feed, now() + 420); // Only 19s later, cooldown is 60s
        assert!(!r2.success);
    }

    #[test]
    fn test_cooldown_expires() {
        let mut sim = SimState::new(Personality::Curious, now());
        sim.tick(now() + 400);
        sim.stats.hunger = 30.0;

        sim.interact(Need::Feed, now() + 401);
        let r2 = sim.interact(Need::Feed, now() + 462); // 61s later
        assert!(r2.success);
    }

    #[test]
    fn test_can_interact_checks() {
        let sim = SimState::new(Personality::Curious, now());
        assert!(!sim.can_interact(Need::Feed, now())); // Egg
    }

    #[test]
    fn test_interaction_count() {
        let mut sim = SimState::new(Personality::Curious, now());
        sim.tick(now() + 400);
        sim.interact(Need::Feed, now() + 401);
        sim.interact(Need::Play, now() + 402);
        assert_eq!(sim.interaction_count, 2);
    }

    #[test]
    fn test_status_summary() {
        let sim = SimState::new(Personality::Curious, now());
        let summary = sim.status_summary();
        assert!(summary.contains("Egg"));
        assert!(summary.contains("Curious"));
    }

    #[test]
    fn test_age_display() {
        let mut sim = SimState::new(Personality::Curious, now());
        sim.age_secs = 30.0;
        assert_eq!(sim.age_display(), "30s old");

        sim.age_secs = 120.0;
        assert_eq!(sim.age_display(), "2m old");

        sim.age_secs = 7500.0;
        assert_eq!(sim.age_display(), "2h 5m old");

        sim.age_secs = 90000.0;
        assert_eq!(sim.age_display(), "1d 1h old");
    }

    #[test]
    fn test_full_lifecycle() {
        let mut sim = SimState::new(Personality::Greedy, now());

        // Egg
        assert_eq!(sim.stage, LifeStage::Egg);

        // Hatch
        sim.tick(now() + 300);
        assert_eq!(sim.stage, LifeStage::Hatchling);

        // Fledgling
        sim.tick(now() + 2100);
        assert_eq!(sim.stage, LifeStage::Fledgling);

        // Adult
        sim.tick(now() + 9300);
        assert_eq!(sim.stage, LifeStage::Adult);

        // Elder
        sim.tick(now() + 86400);
        assert_eq!(sim.stage, LifeStage::Elder);
    }
}
