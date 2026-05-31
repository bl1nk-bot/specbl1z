# Specgen แผนการพัฒนา

## สถานะที่ตรวจสอบแล้ว (2025-05-15)

```
specgen/
├── Cargo.toml          # [workspace] {"core","cli"}
├── core/               # 14 files (parser/renderer/db/memory/rules_engine/sync/task_delegator/markdown...)
├── cli/                # main.rs 1188 lines
├── server/             # web dashboard (TS)
├── model/engine/       # dead code target
└── craft.db            # legacy DB
```

ไม่มี:
- `mcp-server/` (ยังไม่สร้าง)
- `storage.rs`
- `policies/`
- ด้าน docs เช่น `jules.md`, `cookbook.md` (ยังไม่จบ)

## ✅ Phase 0: Baseline Stabilization

- Workspace (core+cli), สคริปต์ QA (`scripts/devops.sh`), CI/CD, rules ที่มี, ทดสอบ 100% ✅

## 🚧 Phase 1: foundation — ~60%

### 1.1 ลบ dead code
| Target | สถานะ |
|---|---|
| `model/engine/` | 1 Chart ✅ (ตรวจเจอแล้ว) |
| `core/src/models.rs` | 1 Chart ✅ |
| `validator.rs` | 1 Chart ✅ |

### 1.2 รวม storage
- [ ] design `storage.rs`
- [ ] write migration (Craft + core to unified)

### 1.3 Semantic Search
- [ ] ต่อ Ollama nomic-embed-text

### 1.4 CLI completeness
- [ ] ใส่ handler verbs
- [ ] 도 check doctor/setup

## ⏳ Phase 2: Direct MCP — NOT STARTED

- ทำ `mcp-server/` crate → call core ทันที
- Benchmark latency <50ms (จาก TS spawn 150ms now)
- Update conifg ให้ point toward rust binary

## ⏳ Phase 3: Polish — NOT STARTED

- Remove craft crate
- policies, docs, hybrid cloud sync

## มาตรฐานสำคัญ

```bash
cargo check --all        # ✅ ผ่านแล้ว
cargo test --all          # ✅ 100%
```

Target ในอนาคต:
- MCP latency <50ms
- Binary size <15MB (release)
- Startup <200ms
