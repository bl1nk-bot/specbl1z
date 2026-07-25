# สถาปัตยกรรม Specgen (v3.1)

## Open Bridge Architecture
- CLI เป็นสะพานโปร่งใสระหว่างผู้ใช้กับ core engine
- ทุก CLI command รองรับ `--json` สำหรับ machine-readable output
- Rust: เร็ว ทนทาน — ไม่ spawn process, call core โดยตรง
- 4 crates ใน workspace: `core` + `cli` + `api` + `sandbox`

## โครงสร้างปัจจุบัน

```
specgen/
├── core/           # engine หลัก
│   ├── lib.rs
│   ├── db.rs           # SQLite database abstraction
│   ├── memory.rs       # Memory engine (CRUD + versioning)
│   ├── parser/         # Template parser (markdown + toml)
│   ├── renderer.rs     # Template renderer
│   ├── validator.rs    # Schema validation
│   ├── schema.rs       # JSON schema loader
│   ├── models.rs       # Domain types
│   ├── rules_engine.rs # Policy/rule evaluation
│   ├── sync.rs         # Git-like versioning (LWW + conflict)
│   ├── task_delegator.rs # Agent task management
│   ├── distiller.rs    # Skill metadata extraction (stub)
│   ├── guardrail.rs    # Policy enforcement
│   ├── sense.rs        # Intent detection
│   └── markdown.rs     # Markdown serialization
├── cli/            # CLI binary
│   └── src/main.rs     # ~1260 lines, 8 commands
├── api/            # REST API server (Axum + Tokio)
│   └── src/lib.rs      # 4 tests
├── sandbox/        # OpenCode SDK Rust
│   └── src/lib.rs      # Daytona + Modal + Webhook
├── .opencode/      # OpenCode plugins (TypeScript)
├── data/           # SQLite database
├── docker/         # Multi-platform images
├── scripts/        # setup.sh, wizard.sh, devops.sh
├── .github/        # CI/CD workflows + agent prompts
└── docs/           # Documentation
```

## Versioning (Git-like)
```
Row → insert-only immutable
Hash (SHA-256) + parent_id
Last-Write-Wins (LWW)

conflict detected (same parent, diff hash)
  → `conflicts` table → manual or auto resolve
```

## การสื่อสารระหว่าง Crates

```
CLI ──→ core (direct fn call, no IPC)
API ──→ core (direct fn call)
Sandbox ──→ external sandbox API (reqwest)

no inter-crate FFI boundary — all in-process
```

## Performance Targets (future)
- MCP latency < 50ms (p99)
- Binary size < 15MB (release)
- Startup < 200ms
- integration tests 100%
- no dead code detected

## Key Design Decisions
1. **SQLite bundled** (rusqlite) — zero external DB setup
2. **MySQL via sqlx** (feature flag) — for production deployments
3. **No Python runtime** — pyo3 removed, skill distiller stubbed
4. **JSON I/O** — all CLI commands emit JSON with `--json` flag
