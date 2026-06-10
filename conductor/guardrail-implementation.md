# Plan: Implement Guardrail Rules Management

## Objective
Add a `guardrail` command to the `specgen` CLI to manage project-level execution guardrails for the "oh-my-product" ecosystem. These rules will enforce standards like Thai language usage, mandatory code markers, and strict memory classification.

## Key Files & Context
- `cli/src/main.rs`: Entry point for the CLI.
- `core/src/lib.rs`: Module registration.
- `core/src/guardrail.rs`: (New file) Implementation of rules and packs.
- `core/src/memory.rs`: Validation for new memory topics.

## Implementation Steps

### Phase 1: Core Logic & Guardrail Definition
1. Create `core/src/guardrail.rs` defining `GuardrailRules` with:
    - `response_language`: Default "th".
    - `doc_language`: Default "th".
    - `required_markers`: `["[FIX]", "[TODO]", "[REF]", "[NOTE]"]`.
    - `memory_topics`: `["LEARN", "WORK", "TOOL", "INTEREST", "PROJECT", "IDENTIFY"]`.
2. Implement built-in packs:
    - `default`: Thai language active, markers required, standard coverage.
    - `strict`: Thai language mandatory, markers strictly enforced, 100% coverage, strict memory validation.
    - `fast`, `security`, `migration`: Special-purpose configurations.

### Phase 2: Memory Engine Update
1. Update `core/src/memory.rs` and `models.rs` to support the new `topic` field (or refine `category`).
2. Add validation logic: Any memory entry NOT matching `LEARN`, `WORK`, `TOOL`, `INTEREST`, `PROJECT`, or `IDENTIFY` must be rejected.

### Phase 3: CLI Implementation
1. Add `specgen guardrail` with subcommands:
    - `show`: Display active rules (showing "ภาษาไทย: เปิดใช้งาน" etc.).
    - `apply <pack>`: Set active pack in `.omp/state/rules.json`.
    - `list`: Show available packs.
    - `create <name>`: Scaffold custom rules.
    - `reset`: Revert to `default`.

### Phase 4: Marker & Language Enforcement (Logic Bridge)
1. Add logic to check if a response or comment block complies with the active guardrail's language and marker requirements.

## Verification & Testing
1. **Thai Enforcement**: Verify `guardrail show` displays Thai as the required language.
2. **Marker Check**: Test a simulated commit/comment block without markers; expect warning/rejection.
3. **Memory Validation**: Attempt to write memory with an invalid topic (e.g., "GENERAL"); expect error.
4. **Valid Memory**: Write memory with `LEARN`; expect success.

