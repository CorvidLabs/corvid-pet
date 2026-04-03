---
spec: pet.spec.md
---

## User Stories

- As a CLI user running long operations, I want a charming ASCII companion so that waiting feels less tedious
- As a spec-sync user, I want my pet to react to validation results so that I get emotional feedback on my work
- As a developer integrating corvid-pet, I want minimal dependencies so that my tool stays lightweight
- As a corvid enthusiast, I want species-specific personalities so that each pet feels unique
- As a terminal user, I want art that fits standard terminals so that rendering works everywhere

## Acceptance Criteria

- `Species` enum includes: Crow, Raven, Magpie, Jay
- `Mood` enum includes: Happy, Sad, Neutral, Confused, Excited, Sleepy
- `Event` enum includes: SpecPassed, SpecFailed, ValidationWarning, NewSpecGenerated, Idle
- `Pet::new()` creates pet with given name and species, defaulting to Neutral mood
- `Pet::render()` returns ASCII art matching current species and mood
- `Pet::set_mood()` changes mood and affects subsequent renders and comments
- `Pet::comment()` returns a random quip from species+mood appropriate pool
- At least 3 unique comments per species per mood (72 total minimum)
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
- Library must compile on stable Rust 1.70+

## Out of Scope

- Persistent state (saving pet to disk)
- Network features
- Complex animation beyond frame cycling
- Actual terminal manipulation (user handles printing)
- Sound effects
- Custom ASCII art from users
- Multiple pets at once
- Pet "aging" or progression systems
