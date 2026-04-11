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

Core implementation complete. Spec promoted to review. Single Minimal art style ships in v1; `art_v2` module provides species-differentiated art for future styles and custom templates.

Life simulation system added: Stats (hunger/energy/happiness/health with time-based decay), LifeStage (Egg -> Hatchling -> Fledgling -> Adult -> Elder), Personality (6 traits affecting decay + interactions), and Needs (Feed/Play/Rest/Clean/Pet with cooldowns). Fully optional -- Pet without simulation behaves identically to v1.

## Notes

- ASCII art should be original or clearly public domain
- Crow is the default species - most users will start here
- Animation frames should feel "alive" but not distracting
- Comments should be helpful but sassy (corvids have personality)
- Consider adding `Display` trait implementations for all enums
