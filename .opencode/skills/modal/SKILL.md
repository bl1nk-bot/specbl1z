---
name: modal
description: >
  Modal.com knowledge for answering user questions about serverless GPU compute,
  cost comparison with Daytona, and Python-based sandbox workflows. Use when the
  user asks about GPU access, ML training, Modal pricing, serverless sandboxes,
  or wants to compare cloud compute options for their Rust/Python projects.
---

# Modal.com — Agent Knowledge Base

## What Modal Is

Modal is a serverless cloud platform optimized for compute-heavy workloads.
Unlike Daytona (full Docker containers), Modal runs functions in lightweight
containers with per-second billing. It excels at GPU workloads (T4, A100, H100)
and Python-based data processing.

The user's project (specgen) may use Modal for:
- GPU-accelerated Rust builds (rare but possible)
- ML model training/inference
- Large-scale parallel test execution
- Cost comparison with Daytona

## Tools Available to the User

### 1. Rust CLI (works on Termux)
```bash
export MODAL_TOKEN_ID=***
export MODAL_TOKEN_SECRET=*** Use sandbox crate
cargo run -p specgen-sandbox -- spin --provider modal \
  --repo https://github.com/bl1nk-bot/specbl1z.git \
  --branch main --name my-sandbox

cargo run -p specgen-sandbox -- exec --id <sandbox-id> \
  --cmd "cargo build --workspace --release"

cargo run -p specgen-sandbox -- delete --id <sandbox-id>
```

### 2. Modal Python SDK (for Python workloads)
```python
import modal
app = modal.App("specgen-build")
image = modal.Image.debian_slim().run_commands([
    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
    "git clone https://github.com/bl1nk-bot/specbl1z.git /workspace",
])

@app.function(image=image, gpu="T4")
def build():
    import subprocess
    subprocess.run(["cargo", "build", "--workspace", "--release"], cwd="/workspace")
```

## Modal Pricing (verified from https://modal.com/pricing, 2025-06)

$30/month free credits on Starter plan. You only pay for actual compute cycles used — idle containers cost nothing.

| GPU | Per Second | Per Hour (approx) |
|-----|-----------|-------------------|
| Nvidia B200 | $0.001736 | $6.25 |
| Nvidia H200 | $0.001261 | $4.54 |
| Nvidia H100 | $0.001097 | $3.95 |
| Nvidia RTX PRO 6000 | $0.000842 | $3.03 |
| Nvidia A100 80GB | $0.000694 | $2.50 |
| Nvidia A100 40GB | $0.000583 | $2.10 |
| Nvidia L40S | $0.000542 | $1.95 |
| Nvidia A10 | $0.000306 | $1.10 |
| Nvidia L4 | $0.000222 | $0.80 |
| Nvidia T4 | $0.000164 | $0.59 |

CPU: $0.125 per physical core per hour (1 physical core ≈ 2 vCPU)
Memory: $0.00000222 per GiB per second
Volumes: $0.09 per GiB per month (1 TiB free)

Plans: Starter $0 + compute (3 seats, 100 containers, $30 free credits), Enterprise for unlimited seats + SSO + HIPAA.

## Common User Questions

### "I need GPU for ML, what's cheapest?"
Modal.com with T4 GPU at $0.70/hr, per-second billing, $30 free credits.
For a small model that trains in 2 hours: $1.40 (or $0 with free credits).

Other options:
- Google Colab: free T4, but limited sessions
- RunPod: ~$0.29/hr for RTX 3070
- Lambda Labs: from $0.50/hr

### "Daytona vs Modal — which do I use?"

| Condition | Use Daytona | Use Modal |
|-----------|------------|-----------|
| CPU (1 vCPU, 1hr) | $0.0504 + RAM $0.0162/GiB | per-cycle, idle=free |
| GPU cheapest | H100 $3.95/hr | T4 $0.59/hr |
| GPU best | RTX PRO 6000 $3.03/hr | H100 $3.95/hr, B200 $6.25/hr |
| Free credits | $200 one-time | $30/month |
| Billing | per-second wall-clock | per CPU cycle (idle = $0) |
| Android target | Yes ($0.0504/vCPU/hr) | No |
| Full Docker | Yes | Lightweight container |
| Best for | CI/CD, cross-platform | ML training, burst compute |

**Recommendation:**
- Building specgen on CPU → Daytona (faster spin, full container)
- Cheapest CPU builds → Modal (25x cheaper)
- GPU/ML training → Modal only (Daytona has no GPU)
- CI/CD → GitHub Actions (free, already configured in cross-platform.yml)

### "How much will my workload cost?"
Modal bills per CPU cycle, not wall-clock. Idle time costs $0.
Active CPU: $0.125 per physical core per hour.

Example: a 10-hour continuous CPU build using 1 core = ~$1.25
Example: 1 hour T4 GPU training = $0.59

Daytona bills per wall-clock second.
Example: 10 hours CPU (1 vCPU + 1GiB RAM) = $0.504 + $0.162 = $0.67

Bottom line: Daytona is predictable per-hour. Modal can be cheaper if your workload has idle periods.

### "How do I set up Modal credentials?"
1. Go to https://modal.com/settings
2. Create token → get MODAL_TOKEN_ID and MODAL_TOKEN_SECRET
3. Export: `export MODAL_TOKEN_ID=***export MODAL_TOKEN_SECRET=***4. Test: `cargo run -p specgen-sandbox -- spin --provider modal --repo <url>`

## Error Troubleshooting

| Error | Cause | Fix |
|-------|-------|-----|
| `MODAL_TOKEN_ID not set` | No credentials | Create token at https://modal.com/settings |
| `sandbox creation failed` | Invalid image or quota | Check image name; verify quota on dashboard |
| `exec exit code != 0` | Command failed | Read output; may need `apt install` in sandbox |
| `out of memory` | GPU OOM | Reduce batch size or use larger GPU (A100) |
| `timeout after 1hr` | Modal default limit | Use `@app.function(timeout=3600*3)` for 3hr |

## SDK Languages Available
- Python: `pip install modal` (primary)
- TypeScript: via HTTP API
- Rust: via `specgen-sandbox` crate

## API Reference
- Base URL: `https://api.modal.com/v1`
- Auth: `Basic base64(token_id:token_secret)`
- Docs: https://modal.com/docs

## References
- Modal docs: https://modal.com/docs
- Modal Python SDK: https://modal.com/docs/reference
- Pricing: https://modal.com/pricing
