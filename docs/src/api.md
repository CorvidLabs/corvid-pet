# API Overview

This page summarizes the main types and methods. For full API documentation, see the [rustdoc](https://corvidlabs.github.io/corvid-pet/rustdoc/corvid_pet/).

## Core Types

### Pet

The main companion struct. Always available, no feature flags required.

```rust
use corvid_pet::{Pet, Species, Mood, Event};

let mut pet = Pet::new("Corvin".to_string(), Species::Crow);

pet.render();              // ASCII art string
pet.render_with_style(s);  // Render with a specific ArtStyle
pet.set_mood(Mood::Happy); // Change mood
pet.comment();             // Random quip
pet.react(Event::Success); // Set mood from event
pet.mood();                // Current mood
pet.name();                // Pet's name
pet.species();             // Pet's species

// Animations
pet.animate_blink();       // Blink animation iterator
pet.animate_hop();         // Hop animation iterator
pet.spinner("msg");        // Progress spinner
```

### Species

Currently: `Species::Crow`. Default name: "Corvin".

### Mood

`Happy`, `Sad`, `Neutral`, `Confused`, `Excited`, `Sleepy`. Default: `Neutral`.

### Event

`Success`, `Failure`, `Warning`, `Progress`, `Idle`.

## Simulation Types

Available on any `Pet` via `with_simulation()`:

- **`Stats`** — hunger, energy, happiness, health (all `f32`, 0-100)
- **`LifeStage`** — Egg, Hatchling, Fledgling, Adult, Elder
- **`Personality`** — Curious, Shy, Mischievous, Stoic, Affectionate, Greedy
- **`Need`** — Feed, Play, Rest, Clean, Pet
- **`SimState`** — Full simulation state
- **`InteractionResult`** — Result of an interaction

## Feature-Gated Types

### `color` feature

- **`PetColor`** — 16 ANSI colors with `Display`, `FromStr`
- **`ColorScheme`** — Body + bubble color pair
- `pet.render_colored()` / `pet.render_colored_with_style(s)`
- `color::colorize()` / `color::colorize_with_scheme()`

### `persistence` feature

- **`PetState`** — Serializable pet snapshot
- **`SimStateData`** — Serializable simulation data
- **`PersistenceError`** — Error type
- `persistence::save_pet()` / `load_pet()` / `list_pets()` / `delete_pet()`

### `live` feature

- **`LivePetApp`** — Async TUI app (ratatui + tokio)
- **`SimpleLivePet`** — Synchronous interactive display

### `cli` feature

- Enables the `corvid-pet` binary (clap + chrono + color + persistence)

## Health Tracking

- **`RepoHealth`** — Aggregated repo health state
- **`HealthEvent`** — Single CI/CD event record
- `health.record()` / `health.mood()` / `health.summary()`
- `health.pr_comment()` / `health.badge_line()` / `health.readme_badge()`

## Modules

| Module | Description |
|--------|-------------|
| `animations` | Animation and spinner types |
| `color` | ANSI color support |
| `comments` | Random quips |
| `health` | Repo health tracking |
| `integrations::specsync` | spec-sync companion |
| `life_stage` | Life stage progression |
| `live` | Real-time TUI |
| `moods` | Mood-specific ASCII art |
| `needs` | Interaction system |
| `persistence` | Save/load state |
| `personality` | Personality traits |
| `sim` | Simulation state machine |
| `species` | Species definitions |
| `stats` | Vital statistics |
| `styles` | Art rendering styles |
| `templates` | Custom art templates |
