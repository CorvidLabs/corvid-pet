//! Repo health tracking for corvid companions.
//!
//! Tracks CI/CD events over time and derives the pet's mood
//! from overall repository health — pass/fail streaks, warning
//! counts, and recent activity.
//!
//! Requires the `persistence` feature for JSON serialization.

use crate::{Event, Mood};

/// A single recorded CI/CD event.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct HealthEvent {
    /// The event type.
    pub event: String,
    /// Unix timestamp when this event occurred.
    pub timestamp: u64,
    /// Optional context (e.g. PR number, commit SHA, workflow name).
    #[cfg_attr(feature = "persistence", serde(default))]
    pub context: Option<String>,
}

/// Aggregated repo health state.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct RepoHealth {
    /// Pet name for this repo.
    pub pet_name: String,
    /// Total successful events.
    pub successes: u64,
    /// Total failed events.
    pub failures: u64,
    /// Total warnings.
    pub warnings: u64,
    /// Current consecutive success streak.
    pub streak: i64,
    /// Recent events (last 50).
    pub recent: Vec<HealthEvent>,
    /// Overall health score (0.0 to 100.0).
    pub score: f32,
}

impl RepoHealth {
    /// Creates a new health tracker with the given pet name.
    pub fn new(pet_name: String) -> Self {
        Self {
            pet_name,
            successes: 0,
            failures: 0,
            warnings: 0,
            streak: 0,
            recent: Vec::new(),
            score: 100.0,
        }
    }

    /// Records a new event and updates the health score.
    pub fn record(&mut self, event: Event, timestamp: u64, context: Option<String>) {
        let event_str = match event {
            Event::Success => "success",
            Event::Failure => "failure",
            Event::Warning => "warning",
            Event::Progress => "progress",
            Event::Idle => "idle",
        };

        self.recent.push(HealthEvent {
            event: event_str.to_string(),
            timestamp,
            context,
        });

        // Keep only last 50 events.
        if self.recent.len() > 50 {
            self.recent.drain(..self.recent.len() - 50);
        }

        match event {
            Event::Success => {
                self.successes += 1;
                if self.streak >= 0 {
                    self.streak += 1;
                } else {
                    self.streak = 1;
                }
            }
            Event::Failure => {
                self.failures += 1;
                if self.streak <= 0 {
                    self.streak -= 1;
                } else {
                    self.streak = -1;
                }
            }
            Event::Warning => {
                self.warnings += 1;
            }
            Event::Progress | Event::Idle => {}
        }

        self.recalculate_score();
    }

    /// Recalculates the health score from recent events.
    fn recalculate_score(&mut self) {
        if self.recent.is_empty() {
            self.score = 100.0;
            return;
        }

        // Weight recent events more heavily (last 10 events count double).
        let total = self.recent.len();
        let mut weighted_score = 0.0_f32;
        let mut weight_sum = 0.0_f32;

        for (i, evt) in self.recent.iter().enumerate() {
            let weight = if i >= total.saturating_sub(10) {
                2.0
            } else {
                1.0
            };

            let value = match evt.event.as_str() {
                "success" => 100.0,
                "failure" => 0.0,
                "warning" => 50.0,
                "progress" => 80.0,
                _ => 70.0, // idle
            };

            weighted_score += value * weight;
            weight_sum += weight;
        }

        self.score = if weight_sum > 0.0 {
            (weighted_score / weight_sum).clamp(0.0, 100.0)
        } else {
            100.0
        };
    }

    /// Derives the pet's mood from the current health state.
    pub fn mood(&self) -> Mood {
        if self.recent.is_empty() {
            return Mood::Neutral;
        }

        // Primarily based on score, with streak influence.
        match self.score as u32 {
            90..=100 => {
                if self.streak >= 5 {
                    Mood::Excited
                } else {
                    Mood::Happy
                }
            }
            70..=89 => Mood::Neutral,
            50..=69 => Mood::Confused,
            20..=49 => Mood::Sad,
            _ => Mood::Sad,
        }
    }

    /// Returns a human-readable health summary.
    pub fn summary(&self) -> String {
        let mood = self.mood();
        let streak_str = if self.streak > 0 {
            format!("{} consecutive passes", self.streak)
        } else if self.streak < 0 {
            format!("{} consecutive failures", self.streak.unsigned_abs())
        } else {
            "no streak".to_string()
        };

        format!(
            "Health: {:.0}/100 | Mood: {} | {} pass, {} fail, {} warn | Streak: {}",
            self.score, mood, self.successes, self.failures, self.warnings, streak_str
        )
    }

    /// Generates a markdown-formatted status for PR comments.
    pub fn pr_comment(&self, event: Event, context: &str) -> String {
        let pet = crate::Pet::new(self.pet_name.clone(), crate::Species::Crow);
        let mood = match event {
            Event::Success => Mood::Happy,
            Event::Failure => Mood::Sad,
            Event::Warning => Mood::Confused,
            Event::Progress => Mood::Excited,
            Event::Idle => Mood::Sleepy,
        };

        let art = crate::styles::ArtStyle::Minimal.render(crate::Species::Crow, mood);
        let comment = crate::comments::random_comment(crate::Species::Crow, mood);

        let status_emoji = match event {
            Event::Success => "✅",
            Event::Failure => "❌",
            Event::Warning => "⚠️",
            Event::Progress => "🔄",
            Event::Idle => "💤",
        };

        format!(
            "## {status_emoji} {name} says...\n\n```\n{art}\n```\n\n> *\"{comment}\"*\n\n{context}\n\n---\n<sub>Powered by [corvid-pet](https://github.com/CorvidLabs/corvid-pet)</sub>\n<!-- corvid-pet-report -->",
            name = pet.name(),
        )
    }

    /// Generates a short status line suitable for badges or README updates.
    pub fn badge_line(&self) -> String {
        let mood = self.mood();
        let emoji = match mood {
            Mood::Happy => "😊",
            Mood::Sad => "😢",
            Mood::Neutral => "😐",
            Mood::Confused => "🤔",
            Mood::Excited => "🎉",
            Mood::Sleepy => "😴",
        };
        format!("{} corvid-pet: {:.0}/100 ({})", emoji, self.score, mood)
    }

    /// Generates markdown for README embedding.
    pub fn readme_badge(&self) -> String {
        let mood = self.mood();
        let art = crate::styles::ArtStyle::Minimal.render(crate::Species::Crow, mood);
        let comment = crate::comments::random_comment(crate::Species::Crow, mood);

        format!(
            "<!-- corvid-pet:start -->\n```\n{art}\n```\n> *\"{comment}\"* — {name} (Health: {score:.0}/100)\n<!-- corvid-pet:end -->",
            name = self.pet_name,
            score = self.score,
        )
    }
}

impl Default for RepoHealth {
    fn default() -> Self {
        Self::new("Corvin".to_string())
    }
}

/// Loads health state from a JSON file path.
#[cfg(feature = "persistence")]
pub fn load_health(path: &std::path::Path) -> Result<RepoHealth, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;
    let health: RepoHealth = serde_json::from_str(&json)?;
    Ok(health)
}

/// Saves health state to a JSON file path.
#[cfg(feature = "persistence")]
pub fn save_health(
    health: &RepoHealth,
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(health)?;
    std::fs::write(path, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_health() {
        let h = RepoHealth::new("Test".to_string());
        assert_eq!(h.score, 100.0);
        assert_eq!(h.streak, 0);
        assert_eq!(h.mood(), Mood::Neutral); // No events yet
    }

    #[test]
    fn test_record_success() {
        let mut h = RepoHealth::new("Test".to_string());
        h.record(Event::Success, 1000, None);
        assert_eq!(h.successes, 1);
        assert_eq!(h.streak, 1);
        assert!(h.score >= 90.0);
    }

    #[test]
    fn test_record_failure() {
        let mut h = RepoHealth::new("Test".to_string());
        h.record(Event::Failure, 1000, None);
        assert_eq!(h.failures, 1);
        assert_eq!(h.streak, -1);
        assert!(h.score < 50.0);
    }

    #[test]
    fn test_streak_resets() {
        let mut h = RepoHealth::new("Test".to_string());
        h.record(Event::Success, 1000, None);
        h.record(Event::Success, 1001, None);
        assert_eq!(h.streak, 2);
        h.record(Event::Failure, 1002, None);
        assert_eq!(h.streak, -1);
    }

    #[test]
    fn test_mood_derivation() {
        let mut h = RepoHealth::new("Test".to_string());
        // All successes -> happy
        for i in 0..10 {
            h.record(Event::Success, 1000 + i, None);
        }
        assert!(matches!(h.mood(), Mood::Happy | Mood::Excited));

        // Many failures -> sad
        let mut h2 = RepoHealth::new("Test".to_string());
        for i in 0..10 {
            h2.record(Event::Failure, 1000 + i, None);
        }
        assert_eq!(h2.mood(), Mood::Sad);
    }

    #[test]
    fn test_summary_format() {
        let h = RepoHealth::new("Test".to_string());
        let s = h.summary();
        assert!(s.contains("Health:"));
        assert!(s.contains("Mood:"));
    }

    #[test]
    fn test_recent_capped_at_50() {
        let mut h = RepoHealth::new("Test".to_string());
        for i in 0..60 {
            h.record(Event::Success, 1000 + i, None);
        }
        assert_eq!(h.recent.len(), 50);
    }

    #[test]
    fn test_pr_comment_format() {
        let h = RepoHealth::new("Corvin".to_string());
        let comment = h.pr_comment(Event::Success, "All checks passed!");
        assert!(comment.contains("Corvin"));
        assert!(comment.contains("✅"));
        assert!(comment.contains("corvid-pet"));
    }

    #[test]
    fn test_readme_badge_format() {
        let h = RepoHealth::new("Corvin".to_string());
        let badge = h.readme_badge();
        assert!(badge.contains("corvid-pet:start"));
        assert!(badge.contains("corvid-pet:end"));
    }
}
