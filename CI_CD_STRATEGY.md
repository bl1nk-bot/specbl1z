# Specgen CI/CD & Automation Strategy

## 1. Vision: Hybrid "Sync-and-Execute" Pipeline
Specgen operates in a unique environment (Android/Termux) which has high latency and low compute power compared to cloud instances. Our CI/CD strategy leverages a **Local-to-Cloud Bridge**.

### Local (Termux)
- **Role**: Source of Truth & Local Coordinator.
- **Tools**: CLI, Local Indexing (CodeSense), Task Queue (SQLite).
- **Automation**: Git hooks, `termux-notification`, Local Cron.

### Cloud (Modal/Kilocode)
- **Role**: High-Performance Executor.
- **Tools**: `opencode.py` (Kilocode Gateway), GPU-accelerated testing, Multi-agent Swarms.

---

## 2. Pipeline Stages

### Stage 1: Change Discovery (Local)
- Trigger: `git commit` or manual `specgen task add`.
- Action: Calculate semantic diffs using `CodeSense`.

### Stage 2: Artifact Packaging (Local)
- Action: Generate a "Work Packet" containing:
  - Git patch of current changes.
  - Project context (relevant memories).
  - Test requirements.

### Stage 3: Remote Execution (Modal)
- Action:
  1. Spin up Modal container.
  2. Apply Git patch.
  3. Run `cargo test` / `npm test`.
  4. Invoke specialized subagents (e.g., `performance-reviewer`).
  5. Generate a "Result Packet".

### Stage 4: Sync & Merge (Local)
- Action: 
  1. Receive "Result Packet".
  2. Apply changes back to Termux filesystem.
  3. Update `craft.db` task status.
  4. Notify user via OS notification.

---

## 3. Automation & Scheduling

### Task Worker (The "Heart")
- A background process (`specgen task worker`) that polls the `tasks` table.
- **Priority Logic**: High-confidence tasks run automatically; others wait for human approval.

### Local Cron (Scheduling)
- Integration with Android/Termux for scheduled maintenance:
  - **Daily 02:00**: Full codebase re-indexing (`specgen index`).
  - **Weekly**: Memory cleanup (deleting low-confidence or expired facts).
  - **On-Demand**: Scheduled subagent runs for dependency updates.

---

## 4. Quality Gates
1. **Linter Gate**: No PR/Task is considered "Done" if `cargo clippy` or `biome` fails.
2. **Standardization Gate**:
   - **EditorConfig**: Files must follow `.editorconfig` rules (LF endings, no trailing spaces).
   - **EOL (End of Line)**: Enforce `LF` via `.gitattributes`.
   - **Spell Check**: Pass `cspell` validation (defined in `.cspell.json`).
3. **Confidence Gate**: AI-generated changes with < 0.8 confidence score require human review.
4. **Semantic Integrity Gate**: Semantic search should confirm no broken logic patterns after merge.
