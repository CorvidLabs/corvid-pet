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

- Action runs as a composite action (`runs: using: composite`)
- All 5 modes produce the expected outputs: `mood`, `score`, `art`, `event`
- `pr-comment` mode posts/updates a single comment on the PR (no duplicates)
- `review-on-pr` mode submits APPROVE on success, REQUEST_CHANGES on failure
- `health-check` mode outputs JSON health data without posting comments
- `greet` mode generates a welcome message with success mood
- `release` mode generates a celebration message incorporating the `context` input
- `badge` mode generates markdown between `corvid-pet:start`/`corvid-pet:end` markers
- `event: auto` correctly maps job.status to event types
- `CORVID_PET_BIN` env var skips Rust install, cache, and build steps
- Binary caching works (cache hit skips `cargo install`)
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
