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
  - src/health.rs
  - src/live.rs
  - src/stats.rs
  - src/life_stage.rs
  - src/personality.rs
  - src/needs.rs
  - src/sim.rs
  - src/bin/corvid-pet.rs
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
| `ColorScheme` | `color` |
| `PetColor` | `color` |
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
| `RepoHealth` | `health` |

### Exported Modules

| Module | Description |
|--------|-------------|
| `animations` | Animation and spinner types |
| `color` | ANSI color support and custom color schemes |
| `comments` | Random species/mood quips |
| `health` | Repo health tracking from CI/CD events |
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
| `Species` | Corvid species: Crow (clever, problem solver). Default: Crow |
| `Mood` | Emotional states: Happy, Sad, Neutral, Confused, Excited, Sleepy. Default: Neutral |
| `Event` | Lifecycle events: Success, Failure, Warning, Progress, Idle |
| `PetColor` | Named ANSI colors: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, plus bright variants. Implements Display, FromStr |
| `PersistenceError` | Error enum for persistence operations: NoDataDir, Io, Serde |
| `ValidationOutcome` | Spec validation result: Success, Warning, Failure, Generated, Idle |

### Exported Structs

| Type | Description |
|------|-------------|
| `Pet` | The main companion with name, species, and mood |
| `ColorScheme` | User-configurable color scheme with body and bubble colors |
| `ColorSchemeData` | Serializable color scheme data for persistence (body/bubble as strings) |
| `HealthEvent` | A single recorded CI/CD event with type, timestamp, and optional context |
| `RepoHealth` | Aggregated repo health state tracking CI events, score, streak, and mood |
| `ArtStyle` | Art rendering style enum (`Minimal`). Default: Minimal |
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
| `fmt` | (Display trait) | `fmt::Result` | Display as "Crow" |

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

### Color Module

#### PetColor Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `random` | `()` | `Self` | Returns a random color |
| `fmt` | (Display trait) | `fmt::Result` | Display as lowercase name (e.g. "red", "bright-blue") |
| `from_str` | (FromStr trait) | `Result<Self, String>` | Parse from string, accepts aliases ("purple" → Magenta, "gray" → BrightBlack) |

`PetColor::ALL` — constant slice of all 16 color variants.

#### ColorScheme Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `new` | `body: PetColor, bubble: PetColor` | `Self` | Create a new color scheme |
| `default_for` | `species: Species` | `Self` | Default scheme for species (Crow: blue body, cyan bubble) |
| `random` | `()` | `Self` | Random body and bubble colors |

#### Color Functions

| Function | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `color::colorize` | `art: &str, species: Species` | `String` | Apply ANSI colors by species. No-op without `color` feature |
| `color::colorize_with_scheme` | `art: &str, scheme: &ColorScheme` | `String` | Apply ANSI colors using custom scheme. No-op without `color` feature |

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

### Health Module

#### RepoHealth Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `new` | `pet_name: String` | `Self` | Create health tracker (starts at score 100) |
| `record` | `&mut self, event: Event, timestamp: u64, context: Option<String>` | `()` | Record a CI/CD event, update score and streak |
| `mood` | `&self` | `Mood` | Derive mood from health score (90+: Happy/Excited, 70-89: Neutral, 50-69: Confused, <50: Sad) |
| `summary` | `&self` | `String` | Human-readable health summary line |
| `pr_comment` | `&self, event: Event, context: &str` | `String` | Markdown PR comment with pet art, quote, and health stats |
| `badge_line` | `&self` | `String` | Short status line with emoji for badges |
| `readme_badge` | `&self` | `String` | Markdown block for README embedding (between corvid-pet:start/end comments) |
| `default` | (Default trait) | `Self` | Default health (pet name "Corvin", score 100) |

#### Health Functions (require `persistence` feature)

| Function | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `health::load_health` | `path: &Path` | `Result<RepoHealth, Box<dyn Error>>` | Load health state from JSON file |
| `health::save_health` | `health: &RepoHealth, path: &Path` | `Result<(), Box<dyn Error>>` | Save health state to JSON file |

### CLI Binary (`src/bin/corvid-pet.rs`)

Command-line interface for interacting with corvid pets. Built with clap. See `cli.spec.md` for the full CLI specification including all flags, arguments, and behavior.

| Command | Description |
|---------|-------------|
| `show` | Display the pet's ASCII art (default command) |
| `react` | Record a CI/CD event and update health state |
| `health` | Show repo health summary (text or JSON) |
| `comment` | Generate a markdown PR comment |
| `badge` | Generate a README badge section |
| `init` | Initialize a new health state file |
| `greet` | Greet with a random corvid message |

Global flags: `--name`, `--no-color`, `--color`, `--bubble-color`, `--random-colors`.

### Art Styles

The library ships with one built-in art style:

- **Minimal** (default): Compact ~6-line crow silhouette with thought bubbles. The crow has a distinctive `<(` beak shape.

Custom art can be provided via the `ArtTemplate` and `TemplateRegistry` types in the `templates` module.

## Invariants

1. `Pet::render()` always returns ASCII art ≤ 15 lines height and ≤ 40 chars width
2. `Pet::comment()` always returns a quip appropriate for current species and mood
3. `Pet::react()` maps events to moods consistently: Success→Happy, Failure→Sad, Warning→Confused, Progress→Excited, Idle→Sleepy
4. `Animation` iterator yields at least 2 frames and at most 10 frames per animation
5. `Spinner::tick()` advances through frames cyclically until `finish()` is called
6. All ASCII art in the Minimal style uses only printable ASCII characters (no Unicode, no ANSI codes in stored art)
7. `Species::default_name()` returns the default name: "Corvin" for Crow

## Behavioral Examples

### Scenario: Create and display a happy crow

- **Given** a new pet created with `Pet::new("Corvin".to_string(), Species::Crow)`
- **When** `pet.render()` is called
- **Then** returns ASCII art of a crow in neutral mood

### Scenario: React to success event

- **Given** a pet in neutral mood
- **When** `pet.react(Event::Success)` is called
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
| 2026-04-11 | v3: Strip to Crow-only with Minimal style — removed art_v2 module, Raven/Magpie/Jay species, Detailed art style |
