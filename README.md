# corvid-pet

[![CI](https://github.com/CorvidLabs/corvid-pet/actions/workflows/ci.yml/badge.svg)](https://github.com/CorvidLabs/corvid-pet/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/corvid-pet)](https://crates.io/crates/corvid-pet)
[![Downloads](https://img.shields.io/crates/d/corvid-pet.svg)](https://crates.io/crates/corvid-pet)
[![docs.rs](https://img.shields.io/docsrs/corvid-pet)](https://docs.rs/corvid-pet)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

ASCII corvid companions for CLI tools.

Bring a little personality to your command-line tools with animated ASCII pets that react to events, display mood-appropriate art, and provide charming commentary.

## Features

- **Crow Companion**: Clever problem-solver with unique art and personality (more species coming soon)
- **Six Moods**: Happy, Sad, Neutral, Confused, Excited, Sleepy
- **Generic Events**: Success, Failure, Warning, Progress, Idle — works with any tool
- **Minimal Art Style**: Compact ~6-line silhouettes with thought bubbles
- **Animations**: Blink and hop animations for bringing pets to life
- **Progress Spinners**: Animated companions for long-running operations
- **Life Simulation**: Tamagotchi-like system with stats, life stages, and personalities
- **Color Support**: ANSI colors (optional feature)
- **Persistence**: Save/load pet state (optional feature)
- **Live TUI Mode**: Interactive real-time pet experience (optional feature)
- **CLI Tool**: Command-line binary for interacting with your pet
- **GitHub Action**: Track repo health and post pet-powered PR reviews
- **Custom Templates**: Define your own ASCII art

## Quick Start

```toml
[dependencies]
corvid-pet = "1"
```

```rust
use corvid_pet::{Pet, Species, Mood, Event};

let mut pet = Pet::new("Corvin".to_string(), Species::Crow);
println!("{}", pet.render());

// React to events from your tool
pet.react(Event::Success);
println!("{}", pet.comment());
```

```
      _
    <(o\  .oO(hmm)
     |/(\
      \(\\
      " "\\
```

## Events

Pets react to generic lifecycle events that any CLI tool can emit:

| Event | Mood | Use case |
|-------|------|----------|
| `Success` | Happy | Tests passed, build succeeded, deploy complete |
| `Failure` | Sad | Tests failed, build broken, errors found |
| `Warning` | Confused | Linting warnings, deprecations, partial success |
| `Progress` | Excited | Step completed, file generated, milestone reached |
| `Idle` | Sleepy | Waiting for input, watching for changes |

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

# Interactive live mode (experimental)
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

## CLI

Install the CLI binary with:

```bash
cargo install corvid-pet --features cli
```

Commands:

| Command | Description |
|---------|-------------|
| `show` | Display the pet's current ASCII art |
| `status` | Show pet stats and life stage |
| `feed` | Feed your pet |
| `play` | Play with your pet |
| `react <event>` | Record a CI/CD event (success, failure, warning, progress, idle) |
| `comment <event>` | Generate a PR comment for an event |
| `health` | Show repo health score (supports `--json`) |
| `badge` | Generate a health badge |
| `sim` | Run a life simulation tick |

Common flags: `--name`, `--state <path>`, `--no-color`, `--context`.

## GitHub Action

Use corvid-pet as a GitHub Action to get an ASCII crow companion that reacts to your CI results:

```yaml
- uses: CorvidLabs/corvid-pet@v1
  with:
    mode: pr-comment    # pr-comment | health-check | greet | release | badge
    event: auto         # auto-detects from job status
    pet-name: Corvin
    review-on-pr: "true"
```

### Modes

| Mode | Description |
|------|-------------|
| `pr-comment` | Post a mood-reactive comment on PRs based on CI results |
| `health-check` | Track repo health score over time |
| `greet` | Welcome new contributors |
| `release` | Celebrate new releases |
| `badge` | Generate and update a health badge in your README |

### Outputs

| Output | Description |
|--------|-------------|
| `mood` | The pet's current mood |
| `score` | Repo health score (0–100) |
| `comment` | Generated comment markdown |
| `art` | ASCII art output |
| `event` | Resolved event type |

See [action.yml](action.yml) for all inputs and options.

## Feature Flags

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `color` | ANSI color support | [colored](https://crates.io/crates/colored) |
| `persistence` | Save/load pet state to disk | serde, serde_json, dirs |
| `live` | Real-time interactive TUI mode (**experimental**) | ratatui, crossterm, tokio |
| `cli` | CLI binary with all commands | clap, chrono, color, persistence |

```toml
[dependencies]
corvid-pet = { version = "1", features = ["color", "persistence"] }
```

## Integrations

### spec-sync

Built-in integration with [spec-sync](https://github.com/CorvidLabs/spec-sync) for visual feedback during spec validation:

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

## CI/CD

We use GitHub Actions with `ubuntu-latest` runners. Every PR and push to `main` triggers:

| Job | What it does |
|-----|-------------|
| **Check & Test** | `cargo fmt --check`, `cargo clippy` (default + all features), build, `cargo test --all-features`, build examples |
| **Spec Validation** | Runs [spec-sync](https://github.com/CorvidLabs/spec-sync) to verify specs match the source |
| **Corvid Pet** | Posts a PR review (APPROVE / REQUEST_CHANGES) with ASCII art and a CI summary table |

Rust dependencies are cached via [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache). Duplicate CI runs on the same PR are automatically cancelled.

## Documentation

Full documentation is available at [corvidlabs.github.io/corvid-pet](https://corvidlabs.github.io/corvid-pet/).

- [Getting Started](https://corvidlabs.github.io/corvid-pet/getting-started.html)
- [API Reference (rustdoc)](https://corvidlabs.github.io/corvid-pet/rustdoc/corvid_pet/)
- [Changelog](CHANGELOG.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and contribution guidelines.

## License

MIT License - See [LICENSE](LICENSE) for details.
