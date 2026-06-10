---
name: daytona
description: >
  Daytona sandbox knowledge for answering user questions about remote development
  environments. Use when the user asks about Daytona, sandboxes, remote execution,
  cross-platform builds, or needs to run code on cloud infrastructure instead of
  locally. Provides API details, pricing, troubleshooting, and CLI commands.
---

# Daytona — Agent Knowledge Base

## What Daytona Is

Daytona is an open-source platform for running AI-generated code in isolated sandboxes.
Each sandbox is a full container (Docker) with its own kernel, filesystem, network stack,
vCPU, RAM, and disk. Sandboxes spin up in under 90ms.

The user's project (specgen) uses Daytona for:
- Cross-platform builds (ARM64 Termux → x86_64 Linux)
- Running CI pipelines remotely
- Testing on clean environments
- Agent-driven development workflows

## Tools Available to the User

The specgen project has TWO ways to interact with Daytona:

### 1. Rust CLI (works on Termux, no Bun needed)
```bash
# Set API key
export DAYTONA_API_KEY="dtn_..."

# Using sandbox crate directly
cargo run -p specgen-sandbox -- spin --provider daytona \
  --repo https://github.com/bl1nk-bot/specbl1z.git \
  --branch main --name my-sandbox

cargo run -p specgen-sandbox -- exec --id <sandbox-id> \
  --cmd "make all"

cargo run -p specgen-sandbox -- sync --id <sandbox-id> \
  --branch cleanup/reorg

cargo run -p specgen-sandbox -- delete --id <sandbox-id>
```

### 2. OpenCode plugin (needs Bun, macOS/Linux only)
```
daytona:spin repoUrl=... branch=... name=...
daytona:exec sandboxId=... command=...
daytona:sync sandboxId=... branch=...
daytona:delete sandboxId=...
```

## Daytona API Reference (verified from https://app.daytona.io/api)

Base URL: `https://app.daytona.io/api`

| Operation | Method | Endpoint | Auth |
|-----------|--------|----------|------|
| Create sandbox | POST | `/sandbox` | Bearer token |
| Clone repo | POST | `/sandbox/{id}/clone` | Bearer token |
| Execute command | POST | `/sandbox/{id}/exec` | Bearer token |
| Delete sandbox | DELETE | `/sandbox/{id}` | Bearer token |
| List sandboxes | GET | `/sandbox` | Bearer token |

Auth header: `Authorization: Bearer *** Pricing (verified from https://www.daytona.io/pricing, 2025-06)

$200 free compute credits included.

| Resource | Per Hour | Per Second |
|----------|----------|------------|
| vCPU | $0.0504 | $0.00001400 |
| GPU H100 | $3.95 | $0.001097 |
| GPU RTX PRO 6000 | $3.03 | $0.000842 |
| Memory | $0.0162/GiB | $0.00000450/GiB |
| Storage | $0.000108/GiB (5GB free) | $0.00000003/GiB |
| Windows | $0.0858/vCPU | — |
| Android | $0.0504/vCPU | — |

Daytona now supports GPU (H100, RTX PRO 6000) and bills per-second.

## Common User Questions

### "How do I test my ARM64 build on x86_64?"
Daytona provisions x86_64 sandboxes by default. The user can:
1. `export DAYTONA_API_KEY="dtn_..."`
2. `cargo run -p specgen-sandbox -- spin --provider daytona --repo <url> --branch <branch>`
3. `cargo run -p specgen-sandbox -- exec --id <id> --cmd "make all"`
4. `cargo run -p specgen-sandbox -- sync --id <id>`
5. `cargo run -p specgen-sandbox -- delete --id <id>`

Alternative without API key: Use GitHub Actions (`cross-platform.yml` already tests x86_64).

### "I left a sandbox running, what now?"
1. List running sandboxes: `curl -H "Authorization: Bearer $DAYTONA_API_KEY" https://app.daytona.io/api/sandbox`
2. Sync unsaved work: `cargo run -p specgen-sandbox -- sync --id <id>`
3. Delete: `cargo run -p specgen-sandbox -- delete --id <id>`
Cost for 3 hours idle: ~$0.15

### "Daytona vs Modal vs Docker?"
| | Daytona | Modal | Docker (local) |
|---|---|---|---|
| Cost/hr (CPU) | $0.0504/vCPU + $0.0162/GiB RAM | per physical core $0.125/hr | free |
| GPU | H100 $3.95/hr, RTX PRO $3.03/hr | T4 $0.59/hr, A100 $2.10/hr, H100 $3.95/hr | depends |
| Free credits | $200 | $30/month | — |
| Setup | API key | token+secret | Docker install |
| Best for | full containers, Android target | serverless, per-cycle billing | local dev |
| Billing | per-second | per CPU cycle (not wall-clock) | — |

Recommendation flow:
- Quick local test → Docker (`make docker-build`)
- CPU build, cross-platform → Daytona
- GPU/ML, cheapest compute → Modal
- CI/CD → GitHub Actions (free for public repos)

## Error Troubleshooting

| Error | Cause | Fix |
|-------|-------|-----|
| `DAYTONA_API_KEY not set` | No API key in env | Get key from https://app.daytona.io/dashboard/keys |
| `sandbox creation failed` | Network/quota issue | Wait 30s, retry; check quota on dashboard |
| `exec exit code != 0` | Command failed in sandbox | Read output, fix command, retry |
| `sync conflict` | Branch diverged from remote | Run `git pull --rebase` inside sandbox first |
| `connection refused` | API endpoint wrong | Verify DAYTONA_API_URL (default: https://app.daytona.io/api) |

## SDK Languages Available
- TypeScript: `npm install @daytonaio/sdk`
- Python: `pip install daytona`
- Go: `go get github.com/daytonaio/daytona/libs/sdk-go`
- Ruby: `gem install daytona`
- Java: `implementation("io.daytona:sdk-java:0.1.0")`

## References
- Docs: https://www.daytona.io/docs
- API Reference: https://www.daytona.io/docs/tools/api/
- TypeScript SDK: https://www.daytona.io/docs/typescript-sdk
- OpenCode Guide: https://www.daytona.io/docs/guides/opencode/
