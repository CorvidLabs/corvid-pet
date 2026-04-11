---
module: pet
version: 1
status: active
files:
  - src/lib.rs
  - src/species.rs
  - src/moods.rs
  - src/animations.rs
  - src/comments.rs
  - src/styles.rs
  - src/art_v2.rs
  - src/templates.rs
  - src/color.rs
  - src/persistence.rs
  - src/live.rs
  - src/stats.rs
  - src/life_stage.rs
  - src/personality.rs
  - src/needs.rs
  - src/sim.rs
db_tables: []
depends_on: []
---

# Pet

## Purpose

ASCII corvid companion library for CLI tools. Provides animated ASCII pets that react to events and display mood-appropriate art and commentary. Designed to integrate with developer tools to provide charming, helpful feedback during long-running operations.

## Public API

### Exported Enums

| Type | Description |
|------|-------------|
| `Species` | Corvid species: Crow (clever, problem solver), Raven (wise, ominous), Magpie (shiny-obsessed), Jay (loud, opinionated) |
| `Mood` | Emotional states: Happy, Sad, Neutral, Confused, Excited, Sleepy |
| `Event` | Lifecycle events: SpecPassed, SpecFailed, ValidationWarning, NewSpecGenerated, Idle |

### Exported Structs

| Type | Description |
|------|-------------|
| `Pet` | The main companion with name, species, and mood |
| `ArtStyle` | Art rendering style enum (currently only `Minimal`) |
| `Animation` | Iterator over animation frames |
| `Spinner` | Progress indicator with animated pet |
| `Stats` | Vital statistics: hunger, energy, happiness, health |
| `LifeStage` | Life progression: Egg, Hatchling, Fledgling, Adult, Elder |
| `Personality` | Trait affecting behavior: Curious, Shy, Mischievous, Stoic, Affectionate, Greedy |
| `Need` | Interactions: Feed, Play, Rest, Clean, Pet |
| `SimState` | Full simulation state coordinating stats, stages, personality, needs |
| `InteractionResult` | Result of performing an interaction |

### Exported Pet Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `new` | `name: String, species: Species` | `Self` | Create a new pet companion |
| `render` | `&self` | `String` | Get ASCII art for current species and mood (uses Minimal style) |
| `render_with_style` | `&self, style: ArtStyle` | `String` | Get ASCII art using a specific style |
| `render_colored` | `&self` | `String` | Get colored ASCII art (requires `color` feature) |
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

### Exported Species Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `default_name` | `&self` | `String` | Get default name for species |
| `personality` | `&self` | `&str` | Get personality description |

### Exported Animation Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `next_frame` | `&mut self` | `Option<String>` | Get next animation frame |
| `is_finished` | `&self` | `bool` | Check if animation completed |

### Exported Spinner Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `tick` | `&mut self` | `()` | Advance spinner animation |
| `set_message` | `&mut self, message: &str` | `()` | Update spinner message |
| `finish` | `&mut self` | `String` | Return final frame with completion message |
| `finish_with_pet` | `&mut self` | `String` | Return pet render with completion |

### Art Style

The library ships with a single art style: **Minimal**. This is a compact crow silhouette with a thought bubble that changes based on mood. The Minimal style renders the same crow art regardless of species — species-specific art is planned for future styles.

An `art_v2` module provides species-differentiated ASCII art (crow, raven, magpie, jay) for use by custom templates or future styles, but is not exposed through the `ArtStyle` enum.

Custom art can be provided via the `ArtTemplate` and `TemplateRegistry` types in the `templates` module.

## Invariants

1. `Pet::render()` always returns ASCII art ≤ 15 lines height and ≤ 40 chars width
2. `Pet::comment()` always returns a quip appropriate for current species and mood
3. `Pet::react()` maps events to moods consistently: SpecPassed→Happy, SpecFailed→Sad, ValidationWarning→Confused, NewSpecGenerated→Excited, Idle→Sleepy
4. `Animation` iterator yields at least 2 frames and at most 10 frames per animation
5. `Spinner::tick()` advances through frames cyclically until `finish()` is called
6. All ASCII art in the Minimal style uses only printable ASCII characters (no Unicode, no ANSI codes in stored art). The `art_v2` module may use Unicode when `use_unicode: true` is passed
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
