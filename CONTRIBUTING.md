# Contributing to corvid-pet

Thank you for your interest in contributing! This guide covers the development workflow and CI setup.

## Quick Start

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests locally: `cargo test --all-features`
5. Push and open a PR

## CI/CD Workflow

We use GitHub Actions with `ubuntu-latest` runners for consistent builds and caching.

### Workflow Overview

The CI runs on:
- Push to `main` branch
- Pull requests targeting `main`
- Manual triggers via `workflow_dispatch`

### Jobs

| Job | Purpose | Runner |
|-----|---------|--------|
| `check` | Format check, clippy, build, tests | `ubuntu-latest` |
| `corvid-pet` | PR comments and auto-approval | `ubuntu-latest` |

### Caching Strategy

We cache the following paths:
- `~/.cargo/registry` - Downloaded dependencies
- `~/.cargo/git` - Git dependencies
- `target` - Build artifacts

**Cache Key:** `${{ runner.os }}-cargo-${{ hashFiles('Cargo.lock') }}`

**Cache Behavior:**
- `main` branch builds create the base cache
- PRs restore from `main` cache first, then create their own
- Fork PRs can read from `main` cache but write to their own scope
- Cache retention: 7 days

### Running Checks Locally

Before pushing, run these locally:

```bash
# Format check
cargo fmt --check

# Linting
cargo clippy --all-features -- -D warnings

# Run tests
cargo test --all-features

# Build all features
cargo build --all-features

# Build examples
cargo build --examples --all-features
```

## Pull Request Process

1. Ensure CI passes (fmt, clippy, tests)
2. The `corvid-pet` job will automatically comment on your PR
3. Maintainers will review and merge

## Feature Development

### Adding New Features

When adding features that change dependencies:

1. Update `Cargo.toml` with the new feature flag
2. Document the feature in the README
3. Ensure CI passes with `--all-features`

### Adding Examples

New examples go in `examples/` directory:
- Name the file descriptively (e.g., `my_feature_demo.rs`)
- Add a comment header explaining what it demonstrates
- Test with: `cargo run --example my_feature_demo --features <required_features>`

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG (if maintained)
3. Tag release: `git tag v1.x.x`
4. Push tags: `git push origin v1.x.x`
5. GitHub Actions will run tests on the tag

## Questions?

Open an issue or discussion on GitHub.