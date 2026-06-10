# Specgen แผนการพัฒนา — v3.1

## ✅ Phase 0-6: Baseline + Cleanup + DevOps (เสร็จแล้ว)

### Phase 0: Safety Baseline
- git branch cleanup/reorg, cargo clean (ลบ 1GB)
- stash uncommitted changes

### Phase 1: Delete Dead Code
- model/engine/ — sovereign_engine crate (9 .rs files)
- model/ — bl1nk conductor files, opencode.py, caches
- logs/, tests/ (empty)
- docs/ARCHITECT_V2.md, docs/PLAN_V2.md, PLAN-RESTRUCTURE.md

### Phase 2: Doc Consolidation
- CLAUDE.md + GEMINI.md → @AGENTS.md stubs
- ยุบจาก ~18 docs เหลือ ~8

### Phase 3: Structure Flatten
- backend/core/ → core/
- backend/api/ → api/
- app/cli/ → cli/
- backend/schema/ → schema/
- backend/craft.db → data/craft.db

### Phase 4: Path Fixes
- Cargo.toml workspace members updated
- All internal paths updated (Cargo.toml, config.toml, api/lib.rs, db.rs)

### Phase 5: Dependency Cleanup
- sqlite-vec removed (dead dep, 0 refs)
- All deps verified as actually used

### Phase 6: Fix Broken Build
- Added 8 missing methods to core::Database
- Fixed clippy errors, formatting
- cargo check + test + clippy + fmt all pass

### DevOps Infrastructure
- .gitignore fixed
- clippy.toml + rust-toolchain.toml
- Makefile (16 targets)
- rust-ci.yml (cache, Swatinem, lockfile check)
- cross-platform.yml (Linux, macOS, Android, MSRV, Docker)
- release.yml (multi-target binary)
- dependabot.yml (cargo weekly)
- .github/prompts/ (4 agent instructions)
- Git hooks installed (pre-commit + commit-msg)

### Sandbox SDK (OpenCode Integration)
- sandbox/ Rust crate (Daytona + Modal + Webhook)
- .opencode/ TypeScript plugins (Bun/macOS/Linux)
- .opencode/skills/ (daytona + modal)

### Cross-Platform
- scripts/setup.sh (6 platforms: Termux, Debian, Alpine, Arch, macOS, Linux)
- scripts/wizard.sh (interactive, 5 steps)
- scripts/integration-test.sh (real-world pipeline)
- Docker images: 5 platforms + docker-compose
- Benchmarks: criterion (serialize, deserialize, create)

## 🎯 Test Coverage

| Crate | Unit Tests | Integration | Benchmarks |
|-------|-----------|-------------|------------|
| core | 36 | 9 | - |
| cli | 7 | - | - |
| api | 4 | - | - |
| sandbox | 1 | - | 4 |
| **Total** | **48** | **9** | **4** |

## ⏳ Phase 7: Database Abstraction (next)
- trait DatabaseBackend
- sqlite.rs (rusqlite) — default
- mysql.rs (sqlx) — feature flag
- Auto-detect via config.toml

## ⏳ Phase 8: Semantic Search
- Ollama nomic-embed-text integration
- Vector search via chunks table

## ⏳ Phase 9: MCP Server
- Rust MCP server crate
- Direct FFI to core (latency < 50ms)

## มาตรฐาน

```bash
cargo check --workspace        # PASS
cargo test --workspace          # PASS (57 tests)
cargo clippy --workspace -- -D warnings  # PASS
cargo fmt --all -- --check      # PASS
cargo bench -p specgen-sandbox  # PASS (4 benchmarks)
```

Target:
- MCP latency < 50ms
- Binary size < 15MB (release)
- Startup < 200ms
