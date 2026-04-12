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

