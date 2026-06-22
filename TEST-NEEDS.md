# Test & Benchmark Requirements

## CRG Grade: C — ACHIEVED 2026-04-04

## Current State
- Unit tests: 1 integration test file (file-soup/crates/fslint-cli/tests/integration_test.rs) — count unknown (cannot build)
- Integration tests: 1
- E2E tests: NONE
- Benchmarks: 1 file exists (unverified)
- panic-attack scan: NEVER RUN

## What's Missing
### Point-to-Point (P2P)
27 Rust source files across multiple crates with only 1 test file:

#### fslint-plugin-api/ (top-level):
- src/lib.rs — no tests
- fuzz/fuzz_targets/fuzz_main.rs — fuzzer exists (verify it runs)

#### fslint-plugin-sdk/ (top-level):
- src/lib.rs — no tests
- fuzz/fuzz_targets/fuzz_main.rs — fuzzer exists (verify it runs)

#### file-soup/ (main project):
**Plugins (8 plugins, 0 tests each):**
- plugins/grouping/src/lib.rs — no tests
- plugins/ocr-status/src/lib.rs — no tests
- plugins/secret-scanner/src/lib.rs — no tests
- plugins/version-detection/src/lib.rs — no tests
- plugins/ai-detection/src/lib.rs — no tests
- plugins/duplicate-finder/src/lib.rs — no tests
- plugins/file-age/src/lib.rs — no tests
- plugins/git-status/src/lib.rs — no tests

**Core crates:**
- crates/fslint-plugin-api/src/lib.rs — no tests
- crates/fslint-plugin-sdk/src/lib.rs — no tests
- crates/fslint-cli/src/commands.rs — no tests
- crates/fslint-cli/src/main.rs — no tests
- crates/fslint-cli/src/output.rs — no tests
- crates/fslint-cli/src/query.rs — no tests
- crates/fslint-cli/tests/integration_test.rs — 1 test file

### End-to-End (E2E)
- CLI: scan directory -> detect files -> apply plugins -> generate report
- Plugin lifecycle: load plugin -> configure -> execute -> report
- Each plugin: provide input -> process -> verify output
- Query: define query -> execute against scan results -> verify
- Duplicate finder accuracy on known duplicate sets
- Secret scanner on known secret patterns
- OCR status on known image files

### Aspect Tests
- [ ] Security (secret-scanner false negatives, path traversal, plugin sandboxing)
- [ ] Performance (large directory scanning, duplicate detection at scale)
- [ ] Concurrency (parallel plugin execution, concurrent file access)
- [ ] Error handling (missing files, permission errors, corrupt files)
- [ ] Accessibility (CLI output formatting)

### Build & Execution
- [ ] cargo build — not verified
- [ ] cargo test — not verified
- [ ] CLI --help works — not verified
- [ ] Plugin loading works — not verified
- [ ] Fuzz targets run — not verified
- [ ] Self-diagnostic — none

### Benchmarks Needed
- Directory scan throughput (files/second)
- Per-plugin execution time
- Duplicate detection performance vs file count
- Secret scanning throughput
- Memory usage on large directory trees

### Self-Tests
- [ ] panic-attack assail on own repo
- [ ] CLI self-test with known test data
- [ ] Each plugin smoke test

## Priority
- **HIGH** — 27 Rust source files (8 plugins + CLI + API + SDK) with only 1 integration test. The secret-scanner plugin having ZERO tests is especially concerning — false negatives in secret scanning are a security risk. Fuzz targets exist for the API/SDK but need verification. Each of the 8 plugins needs at least basic correctness tests.

## FAKE-FUZZ ALERT

- `tests/fuzz/placeholder.txt` is a scorecard placeholder inherited from rsr-template-repo — it does NOT provide real fuzz testing
- Replace with an actual fuzz harness (see rsr-template-repo/tests/fuzz/README.adoc) or remove the file
- Priority: P2 — creates false impression of fuzz coverage
