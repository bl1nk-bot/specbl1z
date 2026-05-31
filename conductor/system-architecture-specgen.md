# System Architecture: specgen v3

## 1. Architecture Overview
**specgen** is an AI-native, spec-driven development platform designed for high-performance, local-first execution. It employs a **Unified Rust-Centric Monorepo** architecture that consolidates a core engine, a transparent CLI, and an embedded MCP (Model Context Protocol) server. The system prioritizes data integrity through a **Git-like Versioned SQLite** storage model and enables cross-platform portability (Android/Termux, Windows, Linux) via a distributed sync layer.

## 2. High-Level Architecture Diagram
```
                     ┌──────────────────────────────────────────┐
                     │          AI Assistant / IDE              │
                     │      (Claude Code, Cursor, etc.)         │
                     └──────────────────┬───────────────────────┘
                                        │ (MCP Protocol)
                     ┌──────────────────▼───────────────────────┐
                     │        specgen MCP Server (Rust)         │
                     └──────────────────┬───────────────────────┘
                                        │ (Direct FFI / Lib)
            ┌───────────────────────────┴───────────────────────────┐
            │               specgen Core Engine (Rust)              │
            ├──────────────┬──────────────┬──────────────┬──────────┤
            │    Parser    │   Renderer   │ Rules Engine │  Sense   │
            │ (MD/TOML/JS) │ (Handlebars) │  (Policy)    │ (Search) │
            └──────┬───────┴──────┬───────┴──────┬───────┴─────┬────┘
                   │              │              │             │
            ┌──────▼──────────────▼──────────────▼─────────────▼────┐
            │             Unified Storage Layer (SQLite)            │
            │        (Append-Only, SHA-256 Versioned Rows)          │
            └──────────────────────────┬────────────────────────────┘
                                       │ (Sync Protocol)
            ┌──────────────────────────▼────────────────────────────┐
            │             Distributed Sync & Cloud Bridge           │
            │           (Cross-OS Persistence / Git-based)          │
            └───────────────────────────────────────────────────────┘
```

## 3. Component Specifications

### 3.1 Unified Core Engine (Rust)
- **Parser/Validator**: Multi-format support (JSON, TOML, Markdown+XML) for templates. Uses `bl1nk.proto` as the single source of truth for schemas.
- **Rules Engine (Policy)**: Evaluates guardrails and coding standards. Categorizes rules into `hard`, `soft`, `context`, and `safety`.
- **Memory Engine**: Manages scoped context (global, project, session). Implements confidence-based retrieval and TTL.
- **Sense (Semantic Search)**: Integrated with Ollama for local embeddings, enabling semantic lookup of code logic and memory entries without cloud dependency.

### 3.2 Transparent CLI
- **Non-Black-Box Philosophy**: Every core function is exposed via CLI with standard JSON I/O.
- **Scripting Bridge**: Injectable context for external Python/Bash scripts, allowing complex workflow extension.

### 3.3 Data Integrity & Storage (Git-like SQLite)
- **Immutable History**: Append-only rows with SHA-256 hashing.
- **Conflict Resolution**: Parent-tracking for every record to handle distributed merges across OS environments.
- **Storage Portability**: Optimized for Android/Termux shared storage vs. internal storage constraints.

## 4. Data Flow

### 4.1 Template Generation Flow
1. **Request**: AI Assistant calls `template_generate` via MCP.
2. **Context Enrichment**: Core Engine retrieves relevant memory and guardrails.
3. **Rendering**: Handlebars-based renderer produces final spec/code.
4. **Validation**: Output is checked against `bl1nk` schemas and active guardrails.
5. **Persistence**: Result is hashed and written to `craft.db`.

### 4.2 Distributed Sync Flow
1. **Local Change**: Change committed to local SQLite with a new hash.
2. **Sync Trigger**: `specgen sync` checks remote state.
3. **Conflict Check**: SHA-256 parents compared (3-way merge).
4. **Resolution**: "Last-writer-wins" or "Manual Merge" recorded as a resolution entry.

## 5. Scaling & Portability Strategy
- **Horizontal**: `specgen` scales by distributing execution across multiple agent instances.
- **Local-to-Cloud**: Local SQLite acting as a cache for a central Neon/PostgreSQL cloud database.
- **Termux Optimization**: Aggressive target directory management (Internal storage for binaries) to bypass Android permission latency on shared storage.

## 6. Fault Tolerance & Reliability
- **Redundancy**: Append-only storage ensures no data is ever overwritten; history is always recoverable.
- **Fault Tolerance**: Direct FFI calls between MCP and Core prevent process-spawn overhead and potential pipe failures.

## 7. Security Architecture
- **PII Redaction**: Automatic scrubbing of sensitive data in prompt blocks.
- **Scoped Access**: Memory entries partitioned by `access_level` (public, private, secret).
- **Execution Guardrails**: Rule-based rejection of unsafe or forbidden code patterns.

## 8. Technology Stack
- **Languages**: Rust (Core, CLI, MCP), TypeScript/Hono (Dashboard).
- **Database**: SQLite (Local), PostgreSQL (Cloud Bridge).
- **Protocols**: Protobuf (`bl1nk.proto`), MCP (Model Context Protocol).
- **AI**: Ollama (Local Embeddings), Claude/Gemini (Execution).

## 9. Capacity Planning (Local-First Context)
- **Latency**: < 50ms p99 for MCP tool calls.
- **Storage**: ~70GB projection over 5 years (Append-only metadata + compressed artifacts).
- **Concurrency**: Optimized for high-frequency tool calls from multiple AI agents.

## 10. Trade-offs & Rationale
- **SQLite over NoSQL**: Chosen for local portability and ACID compliance required for Git-like hashing.
- **Rust over Python for Core**: Essential for sub-50ms latency in MCP tools and memory safety in a distributed environment.
- **Direct FFI over REST**: Minimizes overhead in the critical AI-to-Engine path.

## 11. Deployment Strategy
- **Binary Distribution**: Single compiled binary for all platforms.
- **Automated Bootstrapping**: `specgen setup` handles missing dependencies (`jq`, `rg`, `ollama`) automatically.
