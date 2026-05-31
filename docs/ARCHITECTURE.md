# สถาปัตยกรรม Specgen (v3)

## แนวคิดหลัก: Open Bridge Architecture (ไม่ใช่ black box)
- CLI เป็นสะพานที่โปร่งใสระหว่างผู้ใช้และ core
- ทุก CLI command รองรับ `--json` (ใช้ `jq` / Python ต่อได้)
- Rust: เร็ว ทนทาน
- ไม่ spawn new process แต่ call core ทันที (FFI) → target <50ms

## Git-like versioning (แนวคิด โปรยงาน cross-OS)
```
Row → insert-only immutable
Hash (SHA-256) + parent_id
Last-Write-Wins (LWW)

<--- conflict detected (same parent, diff hash)
  → `conflicts` table → manual or auto resolve
```

## ไฟล์ที่มีอยู่ (Current State)
```
specgen/
├── Cargo.toml          # [workspace] core + cli
├── core/src/           # 14 files:
│   ├── lib.rs, parser/(), renderer.rs, validator.rs, models.rs
│   ├── db.rs, memory.rs, rules_engine.rs, schema.rs, sense.rs
│   ├── sync.rs, task_delegator.rs, distiller.rs, markdown.rs
│   └── bl1nk/ (generated)
├── cli/src/main.rs     # 1188 lines
├── server/             # TypeScript/Hono web
├── model/engine/       # dead code (8KB, 44 files)
└── craft.db            # SQLite DB
```

## ไฟล์หาย (Missing)
- `mcp-server/` (Silver ยังไม่ scaffolf)
- `storage.rs` (เพิ่ม unified adapter)
- `policies/` (8 ไฟล์)
- `docs/jules.md`, `docs/cookbook.md`, etc.

## ลบทิ้ง (Dead Code)
- `model/engine/` (44 files, ~8KB) ← directory
- `core/src/models.rs` ← duplicate proto types
- `core/src/validator.rs` ← ส่งให้ inline เข้า parser

## แนวคิด migration (Phase 1-3)

Phase 1 (foundation):
- ลบ dead code ✅ mapped
- create `storage.rs` (unified DB)
- cipher search

Phase 2 (Direct MCP):
- scaffold `mcp-server/`
- implement tools ที่ call core → latency <50ms (from 150ms)
- deprecate แต่เก็บ `app/src/mcp.ts` เป็น fallback

Phase 3 (polish):
- remove craft crate
- ด้าน policies, docs

---

## Performance Targets (future)
- MCP latency <50ms (p99)
- Binary size <15MB (release)
- Startup <200ms
- integration tests 100%
- no dead code detected

**Source:** SPEC.md v3
