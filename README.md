# Specgen — AI-Native Spec-Driven Development (v3.1)

สถานะ: Phase 6 เสร็จ ✓ | 57 tests | 4 benchmarks | 6 platforms

## โครงสร้าง

```
specgen/
├── core/           # engine หลัก (16 .rs files)
├── cli/            # CLI binary (7 tests)
├── api/            # REST API server (4 tests)
├── sandbox/        # OpenCode SDK Rust (1 test + 4 benchmarks)
├── .opencode/      # OpenCode plugins (TypeScript)
├── data/           # SQLite database
├── docker/         # Multi-platform images (5 variants)
│   └── platforms/  # debian, alpine, android, bare, msrv
├── scripts/        # setup.sh, wizard.sh, devops.sh, integration-test.sh
├── .github/        # CI/CD + prompts + dependabot
└── docs/           # ARCHITECTURE.md, CHANGELOG.md
```

## Quick Start

```bash
# Bootstrap from zero (ทุก platform)
bash scripts/setup.sh

# หรือ interactive wizard
bash scripts/wizard.sh

# หรือ make
make all       # check + test + lint + fmt
make build     # debug build
make release   # release build
make bench     # benchmarks
```

## Commands

| Command | Purpose |
|---|---|
| `specgen validate <file>` | Validate template |
| `specgen generate <id> --out out.md` | Render template |
| `specgen convert <file> --to json\|md\|toml` | Convert format |
| `specgen db init\|status` | Database manage |
| `specgen memory list --json` | Memory query |
| `specgen status` | Project status |
| `specgen guardrail` | Policy enforcement |
| `specgen task add\|list` | Task management |

## Sandbox SDK (OpenCode)

```rust
// Rust (Termux/ทุก platform)
use specgen_sandbox::{Daytona, spin, exec};

let d = Daytona::from_env()?;
let r = spin(&d, "https://github.com/bl1nk-bot/specbl1z.git", "main", "test")?;
exec(&d, &r.sandbox_id.unwrap(), "make all")?;
```

```bash
# OpenCode (Bun/macOS/Linux)
opencode daytona:spin repoUrl=https://github.com/bl1nk-bot/specbl1z.git
opencode daytona:exec sandboxId=<id> command="make all"
```

## CI/CD

- **rust-ci.yml** — check, test, clippy, fmt, lockfile
- **cross-platform.yml** — Linux, macOS, Android, MSRV (1.75/1.80/stable), Docker, integration
- **release.yml** — linux-amd64 + android-arm64 binaries
- **dependabot.yml** — cargo auto-updates ทุกสัปดาห์
- **prompts/** — agent instructions (anti-slop, auto-doc, dead-code-hunter, security-audit)

## Docker

```bash
make docker-build     # single platform (debian)
make docker-all       # all 7 platforms via docker-compose
```

Images: debian, alpine (musl), android (cross-compile), bare (setup test), msrv (1.70/1.75/1.80)

## Build & Test

```bash
cargo check --workspace                      # PASS
cargo test --workspace                       # PASS (57 tests)
cargo clippy --workspace -- -D warnings      # PASS
cargo fmt --all -- --check                   # PASS
cargo bench -p specgen-sandbox               # PASS (4 benchmarks)
```

## Test Coverage

| Crate | Unit | Integration | Benchmarks |
|-------|------|-------------|------------|
| core | 36 | 9 | - |
| cli | 7 | - | - |
| api | 4 | - | - |
| sandbox | 1 | - | 4 |
| **Total** | **48** | **9** | **4** |

## Docs

| File | Detail |
|---|---|
| [SPEC.md](./SPEC.md) | v3.1 specification |
| [PLAN.md](./PLAN.md) | Development roadmap |
| [TODO.md](./TODO.md) | Task list |

**License:** MIT
**Updated:** 2025-06-10
