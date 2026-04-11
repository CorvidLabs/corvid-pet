//! Interactions that satisfy pet needs.
//!
//! Each interaction modifies stats, respects cooldowns, and returns
//! personality-flavored feedback.

use crate::life_stage::LifeStage;
use crate::personality::Personality;
use crate::stats::Stats;

/// Interactions a user can perform on their pet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Need {
    /// Give the pet food. Restores hunger.
    Feed,
    /// Play a game. Restores happiness, costs energy.
    Play,
    /// Let the pet nap. Restores energy, costs hunger.
    Rest,
    /// Groom the pet. Restores health and a bit of happiness.
    Clean,
    /// Quick affection. Small happiness boost.
    Pet,
}

/// Result of attempting an interaction.
#[derive(Debug, Clone)]
pub struct InteractionResult {
    /// Whether the interaction succeeded.
    pub success: bool,
    /// Personality-flavored response message.
    pub message: String,
    /// Stat deltas that were applied (only meaningful if success is true).
    pub hunger_delta: f32,
    pub energy_delta: f32,
    pub happiness_delta: f32,
    pub health_delta: f32,
    /// Whether a life stage transition occurred during this tick.
    pub stage_changed: bool,
}

impl Need {
    /// Applies this interaction to the given stats, respecting personality and stage modifiers.
    /// Returns the stat deltas applied.
    pub fn apply(
        &self,
        stats: &mut Stats,
        personality: &Personality,
        _stage: &LifeStage,
    ) -> (f32, f32, f32, f32) {
        let modifier = personality.interaction_modifier(*self);

        let (dh, de, dhap, dheal) = match self {
            Need::Feed => (30.0 * modifier, 0.0, 0.0, 0.0),
            Need::Play => (0.0, -10.0, 25.0 * modifier, 0.0),
            Need::Rest => (-5.0, 35.0 * modifier, 0.0, 0.0),
            Need::Clean => (0.0, 0.0, 5.0 * modifier, 15.0 * modifier),
            Need::Pet => (0.0, 0.0, 10.0 * modifier, 0.0),
        };

        stats.hunger += dh;
        stats.energy += de;
        stats.happiness += dhap;
        stats.health += dheal;
        stats.clamp();

        (dh, de, dhap, dheal)
    }

    /// Cooldown in seconds before this interaction can be used again.
    pub fn cooldown_secs(&self) -> u64 {
        match self {
            Need::Feed => 60,
            Need::Play => 45,
            Need::Rest => 90,
            Need::Clean => 120,
            Need::Pet => 15,
        }
    }

    /// Short description of this interaction.
    pub fn description(&self) -> &str {
        match self {
            Need::Feed => "feed your pet",
            Need::Play => "play a game",
            Need::Rest => "let your pet nap",
            Need::Clean => "groom your pet",
            Need::Pet => "give quick affection",
        }
    }

    /// Returns a success message flavored by personality.
    pub fn success_message(&self, personality: &Personality) -> String {
        match (self, personality) {
            (Need::Feed, Personality::Greedy) => "NOM NOM NOM! More please!".to_string(),
            (Need::Feed, Personality::Mischievous) => {
                "Eats it... then steals yours too.".to_string()
            }
            (Need::Feed, Personality::Shy) => "Nibbles quietly... thank you.".to_string(),
            (Need::Feed, _) => "Munch munch! That hit the spot.".to_string(),

            (Need::Play, Personality::Mischievous) => {
                "CHAOS! Knocks everything over! Best game ever!".to_string()
            }
            (Need::Play, Personality::Curious) => {
                "Ooh, what's this? And this? And THAT?!".to_string()
            }
            (Need::Play, Personality::Shy) => "Peeks out... okay, that was fun.".to_string(),
            (Need::Play, _) => "Flaps around happily! Caw!".to_string(),

            (Need::Rest, Personality::Stoic) => "Rests. Efficiently.".to_string(),
            (Need::Rest, Personality::Affectionate) => {
                "Snuggles up close and dozes off...".to_string()
            }
            (Need::Rest, _) => "Tucks beak under wing... zzz...".to_string(),

            (Need::Clean, Personality::Mischievous) => {
                "Holds still for exactly 3 seconds.".to_string()
            }
            (Need::Clean, Personality::Affectionate) => {
                "Preens happily and nuzzles you!".to_string()
            }
            (Need::Clean, _) => "Ruffles feathers contentedly.".to_string(),

            (Need::Pet, Personality::Shy) => "...allows a brief head pat.".to_string(),
            (Need::Pet, Personality::Affectionate) => {
                "Leans into your hand! More pets!".to_string()
            }
            (Need::Pet, Personality::Stoic) => "Tolerates this. Barely.".to_string(),
            (Need::Pet, _) => "Happy little chirp!".to_string(),
        }
    }

    /// Returns a cooldown message.
    pub fn cooldown_message(&self) -> &str {
        match self {
            Need::Feed => "Not hungry yet...",
            Need::Play => "Still catching breath...",
            Need::Rest => "Not tired yet...",
            Need::Clean => "Already clean!",
            Need::Pet => "Give me a moment...",
        }
    }
}

impl std::fmt::Display for Need {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Need::Feed => write!(f, "Feed"),
            Need::Play => write!(f, "Play"),
            Need::Rest => write!(f, "Rest"),
            Need::Clean => write!(f, "Clean"),
            Need::Pet => write!(f, "Pet"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feed_restores_hunger() {
        let mut stats = Stats::new();
        stats.hunger = 50.0;
        let (dh, _, _, _) = Need::Feed.apply(&mut stats, &Personality::Curious, &LifeStage::Adult);
        assert!(dh > 0.0);
        assert!(stats.hunger > 50.0);
    }

    #[test]
    fn test_play_costs_energy() {
        let mut stats = Stats::new();
        let (_, de, dhap, _) =
            Need::Play.apply(&mut stats, &Personality::Curious, &LifeStage::Adult);
        assert!(de < 0.0);
        assert!(dhap > 0.0);
    }

    #[test]
    fn test_rest_costs_hunger() {
        let mut stats = Stats::new();
        let (dh, de, _, _) = Need::Rest.apply(&mut stats, &Personality::Curious, &LifeStage::Adult);
        assert!(dh < 0.0);
        assert!(de > 0.0);
    }

    #[test]
    fn test_greedy_feed_bonus() {
        let mut s1 = Stats::new();
        let mut s2 = Stats::new();
        s1.hunger = 50.0;
        s2.hunger = 50.0;
        Need::Feed.apply(&mut s1, &Personality::Curious, &LifeStage::Adult);
        Need::Feed.apply(&mut s2, &Personality::Greedy, &LifeStage::Adult);
        assert!(s2.hunger > s1.hunger);
    }

    #[test]
    fn test_stats_clamped() {
        let mut stats = Stats::new();
        // Already at 100, feeding should clamp
        Need::Feed.apply(&mut stats, &Personality::Curious, &LifeStage::Adult);
        assert!(stats.hunger <= 100.0);
    }

    #[test]
    fn test_cooldowns() {
        assert!(Need::Feed.cooldown_secs() > 0);
        assert!(Need::Pet.cooldown_secs() < Need::Clean.cooldown_secs());
    }

    #[test]
    fn test_success_messages_not_empty() {
        for need in [Need::Feed, Need::Play, Need::Rest, Need::Clean, Need::Pet] {
            for personality in [
                Personality::Curious,
                Personality::Shy,
                Personality::Mischievous,
                Personality::Stoic,
                Personality::Affectionate,
                Personality::Greedy,
            ] {
                assert!(!need.success_message(&personality).is_empty());
            }
        }
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Need::Feed), "Feed");
        assert_eq!(format!("{}", Need::Pet), "Pet");
    }
}
