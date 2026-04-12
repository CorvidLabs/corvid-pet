---
spec: action.spec.md
---

## Key Decisions

- **Composite action**: Uses `runs: using: composite` rather than Docker or JavaScript — keeps it simple and transparent
- **Rust install per-run**: Installs Rust toolchain and builds from source each run (cached). No pre-built binaries distributed yet
- **CORVID_PET_BIN escape hatch**: Self-CI and pre-built workflows can skip the Rust install by pointing to an existing binary
- **Idempotent comments**: PR comments are upserted (find existing, update or create) to avoid spam
- **Review dismiss-and-replace**: Previous reviews are dismissed before posting new ones to prevent review pile-up
- **Event auto-detection**: Maps `job.status` to corvid-pet event types so users don't need to wire status manually
- **No input validation**: Invalid `mode` values silently produce no output rather than failing the action — deliberate choice to avoid breaking CI on config errors

## Files to Read First

- `action.yml` — the entire action definition
- `specs/action/action.spec.md` — full specification

## Current Status

v1.0.0 release candidate. All 5 modes implemented and working. Self-CI uses pr-comment mode with review-on-pr. Binary caching via actions/cache@v4.

## Notes

- The action depends on the CLI which depends on the pet library — changes to CLI subcommand signatures may require action.yml updates
- PR review mode uses `github-actions[bot]` as the reviewer identity — this is determined by the github-token, not configurable
