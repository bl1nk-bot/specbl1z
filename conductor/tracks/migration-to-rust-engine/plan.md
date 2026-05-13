# Track Plan: Migration to Rust-TS Unified Architecture

## Phase 1: Setup Schema
- [x] Define Protobuf schema
- [x] Generate Rust code from Proto
- [x] Generate TS code from Proto

## Phase 2: Core Engine (Rust)
- [x] Implement DB Storage (SQLite)
- [x] Implement Rule Engine logic
- [x] Integrate Clippy & Test suite for stability

## Phase 3: Interface & CLI Integration
- [x] Design Unified CLI structure (`db`, `rule`, `agent`)
- [x] Implement Markdown Serialization
- [ ] Connect CLI commands to Rust Core MemoryStore
- [ ] Implement Semantic Search (`index`/`search`)

## Phase 4: Interface Bridge (TS/MCP)
- [x] Create MCP server using Generated Proto
- [ ] Connect UI to Rust Engine
- [ ] Migrate MCP logic entirely to Rust
