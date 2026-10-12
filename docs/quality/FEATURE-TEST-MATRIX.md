# GridCraft — Feature/Test Traceability (Initial Inventory)

Source: `b3acae6b7782ea236d0fb87a0121fd1ba2e14c2b`. Status as of 2026-10-12: **UNVERIFIED** means tests exist in the repository but their execution was not confirmed on this fork. Individual contract coverage is not inferred from file names.

| ID | Component / contract | Existing evidence | Risk / missing evidence | Status |
|---|---|---|---|---|
| CORE-01 | Addresses, cell/range bounds and parsing | `crates/core/src/addr.rs` inline tests | Runtime collection, hostile limits, coverage | UNVERIFIED |
| CALC-01 | Formula evaluation, dependencies, recalc | `crates/calc/src/tests.rs` | Negative/edge branches, benchmark, coverage | UNVERIFIED |
| FORM-01 | Formula lexer/parser/adjustment | `crates/formula/src/tests.rs`, Spanish input suite | Property corpus and complete parser boundaries | UNVERIFIED |
| FUNC-01 | Built-in functions and coercion | `crates/functions/src/*` | Every function and cross-function invariants | UNVERIFIED |
| FMT-01 | Number formats and decimal handling | `crates/numfmt/src/tests.rs` | Numeric extremes and instrumented branches | UNVERIFIED |
| MODEL-01 | Workbook persistence model | `crates/model/src/*` | Structural invariants and serialization coverage | UNVERIFIED |
| XLSX-01 | XLSX import/export and roundtrip | `crates/xlsx/src/tests/*` | Real corpus, x14 extensions, malformed formats | UNVERIFIED |
| CSV-01 | CSV decoding, separators and import | `crates/xlsx/src/tests/csv_tests.rs` | New truncated UTF-16 regression awaits CI | UNVERIFIED |
| XLSX-02 | Encrypted/legacy XLS, XLSB, ODS | `crates/xlsx/src/tests/*` | Corrupt, rare and encrypted corpus coverage | UNVERIFIED |
| ENGINE-01 | Commands, undo/redo and save/load | `crates/engine/src/tests.rs`, `command_sequence_fuzz.rs` | Seed replay in CI, mutation checks, E2E | UNVERIFIED |
| CHART-01 | Chart resolver and rasterization | `crates/chart/src/tests.rs` | Independent visual oracles and export | UNVERIFIED |
| PDF-01 | Print/PDF output | `crates/pdf/src/tests.rs` | Cross-platform renderer golden tests | UNVERIFIED |
| MCP-01 | MCP tool messages, headless backend | `crates/mcp/src/tests.rs` | Transport/permissions and error injection | UNVERIFIED |
| CLI-01 | Real CLI parsing and outputs | `apps/gridcraft-cli/tests/*` | Native release smoke and hostile I/O | UNVERIFIED |
| UI-01 | Native egui interaction/dialogs | `crates/ui-egui/tests/*` | Actual desktop startup, packaging, native E2E | UNVERIFIED |
| WEB-01 | WASM browser UI and files | `apps/gridcraft-web/src/*` | Browser E2E, import/export persistence | UNVERIFIED |

Status should change only with links to exact commit CI jobs, artifacts, and behavioral assertions. `PARTIAL` is warranted only after actual relevant tests run; `VERIFIED` requires each applicable contract checked.
