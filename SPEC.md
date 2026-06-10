# ข้อกำหนดเฉพาะของระบบ (System Specification) — specgen v3.1

## 1. วิสัยทัศน์ (Vision)
สร้างแพลตฟอร์ม **AI-Native Spec-Driven Development** ที่รวดเร็ว โปร่งใส และทนทาน โดยใช้สถาปัตยกรรม Rust-centric ที่รวมประสิทธิภาพของ Core Engine เข้ากับ MCP Server โดยตรง พร้อมระบบจัดการเวอร์ชันข้อมูลแบบ Git เพื่อการทำงานข้ามแพลตฟอร์มอย่างไร้รอยต่อ

## 2. วัตถุประสงค์ของระบบ (v3.1)

### 2.1 Unified Architecture (Monorepo)
- `core` — engine หลัก (parser, renderer, db, memory, validator, sync, task_delegator)
- `cli` — binary คำสั่งทั้งหมด (validate, generate, db, agent, index, search, sync, guardrail)
- `api` — REST API (axum + tokio)
- `sandbox` — OpenCode SDK ใน Rust (Daytona + Modal sandbox management)
- `.opencode/` — OpenCode plugins TypeScript (สำหรับ Bun/macOS/Linux)

### 2.2 Open Bridge Architecture
- CLI ทำหน้าที่เป็นสะพานระหว่าง Core Engine และสคริปต์ภายนอก
- รองรับ **Standardized JSON I/O** ทุกคำสั่ง ใช้ร่วมกับ `jq` หรือ Python
- Sandbox crate เป็น SDK สำหรับ AI agents เรียกใช้ sandbox ผ่าน REST API

### 2.3 Cross-Platform (Zero-Config Reproducibility)
- `scripts/setup.sh` — bootstrap จากศูนย์ (Termux, Debian, Alpine, Arch, macOS)
- `scripts/wizard.sh` — interactive setup ทุก platform
- `scripts/integration-test.sh` — real-world pipeline test
- Docker images: Debian, Alpine, Android cross-compile, bare sandbox, MSRV matrix
- `Dockerfile` + `docker/docker-compose.yml` — build ทุก platform พร้อมกัน

### 2.4 Dependency Bootstrapper
- ติดตั้งเครื่องมือจำเป็น (`jq`, `rg`, `rust`, `git`, `python3`) อัตโนมัติ
- รองรับ package manager: pkg (Termux), apt (Debian/Ubuntu), apk (Alpine), pacman (Arch), brew (macOS)

### 2.5 DevOps Pipeline
- `.github/workflows/rust-ci.yml` — check, test, clippy, fmt, lockfile
- `.github/workflows/cross-platform.yml` — Linux, macOS, Android, MSRV matrix, integration test, Docker
- `.github/workflows/release.yml` — multi-target binary release (linux-amd64, android-arm64)
- `.github/dependabot.yml` — auto-update cargo deps ทุกสัปดาห์
- `.github/prompts/` — agent instructions (anti-slop, auto-doc, dead-code-hunter, security-audit)
- Git hooks: pre-commit (devops.sh), commit-msg (marker enforcement)
- `Makefile` — make all, check, test, lint, fmt, build, release, setup, wizard, bench, docker-build

## 3. ข้อกำหนดเชิงฟังก์ชัน (Functional Requirements)

### FR1: Unified Data Model
- SQLite (`data/craft.db`) — documents, collections, memory_entries, agent_executions, chunks
- 4 migrations (v1 initial → v3 semantic + agentic unification → v4 memory topic)
- `Database` trait พร้อมรองรับ MySQL ผ่าน sqlx (feature flag)

### FR2: Memory Engine
- CRUD พร้อม Versioning + Confidence scoring
- MemoryStore: scope (global/project/session), category (fact/rule/preference), topic
- Audit log ติดตามทุกการเปลี่ยนแปลง

### FR3: Transparent CLI
- ทุกความสามารถเรียกใช้ผ่าน CLI
- `--json` flag สำหรับ machine-readable output (Open Bridge)
- Commands: validate, generate, convert, db, agent, index, search, sync, task, memory, guardrail, skill, status

### FR4: Sandbox SDK (OpenCode Integration)
- Daytona: spin, exec, sync, delete sandbox
- Modal.com: spin, exec, sync, delete sandbox
- Webhook: spin via webhook, price comparison
- Rust crate (`sandbox/`) + TypeScript plugins (`.opencode/`)

### FR5: REST API
- Axum server: `GET /health`, `GET /api/memory`, `POST /api/memory`
- In-memory DB support for testing

## 4. ข้อกำหนดที่ไม่ใช่ฟังก์ชัน (Non-Functional Requirements)

- **Speed**: CLI startup < 200ms, API response < 50ms
- **Portability**: Termux/Android, Linux (x86_64, musl), macOS, Docker
- **Reliability**: 57 tests (7 cli + 4 api + 36 core + 9 integration + 1 sandbox), 0 failures
- **Code Quality**: cargo clippy -D warnings, cargo fmt, zero warnings
- **Reproducibility**: setup.sh bootstraps from empty sandbox

## 5. เทคโนโลยีหลัก (Tech Stack v3.1)

- **Core/CLI/API/Sandbox**: Rust (workspace 4 crates)
- **Storage**: SQLite (rusqlite bundled), MySQL (sqlx via feature flag)
- **Web**: Axum + Tokio
- **CLI**: Clap derive
- **Sandbox SDK**: reqwest + serde + base64
- **OpenCode Plugins**: TypeScript (.opencode/)
- **CI/CD**: GitHub Actions matrix (5 OS + MSRV)
- **Container**: Docker (4 platform images)
- **External Tools**: jq, rg, python3 (auto-bootstrapped)

## 6. โครงสร้างโปรเจค

```
specgen/
├── core/           # engine (16 .rs files)
├── cli/            # binary (1260 lines + 7 tests)
├── api/            # REST server (4 tests)
├── sandbox/        # OpenCode SDK Rust (1 test + 4 benchmarks)
├── .opencode/      # OpenCode TypeScript plugins
├── data/           # SQLite database
├── schema/         # JSON schemas
├── docker/         # Multi-platform Dockerfiles
│   └── platforms/  # debian, alpine, android, bare, msrv
├── scripts/        # setup.sh, wizard.sh, devops.sh, integration-test.sh
├── .github/        # CI/CD workflows + prompts + dependabot
├── Cargo.toml      # workspace root
├── Makefile        # 16 targets
├── Dockerfile
└── docs/           # ARCHITECTURE.md, CHANGELOG.md
```
