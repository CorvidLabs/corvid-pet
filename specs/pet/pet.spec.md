---
module: pet
version: 1
status: draft
files:
  - src/lib.rs
  - src/species.rs
  - src/moods.rs
  - src/animations.rs
  - src/comments.rs
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
| `Animation` | Iterator over animation frames |
| `Spinner` | Progress indicator with animated pet |

### Exported Pet Methods

| Method | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `new` | `name: String, species: Species` | `Self` | Create a new pet companion |
| `render` | `&self` | `String` | Get ASCII art for current species and mood |
| `set_mood` | `&mut self, mood: Mood` | `()` | Change pet's emotional state |
| `comment` | `&self` | `String` | Get random mood/species-appropriate quip |
| `animate_blink` | `&self` | `Animation` | Iterator yielding blink animation frames |
| `animate_hop` | `&self` | `Animation` | Iterator yielding hop animation frames |
| `spinner` | `&self, message: &str` | `Spinner` | Create progress spinner with pet animation |
| `react` | `&mut self, event: Event` | `()` | Auto-set mood based on event |

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

## Invariants

1. `Pet::render()` always returns ASCII art ≤ 15 lines height and ≤ 40 chars width
2. `Pet::comment()` always returns a quip appropriate for current species and mood
3. `Pet::react()` maps events to moods consistently: SpecPassed→Happy, SpecFailed→Sad, ValidationWarning→Confused, NewSpecGenerated→Excited, Idle→Sleepy
4. `Animation` iterator yields at least 2 frames and at most 10 frames per animation
5. `Spinner::tick()` advances through frames cyclically until `finish()` is called
6. All ASCII art uses only printable ASCII characters (no Unicode, no ANSI codes in stored art)
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
