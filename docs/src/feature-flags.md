# Feature Flags

corvid-pet uses Cargo feature flags to keep the core library lightweight. Only `rand` is required by default.

## Available Features

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `color` | ANSI color support for pet art | [colored](https://crates.io/crates/colored) |
| `persistence` | Save/load pet state to disk | serde, serde_json, dirs |
| `live` | Real-time TUI mode | ratatui, crossterm, tokio, serde, serde_json |
| `cli` | Command-line binary | clap, chrono, color, persistence |

## Usage

```toml
# Core only (no optional deps)
corvid-pet = "1"

# With colors
corvid-pet = { version = "1", features = ["color"] }

# With save/load
corvid-pet = { version = "1", features = ["persistence"] }

# Kitchen sink
corvid-pet = { version = "1", features = ["color", "persistence", "live"] }
```

## Feature Details

### `color`

Adds `render_colored()` and `render_colored_with_style()` methods to `Pet`, plus `PetColor` and `ColorScheme` types. Without this feature, the colorize functions are no-ops.

### `persistence`

Adds `save_pet()`, `load_pet()`, `list_pets()`, and `delete_pet()` functions. Also enables JSON serialization for `ArtTemplate`. Pet state is stored in the platform's data directory.

### `live`

Adds `LivePetApp` (async TUI with ratatui) and `SimpleLivePet` (synchronous interactive display). Implicitly enables serde/serde_json for state management.

### `cli`

Enables the `corvid-pet` binary. Implicitly enables `color` and `persistence`. Adds `clap` for argument parsing and `chrono` for time formatting.
