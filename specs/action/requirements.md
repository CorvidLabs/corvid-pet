---
spec: action.spec.md
---

## User Stories

- As a repo maintainer, I want a corvid pet that reacts to my CI results so that PRs get charming, informative feedback
- As a contributor, I want to see a friendly greeting when I open my first PR so that the project feels welcoming
- As a release manager, I want celebration comments on release PRs so that milestones are marked
- As a repo owner, I want a README health badge so that visitors see the project's CI health at a glance
- As a CI pipeline author, I want to consume health data as JSON so that I can build custom dashboards

## Acceptance Criteria

### REQ-action-001

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- Action runs as a composite action (`runs: using: composite`)
### REQ-action-002

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- All 5 modes produce the expected outputs: `mood`, `score`, `art`, `event`
### REQ-action-003

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `pr-comment` mode posts/updates a single comment on the PR (no duplicates)
### REQ-action-004

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `review-on-pr` mode submits APPROVE on success, REQUEST_CHANGES on failure
### REQ-action-005

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `health-check` mode outputs JSON health data without posting comments
### REQ-action-006

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `greet` mode generates a welcome message with success mood
### REQ-action-007

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `release` mode generates a celebration message incorporating the `context` input
### REQ-action-008

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `badge` mode generates markdown between `corvid-pet:start`/`corvid-pet:end` markers
### REQ-action-009

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `event: auto` correctly maps job.status to event types
### REQ-action-010

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- `CORVID_PET_BIN` env var skips Rust install, cache, and build steps
### REQ-action-011

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- Binary caching works (cache hit skips `cargo install`)
### REQ-action-012

The implementation SHALL satisfy this requirement.

Acceptance Criteria

- Missing state file is created automatically (no setup required)

## Constraints

- Must work on `ubuntu-latest`, `macos-latest`, and `windows-latest` runners
- Composite actions cannot use `if` expressions on the top-level `runs` — conditions are per-step
- GitHub token permissions must include `pull-requests: write` for comment/review posting
- No secrets can be passed through composite action inputs (only `github.token` default)

## Out of Scope

- Pre-built binary distribution (currently builds from source each run, cached)
- Custom art templates via action inputs
- Multi-pet configurations
- Slack/Discord notification integration
