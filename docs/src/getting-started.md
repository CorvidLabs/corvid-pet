# Getting Started

## Installation

Add corvid-pet to your `Cargo.toml`:

```toml
[dependencies]
corvid-pet = "1"
```

Or with optional features:

```toml
[dependencies]
corvid-pet = { version = "1", features = ["color", "persistence"] }
```

## Your First Pet

```rust
use corvid_pet::{Pet, Species, Mood, Event};

fn main() {
    // Create a crow named Corvin
    let mut pet = Pet::new("Corvin".to_string(), Species::Crow);

    // Display the pet
    println!("{}", pet.render());
    println!("  \"{}\"", pet.comment());

    // React to events
    pet.react(Event::Success);
    println!("Mood: {}", pet.mood()); // Happy

    pet.react(Event::Failure);
    println!("Mood: {}", pet.mood()); // Sad
}
```

## Events and Moods

Pets react to generic lifecycle events that any CLI tool can emit:

| Event | Resulting Mood | Use Case |
|-------|---------------|----------|
| `Success` | Happy | Tests passed, build succeeded, deploy complete |
| `Failure` | Sad | Tests failed, build broken, errors found |
| `Warning` | Confused | Linting warnings, deprecations, partial success |
| `Progress` | Excited | Step completed, file generated, milestone reached |
| `Idle` | Sleepy | Waiting for input, watching for changes |

## Running Examples

The repo includes several examples:

```bash
# Meet the crow + mood showcase
cargo run --example basic

# With colors
cargo run --example basic --features color

# Art style showcase
cargo run --example styles

# Life simulation (tamagotchi) demo
cargo run --example sim

# Save/load pet state
cargo run --example persistence --features persistence

# Custom art templates
cargo run --example template --features persistence

# Progress spinner animation
cargo run --example progress

# Interactive live mode
cargo run --example live --features live

# spec-sync integration
cargo run --example spec_buddy
```

## Next Steps

- [Life Simulation](./life-simulation.md) — Add stats, life stages, and personality
- [Custom Colors](./custom-colors.md) — ANSI color support
- [CLI Tool](./cli.md) — Interact with your pet from the terminal
- [GitHub Action](./github-action.md) — Add a pet to your CI/CD pipeline
