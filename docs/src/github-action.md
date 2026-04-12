# GitHub Action

corvid-pet provides a composite GitHub Action that tracks your repo's health and posts pet-powered comments on pull requests.

## Quick Start

```yaml
- uses: CorvidLabs/corvid-pet@v1
  with:
    mode: pr-comment
```

That's it. The action installs the CLI, records the current job status, and posts a comment on the PR with your pet's reaction and health stats.

## Modes

### `pr-comment` (default)

Records a CI event and generates a markdown PR comment with pet art, a mood-appropriate quip, and health stats.

```yaml
- uses: CorvidLabs/corvid-pet@v1
  with:
    mode: pr-comment
    context: "All 42 tests passed"
```

### `health-check`

Records an event and outputs health data as JSON. Useful for downstream steps.

```yaml
- uses: CorvidLabs/corvid-pet@v1
  id: health
  with:
    mode: health-check
```

### `greet`

Posts a welcome comment on new PRs (pair with `types: [opened]`).

```yaml
on:
  pull_request:
    types: [opened]

jobs:
  welcome:
    runs-on: ubuntu-latest
    steps:
      - uses: CorvidLabs/corvid-pet@v1
        with:
          mode: greet
```

### `release`

Generates a celebratory comment for releases.

```yaml
- uses: CorvidLabs/corvid-pet@v1
  with:
    mode: release
    context: "v1.2.0"
```

### `badge`

Generates a status badge line for README embedding.

```yaml
- uses: CorvidLabs/corvid-pet@v1
  with:
    mode: badge
    update-readme: "true"
```

## Inputs

| Input | Default | Description |
|-------|---------|-------------|
| `mode` | `pr-comment` | Action mode: `pr-comment`, `health-check`, `greet`, `release`, `badge` |
| `event` | `auto` | Event type: `auto` (detect from job status), `success`, `failure`, `warning`, `progress`, `idle` |
| `pet-name` | `Corvin` | Name for your corvid companion |
| `state-file` | `.corvid-pet.json` | Path to the health state JSON file |
| `context` | `""` | Additional context for comments (e.g. CI summary) |
| `comment-on-pr` | `true` | Whether to post a comment on the PR |
| `review-on-pr` | `false` | Submit as a PR review (APPROVE/REQUEST_CHANGES) instead of a plain comment |
| `update-readme` | `false` | Update the README badge (for badge mode) |
| `persist-health` | `true` | Persist health state across CI runs using GitHub Actions cache |
| `github-token` | `${{ github.token }}` | GitHub token for posting comments |

## Outputs

| Output | Description |
|--------|-------------|
| `mood` | The pet's current mood |
| `score` | Repo health score (0-100) |
| `comment` | The generated comment markdown |
| `art` | The ASCII art output |
| `event` | The resolved event type |

## PR Reviews

Set `review-on-pr: "true"` to submit the pet comment as a formal PR review. The action posts `APPROVE` on success events and `REQUEST_CHANGES` on failure. Previous corvid-pet reviews are automatically dismissed when a new one is posted.

```yaml
- uses: CorvidLabs/corvid-pet@v1
  with:
    mode: pr-comment
    review-on-pr: "true"
```

## Health Persistence

By default (`persist-health: "true"`), the action uses GitHub Actions cache to persist health state across CI runs. Each run saves state with a run-specific key and restores from the most recent prior run via prefix matching.

## Auto Event Detection

When `event` is set to `auto` (the default), the action detects the event type from the job status:

| Job Status | Event |
|-----------|-------|
| `success` | `success` |
| `failure` | `failure` |
| `cancelled` | `warning` |
| anything else | `progress` |

## Full Example

```yaml
name: CI
on: [pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features

  pet:
    needs: check
    if: always()
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: CorvidLabs/corvid-pet@v1
        with:
          mode: pr-comment
          review-on-pr: "true"
          context: "CI completed"
```
