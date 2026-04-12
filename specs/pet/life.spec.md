---
module: life
version: 2
status: active
files:
  - src/stats.rs
  - src/life_stage.rs
  - src/personality.rs
  - src/needs.rs
  - src/sim.rs
db_tables: []
depends_on: []
---

# Life Simulation

## Purpose

Virtual pet life simulation system for corvid-pet. Adds stats that decay over time, life stages that progress with age, personality traits that affect behavior, and need-based interactions (feed, play, rest, clean). Together these make the pet feel alive -- it gets hungry, tired, bored, and grows from an egg to an elder.

## Design Principles

1. **Time-driven**: Stats decay based on elapsed real time since last update. No background threads -- caller triggers ticks.
2. **Deterministic core**: Given the same state and elapsed time, stat changes are identical. Randomness only in personality-flavored text.
3. **Composable**: Each subsystem (stats, stages, personality, needs) works independently and composes through the `Pet` struct.
4. **Graceful degradation**: If persistence is off, the pet starts fresh each session but still functions.

## Public API

### Exported Structs

| Type | Description |
|------|-------------|
| `Stats` | Vital statistics: hunger, energy, happiness, health (all f32, 0-100) |
| `InteractionResult` | Result of performing an interaction with success flag and stat deltas |
| `SimState` | Full simulation state coordinating stats, stages, personality, and needs |

### Exported Enums

| Type | Description |
|------|-------------|
| `LifeStage` | Life progression: Egg, Hatchling, Fledgling, Adult, Elder |
| `Personality` | Personality trait: Curious, Shy, Mischievous, Stoic, Affectionate, Greedy |
| `Need` | Interaction type: Feed, Play, Rest, Clean, Pet |

### Stats

| Field | Type | Range | Decay Rate | Description |
|-------|------|-------|------------|-------------|
| `hunger` | `f32` | 0.0-100.0 | -1.0/min | How full the pet is. 0 = starving |
| `energy` | `f32` | 0.0-100.0 | -0.5/min | How rested. 0 = exhausted |
| `happiness` | `f32` | 0.0-100.0 | -0.3/min | How content. 0 = miserable |
| `health` | `f32` | 0.0-100.0 | conditional | Overall wellness. Decays at -0.2/min when hunger < 20 or energy < 10 |

```rust
pub struct Stats {
    pub hunger: f32,
    pub energy: f32,
    pub happiness: f32,
    pub health: f32,
}

impl Stats {
    pub fn new() -> Self;              // All stats at 100.0
    pub fn tick(&mut self, elapsed_secs: f64, hunger_mult: f32, energy_mult: f32, happiness_mult: f32);  // Apply decay with multipliers
    pub fn clamp(&mut self);           // Clamp all to 0.0..=100.0
    pub fn overall(&self) -> f32;      // Weighted average (health 40%, happiness 25%, hunger 20%, energy 15%)
    pub fn critical_needs(&self) -> Vec<Need>;  // Stats below 20.0
    pub fn dominant_mood(&self) -> Mood;  // Derive mood from stat levels
}

impl Default for Stats { fn default() -> Self; }  // Delegates to new()
```

### Life Stages

| Stage | Duration | Stat Modifiers | Description |
|-------|----------|---------------|-------------|
| `Egg` | 5 min | No decay | Waiting to hatch. Art is a simple egg |
| `Hatchling` | 30 min | 1.5x hunger decay | Tiny, hungry, needs lots of care |
| `Fledgling` | 2 hrs | 1.2x energy decay | Learning to fly, clumsy, energetic |
| `Adult` | Indefinite | Normal rates | Full-grown, balanced |
| `Elder` | Indefinite | 0.7x all decay | Wise, slower metabolism, bonus quips |

```rust
#[derive(Default)]  // Default: Egg
pub enum LifeStage {
    Egg,
    Hatchling,
    Fledgling,
    Adult,
    Elder,
}

impl LifeStage {
    pub fn next(&self) -> Option<LifeStage>;    // None if Elder
    pub fn duration_secs(&self) -> Option<u64>; // None if indefinite
    pub fn hunger_multiplier(&self) -> f32;
    pub fn energy_multiplier(&self) -> f32;
    pub fn happiness_multiplier(&self) -> f32;
    pub fn can_interact(&self) -> bool;  // false for Egg
    pub fn for_age(age_secs: f64) -> LifeStage;  // Determine stage from total age
    pub fn progress(&self, age_secs: f64) -> f32; // 0.0-1.0 progress through current stage
}

impl Display for LifeStage { ... }  // "Egg", "Hatchling", "Fledgling", "Adult", "Elder"
```

Adult -> Elder transition triggers at 24 hours total age (configurable).

### Personality

Each pet gets a personality at creation (or randomly assigned). Personality affects stat decay rates, interaction effectiveness, and dialogue.

| Trait | Stat Effect | Interaction Effect | Flavor |
|-------|-----------|-------------------|--------|
| `Curious` | +0.1/min happiness near full | Play 1.2x effective | Asks questions, explores |
| `Shy` | -0.1/min happiness decay | Rest 1.3x effective | Quiet, prefers calm |
| `Mischievous` | +0.2/min hunger decay | Play 1.5x effective, feed 0.8x | Gets into trouble |
| `Stoic` | -0.1/min all decay | All interactions 0.9x | Unfazed, dry humor |
| `Affectionate` | +0.2/min happiness decay alone | All interactions 1.2x | Loves attention |
| `Greedy` | +0.3/min hunger decay | Feed 1.5x effective | Food-obsessed |

```rust
#[derive(Default)]  // Default: Curious
pub enum Personality {
    Curious,
    Shy,
    Mischievous,
    Stoic,
    Affectionate,
    Greedy,
}

impl Personality {
    pub fn random() -> Self;
    pub fn hunger_modifier(&self) -> f32;   // Additive modifier to decay rate
    pub fn energy_modifier(&self) -> f32;
    pub fn happiness_modifier(&self) -> f32;
    pub fn interaction_modifier(&self, need: Need) -> f32;  // Multiplier on interaction effect
    pub fn description(&self) -> &str;
}

impl Display for Personality { ... }  // "Curious", "Shy", "Mischievous", "Stoic", "Affectionate", "Greedy"
```

### Needs (Interactions)

| Need | Stat Effect | Cooldown | Description |
|------|-----------|----------|-------------|
| `Feed` | hunger +30.0 | 60s | Give the pet food |
| `Play` | happiness +25.0, energy -10.0 | 45s | Play a game |
| `Rest` | energy +35.0, hunger -5.0 | 90s | Let the pet nap |
| `Clean` | health +15.0, happiness +5.0 | 120s | Groom the pet |
| `Pet` | happiness +10.0 | 15s | Quick affection |

```rust
pub enum Need {
    Feed,
    Play,
    Rest,
    Clean,
    Pet,
}

impl Need {
    pub fn apply(&self, stats: &mut Stats, personality: &Personality, stage: &LifeStage) -> (f32, f32, f32, f32);  // Returns (hunger_delta, energy_delta, happiness_delta, health_delta)
    pub fn cooldown_secs(&self) -> u64;
    pub fn description(&self) -> &str;
    pub fn success_message(&self, personality: &Personality) -> String;  // Personality-flavored success text
    pub fn cooldown_message(&self) -> &str;  // Message when on cooldown
}

impl Display for Need { ... }  // "Feed", "Play", "Rest", "Clean", "Pet"
```

### Simulation State

```rust
pub struct SimState {
    pub stats: Stats,
    pub stage: LifeStage,
    pub personality: Personality,
    pub age_secs: f64,
    pub interaction_count: u64,
    pub last_tick: u64,       // Unix timestamp of last tick
    pub cooldowns: HashMap<Need, u64>,  // Need -> timestamp when available
    pub born_at: u64,         // Unix timestamp of creation
}

impl SimState {
    pub fn new(personality: Personality, now_secs: u64) -> Self;
    pub fn tick(&mut self, now_secs: u64);   // Advance simulation to `now`
    pub fn interact(&mut self, need: Need, now_secs: u64) -> InteractionResult;
    pub fn can_interact(&self, need: Need, now_secs: u64) -> bool;
    pub fn stage_progress(&self) -> f32;     // 0.0-1.0 progress to next stage
    pub fn is_alive(&self) -> bool;          // health > 0
    pub fn health(&self) -> f32;             // Shortcut to stats.health
    pub fn status_summary(&self) -> String;  // One-line status
    pub fn age_display(&self) -> String;     // Human-readable age ("2h 15m old")
}
```

### InteractionResult

```rust
pub struct InteractionResult {
    pub success: bool,
    pub message: String,           // Personality-flavored response
    pub hunger_delta: f32,         // Change applied to hunger
    pub energy_delta: f32,         // Change applied to energy
    pub happiness_delta: f32,      // Change applied to happiness
    pub health_delta: f32,         // Change applied to health
    pub stage_changed: bool,       // Did a stage transition happen
}
```

### Pet Integration

The `Pet` struct gains an optional `SimState`:

```rust
impl Pet {
    pub fn with_simulation(self, personality: Personality, now_secs: u64) -> Self;
    pub fn tick(&mut self, now_secs: u64);
    pub fn feed(&mut self, now_secs: u64) -> Option<InteractionResult>;
    pub fn play(&mut self, now_secs: u64) -> Option<InteractionResult>;
    pub fn rest(&mut self, now_secs: u64) -> Option<InteractionResult>;
    pub fn clean(&mut self, now_secs: u64) -> Option<InteractionResult>;
    pub fn pet_me(&mut self, now_secs: u64) -> Option<InteractionResult>;
    pub fn stats(&self) -> Option<&Stats>;
    pub fn life_stage(&self) -> Option<LifeStage>;
    pub fn pet_personality(&self) -> Option<Personality>;
    pub fn sim(&self) -> Option<&SimState>;
    pub fn age_display(&self) -> Option<String>;  // "2h 15m old"
}
```

Methods return `None` if simulation is not enabled (backwards compatible).

## Invariants

1. All stats clamped to 0.0..=100.0 after every operation
2. Stats never change while in `Egg` stage
3. Life stage transitions are one-way and sequential (Egg -> Hatchling -> ... -> Elder)
4. Interactions respect cooldowns -- calling before cooldown expires returns `success: false`
5. `tick()` is idempotent for the same timestamp
6. `Pet` without simulation enabled behaves identically to current API (full backwards compat)
7. Elapsed time > 24 hours in a single tick caps decay as if 24 hours passed (prevents death from forgetting)
8. Health reaching 0 sets mood to Sad but pet remains functional (no permadeath)
9. `dominant_mood()` derives mood from stats: hunger<20 -> Sad, energy<15 -> Sleepy, happiness>80 -> Happy, happiness<20 -> Sad, health<30 -> Confused, else Neutral

## Behavioral Examples

### Scenario: New pet lifecycle

- **Given** `Pet::new("Pip", Species::Crow).with_simulation(Personality::Curious, now)`
- **When** created
- **Then** stage is `Egg`, all stats 100.0, age 0
- **When** 5 minutes pass and `tick()` called
- **Then** stage transitions to `Hatchling`, stats unchanged (egg has no decay)

### Scenario: Feeding a hungry pet

- **Given** a pet with hunger at 15.0 (critical) and Greedy personality
- **When** `pet.feed()` called
- **Then** hunger increases by 30.0 * 1.5 (greedy bonus) = 45.0, now 60.0
- **And** result message reflects greedy personality

### Scenario: Interaction cooldown

- **Given** a pet just fed (cooldown 60s)
- **When** `pet.feed()` called again at now + 30s
- **Then** returns `InteractionResult { success: false, message: "Not hungry yet..." }`

### Scenario: Stat-driven mood

- **Given** a pet with hunger 10.0 and happiness 90.0
- **When** `stats.dominant_mood()` called
- **Then** returns `Mood::Sad` (hunger critical overrides high happiness)

### Scenario: Long absence (24h cap)

- **Given** a pet last ticked 72 hours ago
- **When** `tick()` called
- **Then** applies max 24 hours of decay (hunger floors at 0, energy floors at 0, etc)
- **And** health decays from low hunger/energy but pet is still alive

## Error Cases

| Condition | Behavior |
|-----------|----------|
| `tick()` with past timestamp | No-op (elapsed = 0) |
| `interact()` during Egg stage | Returns `success: false, message: "Still an egg..."` |
| `interact()` during cooldown | Returns `success: false` with time remaining |
| Stats at 0 after decay | Clamped to 0, mood set to Sad |
| Personality not set | Default to `Curious` |

## Persistence Changes

`PetState` gains optional simulation fields:

```rust
pub struct PetState {
    // ... existing fields ...
    pub sim: Option<SimStateData>,
}

pub struct SimStateData {
    pub hunger: f32,
    pub energy: f32,
    pub happiness: f32,
    pub health: f32,
    pub stage: String,
    pub personality: String,
    pub age_secs: f64,
    pub interaction_count: u64,
    pub born_at: u64,
    pub last_tick: u64,
    pub cooldowns: Vec<(String, u64)>,
}
```

## Dependencies

### Consumes

| Crate | What is used |
|-------|-------------|
| (none new) | Uses existing rand, serde (optional) |

### Consumed By

| Module | What is used |
|--------|-------------|
| pet | Stats, LifeStage, Personality, Need, SimState integrated into Pet |

## Change Log

| Date | Change |
|------|--------|
| 2026-04-11 | Initial spec draft |
| 2026-04-11 | v2: Added all undocumented exports — SimState::health(), SimState::age_display(), LifeStage::for_age(), LifeStage::progress(), Need::success_message(), Need::cooldown_message(), Display impls, Default impls, InteractionResult field corrections (individual deltas not Stats), Stats::tick() multiplier params |
