---
module: cli
version: 1
status: active
files:
  - src/bin/corvid-pet.rs
db_tables: []
depends_on: []
---

# CLI Binary

## Purpose

Command-line interface for interacting with corvid-pet. Provides subcommands for displaying the pet, reacting to CI/CD events, managing health state, generating PR comments and README badges, and greeting users. Built with clap, requires the `cli` feature flag.

## Public API

The CLI exposes the following subcommands as its public interface. All commands accept global flags (`--name`, `--no-color`, `--color`, `--bubble-color`, `--random-colors`).

| Subcommand | Summary |
|------------|---------|
| `show` (default) | Display ASCII art with a random quip |
| `react <EVENT>` | Record a CI/CD event in the health state file |
| `health` | Display health summary (text or `--json`) |
| `comment <EVENT>` | Generate a markdown PR comment |
| `badge` | Generate a README badge section |
| `init` | Initialize a new health state file |
| `greet [NAME]` | Greet with a random corvid message |

## Installation

```bash
cargo install corvid-pet --features cli
```

The `cli` feature enables: `clap`, `chrono`, `color`, and `persistence`.

## Global Flags

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--name <NAME>` | String | `Corvin` | Pet name (applies to all subcommands) |
| `--no-color` | bool | `false` | Disable ANSI colored output |
| `--color <COLOR>` | String | — | Body color (e.g. `red`, `blue`, `bright-magenta`) |
| `--bubble-color <COLOR>` | String | `cyan` | Thought bubble color |
| `--random-colors` | bool | `false` | Randomize body and bubble colors |

Color parsing accepts all `PetColor` variants plus aliases (`purple` → magenta, `gray` → bright-black). Invalid colors print a warning and fall back to defaults (blue body, cyan bubble).

## Subcommands

### `show` (default)

Display the pet's ASCII art with a random quip. This is the default when no subcommand is given.

```
corvid-pet [show] [--mood <MOOD>]
```

| Flag | Values | Description |
|------|--------|-------------|
| `--mood` | `happy`, `sad`, `neutral`, `confused`, `excited`, `sleepy` | Override the displayed mood |

**Behavior:**
- Renders the pet with the specified mood (or neutral if unset)
- Prints a random mood-appropriate quip below the art

### `react`

Record a CI/CD event and update the health state file.

```
corvid-pet react <EVENT> [--state <PATH>] [--context <TEXT>]
```

| Argument/Flag | Type | Default | Description |
|---------------|------|---------|-------------|
| `<EVENT>` | `success`, `failure`, `warning`, `progress`, `idle` | *(required)* | The event type |
| `--state` | Path | `.corvid-pet.json` | Health state file path |
| `--context` | String | — | Optional context (PR number, commit SHA, etc.) |

**Behavior:**
1. Loads health state from `--state` (creates fresh if missing/corrupted)
2. Records the event with current timestamp and optional context
3. Saves updated state back to file
4. Displays the pet reacting (mood set from event), a quip, and the health summary

**Exit codes:** 0 on success, 1 if state file cannot be saved.

### `health`

Display the repo health summary.

```
corvid-pet health [--state <PATH>] [--json]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--state` | Path | `.corvid-pet.json` | Health state file path |
| `--json` | bool | `false` | Output as pretty-printed JSON |

**Behavior:**
- Without `--json`: renders the pet with health-derived mood, quip, and summary line
- With `--json`: outputs the full `RepoHealth` struct as JSON (used by the GitHub Action)

### `comment`

Generate a markdown PR comment.

```
corvid-pet comment <EVENT> [--state <PATH>] [--context <TEXT>]
```

| Argument/Flag | Type | Default | Description |
|---------------|------|---------|-------------|
| `<EVENT>` | `success`, `failure`, `warning`, `progress`, `idle` | *(required)* | The triggering event |
| `--state` | Path | `.corvid-pet.json` | Health state file path |
| `--context` | String | `""` | Context message included in the comment body |

**Behavior:**
- Loads health state and generates a markdown comment via `RepoHealth::pr_comment()`
- Output includes ASCII art in a code block, mood quip, health stats, and context
- Designed to be captured and posted via the GitHub Action or `gh` CLI

### `badge`

Generate a README badge section.

```
corvid-pet badge [--state <PATH>]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--state` | Path | `.corvid-pet.json` | Health state file path |

**Behavior:**
- Generates markdown badge block via `RepoHealth::readme_badge()`
- Output is wrapped in `<!-- corvid-pet:start -->` / `<!-- corvid-pet:end -->` markers
- Designed to be embedded in README.md (manually or via the GitHub Action's `update-readme` input)

### `init`

Initialize a new health state file.

```
corvid-pet init [--state <PATH>]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--state` | Path | `.corvid-pet.json` | Health state file path |

**Behavior:**
- Creates a fresh health state (score 100, no events) at the specified path
- Exits with error if the file already exists (prevents accidental overwrite)
- Shows the pet with a "Ready to watch over your repo!" message

**Exit codes:** 0 on success, 1 if file exists or cannot be written.

### `greet`

Greet with a random corvid message.

```
corvid-pet greet [NAME]
```

| Argument | Type | Description |
|----------|------|-------------|
| `[NAME]` | String | Optional name to greet |

**Behavior:**
- Sets the pet mood to Happy
- Prints a welcome message: `Caw! Welcome, <name>!` or `Caw! Welcome!`

## State File Format

The health state file (`.corvid-pet.json`) is a JSON-serialized `RepoHealth` struct:

```json
{
  "pet_name": "Corvin",
  "score": 85,
  "streak": 3,
  "total_events": 12,
  "events": [
    {
      "event_type": "Success",
      "timestamp": 1712844000,
      "context": "PR #42"
    }
  ]
}
```

State files are created automatically by `react` and `init`. The `health`, `comment`, and `badge` commands read but never write state.

## Invariants

1. Running with no subcommand is equivalent to `show` with no flags
2. `--no-color` suppresses all ANSI escape codes in output
3. `--name` propagates to all subcommands (health state `pet_name`, rendered pet name)
4. State file operations are atomic — partial writes don't corrupt existing state
5. Unknown/invalid color names produce a warning and fall back to defaults, never error
6. `init` refuses to overwrite an existing state file

## Behavioral Examples

### Record a CI success and view health

```bash
$ corvid-pet react success --state .corvid-pet.json --context "PR #42"
# Records event, displays pet reacting with happy mood, prints health summary

$ corvid-pet health --state .corvid-pet.json --json
# Outputs full RepoHealth struct as JSON (used by the GitHub Action)
```

### Generate a PR comment for a failed build

```bash
$ corvid-pet comment failure --state .corvid-pet.json --context "Tests failed: 3 errors"
# Outputs markdown with ASCII art, sad mood quip, health stats, and context
```

### Initialize and display

```bash
$ corvid-pet init --state .corvid-pet.json
# Creates fresh state file (score 100, no events), shows welcome message
# Exits with code 1 if file already exists

$ corvid-pet --name Rook --color bright-magenta
# Displays pet named "Rook" in bright magenta with a random neutral quip
```

## Error Cases

| Condition | Behavior |
|-----------|----------|
| State file missing (react/health/comment/badge) | Fresh state created in memory (score 100) |
| State file corrupted | Warning to stderr, fresh state used |
| State file unwritable (react) | Error message, exit code 1 |
| Init on existing file | Error message, exit code 1 |
| Invalid color string | Warning to stderr, default color used |
| Invalid mood string | Clap rejects with usage help |
| Invalid event string | Clap rejects with usage help |

## Dependencies

### Consumes

| Crate | What is used |
|-------|-------------|
| `clap` (derive) | Argument parsing, subcommand dispatch |
| `chrono` | Timestamp formatting |
| `colored` | ANSI color output (via `color` feature) |
| `serde_json` | Health state serialization (via `persistence` feature) |
| `corvid-pet` (lib) | All core types: Pet, Species, Event, Mood, RepoHealth, ColorScheme, PetColor |

### Consumed By

| Consumer | What is used |
|----------|-------------|
| GitHub Action (`action.yml`) | All subcommands except `init` and `greet` |
| End users | All subcommands |

## Change Log

| Date | Change |
|------|--------|
| 2026-04-11 | Initial spec |
