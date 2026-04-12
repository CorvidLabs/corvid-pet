# Custom Templates

Define your own ASCII art via JSON templates. Requires the `persistence` feature for JSON serialization.

## Template Format

```json
{
  "name": "My Custom Crow",
  "species": "crow",
  "moods": {
    "happy": "( ^ v ^ )",
    "sad": "( ; _ ; )",
    "neutral": "( - _ - )",
    "confused": "( o _ O )",
    "excited": "( * o * )",
    "sleepy": "( - . - ) zzz"
  }
}
```

## Using Templates

```rust
use corvid_pet::templates::{ArtTemplate, TemplateRegistry};
use corvid_pet::{Species, Mood};

// Load from JSON
let json = std::fs::read_to_string("my_template.json")?;
let template = ArtTemplate::from_json(&json)?;

// Register and render
let mut registry = TemplateRegistry::new();
registry.register(template);

let art = registry.render(Species::Crow, Mood::Happy, Some("My Custom Crow"));
println!("{art}");
```

## Template Registry

The `TemplateRegistry` holds templates and falls back to built-in art when a template doesn't define a mood:

```rust
let mut registry = TemplateRegistry::new();
registry.register(my_template);

// Uses custom art if the template defines it, otherwise falls back to default
let art = registry.render(Species::Crow, Mood::Happy, Some("My Custom Crow"));

// Without a template name, uses the built-in art
let art = registry.render(Species::Crow, Mood::Happy, None);

// List all registered templates
let names = registry.list();
```

## Programmatic Templates

You can also build templates in code:

```rust
let mut template = ArtTemplate::new("Kaomoji Crow".to_string(), Species::Crow);
template.set_mood(Mood::Happy, "( ^ v ^ )".to_string());
template.set_mood(Mood::Sad, "( ; _ ; )".to_string());
```

See the `template` example for a full demo:

```bash
cargo run --example template --features persistence
```
