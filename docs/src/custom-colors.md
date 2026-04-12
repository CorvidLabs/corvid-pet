# Custom Colors

Enable the `color` feature to add ANSI color support to your pet's ASCII art.

```toml
[dependencies]
corvid-pet = { version = "1", features = ["color"] }
```

## Basic Colored Rendering

```rust
use corvid_pet::{Pet, Species};

let pet = Pet::new("Corvin".to_string(), Species::Crow);
println!("{}", pet.render_colored()); // Uses default species colors
```

The default color scheme for Crow is blue body with cyan thought bubble.

## Custom Color Schemes

```rust
use corvid_pet::{Pet, Species, ColorScheme, PetColor};

// Specific colors
let pet = Pet::new("Ruby".to_string(), Species::Crow)
    .with_colors(ColorScheme::new(PetColor::Red, PetColor::Yellow));
println!("{}", pet.render_colored());

// Random colors
let pet = Pet::new("Lucky".to_string(), Species::Crow)
    .with_random_colors();
println!("{}", pet.render_colored());
```

## Available Colors

`PetColor` supports 16 ANSI colors:

Black, Red, Green, Yellow, Blue, Magenta, Cyan, White, and their bright variants (BrightBlack, BrightRed, etc.).

Color names are parsed case-insensitively, with aliases:
- `"purple"` → Magenta
- `"gray"` / `"grey"` → BrightBlack

## Colorize Functions

For lower-level control:

```rust
use corvid_pet::color::{colorize, colorize_with_scheme};

// Colorize any string with species default colors
let colored = colorize("some art", Species::Crow);

// Colorize with a custom scheme
let scheme = ColorScheme::new(PetColor::Green, PetColor::BrightGreen);
let colored = colorize_with_scheme("some art", &scheme);
```

Without the `color` feature enabled, these functions are no-ops and return the input unchanged.
