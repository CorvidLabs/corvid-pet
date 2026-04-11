# corvid-pet

ASCII corvid companions for CLI tools.

Bring a little personality to your command-line tools with animated ASCII pets that react to events, display mood-appropriate art, and provide charming commentary.

## Features

- **Crow Companion**: Clever problem-solver with unique art and personality
- **Six Moods**: Happy, Sad, Neutral, Confused, Excited, Sleepy
- **Minimal Art Style**: Compact ~6-line silhouettes with thought bubbles
- **Animations**: Blink and hop animations for bringing pets to life
- **Progress Spinners**: Animated companions for long-running operations
- **Life Simulation**: Tamagotchi-like system with stats, life stages, and personalities
- **Color Support**: ANSI colors (optional feature)
- **Persistence**: Save/load pet state (optional feature)
- **Live TUI Mode**: Interactive real-time pet experience (optional feature)
- **Custom Templates**: Define your own ASCII art

## Quick Start

```rust
use corvid_pet::{Pet, Species, Mood};

let pet = Pet::new("Corvin".to_string(), Species::Crow);
println!("{}", pet.render());
```

```
      _
    <(o\          .oO(hmm)
     |/(\
      \(\\
      "^`".
```

## Examples

```bash
# Meet the crow + mood showcase
cargo run --example basic

# With colors
cargo run --example basic --features color

# Minimal art style showcase
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

## Life Simulation

Optional tamagotchi-like system with hunger, energy, happiness, and health stats. Pets progress through life stages (Egg -> Hatchling -> Fledgling -> Adult -> Elder) and have personalities that affect their behavior.

```rust
use corvid_pet::{Pet, Species, Personality};

let now = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

let mut pet = Pet::new("Pip".to_string(), Species::Crow)
    .with_simulation(Personality::Curious, now);

pet.tick(now + 300);          // Time passes, egg hatches
pet.feed(now + 301);          // Feed the hatchling
pet.play(now + 400);          // Play together
println!("{}", pet.render()); // Mood reflects stats
```

## Feature Flags

- `color` — ANSI color support via [colored](https://crates.io/crates/colored)
- `persistence` — Save/load pet state to disk (uses serde + dirs)
- `live` — Real-time TUI mode (uses ratatui + crossterm + tokio)

```toml
[dependencies]
corvid-pet = { version = "0.1", features = ["color", "persistence"] }
```

## Integration with spec-sync

This crate integrates with [spec-sync](https://github.com/CorvidLabs/spec-sync) for visual feedback during spec validation:

```rust
use corvid_pet::integrations::specsync::{SpecSyncCompanion, ValidationOutcome};
use corvid_pet::Species;

let mut companion = SpecSyncCompanion::new(Species::Crow);
companion.react_to_validation(ValidationOutcome::Success);
println!("{}", companion.summary());
```

## Custom Art Templates

Define your own ASCII art via JSON templates:

```json
{
  "name": "My Custom Crow",
  "species": "crow",
  "moods": {
    "happy": "( ^ v ^ )",
    "sad": "( ; _ ; )"
  }
}
```

## License

MIT License - See [LICENSE](LICENSE) for details.
