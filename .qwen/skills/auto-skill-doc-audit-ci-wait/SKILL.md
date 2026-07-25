---
name: doc-audit-ci-wait
description: >
  Audit and fix documentation (README, ARCHITECTURE, CHANGELOG) against current codebase
  while waiting for CI/bot reviews on a PR. Fix factual drift, dead references, and add
  changelog entries — then commit docs to the same branch.
source: auto-skill
extracted_at: '2026-07-25T11:37:12.012Z'
---

# Doc Audit During CI Wait

When a PR is pushed and CI/bot reviews are pending, use the wait time proactively:
audit documentation for drift against the current codebase, fix what's wrong, and push
docs to the same branch so they become part of the PR.

## Trigger

After pushing a PR, when the user says "wait for bot" / "รอ bot comment" or CI checks are still running.

## Steps

### 1. Audit README.md

Check these against the actual project state:

| Check | Command / Method |
|-------|------------------|
| File counts in structure tree | `ls -la core/src/*.rs \| wc -l`, count actual `.rs` files per crate |
| Directories that no longer exist | `ls` each path mentioned in the structure — remove dead entries |
| Files that exist but aren't listed | `ls` root and each subdirectory — add missing entries (e.g., `config.toml`) |
| Date at bottom | Compare against today's date |
| Build/test counts | `cargo test --workspace 2>&1 \| grep "test result"` — verify counts match |
| Deprecation/spec mentions | Does README reference removed deps, old APIs, or renamed commands? |

Fix: update dates, file counts, structure, and remove dead references.

### 2. Audit docs/ARCHITECTURE.md

| Check | What to look for |
|-------|------------------|
| Dead directories | References to `server/`, `model/engine/`, `model/`, `logs/`, `tests/` — these were deleted in Phase 1 |
| Missing crates | `sandbox/`, `api/` — are they listed? If not, add |
| Outdated migration plans | Phase 1-3 concepts that are already done should be removed or archived |
| File names | `craft.db` → `data/craft.db`, `backend/core/` → `core/` |
| Key decisions | Are removed deps mentioned? (e.g., `pyo3 removed, distiller stubbed`) |

Fix: rewrite the architecture section to reflect current workspace layout only.

### 3. Audit docs/CHANGELOG.md

| Check | Action |
|-------|--------|
| Unreleased section exists | If no `[Unreleased]` section, add one |
| Current PR's changes recorded | Add entries under `### Fixed` or `### Changed` for what this branch does |
| Old repo URLs | Compare compare links against `git remote get-url origin` — fix if project was renamed |
| Date format consistency | Ensure all dates use same format (ISO 8601 recommended) |

### 4. Cross-check against TODO.md

Look at `TODO.md` for any unchecked doc tasks (e.g., `README.md updated`) — these are
explicitly waiting for the doc pass you're doing now.

### 5. Commit and push to the same branch

```bash
git add README.md docs/ARCHITECTURE.md docs/CHANGELOG.md
git commit -m "[DOC] update <summary> for current state"
git push
```

The commit will appear on the same PR, so the CI re-runs with the docs fix included.

## Why this matters

- Documentation drift accumulates fast in active monorepos — CI wait time is the ideal
  window to catch it before merge
- Having docs updated as part of the same PR means reviewers see fresh documentation,
  not a stale artifact
- Changelog entries for the PR's own changes prevent forgotten changelogs at release time
