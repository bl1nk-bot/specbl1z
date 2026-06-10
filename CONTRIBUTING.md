# Contributing to specgen

## Commit Convention

Every commit must start with a marker:

```
[MARKER] Description (BNK-NNN)
```

Valid markers: FEAT FIX DOC CHORE SANDBOX CI SEC PERF TEST REFACTOR DESIGN PLAN SPEC CORE CLI SERVER

Reference the Linear issue: `(BNK-21)` at end of first line or in body.

Examples:
```
[FEAT] add database abstraction layer (BNK-21)
[SANDBOX] test Daytona connection end-to-end
[CI] add cross-platform build matrix
[FIX] resolve clippy warnings (BNK-22)
```

## Branch Naming
```
<type>/<short-desc>
```
Types: feat/ fix/ chore/ sandbox/ ci/ doc/

Examples: `sandbox/daytona-connection`, `chore/cleanup-reorg`, `ci/cross-platform-matrix`

## PR Checklist
Every PR must pass:
- cargo check --workspace
- cargo test --workspace (57 tests)
- cargo clippy --workspace --all-targets -- -D warnings
- cargo fmt --all -- --check

Use the PR template at `.github/PULL_REQUEST_TEMPLATE.md`

## Issue Tracking
- **GitHub Issues**: bug reports, feature requests, config changes
  - Use templates in `.github/ISSUE_TEMPLATE/`
- **Linear**: active development tracking (team BNK, project bl1nkflow)
  - Reference Linear IDs in commits: `(BNK-NNN)`
  - Move issue to "In Progress" when starting work
  - Move to "Done" when merged

## Linear ↔ GitHub Sync
1. Create issue on Linear with full spec
2. Branch from main: `feat/my-feature`
3. Commit with marker + Linear ref: `[FEAT] my feature (BNK-21)`
4. Open PR using template
5. On merge, close both GitHub issue and Linear issue
