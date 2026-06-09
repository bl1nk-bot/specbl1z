# Specgen Restructure Plan

## Audit Snapshot (current)

Dependency tree recorded for specgen workspace (depth 2).

Top deps observed:
- specgen: anyhow, clap, ignore, serde_json, serde_yaml, specgen-core
- specgen-core: anyhow, bincode, chrono, colored, ignore, jsonschema, pulldown-cmark, pyo3, quick-xml, regex, reqwest, rusqlite, serde, serde_json, serde_yaml, sqlite-vec, toml, uuid (+tempfile dev)
- specgen-api: anyhow, axum, serde, serde_json, specgen-core, tokio, tower-http, tracing, tracing-subscriber

## Goal
Clean repo before refactor. Establish restartable structure, reduce duplicate deps, and prepare a safe restructuring plan without merging until approved.

## Phase 0: cleanup first
- Remove unused scripts under scripts/fix_*.py if still present.
- Drop crate directory model/engine/ if it exists.

## Phase 1: dependency layout
- Introduce [workspace.dependencies] in root Cargo.toml for shared versions: serde, anyhow, thiserror, tracing, tracing-subscriber, chrono.
- Remove duplicate direct declares in crates that can inherit this superset.
- Move reqwest + pyo3 to bound crates if unused in core today; keep as local patches if truly shared later.

## Phase 2: crate grouping
- Review whether backend/api and app/cli should unify to one binary crate.
- If unifying, move cli/src into backend/api/src/bin and update workspace members.
- If keeping separate, enforce [features] layers so core exposes only async/none/full and api/cli opt in.

## Phase 3: verification
- cargo clippy --all-targets -D warnings
- cargo test --workspace
- cargo tree --depth 2 (rerun and diff against this snapshot)

## Rollback
git checkout main && git merge --abort || true

## Open questions (decision needed)
1) Should app/cli merge into backend/api as a single binary?
2) Is pyo3 used only by an optional binding; should it be gated behind a feature?
3) Drop sqlite-vec until semantic search scope is approved?

## Conclusion
Stop here until approvals are given; no dependency edits or file removals performed.

## Approval
- [ ] Who approves removal of model/engine and fix_* scripts?
- [ ] Who chooses crate merge strategy?

## Optional tags
dependency-cleanup repo-restructure audit
