---
spec: cli.spec.md
---

## Key Decisions

- **Feature-gated**: CLI requires `--features cli` — the library crate has zero CLI dependencies by default
- **Clap derive**: Uses clap's derive macros for argument parsing — declarative, type-safe, auto-generates help
- **Shared state file**: All subcommands use the same `.corvid-pet.json` format — no separate configs
- **Color fallback**: Invalid color names warn and fall back to defaults rather than erroring — CLI should never fail on cosmetic input
- **No subcommand = show**: Running `corvid-pet` with no args shows the pet (equivalent to `corvid-pet show`)

## Files to Read First

- `src/bin/corvid-pet.rs` — the CLI entrypoint
- `specs/cli/cli.spec.md` — full specification

## Current Status

v1.0.0 release candidate. All 7 subcommands implemented. Used by the GitHub Action for all CI operations.

## Notes

- The CLI is a thin wrapper around the pet library — all logic lives in `src/lib.rs` and related modules
- Changes to `RepoHealth`, `Pet`, or `Event` types directly affect CLI behavior
