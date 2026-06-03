# Proof Requirements

## Current state
- ABI directory exists (template-level)
- No dangerous patterns found
- 5K lines; file system linter with plugin architecture

## What needs proving
- **Plugin isolation**: Prove that FSLint plugins cannot access files outside their declared scope
- **Linting rule determinism**: Prove that lint results are deterministic for the same input (no order-dependent results)
- **No data loss**: Prove that linting operations (including any rename/move suggestions) never delete or corrupt files when applied

## Recommended prover
- **Idris2** — Plugin capability model fits dependent types naturally

## Priority
- **LOW** — FSLint is a linting tool, not a file mutator in its primary mode. Plugin isolation becomes important if plugins gain write access.

## Template ABI Cleanup (2026-03-29)

Template ABI removed -- was creating false impression of formal verification.
The removed files (Types.idr, Layout.idr, Foreign.idr) contained only RSR template
scaffolding with unresolved {{PROJECT}}/{{AUTHOR}} placeholders and no domain-specific proofs.
