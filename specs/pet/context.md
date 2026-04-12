---
spec: pet.spec.md
---

## Key Decisions

- **Minimal dependencies**: Only `rand` for randomization, keeping the library lightweight
- **Sync API**: No async/await - user handles their own async runtime
- **No terminal control**: Library returns strings, user handles actual printing
- **Frame-based animation**: Simple iterator pattern for animations, not time-based
- **Static ASCII art**: All art compiled into binary as static strings, not loaded at runtime
- **Single style (Minimal)**: v1 ships with one art style — a compact crow with thought bubble. The `ArtStyle` enum is extensible for future styles
- **Feature-gated colors**: ANSI colors optional via `color` feature flag
- **Feature-gated persistence**: Pet state saving via `persistence` feature flag
- **Iterator animations**: Animation yields `Vec<String>` frames that caller displays
- **Custom templates**: `TemplateRegistry` allows user-defined art via JSON (requires `persistence` feature for serialization)

## Files to Read First

- `src/species.rs` - Species enum and characteristics
- `src/moods.rs` - Mood enum and ASCII art templates
- `src/lib.rs` - Pet struct and main API

## Current Status

v1.0.0 release candidate. Single species (Crow), single art style (Minimal). All major systems implemented and tested (117 tests passing):

- **Core**: Pet struct with mood, render, comment, animation, spinner APIs
- **Life simulation**: Stats decay, life stages (Egg→Elder), 6 personality traits, 5 need interactions with cooldowns
- **Persistence**: Save/load pet state to disk (feature-gated)
- **Color**: ANSI color schemes with custom body/bubble colors (feature-gated)
- **Health**: CI/CD repo health tracking with badge generation
- **Live TUI**: Interactive terminal display with ratatui (feature-gated)
- **Integrations**: SpecSync companion for spec validation feedback
- **CLI**: Binary with show/feed/play/status/sim/health commands
- **GitHub Action**: action.yml with pr-comment, health-check, greet, release, badge modes

## Notes

- ASCII art should be original or clearly public domain
- Crow is the only species in v1 — extensible via the Species enum for future additions
- Animation frames should feel "alive" but not distracting
- Comments should be helpful but sassy (corvids have personality)
- All public enums implement Display
