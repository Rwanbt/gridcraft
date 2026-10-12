# GridCraft — Remaining Quality Risks (2026-10-12)

1. **BLOCKER: no runner evidence yet.** Fork had no accessible GitHub Actions runs at start. A draft PR must trigger existing `CI` and scoped coverage; if jobs do not start, enable Actions for the fork using repository settings.
2. **HIGH: spreadsheet data fidelity.** Upstream issue [#703](https://github.com/storytold/gridcraft/issues/703) documents Excel 2010+ x14 conditional-format extension loss on read/save. This is not fixed by the CSV patch and deserves an isolated regression and implementation task on this fork.
3. **HIGH: whole-workspace coverage absent.** Scoped core/XLSX LCOV is a first baseline only. Functions, engine, native GUI, web, MCP, CLI, PDF, and platform-specific code remain unmeasured.
4. **HIGH: native and browser E2E unverified.** A passing Rust unit suite is not a substitute for desktop packaging/open/edit/save/reload or browser tests.
5. **MEDIUM: coverage of malformed/hostile formats.** Existing XLSX mutation and command-sequence tests should run deterministically at recorded seeds, with bounded memory and verified reports.
6. **MEDIUM: cross-platform gates.** Existing main CI runs on Ubuntu; Windows/macOS and native app lifecycle should have real reproducible jobs before release certification.
7. **LOW: unreviewed external standards.** `AGENTS.md` points to a separate `storytold/craftrules` `AGENTS.md`; it was not retrievable using the current connector. Do not infer compliance without reading it.

No PR merge or upstream contribution is authorized. `READY FOR INTEGRATION` and `production-ready` are **not** established here.

8. **CI failure reproduced:** On `90cdfa2`, 2 / 228 engine library tests failed because CSV General formatting narrowed 12-digit integers; fix and added tests must be verified on the *next* SHA before changing readiness status.
