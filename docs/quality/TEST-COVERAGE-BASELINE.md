# GridCraft — Test Coverage Baseline

Measured on: **not yet measured** (initial audit: 2026-10-12).

## Provenance and initial state

- Repository: `Rwanbt/gridcraft`; base: `main`; upstream: `storytold/gridcraft`.
- Base commit: `b3acae6b7782ea236d0fb87a0121fd1ba2e14c2b` (fork and upstream identical at audit).
- Initial branches on fork: `main` only. Initial open PRs: none.
- Workspace: 12 `crates/*`, three `apps/*`, one `xtask` (16 packages by manifests/tree; runner verification pending).
- Rust edition 2024, minimum Rust version 1.90. `Cargo.lock` exists.
- Baseline coverage: **UNKNOWN**. No instrumented LCOV reports or GitHub Actions runs were accessible at audit time.
- Collected test count: **UNKNOWN**. Source files containing `#[test]` do not establish runner-collected test count.
- Execution environment of this audit: GitHub repository API access; no local Rust toolchain and no network-enabled Git checkout. **No tests were executed locally.**
- Existing CI: `.github/workflows/ci.yml` invokes `cargo xtask ci` on PRs and main. No run records were returned from the fork's Actions runs endpoint before this change.

## Initial targeted change

- `crates/xlsx/src/csv.rs`: preserve malformed trailing UTF-16 code units as U+FFFD instead of silently discarding them after UTF-16 LE/BE BOMs.
- `crates/xlsx/src/tests/csv_tests.rs`: deterministic tests for UTF-16 LE/BE incomplete code units, valid inputs and end-to-end CSV import.
- `.github/workflows/coverage.yml`: PR/dispatch Linux coverage for `gridcraft-core` and `gridcraft-xlsx`, retained as an LCOV artifact. This **does not** represent whole-workspace coverage.

## How to verify against the exact PR head

1. Run the existing `CI` PR workflow (`cargo xtask ci`).
2. Run `Coverage baseline (core + XLSX)` for the same head SHA, downloading `gridcraft-core-xlsx-lcov`.
3. Record line/function (and, where supported, branch) coverage from the LCOV file; note that cargo llvm-cov's LCOV output may not include every branch metric.
4. Run `cargo xtask stats --exact` with a real Rust runner; collect exact runner test counts.
5. Expand measurement to the remaining packages, configurations, and platforms. Never aggregate incompatible denominators.

## Result recording

| Metric | Before | This change |
|---|---|---|
| Runner-collected tests | Not measured | Awaiting Actions |
| Core line/function coverage | Not measured | Awaiting LCOV |
| XLSX line/function coverage | Not measured | Awaiting LCOV |
| Other 14 workspace packages | Not measured | Not in this scoped coverage job |
| Full workspace CI | No runs returned | Awaiting new PR workflow |
| Desktop/native GUI E2E | Not measured | Not verified |
| Web/WASM E2E | Not measured | Not verified |

The goal is full defensible behavioral coverage, **not** a manufactured 100% threshold. Do not mark a component verified until tests and instrumented evidence exist.
