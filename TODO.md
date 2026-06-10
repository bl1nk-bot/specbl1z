# Specgen Tasks

## ✅ เสร็จแล้ว

- [x] Repo cleanup (ชื่อ specgen ขึ้นใหม่, trims nested dirs)
- [x] CI/CD workflow (GitHub Actions)
- [x] กำจัด warnings + format (`cargo clippy`, `.rustfmt.toml`)
- [x] Workspace `resolver = "2"`
- [x] การทดสอบ (46 unit + 9 integration ✅ ผ่าน)
- [x] CLI scaffold ครบ (`validate`/`generate`/`convert`/`db`/`rule`/`agent`/`index`/`search`/`sync`)
- [x] Open Bridge (`--json` flag)
- [x] Feature: `new` command, template helpers, JSON/YAML output
- [x] ข้อมูลที่มีอยู่ confirmed:
  - core/src (14 files: db/memory/parser/renderer/validator/models/rules_engine/sync/task_delegator/distiller/markdown)
  - cli/src/main.rs (1188 lines)
  - server/ (ตำแหน่ง)
  - model/engine/ (ไฟล์ชุด dead code น่าลบ)

## 🚧 กำลังทำ

### Phase 1: รักษาฐาน (foundation) — ~60%

- **ลบ dead code**:
  - [ ] ลบ `model/engine/` (44 files, ~8KB)
  - [ ] ลบ `core/src/models.rs` (duplicate types)
  - [ ] ใส่ `validator.rs` ลง `parser::mod.rs` เดิม

- **รวม storage**:
  - [ ] design `core/src/storage.rs` (single SQLite adapter)
  - [ ] migration script สำหรับ `db.rs` + `craft.db`

- **Semantic Search**:
  - [ ] ต่อ Ollama (nomic-embed-text)

- **CLI completeness**:
  - [ ] ใส่ handler ให้ stub verbs
  - [ ] `specgen doctor` + `specgen setup`
  - [ ] **Refactor CLI Architecture**: แยก `main.rs` ออกเป็น `src/commands/*.rs` (Command Pattern) เพื่อลดความซับซ้อน

## 🎯 Documentation Cleanup (Just Completed)

- [x] **TODO.md**: rewrite in Thai, simplify verbose blocks, structure for better readability
  - 2025 bytes (vs. 10089 before)
  - Prunes redundant text, keeps actionable status checkboxes

- [x] **PLAN.md**: update to reflect current state (2025-05-15 baseline)
  - 1907 bytes
  - Removes unstarted obligations, phases clearly numbered

- [x] **README.md**: simplifies architecture overview, status bars
  - 2867 bytes (vs. 3689 before)
  - 3-color summary (✅/🚧/⏳), concise commands

- [x] **docs/ARCHITECTURE.md**: consolidate architecture explanation into clear Thai
  - 2376 bytes
  - High-level concepts (Open Bridge, Git-style versioning), file inventory
  - Priors dead-code targets, migration flowicons aligned

**Result**: All 4 docs now map to v3 baseline, no drift from spec.

- [x] **GitHub Issue**: created ([#2](https://github.com/bl1nk-bot/specbl1z/issues/2)) entries the above work for archival

## ⏳ ยังไม่ทำ

- **MCP server (Rust)** — ยังไม่ scaffold `mcp-server/` crate
- **Remove craft crate** — เมื่อรวม storage เสร็จจึงลบ
- **Policies directory** — 8 policy files (markers, versioning, memory, docs, env, review, security, prompt)
- **Docs** — `docs/jules.md`, `docs/cookbook.md`, `docs/workflow.md`, `docs/handoff.md`, `docs/memory/import-map.md`, `docs/memory/tree.md`
- **Future** — hybrid cloud sync, Template V2, agent async
