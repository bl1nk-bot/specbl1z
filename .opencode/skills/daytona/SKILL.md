---
name: daytona
description: Spin Daytona sandboxes, run agents, and sync back to GitHub
---

# Daytona Skill

Use when you need a remote sandbox with antigravity + opencode + Rust + Python.

## When to use
- You need a compute sandbox
- You want to run agents remotely
- You want automatic GitHub sync
- Memory says Daytona sandbox cant see local Termux paths — use git URL

## Workflow

1. Spin: `daytona:spin repoUrl=https://github.com/bl1nk-bot/specbl1z.git`
2. Setup: `daytona:exec sandboxId=<id> command="bash scripts/setup.sh"`
3. Run tests: `daytona:exec sandboxId=<id> command="make all"`
4. Sync: `daytona:sync sandboxId=<id> branch=cleanup/reorg`
5. Delete: `daytona:delete sandboxId=<id>`

## Variables
- DAYTONA_API_KEY — required
- DAYTONA_API_URL — default https://api.daytona.io

## Cost
~$0.05/hour
