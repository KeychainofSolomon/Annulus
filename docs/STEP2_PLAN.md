# Step 2: the minimal interpreter (plan)

Drafted 2026-11-11, after the owner approved step 1. **Approved by the owner, 2026-11-11 16:41**, with all three points below.

## Goal

A small Annulus that really runs a rite end to end: a faculty, a Circle, a simulated spirit, a request, examination by his angel, a commit only of what passed and was granted, quarantine and consecration, release, and the three stores on disk. It **replaces** the v0.2.1 prototype; the prototype is not extended.

## How it is built

Three parts, in this order, so the core never depends on tradition:

| Part | What it is | Source of truth |
|---|---|---|
| **core** | the state, every transition and its enabling condition, the commit re-check, marks, release and abandonment, the Index, the Vessel, grave grants and sealed petitions, the audit | `core.md` §9 and `formal/Annulus/Step.lean`: one Rust function per transition rule, named after it |
| **rite** | the oracle the core asks, and only through the questions of `core.md` §3.1: admission, signatures, examiner, verdict, counterbalances, bounds, the will's open outcomes, the validator | the registers' JSON, loaded as data; first rite `goetia` |
| **lang** | the lexer, parser and type checker for the subset below, and the evaluator, which drives the core with one instruction at a time, as `Lang.next` does in the model | `grammar.md`, `static.md` |

The core is its own module, reached only through the interface of `core.md` §3.1. The implementation source has no comments (the owner's rule).

## The subset

**In:** `FACULTAS` with `liber` grants (`legere`, `scribere`, `creare`, `delere`, and reserved `… angelis`); `CIRCULUS`; `INVOCA` of a **simulated** spirit (`--voluntas simulata`, scripted answers via `--responsa`, `--sors` for hostile tests); `PETE` for one telling office and one working office; `EXAMINA`, returning `RATUM`, `SUSPENSUM` or `IRRITUM`; `RESPONSUM`; `CONSECRA` with one validator entry; `MANIFESTA`; `VADE IN PACE`, `DIMITTE`, `RELEGA`; `LET`/`SIT`, `SI`/`ALITER`, `DUM`; the deterministic scheduler; denials ending the run with class 2.

**Out, for later steps:** hours and the Moon (`horae`), kings, Earls and Dukes, angels' petitions, real AI agents (step 4), grave consecration and the second key (they need the real wand), `PER`, functions, concurrency beyond the scheduler, the mirror and the crystal, the bytecode VM.

## Stores on disk

The audit (one file per run), the Index (`.proscriptio`), the Vessel (beginning with its three records), the stores of grave grants and sealed petitions. Each is append-only and hash-chained, and checked at launch; a mismatch refuses to launch.

## The wand, for now

**Superseded 2026-11-12:** the real wand exists (`operator.md` §8.7). The test wand stays for tests, on real signatures with published keys.

The real wand (`virga`) comes later, but without a wand the faculty is empty. **Proposed:** a *test wand*, a key kept in a file and marked as such everywhere, accepted only with `--virga-probationis` and refused by any installation set up with a real wand. Every audit made with it says so on its first line.

## Tests

1. **One test per transition rule**, its enabling condition and its effect, named after the Lean constructor.
2. **The Lean statements as checks**: after every test run, the audit is checked for P1 (every commit within the handed set and *G*), P2 (every commit follows an independent approval), P4 (one terminal step, no later use), P7 (the audit only grows) and the damnation law.
3. **Hostile spirits**: simulated spirits that lie, overreach, try another Circle, forge a seal, and stall; every one refused, voided or held, and recorded.
4. The existing grammar tests (`tests/grammar`, 69 programs) kept passing.
5. CI on GitHub, Windows and Linux.

## Milestones

| | Milestone | Done when |
|---|---|---|
| 2a | **core**, with its own tests | every transition rule implemented and tested; property checks pass. **First pass done 2026-11-11:** `src/nucleus/` (the core is named `nucleus`, since `core` is Rust's own library), 31 rule tests in `tests/nucleus_rules.rs`, each checking P1, P2, P4, P7 and the damnation law over its audit. **Completed the same day:** suspicion at examination (proscription entries carry the agent's mark, `Reach`), angels' petitions end to end with reserved grants, and the seeded simulated scheduler and will (`nucleus::Sim`), with a replay test (P9) and 50 seeds checked against the theorems. 38 tests. **2a done.** |
| 2b | **lang** for the subset | the subset's grammar tests pass; type errors A5, A14, A15 reported. **Front end done 2026-11-11:** `src/lingua/` (`lexis` lexer, `syntaxis` tree, `parse` LL(1) parser for the **whole** grammar, `registra` reading `goetia.json` at build time, `check` the static checker for the subset). All 69 grammar programs agree with `tools/grammar_check.py`, including the position of every rejection. 43 checker tests: N1–N4, T1–T3, T7, A1–A3, A5, A7–A11, A13–A15. Constructs outside the subset are refused with the implementation code `X1` (not a spec code). The evaluator moves to 2c, with the rite it drives. |
| 2c | **rite** `goetia` and the command line | `annulus` runs the example rites; the stores are written and checked. **First half done 2026-11-11:** `src/ritus.rs` (the `goetia` rite as the nucleus's oracle, read from the register; the Constitution with one validator, `forma`; the test wand) and `src/lingua/exsequi.rs` (the evaluator, one Operator step at a time through the seeded scheduler, the simulated will with `sors` and scripted answers). Furfur (telling) and Halphas and Sabnock (working, `AEDIFICATIO`) run end to end in memory; 14 tests, each checking P1, P2, P4 and P7 over its audit. **Left:** the command line, the stores and the audit on disk, real files, the `--responsa` file. |
| 2d | **attack tests** | every hostile spirit refused; the prototype is removed |

## Approved by the owner (2026-11-11)

1. **The test wand**, as above: yes.
2. **The subset**, as above: yes.
3. **The example to aim at** (yes): a program that invokes **Furfur** for a telling office (he must be in his triangle to speak true), and a spirit whose office is working, for one small file effect, each examined by his angel, with a hostile run of each.
