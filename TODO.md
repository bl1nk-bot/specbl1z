# Specgen Tasks — v3.1

## ✅ เสร็จแล้ว

### Cleanup & Structure
- [x] Dead code removed (model/engine/, model/, logs/, tests/)
- [x] Docs consolidated (CLAUDE.md, GEMINI.md → @AGENTS.md stubs)
- [x] Structure flattened (backend/core→core, backend/api→api, app/cli→cli)
- [x] Paths fixed (Cargo.toml, config.toml, db.rs, api/lib.rs)
- [x] Dependencies cleaned (sqlite-vec removed)

### DevOps
- [x] .gitignore fixed
- [x] clippy.toml + rust-toolchain.toml
- [x] Makefile (16 targets: all, check, test, lint, fmt, build, release, setup, wizard, bench, docker-*)
- [x] Git hooks (pre-commit: devops.sh, commit-msg: [MARKER] format)
- [x] rust-ci.yml (cache + lockfile check)
- [x] cross-platform.yml (Linux, macOS, Android, MSRV, Docker, integration)
- [x] release.yml (linux-amd64 + android-arm64 + sha256)
- [x] dependabot.yml (cargo weekly)
- [x] .github/prompts/ (anti-slop, auto-doc, dead-code-hunter, security-audit)

### Build Fixes
- [x] 8 missing Database methods added (create_document, list_documents, create_collection, add_property, list_agents, list_skills, table_exists, count_table_rows)
- [x] All clippy warnings fixed
- [x] All formatting fixed

### Sandbox SDK
- [x] sandbox/ Rust crate (Daytona + Modal + Webhook)
- [x] .opencode/ TypeScript plugins + skills
- [x] Benchmarks (criterion: serialize, deserialize, create)

### Cross-Platform
- [x] scripts/setup.sh (6 platforms)
- [x] scripts/wizard.sh (interactive 5-step)
- [x] scripts/integration-test.sh (real-world pipeline)
- [x] Docker images (debian, alpine, android, bare, msrv)
- [x] docker-compose.yml (7 services)

### Testing
- [x] 57 tests total (7 cli + 4 api + 36 core + 9 integration + 1 sandbox)
- [x] 4 benchmarks
- [x] cargo check + test + clippy + fmt all pass

### Documentation
- [x] SPEC.md updated (v3.1)
- [x] PLAN.md updated
- [x] TODO.md updated
- [ ] README.md updated

## 🚧 กำลังทำ

### Phase 7: Database Abstraction
- [ ] trait DatabaseBackend
- [ ] sqlite.rs (rusqlite)
- [ ] mysql.rs (sqlx via feature flag)
- [ ] config.toml backend selection

### Phase 8: Semantic Search
- [ ] Ollama nomic-embed-text
- [ ] Vector search via chunks table

### Phase 9: MCP Server
- [ ] Rust MCP server crate
- [ ] Direct FFI to core (< 50ms)

## ⏳ ยังไม่ทำ

- MySQL via sqlx feature flag
- policies/ directory (8 policy files)
- Docs: jules.md, cookbook.md, workflow.md, handoff.md
- Template V2, agent async
- Hybrid cloud sync
