# GridCraft — Test Execution Ledger

All statuses below are for the exact commit indicated. Previous green jobs do **not** validate a later SHA.

| SHA | Run | Job | Verified outcome |
|---|---|---|---|
| `90cdfa2` | [38268012224](https://github.com/Rwanbt/gridcraft/actions/runs/38268012224) | Core/XLSX LLVM | PASS, LCOV artifacts; core 878/1033 lines, XLSX 9847/10780 |
| `90cdfa2` | [38268012152](https://github.com/Rwanbt/gridcraft/actions/runs/38268012152) | Full `cargo xtask ci` | FAIL; 226 engine library tests passed, 2 CSV export regressions failed |
| `c6721cc` | [38273895108](https://github.com/Rwanbt/gridcraft/actions/runs/38273895108) | Core/XLSX LLVM | Core/XLSX job PASS: core 985/1106 lines (89.06%), 129/150 functions (86%); XLSX 9847/10780 lines (91.35%). Engine coverage job FAIL: 228 tests passed, one new test expected byte-identical scientific formatting after reimport |
| `c6721cc` | [38273894961](https://github.com/Rwanbt/gridcraft/actions/runs/38273894961) | Full CI | FAIL at rustfmt; code tests were not reached |
| `bba7e79` | [38274825513](https://github.com/Rwanbt/gridcraft/actions/runs/38274825513) | Full CI | Pending at the time this log entry was authored; verify from GitHub |
| `bba7e79` | [38274825491](https://github.com/Rwanbt/gridcraft/actions/runs/38274825491) | Core/XLSX + engine LLVM | Pending at the time this log entry was authored; verify from GitHub |

The following cannot yet be asserted: whole-workspace success, full workspace coverage, browser/native E2E, macOS/Windows runtime behavior, or complete test inventory. An `xtask stats --exact` step is included in the CI workflow to collect runner-discovered test counts after the full gate succeeds.

## Determinism

- CSV LE/BE truncated code unit and invalid surrogate inputs are constant binary fixtures, no network or wall-clock dependence.
- Core array/bounds and value coercion tests use fixed values.
- Range intersection oracle is independently computed on a small bounded grid using a fixed seed.
- Existing `command_sequence_fuzz` uses deterministic seeds; scaling its CI seed matrix remains future work.
