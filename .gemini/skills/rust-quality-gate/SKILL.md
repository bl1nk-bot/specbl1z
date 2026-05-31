---
name: rust-quality-gate
description: Mandatory project quality workflow for the specgen Rust monorepo. Enforces zero-warning policy, formatting standards, and test compliance. Trigger before any commit, pull request, or completion of a coding task. Also trigger when the user says "check quality", "run QA", or "is it ready?".
---

# Rust Quality Gate

This skill enforces the "Zero Warnings and Stability" priority for the specgen project. It ensures that all code meets production standards before being finalized.

## Prerequisites
- Rust toolchain (cargo, clippy, rustfmt)
- `scripts/devops.sh` (local automation script)

## Procedure

1. **Local Validation**: Always run the comprehensive check script before proposing any commit.
   ```bash
   ./scripts/devops.sh
   ```
   This script executes:
   - `cargo fmt --all -- --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo test --all-targets --all-features`

2. **Commit Message Compliance**: Verify that the commit message follows the mandatory marker format.
   - Format: `[MARKER] Description` or `MARKER: Description`
   - Allowed Markers: `CORE`, `CLI`, `SERVER`, `FEAT`, `FIX`, `CHORE`, `DESIGN`, `PLAN`, `SPEC`, `SEC`, `REVIEW`, `LEARN`, `LOOP`.

3. **Dependency Check**: If new dependencies were added, ensure they are compatible with the Termux environment.

## Verification Checklist
- [ ] `cargo clippy` returns 0 warnings and 0 errors.
- [ ] `cargo fmt` returns no diffs.
- [ ] All 50+ unit and 9+ integration tests pass.
- [ ] Commit message includes a valid project marker.

## Common Pitfalls
- **Termux Binary Issues**: Some tools (like Biome) fail on Termux. The `devops.sh` script handles this by skipping with a warning; do not fail the build for this specific case on Android.
- **Unused Imports**: The zero-warning policy includes `unused_imports`. Remove them surgically before finishing.
