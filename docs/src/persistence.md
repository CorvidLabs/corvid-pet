# Persistence

Save and load pet state across sessions. Enable the `persistence` feature:

```toml
[dependencies]
corvid-pet = { version = "1", features = ["persistence"] }
```

## Save and Load

```rust
use corvid_pet::{Pet, Species};
use corvid_pet::persistence::{save_pet, load_pet, PetState};

// Create and save
let pet = Pet::new("Corvin".to_string(), Species::Crow);
let state = PetState::from_pet(&pet);
save_pet(&state, "corvin")?;

// Load later
let state = load_pet("corvin")?;
let pet = state.to_pet();
```

## Storage Location

Pet state is saved to the platform's data directory:
- **Linux**: `~/.local/share/corvid-pet/`
- **macOS**: `~/Library/Application Support/corvid-pet/`
- **Windows**: `{FOLDERID_LocalAppData}/corvid-pet/`

Each pet is stored as `{id}.json`.

## Managing Saved Pets

```rust
use corvid_pet::persistence::{list_pets, delete_pet};

// List all saved pet IDs
let pets = list_pets()?;
for id in &pets {
    println!("Saved pet: {id}");
}

// Delete a pet
delete_pet("old-pet")?;
```

## Simulation State

When a pet has an active life simulation, the full simulation state is preserved — stats, life stage, personality, age, interaction count, and cooldowns are all saved and restored.

```rust
use corvid_pet::{Pet, Species, Personality};
use corvid_pet::persistence::{save_pet, load_pet, PetState};

let now = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

let mut pet = Pet::new("Pip".to_string(), Species::Crow)
    .with_simulation(Personality::Curious, now);

// Play for a while...
pet.tick(now + 600);
pet.feed(now + 601);

// Save
save_pet(&PetState::from_pet(&pet), "pip")?;

// Later: load and continue
let state = load_pet("pip")?;
let mut pet = state.to_pet();
pet.tick(later_timestamp);
```

## Error Handling

Persistence operations return `Result<_, PersistenceError>`:

| Error | Cause |
|-------|-------|
| `NoDataDir` | Platform data directory not found |
| `Io(e)` | File system error |
| `Serde(e)` | JSON serialization/deserialization error |
