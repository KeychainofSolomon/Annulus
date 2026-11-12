# Implementation status — v0.2.1

**The specification and the code are at different stages.** The specification (`docs/spec/`, thirteen files in draft) is being completed under roadmap step 1. Two machines sit side by side until step 2d. **`annulus ritus`** (branch `step-2c`) runs the step-2 subset of the specification on the new core: checker, evaluator, nucleus, stores on disk, the second key; this is also what Spiri72 calls for every question. **`annulus <file>`** still runs the v0.2.1 prototype (a small expression language with English keywords, `docs/PROTOTYPE.md`), which implements none of the specification and is removed in 2d.

## Step 2 in progress: the nucleus

`src/nucleus/` is the new core, built from `docs/spec/core.md` §9 and `formal/Annulus/Step.lean`: the state, one method per transition rule, the commit re-check, marks, release and abandonment, the Index, the Vessel (with its three first records), sealed petitions, the hash-chained audit, launch checks with a **test wand** (`sign_for_test`; never valid for a real installation), and SHA-256 written in-house (the project has no outside dependencies). 38 tests in `tests/nucleus_rules.rs`, each checking P1, P2, P4, P7 and the damnation law over its audit, including hostile spirits (overreach, forged seals, intrusion, injected words), suspicion by the Index's marks, angels' petitions with reserved grants, and seeded simulated runs that replay exactly (P9). Reachable from the command line as `annulus ritus` (and from Spiri72) since 2026-11-12; plain `annulus <file>` still runs the prototype below. The properties are tested, not yet proved (`formal/README.md`).

## Implemented and tested (v0.2.1 prototype)

| Feature | Notes |
|---|---|
| Lexer with line and column tracking | Section 2 of `PROTOTYPE.md` |
| Parser with precedence, associativity and a nesting limit | Section 3 of `PROTOTYPE.md` |
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
| `annulus-linux-x86_64` | Built and verified for this release (requires glibc 2.34+) |
| `annulus-windows-x86_64.exe` | **Not built.** No Windows toolchain is available in the environment used for this milestone. The CI workflow builds it on a Windows runner. |

## Not implemented

| Area (handoff section) | State |
|---|---|
| Formal normative specification (12, step 2) | **In draft:** thirteen files in `docs/spec/`, plus the registers; awaiting the owner's approval (roadmap step 1) |
| Serialised, versioned bytecode format and verifier (12, step 4) | Not started |
| Comments | **Ruled out:** Annulus has no comment syntax (the owner's ruling) |
| String escapes | Specified (`grammar.md` §2); not in the prototype |
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
