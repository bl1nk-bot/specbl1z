---
name: modal
description: Spin Modal.com sandboxes, run agents, and sync back to GitHub
---

# Modal.com Skill

Use when you need a remote sandbox on Modal.com with antigravity + opencode + Rust + Python.

## When to use
- You need GPU compute (Modal has GPUs)
- You want cheaper CPU sandboxes (~$0.002/hr)
- You want to compare Daytona vs Modal pricing
- You need long-running compute tasks

## Workflow

1. Spin: `modal:spin repoUrl=https://github.com/bl1nk-bot/specbl1z.git`
2. Setup: `modal:exec sandboxId=<id> command="bash scripts/setup.sh"`
3. Build: `modal:exec sandboxId=<id> command="cargo build --workspace --release"`
4. Sync: `modal:sync sandboxId=<id> branch=cleanup/reorg`
5. Delete: `modal:delete sandboxId=<id>`

## Variables
- MODAL_TOKEN_ID — required
- MODAL_TOKEN_SECRET — required

## Cost
- CPU: ~$0.002/hour
- GPU: ~$0.70/hour
