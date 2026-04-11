---
module: pet
version: 2
status: active
files:
  - src/lib.rs
  - src/species.rs
  - src/moods.rs
  - src/animations.rs
  - src/comments.rs
  - src/styles.rs
  - src/templates.rs
  - src/color.rs
  - src/persistence.rs
  - src/live.rs
  - src/stats.rs
  - src/life_stage.rs
  - src/personality.rs
  - src/needs.rs
  - src/sim.rs
  - src/integrations/mod.rs
  - src/integrations/specsync.rs
db_tables: []
depends_on: []
---

# Pet

## Purpose

ASCII corvid companion library for CLI tools. Provides animated ASCII pets that react to events and display mood-appropriate art and commentary. Designed to integrate with developer tools to provide charming, helpful feedback during long-running operations.

## Public API

### Re-exports (lib.rs)

The crate root re-exports these types for convenience:

| Type | Source Module |
|------|--------------|
| `Animation` | `animations` |
| `Spinner` | `animations` |
| `LifeStage` | `life_stage` |
| `Mood` | `moods` |
| `InteractionResult` | `needs` |
| `Need` | `needs` |
| `PetState` | `persistence` |
| `Personality` | `personality` |
| `SimState` | `sim` |
| `Species` | `species` |
| `Stats` | `stats` |
| `ArtStyle` | `styles` |

### Exported Modules

| Module | Description |
|--------|-------------|
| `animations` | Animation and spinner types |
| `color` | ANSI color support |
| `comments` | Random species/mood quips |
| `integrations` | Third-party integrations |
| `life_stage` | Life stage progression |
| `live` | Real-time TUI display |
| `moods` | Mood-specific ASCII art |
| `needs` | Interaction system |
| `persistence` | Save/load pet state |
| `personality` | Personality traits |
| `sim` | Simulation state machine |
| `species` | Corvid species definitions |
| `stats` | Vital statistics |
| `styles` | Art rendering styles |
| `templates` | Custom art templates |
| `specsync` | SpecSync companion integration |


### Exported Enums

| Type | Description |
|------|-------------|
| `Species` | Corvid species: Crow (clever, problem solver), Raven (wise, ominous), Magpie (shiny-obsessed), Jay (loud, opinionated). Default: Crow |
| `Mood` | Emotional states: Happy, Sad, Neutral, Confused, Excited, Sleepy. Default: Neutral |
| `Event` | Lifecycle events: SpecPassed, SpecFailed, ValidationWarning, NewSpecGenerated, Idle |
| `PersistenceError` | Error enum for persistence operations: NoDataDir, Io, Serde |
| `ValidationOutcome` | Spec validation result: Success, Warning, Failure, Generated, Idle |

### Exported Structs

| Type | Description |
|------|-------------|
| `Pet` | The main companion with name, species, and mood |
| `ArtStyle` | Art rendering style enum (`Minimal`, `Detailed`). Default: Minimal |
| `Animation` | Iterator over animation frames |
| `Spinner` | Progress indicator with animated pet |
| `Stats` | Vital statistics: hunger, energy, happiness, health |
| `LifeStage` | Life progression: Egg, Hatchling, Fledgling, Adult, Elder |
| `Personality` | Trait affecting behavior: Curious, Shy, Mischievous, Stoic, Affectionate, Greedy |
| `Need` | Interactions: Feed, Play, Rest, Clean, Pet |
| `SimState` | Full simulation state coordinating stats, stages, personality, needs |
| `InteractionResult` | Result of performing an interaction |
| `ArtTemplate` | Custom art template with mood-specific ASCII art |
| `TemplateRegistry` | Registry of art templates for rendering |
| `PetState` | Serializable snapshot of pet state for persistence |
| `SimStateData` | Serializable simulation data for persistence |
| `LivePetApp` | Async TUI application for real-time pet interaction |
| `SimpleLivePet` | Simple synchronous interactive pet display |
| `SpecSyncCompanion` | Pet companion that reacts to spec validation outcomes |

### Pet Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `new` | `name: String, species: Species` | `Self` | Create a new pet companion. Empty name uses species default |
| `name` | `&self` | `&str` | Returns the pet's name |
| `species` | `&self` | `Species` | Returns the pet's species |
| `mood` | `&self` | `Mood` | Returns the pet's current mood |
| `render` | `&self` | `String` | Get ASCII art for current species and mood (uses Minimal style) |
| `render_with_style` | `&self, style: ArtStyle` | `String` | Get ASCII art using a specific style |
| `render_colored` | `&self` | `String` | Get colored ASCII art (requires `color` feature) |
| `render_colored_with_style` | `&self, style: ArtStyle` | `String` | Get colored ASCII art using a specific style |
| `set_mood` | `&mut self, mood: Mood` | `()` | Change pet's emotional state |
| `comment` | `&self` | `String` | Get random mood/species-appropriate quip |
| `animate_blink` | `&self` | `Animation` | Iterator yielding blink animation frames |
| `animate_hop` | `&self` | `Animation` | Iterator yielding hop animation frames |
| `spinner` | `&self, message: &str` | `Spinner` | Create progress spinner with pet animation |
| `react` | `&mut self, event: Event` | `()` | Auto-set mood based on event |
| `with_simulation` | `self, personality: Personality, now_secs: u64` | `Self` | Enable life simulation |
| `tick` | `&mut self, now_secs: u64` | `()` | Advance simulation clock |
| `feed` | `&mut self, now_secs: u64` | `Option<InteractionResult>` | Feed the pet |
| `play` | `&mut self, now_secs: u64` | `Option<InteractionResult>` | Play with the pet |
| `rest` | `&mut self, now_secs: u64` | `Option<InteractionResult>` | Let the pet rest |
| `clean` | `&mut self, now_secs: u64` | `Option<InteractionResult>` | Groom the pet |
| `pet_me` | `&mut self, now_secs: u64` | `Option<InteractionResult>` | Quick affection |
| `stats` | `&self` | `Option<&Stats>` | Current vital statistics |
| `life_stage` | `&self` | `Option<LifeStage>` | Current life stage |
| `pet_personality` | `&self` | `Option<Personality>` | Pet's personality trait |
| `sim` | `&self` | `Option<&SimState>` | Full simulation state |
| `age_display` | `&self` | `Option<String>` | Human-readable age |
| `default` | (Default trait) | `Self` | Creates a default pet (Crow, empty name, Neutral mood) |

### Species Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `default_name` | `&self` | `String` | Get default name for species |
| `personality` | `&self` | `&str` | Get personality description |
| `fmt` | (Display trait) | `fmt::Result` | Display as "Crow", "Raven", "Magpie", "Jay" |

### Mood Display

`Mood` implements `Display`, rendering as "Happy", "Sad", "Neutral", "Confused", "Excited", "Sleepy".

### Moods Module Functions

| Function | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `moods::ascii_art` | `species: Species, mood: Mood` | `String` | Returns species+mood-specific ASCII art |
| `moods::ascii_art_open_eyes` | `species: Species, mood: Mood` | `String` | Returns open-eye variant for animations |
| `moods::ascii_art_closed_eyes` | `species: Species, mood: Mood` | `String` | Returns closed-eye variant for animations |

### Comments Module Functions

| Function | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `comments::random_comment` | `species: Species, mood: Mood` | `String` | Returns a random mood/species-appropriate quip |

### Animation Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `blink` | `species: Species, mood: Mood` | `Self` | Create a blink animation (4 frames) |
| `hop` | `species: Species, mood: Mood` | `Self` | Create a hop animation (6 frames) |
| `next_frame` | `&mut self` | `Option<String>` | Get next animation frame |
| `is_finished` | `&self` | `bool` | Check if animation completed |
| `next` | (Iterator trait) | `Option<String>` | Iterator impl, delegates to `next_frame` |

### Spinner Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `new` | `species: Species, mood: Mood, message: String` | `Self` | Create a new spinner |
| `tick` | `&mut self` | `()` | Advance spinner animation |
| `set_message` | `&mut self, message: &str` | `()` | Update spinner message |
| `current_frame` | `&self` | `String` | Return current frame with message |
| `finish` | `&mut self` | `String` | Return final frame with completion message |
| `finish_with_pet` | `&mut self` | `String` | Return pet render with completion |

### ArtStyle Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `render` | `&self, species: Species, mood: Mood` | `String` | Render ASCII art in this style |
| `name` | `&self` | `&'static str` | Returns the style name ("minimal") |
| `fmt` | (Display trait) | `fmt::Result` | Display as style name |
| `from_str` | (FromStr trait) | `Result<Self, String>` | Parse from string ("minimal") |

### Color Module Functions

| Function | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `color::colorize` | `art: &str, species: Species` | `String` | Apply ANSI colors by species. No-op without `color` feature |

### Templates Module

#### ArtTemplate

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `new` | `name: String, species: Species` | `Self` | Create empty template |
| `set_mood` | `&mut self, mood: Mood, art: String` | `()` | Set art for a mood |
| `get_mood` | `&self, mood: Mood` | `Option<&String>` | Get art for a mood |
| `render` | `&self, species: Species, mood: Mood` | `Option<String>` | Render if species matches |
| `from_json` | `json: &str` | `Result<Self, Error>` | Load from JSON (requires `persistence` feature) |
| `to_json` | `&self` | `Result<String, Error>` | Save to JSON (requires `persistence` feature) |
| `default` | (Default trait) | `Self` | Default empty crow template |

ArtTemplate fields: `name: String`, `species: String`, `moods: HashMap<String, String>`, `closed_eyes: Option<String>`, `open_eyes: Option<String>`.

#### TemplateRegistry

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `new` | `()` | `Self` | Create registry with default species art |
| `register` | `&mut self, template: ArtTemplate` | `()` | Register a template (replaces existing with same name/species) |
| `find` | `&self, name: &str` | `Option<&ArtTemplate>` | Find template by name |
| `render` | `&self, species, mood, template_name: Option<&str>` | `String` | Render with optional template override |
| `list` | `&self` | `Vec<&String>` | List registered template names |

#### Standalone Functions

| Function | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `example_crow_template` | `()` | `ArtTemplate` | Example "Cyber Crow" template |

### Persistence Module

#### PetState

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `from_pet` | `pet: &Pet` | `Self` | Snapshot pet state for serialization |
| `to_pet` | `&self` | `Pet` | Reconstruct pet from saved state |
| `default` | (Default trait) | `Self` | Default state (Crow, Neutral) |

PetState fields: `name: String`, `species: String`, `mood: String`, `interaction_count: u64`, `last_saved: Option<u64>`, `sim: Option<SimStateData>`.

#### SimStateData

Serializable simulation data. Fields: `hunger: f32`, `energy: f32`, `happiness: f32`, `health: f32`, `stage: String`, `personality: String`, `age_secs: f64`, `interaction_count: u64`, `born_at: u64`, `last_tick: u64`, `cooldowns: Vec<(String, u64)>`.

#### PersistenceError

Error enum with variants: `NoDataDir`, `Io(std::io::Error)` (persistence feature), `Serde(serde_json::Error)` (persistence feature). Implements `Display` and `Error`.

#### Persistence Functions (require `persistence` feature)

| Function | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `data_dir` | `()` | `Option<PathBuf>` | Default storage directory |
| `save_pet` | `state: &PetState, id: &str` | `Result<(), PersistenceError>` | Save pet state to disk |
| `load_pet` | `id: &str` | `Result<PetState, PersistenceError>` | Load pet state from disk |
| `list_pets` | `()` | `Result<Vec<String>, PersistenceError>` | List saved pet IDs |
| `delete_pet` | `id: &str` | `Result<(), PersistenceError>` | Delete a saved pet |

### Live Module

#### LivePetApp

Async TUI application for real-time pet interaction. Requires `live` feature.

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `new` | `pet: Pet` | `Self` | Create live pet app |
| `run` | `&mut self` | `Result<(), Box<dyn Error>>` | Run the TUI event loop (async) |

#### SimpleLivePet

Simpler synchronous alternative for interactive pet display.

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `new` | `pet: Pet` | `Self` | Create simple live pet |
| `run_interactive` | `&mut self` | `Result<(), Box<dyn Error>>` | Run interactive display (requires `live` feature) |

### Integrations Module

#### SpecSyncCompanion (integrations::specsync)

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `new` | `species: Species` | `Self` | Create companion with default name |
| `with_name` | `name: String, species: Species` | `Self` | Create companion with custom name |
| `pet` | `&self` | `&Pet` | Reference to underlying pet |
| `pet_mut` | `&mut self` | `&mut Pet` | Mutable reference to underlying pet |
| `react_to_validation` | `&mut self, outcome: ValidationOutcome` | `()` | React to a validation outcome |
| `react_to_results` | `&mut self, errors: usize, warnings: usize` | `()` | React to error/warning counts |
| `validation_count` | `&self` | `usize` | Number of validations processed |
| `last_outcome` | `&self` | `Option<ValidationOutcome>` | Last validation outcome |
| `summary` | `&self` | `String` | Summary message based on mood |
| `default` | (Default trait) | `Self` | Default companion (Crow) |

#### ValidationOutcome (integrations::specsync)

| Variant | Description |
|---------|-------------|
| `Success` | All specs passed |
| `Warning` | Warnings but no errors |
| `Failure` | One or more specs failed |
| `Generated` | New specs generated |
| `Idle` | No activity |

#### Standalone Functions (integrations::specsync)

| Function | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `create_validation_spinner` | `pet: &Pet` | `Spinner` | Create spinner with mood-appropriate message |

### Art Styles

The library ships with one built-in art style:

- **Minimal** (default): Compact ~6-line species silhouettes with thought bubbles. Each species has a unique shape (crow has `<(` beak, raven has `>` beak, magpie has `*` markings with tail feathers, jay has crest `/\/\`).

Custom art can be provided via the `ArtTemplate` and `TemplateRegistry` types in the `templates` module.

## Invariants

1. `Pet::render()` always returns ASCII art ≤ 15 lines height and ≤ 40 chars width
2. `Pet::comment()` always returns a quip appropriate for current species and mood
3. `Pet::react()` maps events to moods consistently: SpecPassed→Happy, SpecFailed→Sad, ValidationWarning→Confused, NewSpecGenerated→Excited, Idle→Sleepy
4. `Animation` iterator yields at least 2 frames and at most 10 frames per animation
5. `Spinner::tick()` advances through frames cyclically until `finish()` is called
6. All ASCII art in the Minimal style uses only printable ASCII characters (no Unicode, no ANSI codes in stored art)
7. `Species::default_name()` returns unique names per species: "Corvin", "Nevermore", "Shiny", "Jay"

## Behavioral Examples

### Scenario: Create and display a happy crow

- **Given** a new pet created with `Pet::new("Corvin".to_string(), Species::Crow)`
- **When** `pet.render()` is called
- **Then** returns ASCII art of a crow in neutral mood

### Scenario: React to spec pass

- **Given** a pet in neutral mood
- **When** `pet.react(Event::SpecPassed)` is called
- **Then** pet's mood changes to `Mood::Happy`
- **And** `pet.comment()` returns a happy crow-themed quip

### Scenario: Animation frames

- **Given** a crow pet
- **When** `pet.animate_blink()` is created and iterated
- **Then** yields frame sequence: open eyes → closed eyes → open eyes

### Scenario: Spinner progress

- **Given** a pet with spinner created via `pet.spinner("Checking specs...")`
- **When** `spinner.tick()` is called repeatedly
- **Then** cycles through animation frames with message
- **And** `spinner.finish()` returns completion message with pet

## Error Cases

| Condition | Behavior |
|-----------|----------|
| Empty name string | `Pet::new` uses species default name |
| Unknown mood transition | Maintains current mood (no change) |
| Animation exhausted | Iterator yields `None` |

## Dependencies

### Consumes

| Crate | What is used |
|-------|-------------|
| rand | Random comment selection |

### Consumed By

| Project | What is used |
|---------|-------------|
| spec-sync | Progress indication, validation feedback |

## Change Log

| Date | Change |
|------|--------|
| 2026-04-02 | Initial spec draft |
| 2026-04-11 | Updated to review status; documented single Minimal style; added ArtStyle, render_with_style, render_colored to API; added art_v2 and templates modules to file list |
| 2026-04-11 | Added life simulation system: Stats, LifeStage, Personality, Need, SimState. Pet gains optional simulation with tick/interact API. Full backwards compatibility |
| 2026-04-11 | v2: Comprehensive API documentation — added all undocumented exports: Pet accessor methods, moods/comments/color/art_v2 module functions, templates module types, persistence module types and functions, live module types, integrations/specsync module types, Display/Default/FromStr trait impls |
