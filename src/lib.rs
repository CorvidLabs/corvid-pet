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
pub mod life_stage;
pub mod live;
pub mod moods;
pub mod needs;
pub mod personality;
pub mod persistence;
pub mod sim;
pub mod species;
pub mod stats;
pub mod styles;
pub mod templates;

pub use animations::{Animation, Spinner};
pub use life_stage::LifeStage;
pub use moods::Mood;
pub use needs::{InteractionResult, Need};
pub use persistence::PetState;
pub use personality::Personality;
pub use sim::SimState;
pub use species::Species;
pub use stats::Stats;
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
    /// Optional life simulation state. None = static pet (backwards compat).
    sim_state: Option<SimState>,
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
            sim_state: None,
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

    // -- Life simulation methods --

    /// Enables life simulation with the given personality.
    /// `now_secs` is the current unix timestamp.
    pub fn with_simulation(mut self, personality: Personality, now_secs: u64) -> Self {
        self.sim_state = Some(SimState::new(personality, now_secs));
        self
    }

    /// Advances the simulation clock. No-op if simulation is not enabled.
    pub fn tick(&mut self, now_secs: u64) {
        if let Some(sim) = &mut self.sim_state {
            sim.tick(now_secs);
            self.mood = sim.stats.dominant_mood();
        }
    }

    /// Feed the pet. Returns None if simulation is not enabled.
    pub fn feed(&mut self, now_secs: u64) -> Option<InteractionResult> {
        self.do_interact(Need::Feed, now_secs)
    }

    /// Play with the pet.
    pub fn play(&mut self, now_secs: u64) -> Option<InteractionResult> {
        self.do_interact(Need::Play, now_secs)
    }

    /// Let the pet rest.
    pub fn rest(&mut self, now_secs: u64) -> Option<InteractionResult> {
        self.do_interact(Need::Rest, now_secs)
    }

    /// Clean/groom the pet.
    pub fn clean(&mut self, now_secs: u64) -> Option<InteractionResult> {
        self.do_interact(Need::Clean, now_secs)
    }

    /// Give the pet quick affection.
    pub fn pet_me(&mut self, now_secs: u64) -> Option<InteractionResult> {
        self.do_interact(Need::Pet, now_secs)
    }

    /// Internal helper for interactions.
    fn do_interact(&mut self, need: Need, now_secs: u64) -> Option<InteractionResult> {
        let sim = self.sim_state.as_mut()?;
        let result = sim.interact(need, now_secs);
        if result.success {
            self.mood = sim.stats.dominant_mood();
        }
        Some(result)
    }

    /// Returns a reference to the current stats, if simulation is enabled.
    pub fn stats(&self) -> Option<&Stats> {
        self.sim_state.as_ref().map(|s| &s.stats)
    }

    /// Returns the current life stage, if simulation is enabled.
    pub fn life_stage(&self) -> Option<LifeStage> {
        self.sim_state.as_ref().map(|s| s.stage)
    }

    /// Returns the pet's personality, if simulation is enabled.
    pub fn pet_personality(&self) -> Option<Personality> {
        self.sim_state.as_ref().map(|s| s.personality)
    }

    /// Returns a reference to the full simulation state.
    pub fn sim(&self) -> Option<&SimState> {
        self.sim_state.as_ref()
    }

    /// Returns a human-readable age string, if simulation is enabled.
    pub fn age_display(&self) -> Option<String> {
        self.sim_state.as_ref().map(|s| s.age_display())
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

    // -- Simulation integration tests --

    #[test]
    fn test_pet_without_sim_returns_none() {
        let pet = Pet::new("Test".to_string(), Species::Crow);
        assert!(pet.stats().is_none());
        assert!(pet.life_stage().is_none());
        assert!(pet.pet_personality().is_none());
        assert!(pet.age_display().is_none());
    }

    #[test]
    fn test_pet_with_sim_starts_as_egg() {
        let pet = Pet::new("Pip".to_string(), Species::Crow)
            .with_simulation(Personality::Curious, 1000);
        assert_eq!(pet.life_stage(), Some(LifeStage::Egg));
        assert!(pet.stats().is_some());
        assert_eq!(pet.stats().unwrap().hunger, 100.0);
    }

    #[test]
    fn test_pet_tick_updates_mood() {
        let mut pet = Pet::new("Pip".to_string(), Species::Crow)
            .with_simulation(Personality::Curious, 1000);
        // Fast-forward past egg into hatchling, then let stats decay a lot
        pet.tick(1000 + 300); // Hatch
        pet.tick(1000 + 300 + 86400); // 24h of decay
        // With 24h of decay, hunger should be very low -> Sad mood
        assert_eq!(pet.mood(), Mood::Sad);
    }

    #[test]
    fn test_pet_feed_works() {
        let mut pet = Pet::new("Pip".to_string(), Species::Crow)
            .with_simulation(Personality::Curious, 1000);
        pet.tick(1400); // Past egg
        let result = pet.feed(1401);
        assert!(result.is_some());
        assert!(result.unwrap().success);
    }

    #[test]
    fn test_pet_feed_returns_none_without_sim() {
        let mut pet = Pet::new("Test".to_string(), Species::Crow);
        assert!(pet.feed(1000).is_none());
    }

    #[test]
    fn test_pet_sim_round_trip() {
        let pet = Pet::new("Pip".to_string(), Species::Crow)
            .with_simulation(Personality::Greedy, 1000);
        let state = PetState::from_pet(&pet);
        assert!(state.sim.is_some());
        let pet2 = state.to_pet();
        assert_eq!(pet2.pet_personality(), Some(Personality::Greedy));
        assert_eq!(pet2.life_stage(), Some(LifeStage::Egg));
    }
}
