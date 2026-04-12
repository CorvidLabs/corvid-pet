//! Custom art template system for corvid-pets.
//!
//! Allows users to define their own ASCII art for species and moods
//! via JSON or YAML template files.
//!
//! # Template Format
//!
//! ```json
//! {
//!   "name": "My Custom Crow",
//!   "species": "crow",
//!   "moods": {
//!     "happy": "art for happy...",
//!     "sad": "art for sad..."
//!   }
//! }
//! ```

use crate::{Mood, Species};
use std::collections::HashMap;

/// A custom art template.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct ArtTemplate {
    /// Template name.
    pub name: String,
    /// Species this template applies to.
    pub species: String,
    /// Art for each mood.
    pub moods: HashMap<String, String>,
    /// Art for closed eyes (animation).
    #[cfg_attr(feature = "persistence", serde(default))]
    pub closed_eyes: Option<String>,
    /// Art for open eyes (animation).
    #[cfg_attr(feature = "persistence", serde(default))]
    pub open_eyes: Option<String>,
}

impl ArtTemplate {
    /// Creates a new empty template.
    pub fn new(name: String, species: Species) -> Self {
        Self {
            name,
            species: species.to_string().to_lowercase(),
            moods: HashMap::new(),
            closed_eyes: None,
            open_eyes: None,
        }
    }

    /// Sets art for a specific mood.
    pub fn set_mood(&mut self, mood: Mood, art: String) {
        self.moods.insert(mood.to_string().to_lowercase(), art);
    }

    /// Gets art for a specific mood.
    pub fn get_mood(&self, mood: Mood) -> Option<&String> {
        self.moods.get(&mood.to_string().to_lowercase())
    }

    /// Renders the template for a species and mood.
    pub fn render(&self, species: Species, mood: Mood) -> Option<String> {
        // Check if this template applies to the species
        if self.species != species.to_string().to_lowercase() {
            return None;
        }

        self.get_mood(mood).cloned()
    }

    /// Loads a template from JSON.
    #[cfg(feature = "persistence")]
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Saves the template to JSON.
    #[cfg(feature = "persistence")]
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for ArtTemplate {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            species: "crow".to_string(),
            moods: HashMap::new(),
            closed_eyes: None,
            open_eyes: None,
        }
    }
}

/// Template registry that manages multiple templates.
#[derive(Debug, Clone, Default)]
pub struct TemplateRegistry {
    templates: Vec<ArtTemplate>,
    default_species_art: HashMap<Species, HashMap<Mood, String>>,
}

impl TemplateRegistry {
    /// Creates a new empty registry.
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),
            default_species_art: Self::build_defaults(),
        }
    }

    fn build_defaults() -> HashMap<Species, HashMap<Mood, String>> {
        use crate::moods;

        let mut map = HashMap::new();

        for species in [Species::Crow, Species::Magpie, Species::Raven, Species::Jackdaw] {
            let mut moods_map = HashMap::new();
            for mood in [
                Mood::Happy,
                Mood::Sad,
                Mood::Neutral,
                Mood::Confused,
                Mood::Excited,
                Mood::Sleepy,
            ] {
                moods_map.insert(mood, moods::ascii_art(species, mood));
            }
            map.insert(species, moods_map);
        }

        map
    }

    /// Registers a template.
    pub fn register(&mut self, template: ArtTemplate) {
        // Remove any existing template with same name/species
        self.templates
            .retain(|t| !(t.name == template.name && t.species == template.species));
        self.templates.push(template);
    }

    /// Finds a template by name.
    pub fn find(&self, name: &str) -> Option<&ArtTemplate> {
        self.templates.iter().find(|t| t.name == name)
    }

    /// Renders art for a species and mood, using templates if available.
    pub fn render(&self, species: Species, mood: Mood, template_name: Option<&str>) -> String {
        // Try specific template first
        if let Some(name) = template_name
            && let Some(template) = self.find(name)
            && let Some(art) = template.render(species, mood)
        {
            return art;
        }

        // Fall back to defaults
        self.default_species_art
            .get(&species)
            .and_then(|moods| moods.get(&mood))
            .cloned()
            .unwrap_or_else(|| "(no art)".to_string())
    }

    /// Lists all registered template names.
    pub fn list(&self) -> Vec<&String> {
        self.templates.iter().map(|t| &t.name).collect()
    }
}

/// Example template for a custom crow.
pub fn example_crow_template() -> ArtTemplate {
    let mut template = ArtTemplate::new("Cyber Crow".to_string(), Species::Crow);

    template.set_mood(
        Mood::Happy,
        r#"      __
    <(^ \  [OK]
     |/( \
      \( \\
      " "\\"#
            .to_string(),
    );

    template.set_mood(
        Mood::Sad,
        r#"      __
    <(; \  [ERR]
     |/( \
      \( \\
      " "\\"#
            .to_string(),
    );

    template.closed_eyes = Some(
        r#"      __
    <(- \
     |/( \
      \( \\
      " "\\"#
            .to_string(),
    );

    template
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_creation() {
        let template = ArtTemplate::new("Test".to_string(), Species::Crow);
        assert_eq!(template.name, "Test");
        assert_eq!(template.species, "crow");
    }

    #[test]
    fn test_set_get_mood() {
        let mut template = ArtTemplate::new("Test".to_string(), Species::Crow);
        template.set_mood(Mood::Happy, "happy art".to_string());

        assert_eq!(
            template.get_mood(Mood::Happy),
            Some(&"happy art".to_string())
        );
        assert_eq!(template.get_mood(Mood::Sad), None);
    }

    #[test]
    fn test_registry_render_default() {
        let registry = TemplateRegistry::new();
        let art = registry.render(Species::Crow, Mood::Happy, None);
        assert!(!art.is_empty());
        assert_ne!(art, "(no art)");
    }

    #[test]
    fn test_registry_custom_template() {
        let mut registry = TemplateRegistry::new();
        let template = example_crow_template();
        registry.register(template);

        let art = registry.render(Species::Crow, Mood::Happy, Some("Cyber Crow"));
        assert!(art.contains("[OK]"));
    }

    #[cfg(feature = "persistence")]
    #[test]
    fn test_json_roundtrip() {
        let template = example_crow_template();
        let json = template.to_json().unwrap();
        let loaded = ArtTemplate::from_json(&json).unwrap();

        assert_eq!(template.name, loaded.name);
        assert_eq!(template.species, loaded.species);
    }

    #[test]
    fn test_registry_list() {
        let mut registry = TemplateRegistry::new();
        registry.register(example_crow_template());

        let names: Vec<String> = registry.list().into_iter().cloned().collect();
        assert!(names.contains(&"Cyber Crow".to_string()));
    }
}
