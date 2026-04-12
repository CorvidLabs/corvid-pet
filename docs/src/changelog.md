# Changelog

All notable changes to corvid-pet are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [1.1.0] - 2026-04-11

### Fixed

- Replace fragile JSON parsing with `jq` in GitHub Action ([#68](https://github.com/CorvidLabs/corvid-pet/pull/68))
- Use hidden HTML marker for PR comment deduplication ([#65](https://github.com/CorvidLabs/corvid-pet/pull/65))
- Replace fragile `sed -i` with portable `awk` in badge mode ([#63](https://github.com/CorvidLabs/corvid-pet/pull/63))
- Download pre-built binary in GitHub Action instead of compiling from source ([#64](https://github.com/CorvidLabs/corvid-pet/pull/64))

### Changed

- Live TUI feature marked as experimental — API may change without notice ([#67](https://github.com/CorvidLabs/corvid-pet/pull/67))

### Docs

- Add crate downloads badge to README ([#62](https://github.com/CorvidLabs/corvid-pet/pull/62))
- Clarify Crow is the current species with more coming soon

## [1.0.0] - 2026-04-11

### Added

- **Core library**: `Pet`, `Species` (Crow), `Mood` (6 moods), `Event` (5 events)
- **Minimal art style**: Compact ~6-line crow silhouettes with thought bubbles
- **Animations**: Blink and hop frame iterators
- **Progress spinners**: Animated pet companion for long-running operations
- **Comment system**: Random mood/species-appropriate quips (3+ per mood)
- **Life simulation**: Tamagotchi-like system with stats, life stages, personalities, and interactions
  - Four vital stats: hunger, energy, happiness, health (time-based decay)
  - Five life stages: Egg, Hatchling, Fledgling, Adult, Elder
  - Six personalities: Curious, Shy, Mischievous, Stoic, Affectionate, Greedy
  - Five interactions: Feed, Play, Rest, Clean, Pet (with cooldowns)
- **Color support** (`color` feature): ANSI colors via `colored` crate, custom `ColorScheme`
- **Persistence** (`persistence` feature): Save/load pet state to platform data directory
- **Live TUI** (`live` feature): Interactive real-time pet display with ratatui
- **CLI tool** (`cli` feature): `corvid-pet` binary with show, feed, play, status, sim, health, react, comment, badge commands
- **GitHub Action**: Composite action with 5 modes (pr-comment, health-check, greet, release, badge)
  - Auto event detection from job status
  - PR comments and PR reviews (APPROVE/REQUEST_CHANGES)
  - Health state persistence via GitHub Actions cache
  - README badge generation
- **Custom templates**: JSON-based art templates with `ArtTemplate` and `TemplateRegistry`
- **Health tracking**: `RepoHealth` for CI/CD event aggregation, scoring, and mood derivation
- **spec-sync integration**: `SpecSyncCompanion` for visual validation feedback
- **CI/CD**: GitHub Actions workflow with fmt, clippy, tests, and pet review comments

[1.1.0]: https://github.com/CorvidLabs/corvid-pet/releases/tag/v1.1.0
[1.0.0]: https://github.com/CorvidLabs/corvid-pet/releases/tag/v1.0.0
