---
spec: action.spec.md
---

## Tasks

- [x] Define action.yml with all inputs, outputs, branding
- [x] Implement composite steps (Rust install, cache, CLI run)
- [x] Implement all 5 modes (pr-comment, health-check, greet, release, badge)
- [x] Add event auto-detection from job.status
- [x] Implement PR comment posting/updating via github-script
- [x] Implement PR review mode with dismiss-and-replace
- [x] Implement README badge update via sed
- [x] Add CORVID_PET_BIN override for self-CI
- [x] Add review-on-pr input and approval logic
- [x] Add job-status input for combined check overrides
- [x] Write action.spec.md covering all modes, inputs, outputs, steps
- [x] Test self-CI with pr-comment and review-on-pr modes

## Done

- [x] Initial action.yml implementation
- [x] Spec written and reviewed

## Gaps

- No integration tests for badge mode README update
- No validation of `mode` input (shell case falls through silently on invalid values)

## Review Sign-offs

- **Product**: pending (pre-1.0.0)
- **QA**: tested in self-CI (pr-comment + review modes)
- **Dev**: implementation complete, spec written 2026-04-11
