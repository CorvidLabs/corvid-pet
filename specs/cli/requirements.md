---
spec: cli.spec.md
---

## User Stories

- As a developer, I want to run `corvid-pet` in my terminal to see my project's mascot
- As a CI pipeline author, I want CLI subcommands to record events and generate comments programmatically
- As an action author, I want `--json` output from the health command to parse structured data
- As a user, I want to customize colors and pet name via flags

## Acceptance Criteria

- `corvid-pet` with no args shows the pet (default subcommand)
- `react <EVENT>` updates the state file and displays the pet reacting
- `health --json` outputs valid JSON matching the `RepoHealth` struct
- `comment <EVENT>` outputs markdown suitable for PR posting
- `badge` outputs markdown with corvid-pet markers
- `init` creates a fresh state file, refuses to overwrite
- `greet [NAME]` prints a welcome message
- `--no-color` suppresses all ANSI escape codes
- `--name` propagates to all subcommands
- Invalid colors warn but don't error

## Constraints

- Requires `cli` feature flag (`cargo install corvid-pet --features cli`)
- State file must be readable/writable by the process
- Output must be valid UTF-8

## Out of Scope

- Interactive/TUI mode
- Configuration file (all config via CLI flags)
- Daemon/watch mode
