# corvid-pet

ASCII corvid companions for CLI tools.

Bring a little personality to your command-line tools with animated ASCII pets that react to events, display mood-appropriate art, and provide charming commentary.

## Features

- **Four Species**: Crow, Raven, Magpie, Jay — each with unique personalities
- **Six Moods**: Happy, Sad, Neutral, Confused, Excited, Sleepy
- **Animations**: Blink and hop animations for bringing pets to life
- **Progress Spinners**: Animated companions for long-running operations
- **Multiple Art Styles**: Detailed, minimal, blocky, and emoji modes
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

## Examples

```bash
# Basic demo (shows all species and moods)
cargo run --example basic

# With colors
cargo run --example basic --features color

# Different art styles
cargo run --example styles

# Custom templates
cargo run --example template

# Interactive live mode
cargo run --example live --features live

# Persistence demo
cargo run --example persistence --features persistence
```

## Feature Flags

- `color` — ANSI color support via [colored](https://crates.io/crates/colored)
- `persistence` — Save/load pet state to disk (uses serde + dirs)
- `live` — Real-time TUI mode (uses ratatui + crossterm + tokio)

```toml
[dependencies]
corvid-pet = { version = "0.1", features = ["color", "live"] }
```

## Integration with spec-sync

This crate is designed to integrate with [spec-sync](https://github.com/CorvidLabs/spec-sync) for providing visual feedback during spec validation:

```rust
use corvid_pet::integrations::specsync::{SpecSyncCompanion, ValidationOutcome};

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
    "happy": "( ◕ ‿ ◕ )",
    "sad": "( ◕︵◕ )"
  }
}
```

## License

MIT License - See [LICENSE](LICENSE) for details.
