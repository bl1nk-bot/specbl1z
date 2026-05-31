# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Unified CLI Structure**: Implemented a comprehensive command hierarchy including `template`, `db`, `rule`, `agent`, `index`, and `search`.
- **Markdown Serialization**: Added the ability to convert internal JSON template values back into the Markdown+XML format.
- **Multi-format Template Support**: Added a `convert` command to seamlessly translate templates between JSON, TOML, and Markdown.
- **SQLite Memory Engine**: Established a production-ready SQLite schema for the Memory and Policy engines (`core/schema.sql`).
- **Conductor Protocol Integration**: Established the `/conductor` directory for strategic track planning and project orchestration.
- **Documentation Hierarchy**: Formalized the use of `SPEC.md`, `PLAN.md`, `ARCHITECT.md`, and `TODO.md` as the authoritative project context.
- **Semantic Search Framework**: Added the `CodeSense` module for indexing and searching codebase logic using embeddings.

### Changed
- **Monorepo Consolidation**: Merged core logic, CLI, and database layers into a unified workspace for better dependency management.
- **Rust 2021 Update**: Migrated the workspace to resolver version "2" and updated core dependencies.
- **Refactored CLI**: Completely overhauled `cli/src/main.rs` to use a nested subcommand structure for better scalability.
- **Optimized .gitignore**: Refined tracking rules to exclude local persistent data (SQLite, semantic indices) while protecting source integrity.

### Fixed
- **Clippy Compliance**: Resolved all linting warnings including collapsible matches, redundant imports, and unnecessary type casts.
- **Test Stability**: Fixed flaky unit tests in the memory engine related to audit trail ordering and category naming mismatches.
- **TOML Handling**: Improved TOML serialization by implementing recursive null-filtering to ensure clean output.
- **Integration Tests**: Verified all 9 integration scenarios pass with the new monorepo structure.

### Security
- **Identity Protection**: Implemented read-only enforcement for the `identity` memory scope to prevent unauthorized modification of agent persona.
- **Credential Leak Prevention**: Strengthened `.gitignore` to explicitly block `.env` files and local database artifacts.

## [0.1.0] - 2024-05-18

### Added
- Initial project structure for `specgen`.
- Basic template rendering engine (Handlebars-inspired).
- Initial Markdown/XML parser for workflow definitions.
- Basic JSON schema validation for templates.

[Unreleased]: https://github.com/bl1nk-bot/specgen/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/bl1nk-bot/specgen/releases/tag/v0.1.0
