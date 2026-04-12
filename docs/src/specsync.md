# spec-sync Integration

corvid-pet includes a built-in integration with [spec-sync](https://github.com/CorvidLabs/spec-sync) for visual feedback during spec validation.

## Usage

```rust
use corvid_pet::integrations::specsync::{SpecSyncCompanion, ValidationOutcome};
use corvid_pet::Species;

let mut companion = SpecSyncCompanion::new(Species::Crow);

// React to validation results
companion.react_to_validation(ValidationOutcome::Success);
println!("{}", companion.summary());

// Or react to error/warning counts
companion.react_to_results(0, 2); // 0 errors, 2 warnings
println!("{}", companion.pet().render());
```

## ValidationOutcome

| Variant | Description |
|---------|-------------|
| `Success` | All specs passed |
| `Warning` | Warnings but no errors |
| `Failure` | One or more specs failed |
| `Generated` | New specs generated |
| `Idle` | No activity |

## Spinner

Create a progress spinner for long-running validation:

```rust
use corvid_pet::integrations::specsync::create_validation_spinner;
use corvid_pet::{Pet, Species};

let pet = Pet::new("Corvin".to_string(), Species::Crow);
let mut spinner = create_validation_spinner(&pet);

// In your validation loop
spinner.tick();
println!("{}", spinner.current_frame());

// When done
println!("{}", spinner.finish());
```

See the `spec_buddy` example for a complete demo:

```bash
cargo run --example spec_buddy
```
