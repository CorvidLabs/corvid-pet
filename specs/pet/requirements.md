---
spec: pet.spec.md
---

## User Stories

- As a CLI user running long operations, I want a charming ASCII companion so that waiting feels less tedious
- As a spec-sync user, I want my pet to react to validation results so that I get emotional feedback on my work
- As a developer integrating corvid-pet, I want minimal dependencies so that my tool stays lightweight
- As a corvid enthusiast, I want personality traits so that each pet feels unique
- As a terminal user, I want art that fits standard terminals so that rendering works everywhere

## Acceptance Criteria

- `Species` enum includes: Crow
- `Mood` enum includes: Happy, Sad, Neutral, Confused, Excited, Sleepy
- `Event` enum includes: Success, Failure, Warning, Progress, Idle
- `Pet::new()` creates pet with given name and species, defaulting to Neutral mood
- `Pet::render()` returns ASCII art using the Minimal style (single crow with thought bubble)
- `ArtStyle` enum has a single variant: `Minimal`
- `Pet::render_with_style()` accepts an `ArtStyle` for forward-compatible style selection
- `Pet::set_mood()` changes mood and affects subsequent renders and comments
- `Pet::comment()` returns a random quip from species+mood appropriate pool
- At least 3 unique comments per mood (18 total minimum for Crow)
- ASCII art fits within 15 lines × 40 characters
- `Pet::animate_blink()` returns iterator yielding at least 3 frames
- `Pet::animate_hop()` returns iterator yielding at least 4 frames
- `Pet::spinner()` returns `Spinner` that cycles through animation frames
- `Spinner::tick()` advances animation without blocking
- `Spinner::finish()` returns string with completion indicator
- `Pet::react()` maps events to moods according to invariant 3

## Constraints

- Minimal dependencies: only `rand` for randomization
- No Unicode in ASCII art (portability)
- No async/await in core library (sync API)
- Terminal colors are optional (via feature flag or separate method)
- Library must compile on stable Rust 1.86+ (edition 2024)

## Out of Scope

- Multiple art styles (v1 ships with Minimal only; style enum is extensible)
- Network features
- Complex animation beyond frame cycling
- Actual terminal manipulation (user handles printing)
- Sound effects
- Multiple pets at once
