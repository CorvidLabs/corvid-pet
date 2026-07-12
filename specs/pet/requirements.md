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

### REQ-pet-001

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Species` enum includes: Crow
### REQ-pet-002

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Mood` enum includes: Happy, Sad, Neutral, Confused, Excited, Sleepy
### REQ-pet-003

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Event` enum includes: Success, Failure, Warning, Progress, Idle
### REQ-pet-004

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Pet::new()` creates pet with given name and species, defaulting to Neutral mood
### REQ-pet-005

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Pet::render()` returns ASCII art using the Minimal style (single crow with thought bubble)
### REQ-pet-006

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `ArtStyle` enum has a single variant: `Minimal`
### REQ-pet-007

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Pet::render_with_style()` accepts an `ArtStyle` for forward-compatible style selection
### REQ-pet-008

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Pet::set_mood()` changes mood and affects subsequent renders and comments
### REQ-pet-009

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Pet::comment()` returns a random quip from species+mood appropriate pool
### REQ-pet-010

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- At least 3 unique comments per mood (18 total minimum for Crow)
### REQ-pet-011

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- ASCII art fits within 15 lines × 40 characters
### REQ-pet-012

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Pet::animate_blink()` returns iterator yielding at least 3 frames
### REQ-pet-013

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Pet::animate_hop()` returns iterator yielding at least 4 frames
### REQ-pet-014

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Pet::spinner()` returns `Spinner` that cycles through animation frames
### REQ-pet-015

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Spinner::tick()` advances animation without blocking
### REQ-pet-016

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Spinner::finish()` returns string with completion indicator
### REQ-pet-017

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `Pet::react()` maps events to moods according to invariant 3

## Constraints

- Minimal dependencies: only `rand` for randomization
- No Unicode in ASCII art (portability)
- No async/await in core library (sync API)
- Terminal colors are optional (via feature flag or separate method)
- Library must compile on stable Rust 1.88+ (edition 2024)

## Out of Scope

- Multiple art styles (v1 ships with Minimal only; style enum is extensible)
- Network features
- Complex animation beyond frame cycling
- Actual terminal manipulation (user handles printing)
- Sound effects
- Multiple pets at once
