# Research & Design: SessionDB (FTS5) and Agentic Loop

## 1. Objective
Implement a high-performance session memory system using SQLite FTS5 and a robust Agentic State Machine (Loop) to enable seamless context retrieval and task execution.

## 2. SessionDB Architecture (SQLite + FTS5)
To handle concurrent reads/writes and enable fast text searching, the system adopts the following patterns:

### High Concurrency with WAL Mode
- **WAL (Write-Ahead Logging)**: Enables multiple readers to operate while a writer is active, significantly reducing "database is locked" errors during agentic execution.
- **Busy Timeout**: Set to 1000ms to allow retry logic for write contention.

### Full-Text Search (FTS5)
- **Virtual Table**: `messages_fts` mirrors the `messages` table content.
- **Triggers**: Automated synchronization via `AFTER INSERT` and `AFTER DELETE` triggers ensures the index is always up-to-date without manual overhead.
- **Querying**: Uses the `MATCH` operator with BM25 ranking to return the most relevant context snippets.

## 3. Agentic Loop (State Machine)
The loop acts as the "brain" of the agent, transitioning between states based on internal logic and external database state.

### Planned States
1. **IDLE**: Waiting for task or input.
2. **PLANNING**: Retrieving context from `SessionDB` and generating execution steps.
3. **EXECUTING**: Running commands (via `TaskDelegator`).
4. **VALIDATING**: Checking results against expectations.
5. **FINALIZING**: Updating memory and reporting status.

## 4. Integration Path (Flat Monorepo)
- **Database Core**: `core/src/db.rs` will be upgraded to support WAL/Timeout globally.
- **Memory Module**: `core/src/memory.rs` will incorporate the `SessionDB` logic.
- **State Machine**: `core/src/loop.rs` (New) will implement the `AgenticLoop` struct.

## 5. References
- [1] SQLite WAL Mode Documentation
- [2] SQLite FTS5 Extension Guide
- [3] Agentic Workflows: State Machine Patterns
