# corvid-pet

ASCII corvid companions for CLI tools.

Bring a little personality to your command-line tools with animated ASCII pets that react to events, display mood-appropriate art, and provide charming commentary.

```
      _
    <(o\  .oO(hmm)
     |/(\
      \(\\
      " "\\
```

## Features

- **Crow companion** with unique art and personality
- **Six moods**: Happy, Sad, Neutral, Confused, Excited, Sleepy
- **Generic events**: Success, Failure, Warning, Progress, Idle — works with any tool
- **Minimal art style**: Compact ~6-line silhouettes with thought bubbles
- **Animations**: Blink and hop animations for bringing pets to life
- **Progress spinners**: Animated companions for long-running operations
- **Life simulation**: Tamagotchi-like system with stats, life stages, and personalities
- **Color support**: ANSI colors (optional feature)
- **Persistence**: Save/load pet state (optional feature)
- **Live TUI mode**: Interactive real-time pet experience (optional feature)
- **Custom templates**: Define your own ASCII art
- **CLI tool**: Command-line binary for interacting with your pet
- **GitHub Action**: Track repo health and post pet-powered PR comments

## Quick Example

```rust
use corvid_pet::{Pet, Species, Event};

let mut pet = Pet::new("Corvin".to_string(), Species::Crow);
println!("{}", pet.render());

// React to events from your tool
pet.react(Event::Success);
println!("{}", pet.comment());
```

## Source Code

[github.com/CorvidLabs/corvid-pet](https://github.com/CorvidLabs/corvid-pet)
