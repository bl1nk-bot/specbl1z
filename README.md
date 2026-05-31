# Specgen — AI-Native Spec-Driven Development (v3)

**สถานะ:** Phase 1 กำลังทำ (dead code audit ✓, storage unification pending)  
**จำนวน file:**
- 14 files ใน `core/src`
- 1188 lines ใน `cli/src/main.rs`
- server/ มีอยู่ (web dashboard)
- `model/engine/` ยังอยู่ (dead code target)
- `mcp-server/` ยังไม่มี

---

## โครงสร้างปัจจุบัน

```
specgen/
├── Cargo.toml          # [workspace] members: ["core", "cli"]
├── core/               # 14 files
│   ├── parser/, renderer/, db/, memory/, rules_engine/, sense/, sync/
│   ├── task_delegator/, distiller/, markdown/, validator/, models.rs
│   └── schema.rs
├── cli/                # 1188 lines main.rs
├── server/             # TypeScript/Hono web dashboard
└── craft.db            # Legacy SQLite
```

## Quick Start

```bash
cargo build
cargo run -p specgen -- validate path/to/template.json
cargo run -p specgen -- generate MY_TEMPLATE -- to json
cargo run -p specgen -- convert template.md --to json
cargo run -p specgen -- memory list --json | jq '.[].key'
```

## สรุปสถานะ

✅ **เสร็จแล้ว:**
- Workspace core+cli
- CLI scaffold, `--json` flag
- Zero warnings
- Tests 100%
- Template helpers, output format, `convert`, Markdown round-trip server/, docs/ anchoring

🚧 **กำลังทำ:**
- Dead code removal (2 targets จับต้องแล้ว: models.rs + validator.rs)
- Unified storage (plan design pending)
- Semantic search (Ollama ยังไม่ต่อ)
- CLI verbs implementation (parse/stubs)

⏳ **ยังไม่ทำ:**
- `mcp-server/` (Rust MCP binary)
- Remove craft crate
- `policies/` directory (8 files)
- Docs (jules, cookbook, workflow, handoff, import-map, tree)

---

## Commands

| Command | Purpose |
|---|---|
| `specgen validate <file>` | Verify template |
| `specgen generate <id>` | Render output |
| `specgen convert <file> --to json\|md\|toml` | Switch format |
| `specgen db list / show / delete` | View Craft DB |
| `specgen rule ...` | Rules management |
| `specgen agent ...` | Agent tasks |
| `specgen index build` | Rebuild index (Ollama) |
| `specgen search <query>` | Semantic search |
| `specgen sync` | Craft DB sync (cross-OS) |
| `specgen new` | Scaffold new template |
| `specgen doctor` | Checklist tools |
| `specgen setup` | Install missing tools |

---

## Specsup>

| File | Detail |
|---|---|
| [SPEC.md](./SPEC.md) | V3 vision, features |
| [PLAN.md](./PLAN.md) | Roadmap 3 phases |
| [TODO.md](./TODO.md) | Task list ✅/🚧/⏳ |

---

## Build & Test

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo build --release
cargo run -p specgen -- --help
```

---

**License:** MIT
**Updated:** 2025-05-15  ✅
