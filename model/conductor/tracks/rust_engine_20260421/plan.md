# Implementation Plan: Rust Core Engine Integration

## Phase 0 - Rust Core Engine (PyO3)
- [ ] Task: Initialize Rust Crate `sovereign_engine`
    - [ ] Create `engine` directory and run `cargo new sovereign_engine --lib`.
    - [ ] Update `Cargo.toml` with `pyo3`, `maturin` dependencies and `crate-type = ["cdylib"]`.
- [ ] Task: Implement Rust Core Logic
    - [ ] Create `engine/src/label_parser.rs` for `normalize_labels(labels)`.
    - [ ] Create `engine/src/state_resolver.rs` for `resolve_state()`.
    - [ ] Create `engine/src/constraint_engine.rs` for `detect_blockers()`.
    - [ ] Create `engine/src/timeline.rs` for `build_timeline_event()`.
    - [ ] Expose these functions via PyO3 in `engine/src/lib.rs`.
- [ ] Task: Write Tests for Rust Core Engine
    - [ ] Write unit tests for label parsing and state resolution in Rust.
- [ ] Task: Conductor - User Manual Verification 'Phase 0 - Rust Core Engine (PyO3)' (Protocol in workflow.md)

## Phase 1 - Modal Webhook Receiver
- [ ] Task: Create FastAPI Webhook Endpoint
    - [ ] Implement `POST /api/github/webhook` endpoint.
    - [ ] Create `GET /install/callback` endpoint.
- [ ] Task: Configure Modal Secrets
    - [ ] Set up handling for `GITHUB_WEBHOOK_SECRET`, `GITHUB_APP_ID`, `GITHUB_PRIVATE_KEY`.
- [ ] Task: Write Tests for Webhook Receiver
    - [ ] Write unit tests for FastAPI endpoints.
- [ ] Task: Conductor - User Manual Verification 'Phase 1 - Modal Webhook Receiver' (Protocol in workflow.md)

## Phase 2 - Storage Layer (SQLite)
- [ ] Task: Setup SQLite Database
    - [ ] Configure SQLite connection with Modal Volumes.
- [ ] Task: Define Database Schemas
    - [ ] Create schema for `installation` table.
    - [ ] Create schema for `timeline` table.
    - [ ] Create schema for `current_state` table.
- [ ] Task: Write Tests for Storage Layer
    - [ ] Write unit tests for DB initialization and schema creation.
- [ ] Task: Conductor - User Manual Verification 'Phase 2 - Storage Layer (SQLite)' (Protocol in workflow.md)

## Phase 3 - Webhook Logic & Rust Processing
- [ ] Task: Implement Webhook Request Verification
    - [ ] Verify `X-Hub-Signature-256` signature.
- [ ] Task: Process Payload and Call Rust Engine
    - [ ] Extract event type and labels from payload.
    - [ ] Call `sovereign_engine.normalize_labels`, `resolve_state`, and `detect_blockers`.
- [ ] Task: Update SQLite Tracking State
    - [ ] Append event to `timeline` table.
    - [ ] Update/insert into `current_state` table.
- [ ] Task: Write Tests for Webhook Logic
    - [ ] Write unit tests for full webhook processing flow (mocking GitHub API and DB).
- [ ] Task: Conductor - User Manual Verification 'Phase 3 - Webhook Logic & Rust Processing' (Protocol in workflow.md)

## Phase 4 - Deployment & App Configuration
- [ ] Task: Deploy to Modal
    - [ ] Write Modal configuration (`app.py` or similar) to deploy the FastAPI app and compiled Rust wheel.
- [ ] Task: Create GitHub App Documentation
    - [ ] Document steps to create GitHub App, configure webhooks, and generate private keys.
- [ ] Task: Handle GitHub App Installation Logic
    - [ ] Implement token generation (`installation_access_token`) logic.
    - [ ] Fetch and store initial repository metadata on installation.
- [ ] Task: Write Tests for App Installation Logic
    - [ ] Write unit tests for token generation and initial metadata sync.
- [ ] Task: Conductor - User Manual Verification 'Phase 4 - Deployment & App Configuration' (Protocol in workflow.md)

## Phase 5 - Dashboard API and UI
- [ ] Task: Implement Dashboard API
    - [ ] Create `GET /state`, `GET /timeline`, `GET /repos`, `GET /issues` endpoints.
- [ ] Task: Build UI Layer
    - [ ] Implement basic frontend (HTML/JS or React) to fetch and display the Current State, Timeline, Blockers, and Agent Activity.
- [ ] Task: Write Tests for Dashboard
    - [ ] Write tests for Dashboard API endpoints.
- [ ] Task: Conductor - User Manual Verification 'Phase 5 - Dashboard API and UI' (Protocol in workflow.md)