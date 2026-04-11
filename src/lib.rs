//! ASCII corvid companions for CLI tools.
//!
//! corvid-pet provides animated ASCII pets that react to events
//! and display mood-appropriate art and commentary.
//!
//! # Example
//!
//! ```
//! use corvid_pet::{Pet, Species, Mood};
//!
//! let pet = Pet::new("Corvin".to_string(), Species::Crow);
//! println!("{}", pet.render());
//! ```

pub mod animations;
pub mod art_v2;
pub mod color;
pub mod comments;
pub mod integrations;
pub mod live;
pub mod moods;
pub mod persistence;
pub mod species;
pub mod styles;
pub mod templates;

pub use animations::{Animation, Spinner};
pub use moods::Mood;
pub use persistence::PetState;
pub use species::Species;
pub use styles::ArtStyle;

/// Lifecycle events that can trigger pet reactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    /// Spec validation passed.
    SpecPassed,
    /// Spec validation failed.
    SpecFailed,
    /// Validation produced warnings.
    ValidationWarning,
    /// A new spec was generated.
    NewSpecGenerated,
    /// System is idle.
    Idle,
}

/// The main corvid companion.
#[derive(Debug, Clone)]
pub struct Pet {
    name: String,
    species: Species,
    mood: Mood,
}

impl Pet {
    /// Creates a new pet with the given name and species.
    ///
    /// Defaults to Neutral mood. If name is empty, uses species default.
    pub fn new(name: String, species: Species) -> Self {
        let name = if name.is_empty() {
            species.default_name()
        } else {
            name
        };

        Self {
            name,
            species,
            mood: Mood::Neutral,
        }
    }

    /// Returns the pet's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the pet's species.
    pub fn species(&self) -> Species {
        self.species
    }

    /// Returns the pet's current mood.
    pub fn mood(&self) -> Mood {
        self.mood
    }

    /// Returns ASCII art for the current species and mood.
    /// Uses the clean minimal style by default.
    pub fn render(&self) -> String {
        styles::ArtStyle::Minimal.render(self.species, self.mood)
    }

    /// Returns ASCII art using a specific style.
    pub fn render_with_style(&self, style: styles::ArtStyle) -> String {
        style.render(self.species, self.mood)
    }

    /// Returns colored ASCII art when the `color` feature is enabled.
    /// Falls back to plain ASCII art when disabled.
    pub fn render_colored(&self) -> String {
        let art = self.render();
        color::colorize(&art, self.species)
    }

    /// Returns colored ASCII art using a specific style.
    pub fn render_colored_with_style(&self, style: styles::ArtStyle) -> String {
        let art = style.render(self.species, self.mood);
        color::colorize(&art, self.species)
    }

    /// Changes the pet's mood.
    pub fn set_mood(&mut self, mood: Mood) {
        self.mood = mood;
    }

    /// Returns a random mood/species-appropriate comment.
    pub fn comment(&self) -> String {
        comments::random_comment(self.species, self.mood)
    }

    /// Returns a blink animation iterator.
    pub fn animate_blink(&self) -> Animation {
        Animation::blink(self.species, self.mood)
    }

    /// Returns a hop animation iterator.
    pub fn animate_hop(&self) -> Animation {
        Animation::hop(self.species, self.mood)
    }

    /// Creates a progress spinner with this pet.
    pub fn spinner(&self, message: &str) -> Spinner {
        Spinner::new(self.species, self.mood, message.to_string())
    }

    /// Reacts to an event by changing mood appropriately.
    pub fn react(&mut self, event: Event) {
        let new_mood = match event {
            Event::SpecPassed => Mood::Happy,
            Event::SpecFailed => Mood::Sad,
            Event::ValidationWarning => Mood::Confused,
            Event::NewSpecGenerated => Mood::Excited,
            Event::Idle => Mood::Sleepy,
        };
        self.set_mood(new_mood);
    }
}

impl Default for Pet {
    fn default() -> Self {
        Self::new(String::new(), Species::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pet_creation() {
        let pet = Pet::new("Test".to_string(), Species::Crow);
        assert_eq!(pet.name(), "Test");
        assert_eq!(pet.species(), Species::Crow);
        assert_eq!(pet.mood(), Mood::Neutral);
    }

    #[test]
    fn test_default_name() {
        let pet = Pet::new(String::new(), Species::Raven);
        assert_eq!(pet.name(), "Nevermore");
    }

    #[test]
    fn test_set_mood() {
        let mut pet = Pet::new("Test".to_string(), Species::Crow);
        pet.set_mood(Mood::Happy);
        assert_eq!(pet.mood(), Mood::Happy);
    }

    #[test]
    fn test_react_spec_passed() {
        let mut pet = Pet::new("Test".to_string(), Species::Crow);
        pet.react(Event::SpecPassed);
        assert_eq!(pet.mood(), Mood::Happy);
    }

    #[test]
    fn test_react_spec_failed() {
        let mut pet = Pet::new("Test".to_string(), Species::Crow);
        pet.react(Event::SpecFailed);
        assert_eq!(pet.mood(), Mood::Sad);
    }

    #[test]
    fn test_react_validation_warning() {
        let mut pet = Pet::new("Test".to_string(), Species::Crow);
        pet.react(Event::ValidationWarning);
        assert_eq!(pet.mood(), Mood::Confused);
    }

    #[test]
    fn test_react_new_spec() {
        let mut pet = Pet::new("Test".to_string(), Species::Crow);
        pet.react(Event::NewSpecGenerated);
        assert_eq!(pet.mood(), Mood::Excited);
    }

    #[test]
    fn test_react_idle() {
        let mut pet = Pet::new("Test".to_string(), Species::Crow);
        pet.react(Event::Idle);
        assert_eq!(pet.mood(), Mood::Sleepy);
    }

    #[test]
    fn test_render_not_empty() {
        let pet = Pet::new("Test".to_string(), Species::Crow);
        let art = pet.render();
        assert!(!art.is_empty());
        assert!(art.contains("<(") || art.contains("_"));
    }

    #[test]
    fn test_comment_not_empty() {
        let pet = Pet::new("Test".to_string(), Species::Crow);
        let comment = pet.comment();
        assert!(!comment.is_empty());
    }

    #[test]
    fn test_animation_frames() {
        let pet = Pet::new("Test".to_string(), Species::Crow);
        let mut anim = pet.animate_blink();
        let frame1 = anim.next_frame();
        assert!(frame1.is_some());
        let frame2 = anim.next_frame();
        assert!(frame2.is_some());
    }

    #[test]
    fn test_animation_is_finished() {
        let pet = Pet::new("Test".to_string(), Species::Crow);
        let mut anim = pet.animate_blink();
        // Blink has 4 frames
        anim.next_frame();
        anim.next_frame();
        anim.next_frame();
        anim.next_frame();
        assert!(anim.is_finished());
        assert!(anim.next_frame().is_none());
    }

    #[test]
    fn test_spinner_tick() {
        let pet = Pet::new("Test".to_string(), Species::Crow);
        let mut spinner = pet.spinner("Testing...");
        let frame1 = spinner.current_frame();
        spinner.tick();
        let frame2 = spinner.current_frame();
        assert_ne!(frame1, frame2);
    }

    #[test]
    fn test_spinner_finish() {
        let pet = Pet::new("Test".to_string(), Species::Crow);
        let mut spinner = pet.spinner("Testing...");
        let finished = spinner.finish();
        assert!(finished.contains('✓'));
    }

    #[test]
    fn test_species_display() {
        assert_eq!(format!("{}", Species::Crow), "Crow");
        assert_eq!(format!("{}", Species::Raven), "Raven");
        assert_eq!(format!("{}", Species::Magpie), "Magpie");
        assert_eq!(format!("{}", Species::Jay), "Jay");
    }

    #[test]
    fn test_mood_display() {
        assert_eq!(format!("{}", Mood::Happy), "Happy");
        assert_eq!(format!("{}", Mood::Sad), "Sad");
    }

    #[test]
    fn test_default_pet() {
        let pet = Pet::default();
        assert_eq!(pet.species(), Species::Crow);
        assert_eq!(pet.mood(), Mood::Neutral);
    }
}
