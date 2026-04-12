# Life Simulation

corvid-pet includes an optional tamagotchi-like life simulation system. Pets have stats that decay over time, progress through life stages, and have personalities that affect their behavior.

## Enabling Simulation

Attach a simulation to any pet with `with_simulation`:

```rust
use corvid_pet::{Pet, Species, Personality};

let now = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

let mut pet = Pet::new("Pip".to_string(), Species::Crow)
    .with_simulation(Personality::Curious, now);
```

Without simulation, the pet works exactly as before — rendering art, reacting to events, and generating comments. Simulation adds stats, aging, and interactions on top.

## Stats

Four vital statistics decay over real time:

| Stat | Decay Rate | Description |
|------|------------|-------------|
| Hunger | -1.0/min | How full the pet is. 0 = starving |
| Energy | -0.5/min | How rested. 0 = exhausted |
| Happiness | -0.3/min | How content. 0 = miserable |
| Health | conditional | Decays at -0.2/min when hunger < 20 or energy < 10 |

Stats are driven by elapsed real time. There are no background threads — the caller triggers updates by calling `tick()`.

## Life Stages

Pets grow through five stages:

| Stage | Duration | Effect |
|-------|----------|--------|
| Egg | 5 min | No stat decay, no interactions |
| Hatchling | 30 min | 1.5x hunger decay — needs lots of feeding |
| Fledgling | 2 hours | 1.2x energy decay — learning to fly |
| Adult | Indefinite | Normal decay rates |
| Elder | Indefinite | 0.7x all decay — slower metabolism |

The Adult → Elder transition happens at 24 hours of total age.

## Personalities

Each pet gets a personality at creation that affects stat decay and interaction effectiveness:

| Personality | Stat Effect | Interaction Effect |
|-------------|------------|-------------------|
| Curious | +0.1/min happiness near full | Play 1.2x effective |
| Shy | -0.1/min happiness decay | Rest 1.3x effective |
| Mischievous | +0.2/min hunger decay | Play 1.5x, feed 0.8x |
| Stoic | -0.1/min all decay | All interactions 0.9x |
| Affectionate | +0.2/min happiness decay alone | All interactions 1.2x |
| Greedy | +0.3/min hunger decay | Feed 1.5x effective |

## Interactions

Care for your pet with five interactions:

| Interaction | Effect | Cooldown |
|------------|--------|----------|
| Feed | hunger +30 | 60s |
| Play | happiness +25, energy -10 | 45s |
| Rest | energy +35, hunger -5 | 90s |
| Clean | health +15, happiness +5 | 120s |
| Pet | happiness +10 | 15s |

```rust
// Time passes, egg hatches
pet.tick(now + 300);

// Interact with the pet
if let Some(result) = pet.feed(now + 301) {
    println!("{}", result.message); // Personality-flavored response
}

if let Some(result) = pet.play(now + 400) {
    println!("{}", result.message);
}
```

Interactions return `None` if simulation isn't enabled, and `InteractionResult { success: false, .. }` if on cooldown or in the Egg stage.

## Stat-Driven Mood

When simulation is active, the pet's mood reflects its stats:

- Hunger < 20 → Sad
- Energy < 15 → Sleepy
- Happiness > 80 → Happy
- Happiness < 20 → Sad
- Health < 30 → Confused
- Otherwise → Neutral

## Long Absence Protection

If more than 24 hours pass between ticks, decay caps at 24 hours worth. This prevents a forgotten pet from instantly dying after a weekend away.

## Persistence

Combine with the `persistence` feature to save simulation state across sessions:

```rust
use corvid_pet::persistence::{save_pet, load_pet, PetState};

// Save
let state = PetState::from_pet(&pet);
save_pet(&state, "pip")?;

// Load
let state = load_pet("pip")?;
let mut pet = state.to_pet();
```

The full simulation state (stats, stage, personality, age, cooldowns) is preserved.
