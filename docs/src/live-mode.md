# Live TUI Mode

The `live` feature provides an interactive real-time pet experience using a terminal UI.

```toml
[dependencies]
corvid-pet = { version = "1", features = ["live"] }
```

## LivePetApp

The full async TUI application with keyboard controls:

```rust
use corvid_pet::{Pet, Species, Personality};
use corvid_pet::live::LivePetApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?.as_secs();

    let pet = Pet::new("Corvin".to_string(), Species::Crow)
        .with_simulation(Personality::Curious, now);

    let mut app = LivePetApp::new(pet);
    app.run().await
}
```

## SimpleLivePet

A simpler synchronous alternative:

```rust
use corvid_pet::{Pet, Species};
use corvid_pet::live::SimpleLivePet;

let pet = Pet::new("Corvin".to_string(), Species::Crow);
let mut live = SimpleLivePet::new(pet);
live.run_interactive()?;
```

## Running the Example

```bash
cargo run --example live --features live
```

The `live` feature pulls in ratatui, crossterm, and tokio as dependencies.
