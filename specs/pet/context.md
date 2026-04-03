---
spec: pet.spec.md
---

## Key Decisions

- **Minimal dependencies**: Only `rand` for randomization, keeping the library lightweight
- **Sync API**: No async/await - user handles their own async runtime
- **No terminal control**: Library returns strings, user handles actual printing
- **Frame-based animation**: Simple iterator pattern for animations, not time-based
- **Static ASCII art**: All art compiled into binary as static strings, not loaded at runtime
- **Feature-gated colors**: ANSI colors optional via feature flag
- **Iterator animations**: Animation yields `Vec<String>` frames that caller displays

## Files to Read First

- `src/species.rs` - Species enum and characteristics
- `src/moods.rs` - Mood enum and ASCII art templates
- `src/lib.rs` - Pet struct and main API

## Current Status

Draft spec created. Implementation pending.

## Notes

- ASCII art should be original or clearly public domain
- Crow is the default species - most users will start here
- Animation frames should feel "alive" but not distracting
- Comments should be helpful but sassy (corvids have personality)
- Consider adding `Display` trait implementations for all enums
