---
module: action
version: 1
status: active
files:
  - action.yml
db_tables: []
depends_on: [pet]
---

# GitHub Action

## Purpose

Composite GitHub Action that brings corvid-pet into CI/CD pipelines. Installs the CLI, runs it against the repo's health state, and optionally posts PR comments, PR reviews, README badges, or health reports. Published to the GitHub Marketplace as `corvid-pet`.

## Public API

### Exported YAML Symbols

| Symbol | Description |
|--------|-------------|
| `name` | Marketplace action name |
| `description` | Marketplace action description |
| `author` | Marketplace action author |
| `branding` | Marketplace presentation metadata |
| `inputs` | Action input map |
| `outputs` | Action output map |
| `runs` | Composite action implementation |
| `inputs.mode` | Operating mode |
| `inputs.event` | Explicit or automatic event |
| `inputs.pet-name` | Display name for the pet |
| `inputs.state-file` | Health-state path |
| `inputs.context` | Additional rendered context |
| `inputs.comment-on-pr` | PR comment toggle |
| `inputs.review-on-pr` | PR review toggle |
| `inputs.update-readme` | README update toggle |
| `inputs.job-status` | Job-status override |
| `inputs.github-token` | GitHub API token |
| `outputs.mood` | Current pet mood |
| `outputs.score` | Repository health score |
| `outputs.comment` | Generated Markdown comment |
| `outputs.art` | Generated ASCII art |
| `outputs.event` | Resolved event type |

### Branding

- **Icon**: heart
- **Color**: purple

### Inputs

| Input | Required | Default | Description |
|-------|----------|---------|-------------|
| `mode` | yes | `pr-comment` | Action mode (see Modes below) |
| `event` | no | `auto` | Event type: `auto`, `success`, `failure`, `warning`, `progress`, `idle` |
| `pet-name` | no | `Corvin` | Name for the corvid companion |
| `state-file` | no | `.corvid-pet.json` | Path to the health state JSON file |
| `context` | no | `""` | Additional context for comments (e.g. CI summary markdown) |
| `comment-on-pr` | no | `true` | Post a comment on the PR (requires `github-token`) |
| `review-on-pr` | no | `false` | Submit as a PR review (APPROVE/REQUEST_CHANGES) instead of a plain comment |
| `update-readme` | no | `false` | Update the README badge section (for `badge` mode) |
| `job-status` | no | `""` | Override job status for event auto-detection (falls back to `job.status`) |
| `github-token` | no | `${{ github.token }}` | GitHub token for posting comments/reviews |

### Event Auto-Detection

When `event` is `auto` (default), the action detects the event from `job-status` (or `job.status` if unset):

| Job Status | Resolved Event |
|------------|----------------|
| `success` | `success` |
| `failure` | `failure` |
| `cancelled` | `warning` |
| *(other)* | `progress` |

The resolved event is available in the `event` output.

### Outputs

| Output | Description |
|--------|-------------|
| `mood` | The pet's current mood (e.g. `happy`, `sad`, `neutral`) |
| `score` | The repo health score (0–100) |
| `comment` | The generated comment markdown |
| `art` | The ASCII art output |
| `event` | The resolved event type |

## Modes

### `pr-comment`

Records the event in the health state file, then generates a markdown PR comment with ASCII art, a mood-appropriate quip, and health stats.

- Runs `corvid-pet react <event>` to update state
- Runs `corvid-pet comment <event>` to generate markdown
- If `comment-on-pr` is `true` and this is a `pull_request` event, posts/updates the comment on the PR
- If `review-on-pr` is `true`, submits as a PR review instead (APPROVE on success, REQUEST_CHANGES on failure)

### `health-check`

Records the event and outputs the health state as JSON. Does not generate a comment.

- Runs `corvid-pet react <event>` to update state
- Runs `corvid-pet health --json` to output health data
- Health JSON available in the `health` output

### `greet`

Generates a welcome comment for new contributors. Always uses `success` event.

- Runs `corvid-pet comment success` with a welcome context message
- Posts comment on the PR if `comment-on-pr` is `true`

### `release`

Generates a release celebration comment. Always uses `success` event.

- Runs `corvid-pet comment success` with release context
- The `context` input is appended to the release message

### `badge`

Generates a README badge section showing the pet's health score and mood.

- Runs `corvid-pet badge` to generate the badge markdown
- If `update-readme` is `true`, updates the README.md between `<!-- corvid-pet:start -->` and `<!-- corvid-pet:end -->` markers
- If no markers exist, appends the badge section to the README

## Composite Steps

The action executes as a composite action with these steps:

1. **Install Rust toolchain** — via `dtolnay/rust-toolchain@stable` (skipped if `CORVID_PET_BIN` env var is set)
2. **Cache binary** — caches `~/.cargo/bin/corvid-pet` keyed on `Cargo.toml` + `Cargo.lock` hash (skipped if `CORVID_PET_BIN` set)
3. **Install CLI** — `cargo install corvid-pet --features cli` (skipped on cache hit or if `CORVID_PET_BIN` set)
4. **Run corvid-pet** — executes the appropriate CLI commands based on `mode`
5. **Post PR comment/review** — via `actions/github-script@v7` (conditional on `comment-on-pr`/`review-on-pr` and `pull_request` event)
6. **Update README badge** — sed-based replacement in README.md (conditional on `update-readme` and `badge` mode)

### CORVID_PET_BIN Override

Set the `CORVID_PET_BIN` environment variable to skip the Rust install/cache/build steps and use a pre-built binary. This is useful when the consuming repo already builds corvid-pet (e.g. self-CI in this repo uses `./target/debug/corvid-pet`).

## PR Comment/Review Behavior

### Comment Mode (`comment-on-pr: true`, `review-on-pr: false`)

- Searches existing PR comments for one containing `corvid-pet`
- If found, updates in place (no duplicate comments)
- If not found, creates a new comment

### Review Mode (`review-on-pr: true`)

- Dismisses any previous `github-actions[bot]` reviews containing `corvid-pet` (prevents review pile-up)
- Submits a new review: `APPROVE` if event is `success`, `REQUEST_CHANGES` otherwise
- The review body contains the pet comment markdown

## Invariants

1. The action always produces `mood`, `score`, `art`, and `event` outputs regardless of mode
2. State file is created automatically if it doesn't exist (via `RepoHealth::new()`)
3. PR comments are idempotent — running the action multiple times on the same PR updates the existing comment
4. Previous PR reviews are dismissed before submitting a new one (no review pile-up)
5. The `CORVID_PET_BIN` override skips all Rust installation steps
6. Auto-event detection maps `cancelled` job status to `warning`, not `failure`

## Usage Examples

### Basic PR comment

```yaml
- uses: CorvidLabs/corvid-pet@v1
  with:
    mode: pr-comment
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

### PR review with CI context

```yaml
- uses: CorvidLabs/corvid-pet@v1
  with:
    mode: pr-comment
    review-on-pr: "true"
    context: |
      | Check | Status |
      |-------|--------|
      | Tests | ✅ Passed |
      | Lint  | ✅ Passed |
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

### Health check (no comment)

```yaml
- uses: CorvidLabs/corvid-pet@v1
  id: pet
  with:
    mode: health-check

- run: echo "Health score: ${{ steps.pet.outputs.score }}"
```

### README badge

```yaml
- uses: CorvidLabs/corvid-pet@v1
  with:
    mode: badge
    update-readme: "true"
```

### Pre-built binary (self-CI)

```yaml
- run: cargo build --features cli
- uses: ./
  with:
    mode: pr-comment
  env:
    CORVID_PET_BIN: ./target/debug/corvid-pet
```

## Behavioral Examples

### PR comment on successful CI

```yaml
- uses: CorvidLabs/corvid-pet@v1
  if: always()
  with:
    mode: pr-comment
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

Event auto-detects as `success` from `job.status`. The action runs `react success` to record the event, then `comment success` to generate markdown. Since `comment-on-pr` defaults to `true` and this is a `pull_request` event, the comment is posted (or updated if one already exists).

### Overriding job status with combined check result

```yaml
- uses: CorvidLabs/corvid-pet@v1
  if: always()
  with:
    mode: pr-comment
    job-status: ${{ steps.status.outputs.result }}
    review-on-pr: "true"
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

The `job-status` input overrides `job.status` for event detection, allowing a combined check result to drive the pet's reaction. With `review-on-pr`, the action submits a PR review (APPROVE on success, REQUEST_CHANGES on failure) instead of a plain comment.

### Self-CI with pre-built binary

```yaml
- run: cargo build --features cli
- uses: ./
  if: always()
  with:
    mode: pr-comment
  env:
    CORVID_PET_BIN: ./target/debug/corvid-pet
```

Setting `CORVID_PET_BIN` skips the Rust toolchain install, cargo cache, and `cargo install` steps entirely.

## Error Cases

| Condition | Behavior |
|-----------|----------|
| State file doesn't exist | Created automatically with default health (score 100) |
| State file corrupted | Warning printed to stderr, fresh state used |
| Missing `github-token` | Comment/review step fails silently (action still succeeds) |
| Invalid `mode` value | Shell case statement falls through, no outputs for `comment`/`badge` |
| `CORVID_PET_BIN` points to missing binary | Step fails with exit code |
| PR review dismissal fails | Caught and ignored (review may already be dismissed) |

## Dependencies

### Consumes

| Component | What is used |
|-----------|-------------|
| `corvid-pet` CLI | All subcommands: react, comment, health, badge, show |
| `dtolnay/rust-toolchain@stable` | Rust installation |
| `actions/cache@v4` | Binary caching |
| `actions/github-script@v7` | PR comment/review posting |

### Consumed By

| User | What is used |
|------|-------------|
| Any GitHub repo | All modes via Marketplace |
| corvid-pet CI (self) | pr-comment mode with review-on-pr |

## Change Log

| Date | Change |
|------|--------|
| 2026-04-11 | Initial spec |
