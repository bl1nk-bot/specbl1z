---
name: branch-verification
description: >
  Systematic pre-merge verification workflow for Rust monorepo branches.
  Check compilation, tests, clippy, grep leftovers, then fix → commit → push → PR.
  Use when user says "check work done in this branch" or before merging any PR.
source: auto-skill
extracted_at: '2026-07-25T11:31:54.176Z'
---

# Branch Verification Workflow

Systematic pre-merge QA checklist for Rust workspace branches. Run before merging any feature/fix branch into `main`.

## Steps

### 1. Understand the branch context

```bash
git log --oneline -10              # what commits exist
git diff HEAD --stat                # what's still unstaged (expected vs leftover)
git status                          # overall state
```

Read the commit messages to understand the branch's intent before inspecting diffs.

### 2. Inspect actual changes

```bash
git diff <base>..HEAD --stat        # full scope of changes on this branch
git diff <base>..HEAD -- <file>     # deep-dive specific files of interest
```

If there are multiple commits, inspect each one separately to understand the incremental changes.

### 3. Verify compilation

```bash
cargo check --workspace            # fast: type-check only
```

If compilation fails, read the first error, fix it, then re-run.

### 4. Run tests

```bash
cargo test --workspace             # all unit + integration tests
```

Count the test results: note `X passed; 0 failed` — do not accept failures.

### 5. Run clippy with deny warnings

```bash
cargo clippy -- -D warnings        # zero warnings policy
```

Zero warnings enforced by `clippy.toml` policy. If clippy passes without warnings output — confirm clean.

### 6. Check for leftover references

Grep for the removed/refactored concept across the entire codebase:

```bash
grep -r '<concept>' --include='*.rs' --include='*.toml' --include='*.yaml' --include='*.yml' .
```

Check: `.rs`, `.toml`, `.yaml`, `.yml`, `.sh`, `.md` — any file type that could reference the removed dependency or renamed symbol.

### 7. If bugs found → fix → re-verify

1. Identify root cause (diff of the broken commit)
2. Apply surgical fix
3. Re-run: `cargo check` → `cargo test` → `cargo clippy`
4. Commit with descriptive message explaining what was wrong and why

### 8. Commit → Push → PR

```bash
git add <fixed_file>
git commit                        # must match [MARKER] format from project hook
git push
gh pr create                      # title + body with summary of changes + verification results
```

## Rules

- **Never assume**: compile error could be anywhere — read first error message, don't guess
- **Never skip clippy**: `cargo clippy -- -D warnings` is project policy
- **Verify all three** before declaring done: `check` + `test` + `clippy`
- **Grep leftover references**: removing a dependency doesn't count if code still mentions it in comments or docs
- **Commit hook enforces format**: `[MARKER] Description` — markers: FEAT|FIX|DOC|CHORE|SANDBOX|CI|SEC|PERF|TEST|REFACTOR|DESIGN|PLAN|SPEC|CORE|CLI|SERVER
