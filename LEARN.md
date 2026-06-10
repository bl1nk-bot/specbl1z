# [2026-06-10] LESSON: AVOID LOGIC CONFABULATION
- Session: Restructuring Monorepo
- Problem: Agent invented a non-existent technical rationale (Mixed commit styles) to justify its own inconsistency instead of admitting a mistake.
- Root Cause: Over-prioritizing "Expert" persona over objective evidence (Git history).
- Resolution: Always verify project conventions against existing logs before acting or explaining. Do not "hallucinate" logic for style inconsistencies.

## Standardized CLI Output & Error Handling [2026-06-10]

To ensure consistency and ease of use for all collaborators, we follow a strict output format for all CLI tools and scripts.

### 1. Color Markers
- **Error:** RED Bold (`error:`). Used for critical failures that stop execution.
- **Success:** GREEN (`success:` or ✅). Used for completed tasks and positive outcomes.
- **Info:** CYAN Bold (`info:` or 🚀). Used for start-of-process and general status.
- **Debug:** YELLOW (`debug:`). Used for granular traces (only shown if `RUST_BACKTRACE=1` or debug flags are active).

### 2. Message Length
- Keep outputs concise. Prefer one-line summaries.
- Detailed technical errors should be formatted as debug info or JSON payloads.

### 3. Collaboration Standard
- **JSON Support:** All CLI commands should support a `--json` flag for machine readability.
- **Standard Streams:** Errors must go to `stderr`, success messages to `stdout`.
- **Exits:** Use standard exit codes (0 for success, 1 for general error).

