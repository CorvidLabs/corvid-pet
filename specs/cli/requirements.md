---
spec: cli.spec.md
---

## User Stories

- As a developer, I want to run `corvid-pet` in my terminal to see my project's mascot
- As a CI pipeline author, I want CLI subcommands to record events and generate comments programmatically
- As an action author, I want `--json` output from the health command to parse structured data
- As a user, I want to customize colors and pet name via flags

## Acceptance Criteria

### REQ-cli-001

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `corvid-pet` with no args shows the pet (default subcommand)
### REQ-cli-002

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `react <EVENT>` updates the state file and displays the pet reacting
### REQ-cli-003

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `health --json` outputs valid JSON matching the `RepoHealth` struct
### REQ-cli-004

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `comment <EVENT>` outputs markdown suitable for PR posting
### REQ-cli-005

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `badge` outputs markdown with corvid-pet markers
### REQ-cli-006

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `init` creates a fresh state file, refuses to overwrite
### REQ-cli-007

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `greet [NAME]` prints a welcome message
### REQ-cli-008

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `--no-color` suppresses all ANSI escape codes
### REQ-cli-009

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `--name` propagates to all subcommands
### REQ-cli-010

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- Invalid colors warn but don't error

## Constraints

- Requires `cli` feature flag (`cargo install corvid-pet --features cli`)
- State file must be readable/writable by the process
- Output must be valid UTF-8

## Out of Scope

- Interactive/TUI mode
- Configuration file (all config via CLI flags)
- Daemon/watch mode
