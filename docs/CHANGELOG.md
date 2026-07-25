# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- **Dependency Cleanup**: Removed `pyo3` dependency from workspace (root + core Cargo.toml), replacing Python-based skill distiller with a no-op stub — eliminates heavyweight CPython link-time dep for Android/Termux cross-compilation.
- **Compile Fix**: Restored missing `anyhow::Result` import in `core/src/distiller.rs` after removing PyO3 import block stripped the anyhow re-export.

## [3.1.0] - 2026-06-10

### Added
- **Monorepo Structure**: Fully consolidated core, cli, api, and sandbox into a flat workspace structure.
- **API Server**: Added a new REST API crate using Axum and Tokio.
- **Sandbox SDK**: Implemented a Rust SDK for Daytona and Modal sandbox management.
- **DevOps Infrastructure**: Added Makefile, setup scripts, and GitHub Actions for cross-platform builds (Linux, macOS, Android).
- **Docker Support**: Provided multi-platform Docker images for development and production.
- **Unified CLI Structure**: Implemented a comprehensive command hierarchy including `template`, `db`, `rule`, `agent`, `index`, and `search`.
- **Markdown Serialization**: Added the ability to convert internal JSON template values back into the Markdown+XML format.
- **Multi-format Template Support**: Added a `convert` command to seamlessly translate templates between JSON, TOML, and Markdown.
- **SQLite Memory Engine**: Established a production-ready SQLite schema for the Memory and Policy engines (`core/schema.sql`).
- **Conductor Protocol Integration**: Established the `/conductor` directory for strategic track planning and project orchestration.
- **Documentation Hierarchy**: Formalized the use of `SPEC.md`, `PLAN.md`, `ARCHITECT.md`, and `TODO.md` as the authoritative project context.

### Changed
- **Structure Flattening**: backend/core -> core, backend/api -> api, app/cli -> cli.
- **Dependency Optimization**: Removed unused dependencies like `sqlite-vec`.
- **Rust 2021 Update**: Migrated the workspace to resolver version "2" and updated core dependencies.

### Fixed
- **Database Methods**: Added missing methods to `core::Database` to ensure full CLI functionality.
- **Clippy Compliance**: Resolved all linting warnings across the workspace.
- **Test Stability**: Verified all 57 tests pass in the new monorepo structure.

### Security
- **Identity Protection**: Implemented read-only enforcement for the `identity` memory scope.
- **Credential Leak Prevention**: Strengthened `.gitignore` to protect sensitive local data.

## [0.1.0] - 2024-05-18

### Added
- Initial project structure for `specgen`.
- Basic template rendering engine (Handlebars-inspired).
- Initial Markdown/XML parser for workflow definitions.
- Basic JSON schema validation for templates.

[Unreleased]: https://github.com/bl1nk-bot/specgen/compare/v3.1.0...HEAD
[3.1.0]: https://github.com/bl1nk-bot/specgen/compare/v0.1.0...v3.1.0
[0.1.0]: https://github.com/bl1nk-bot/specgen/releases/tag/v0.1.0
