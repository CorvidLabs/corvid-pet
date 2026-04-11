//! Personality traits that make each pet unique.
//!
//! Personality affects stat decay rates, interaction effectiveness,
//! and the flavor of dialogue/quips.

use crate::needs::Need;

/// Personality traits for corvid pets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Personality {
    /// Asks questions, explores. Play is more effective.
    #[default]
    Curious,
    /// Quiet, prefers calm. Rest is more effective.
    Shy,
    /// Gets into trouble. Play very effective, feed less so.
    Mischievous,
    /// Unfazed, dry humor. All decay slightly reduced.
    Stoic,
    /// Loves attention. Happiness decays faster alone, all interactions boosted.
    Affectionate,
    /// Food-obsessed. Hunger decays faster, feed very effective.
    Greedy,
}

impl Personality {
    /// Returns a random personality.
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..6) {
            0 => Personality::Curious,
            1 => Personality::Shy,
            2 => Personality::Mischievous,
            3 => Personality::Stoic,
            4 => Personality::Affectionate,
            _ => Personality::Greedy,
        }
    }

    /// Additive modifier to hunger decay rate (per minute).
    pub fn hunger_modifier(&self) -> f32 {
        match self {
            Personality::Mischievous => 0.2 / 60.0,
            Personality::Stoic => -0.1 / 60.0,
            Personality::Greedy => 0.3 / 60.0,
            _ => 0.0,
        }
    }

    /// Additive modifier to energy decay rate (per minute).
    pub fn energy_modifier(&self) -> f32 {
        match self {
            Personality::Stoic => -0.1 / 60.0,
            _ => 0.0,
        }
    }

    /// Additive modifier to happiness decay rate (per minute).
    pub fn happiness_modifier(&self) -> f32 {
        match self {
            Personality::Curious => -0.1 / 60.0,
            Personality::Shy => -0.1 / 60.0,
            Personality::Stoic => -0.1 / 60.0,
            Personality::Affectionate => 0.2 / 60.0,
            _ => 0.0,
        }
    }

    /// Multiplier on interaction effectiveness for a given need.
    pub fn interaction_modifier(&self, need: Need) -> f32 {
        match (self, need) {
            (Personality::Curious, Need::Play) => 1.2,
            (Personality::Shy, Need::Rest) => 1.3,
            (Personality::Mischievous, Need::Play) => 1.5,
            (Personality::Mischievous, Need::Feed) => 0.8,
            (Personality::Stoic, _) => 0.9,
            (Personality::Affectionate, _) => 1.2,
            (Personality::Greedy, Need::Feed) => 1.5,
            _ => 1.0,
        }
    }

    /// Short description of this personality.
    pub fn description(&self) -> &str {
        match self {
            Personality::Curious => "curious and inquisitive",
            Personality::Shy => "quiet and gentle",
            Personality::Mischievous => "playful troublemaker",
            Personality::Stoic => "calm and unfazed",
            Personality::Affectionate => "loving and social",
            Personality::Greedy => "food-obsessed gourmand",
        }
    }
}

impl std::fmt::Display for Personality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Personality::Curious => write!(f, "Curious"),
            Personality::Shy => write!(f, "Shy"),
            Personality::Mischievous => write!(f, "Mischievous"),
            Personality::Stoic => write!(f, "Stoic"),
            Personality::Affectionate => write!(f, "Affectionate"),
            Personality::Greedy => write!(f, "Greedy"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_is_valid() {
        // Just ensure it doesn't panic.
        for _ in 0..20 {
            let _ = Personality::random();
        }
    }

    #[test]
    fn test_greedy_hungry() {
        assert!(Personality::Greedy.hunger_modifier() > 0.0);
    }

    #[test]
    fn test_stoic_reduces_all() {
        assert!(Personality::Stoic.hunger_modifier() < 0.0);
        assert!(Personality::Stoic.energy_modifier() < 0.0);
        assert!(Personality::Stoic.happiness_modifier() < 0.0);
    }

    #[test]
    fn test_greedy_feed_bonus() {
        assert!(Personality::Greedy.interaction_modifier(Need::Feed) > 1.0);
    }

    #[test]
    fn test_mischievous_play_bonus() {
        assert!(Personality::Mischievous.interaction_modifier(Need::Play) > 1.0);
    }

    #[test]
    fn test_mischievous_feed_penalty() {
        assert!(Personality::Mischievous.interaction_modifier(Need::Feed) < 1.0);
    }

    #[test]
    fn test_default_is_curious() {
        assert_eq!(Personality::default(), Personality::Curious);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Personality::Shy), "Shy");
        assert_eq!(format!("{}", Personality::Greedy), "Greedy");
    }
}
