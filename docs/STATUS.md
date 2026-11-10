# Implementation status — v0.2.1

ARS MAGNA v0.2.1 is a small expression language with immutable bindings and output. It is **not** yet the language described in the engineering handoff.

## Implemented and tested

| Feature | Notes |
|---|---|
| Lexer with line and column tracking | Section 2 of `SPEC.md` |
| Parser with precedence, associativity and a nesting limit | Section 3 of `SPEC.md` |
| Immutable declarations, resolved at compile time | `LET` / `DECLARE` |
| Output | `MANIFEST` / `PRINT` |
| Values: 64-bit integer, boolean, string | |
| Checked integer arithmetic: `+ - * /`, unary `-` | Overflow and division by zero are errors |
| Equality `==` across all value types | Mixed types compare as `false` |
| Compilation to an in-memory instruction list, executed by a stack VM | Not a serialised bytecode format |
| Diagnostics with file, line, column, phase and hints | |
| Command line: run, `--version`, `--help`, defined exit codes | |
| Quiet exit when output is closed | |
| Automated tests | 50 tests (39 unit, 11 integration) |
| Reproducible Linux build | Verified byte-identical across build folders |
| CI: format, lint, tests on Windows and Linux, reproducibility check, packaging | Not yet run: the repository has not been pushed |

## Binaries

| Binary | State |
|---|---|
| `ars-magna-linux-x86_64` | Built and verified for this release (requires glibc 2.34+) |
| `ars-magna-windows-x86_64.exe` | **Not built.** No Windows toolchain is available in the environment used for this milestone. The CI workflow builds it on a Windows runner. |

## Not implemented

| Area (handoff section) | State |
|---|---|
| Formal normative specification (12, step 2) | Not started; `SPEC.md` describes v0.2.1 only |
| Serialised, versioned bytecode format and verifier (12, step 4) | Not started |
| Comments, string escapes | Not started |
| Types beyond integer, boolean, string; composite types (10) | Not started |
| Static type checking (10) | Not started |
| Functions and procedures (10) | Not started |
| Control flow and iteration (10) | Not started |
| Scopes and modules (10) | Not started |
| Error handling within programs (10) | Not started |
| Controlled external effects beyond standard output (10) | Not started |
| Standard library (1) | Not started |
| The eight foundational laws (6) | Not started; only DECLARATION is partly reflected (names must be declared before use) |
| Circles, capabilities, containment (6, 7) | Not started |
| Transactional execution and examination (8) | Not started |
| Invocation lifecycle: INVOKE, EXAMINE, LICENSE TO DEPART, DISMISS, BANISH (9) | Not started |
| Cosmological hierarchy: the Absolute, hosts, Lucifer, operator, spirits (4) | Not started |
| Angelic orders and ministries (5) | Not started |
| History: War in Heaven, the Fall (5, 7) | Not started |
| Subsystems LOGOS to EXAMINATIO (7) | Not started; LOGOS and NUMERUS exist only in the rudimentary form above |
