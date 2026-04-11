//! spec-sync integration for corvid-pet.
//!
//! This module provides integration with the spec-sync tool, mapping
//! validation results to pet reactions and providing TUI components.
//!
//! # Example
//!
//! ```no_run
//! use corvid_pet::{Pet, Species};
//! use corvid_pet::integrations::specsync::{SpecSyncCompanion, ValidationOutcome};
//!
//! let mut companion = SpecSyncCompanion::new(Species::Crow);
//! companion.react_to_validation(ValidationOutcome::Success);
//! println!("{}", companion.pet().render());
//! ```

use crate::{Event, Mood, Pet, Species};

/// Represents the outcome of a spec-sync validation run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationOutcome {
    /// All specs passed validation.
    Success,
    /// Some specs have warnings but no errors.
    Warning,
    /// One or more specs failed validation.
    Failure,
    /// New specs were generated.
    Generated,
    /// No activity (idle state).
    Idle,
}

/// A companion that integrates with spec-sync validation workflows.
#[derive(Debug, Clone)]
pub struct SpecSyncCompanion {
    pet: Pet,
    validation_count: usize,
    last_outcome: Option<ValidationOutcome>,
}

impl SpecSyncCompanion {
    /// Creates a new companion with the given species.
    pub fn new(species: Species) -> Self {
        Self {
            pet: Pet::new(String::new(), species),
            validation_count: 0,
            last_outcome: None,
        }
    }

    /// Creates a new companion with a custom name and species.
    pub fn with_name(name: String, species: Species) -> Self {
        Self {
            pet: Pet::new(name, species),
            validation_count: 0,
            last_outcome: None,
        }
    }

    /// Returns a reference to the underlying pet.
    pub fn pet(&self) -> &Pet {
        &self.pet
    }

    /// Returns a mutable reference to the underlying pet.
    pub fn pet_mut(&mut self) -> &mut Pet {
        &mut self.pet
    }

    /// Reacts to a validation outcome.
    pub fn react_to_validation(&mut self, outcome: ValidationOutcome) {
        self.last_outcome = Some(outcome);
        self.validation_count += 1;

        let event = match outcome {
            ValidationOutcome::Success => Event::SpecPassed,
            ValidationOutcome::Warning => Event::ValidationWarning,
            ValidationOutcome::Failure => Event::SpecFailed,
            ValidationOutcome::Generated => Event::NewSpecGenerated,
            ValidationOutcome::Idle => Event::Idle,
        };

        self.pet.react(event);
    }

    /// Reacts to validation results (convenience method for spec-sync integration).
    ///
    /// Takes the number of errors and warnings and determines the appropriate outcome.
    pub fn react_to_results(&mut self, errors: usize, warnings: usize) {
        let outcome = if errors > 0 {
            ValidationOutcome::Failure
        } else if warnings > 0 {
            ValidationOutcome::Warning
        } else {
            ValidationOutcome::Success
        };
        self.react_to_validation(outcome);
    }

    /// Returns the number of validations processed.
    pub fn validation_count(&self) -> usize {
        self.validation_count
    }

    /// Returns the last validation outcome.
    pub fn last_outcome(&self) -> Option<ValidationOutcome> {
        self.last_outcome
    }

    /// Returns a summary message based on current state.
    pub fn summary(&self) -> String {
        match self.pet.mood() {
            Mood::Happy => format!("✓ {} specs validated successfully!", self.validation_count),
            Mood::Sad => "✗ Validation failed. Check the errors above.".to_string(),
            Mood::Confused => "⚠ Validation passed with warnings.".to_string(),
            Mood::Excited => "✨ New specs generated!".to_string(),
            Mood::Sleepy => "💤 Waiting for changes...".to_string(),
            Mood::Neutral => "Ready to validate specs.".to_string(),
        }
    }
}

impl Default for SpecSyncCompanion {
    fn default() -> Self {
        Self::new(Species::Crow)
    }
}

/// Creates a spinner for spec-sync validation progress.
///
/// Returns a spinner with an appropriate message based on the current mood.
pub fn create_validation_spinner(pet: &Pet) -> crate::Spinner {
    let message = match pet.mood() {
        Mood::Happy => "Validating specs...",
        Mood::Sad => "Checking for errors...",
        Mood::Confused => "Reviewing warnings...",
        Mood::Excited => "Generating specs...",
        Mood::Sleepy => "Watching for changes...",
        Mood::Neutral => "Running validation...",
    };

    pet.spinner(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_companion_creation() {
        let companion = SpecSyncCompanion::new(Species::Crow);
        assert_eq!(companion.pet().species(), Species::Crow);
        assert_eq!(companion.validation_count(), 0);
        assert!(companion.last_outcome().is_none());
    }

    #[test]
    fn test_companion_with_name() {
        let companion = SpecSyncCompanion::with_name("Test".to_string(), Species::Crow);
        assert_eq!(companion.pet().name(), "Test");
        assert_eq!(companion.pet().species(), Species::Crow);
    }

    #[test]
    fn test_react_to_success() {
        let mut companion = SpecSyncCompanion::new(Species::Crow);
        companion.react_to_validation(ValidationOutcome::Success);
        assert_eq!(companion.pet().mood(), Mood::Happy);
        assert_eq!(companion.validation_count(), 1);
        assert_eq!(companion.last_outcome(), Some(ValidationOutcome::Success));
    }

    #[test]
    fn test_react_to_failure() {
        let mut companion = SpecSyncCompanion::new(Species::Crow);
        companion.react_to_validation(ValidationOutcome::Failure);
        assert_eq!(companion.pet().mood(), Mood::Sad);
    }

    #[test]
    fn test_react_to_warning() {
        let mut companion = SpecSyncCompanion::new(Species::Crow);
        companion.react_to_validation(ValidationOutcome::Warning);
        assert_eq!(companion.pet().mood(), Mood::Confused);
    }

    #[test]
    fn test_react_to_results() {
        let mut companion = SpecSyncCompanion::new(Species::Crow);

        // Success case
        companion.react_to_results(0, 0);
        assert_eq!(companion.pet().mood(), Mood::Happy);

        // Warning case
        companion.react_to_results(0, 3);
        assert_eq!(companion.pet().mood(), Mood::Confused);

        // Failure case
        companion.react_to_results(2, 0);
        assert_eq!(companion.pet().mood(), Mood::Sad);
    }

    #[test]
    fn test_react_to_results_mixed() {
        let mut companion = SpecSyncCompanion::new(Species::Crow);
        // Errors take precedence over warnings
        companion.react_to_results(2, 3);
        assert_eq!(companion.pet().mood(), Mood::Sad);
    }

    #[test]
    fn test_summary_messages() {
        let mut companion = SpecSyncCompanion::new(Species::Crow);

        companion.react_to_validation(ValidationOutcome::Success);
        assert!(companion.summary().contains("validated"));

        companion.react_to_validation(ValidationOutcome::Failure);
        assert!(companion.summary().contains("failed"));

        companion.react_to_validation(ValidationOutcome::Warning);
        assert!(companion.summary().contains("warnings"));
    }

    #[test]
    fn test_default_companion() {
        let companion = SpecSyncCompanion::default();
        assert_eq!(companion.pet().species(), Species::Crow);
    }
}
