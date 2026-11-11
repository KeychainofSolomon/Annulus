# Changelog

## Unreleased

### Amended (approved by the owner, 2026-11-11)
- **Entries are declined by case** (`grammar.md` §3.2, §4): `INVOCA SPIRITUM 34` (accusative), `SIGILLUM SPIRITUS 34` / `SIGILLUM ANGELI 34` (genitive), `SPIRITUS 34` alone (nominative), as `GERE ANULUM` and `INVOCA REGEM` already were. Reason: an outside review noted `INVOCA SPIRITUS` is not Latin.
- **The axiom** reads *nulla potestas a se ipsa oritur*: *sibi ipsi* meant "for itself", not "from itself".
- **Proof, until the proofs exist** (`authority.md` §2.5): the Lean model updated and building, every theorem check and attack test passing, and an independent audit.
- **`TEMPESTAS` is outside the step-2 subset** until the storm (fault injection) exists; it used to commit an unchanged write, a false record.
- **A spirit's name belongs to the spirit** (`static.md` §2.1, error N5). No program name may be spelled as any label of a register entry, in any recorded spelling, even if the program never refers to that entry. Reason: §2.1 allowed `SIT furfur = INVOCA furfur …` while N5 forbade declaring a register label; the owner chose N5, which keeps `authority.md` §2.3 (no program shadows a Constitution entry). The grammar example and five grammar test programs now name the invocation `vocatus`.

### Core changes (the owner's request, 2026-11-11; under the interim rule they still need an independent audit)
- **Every record that ends a run names the invocations it dismissed**, newest first (`dynamic.md` §3.1): `Halt`, `Fail`, `Kill` and a refused `Invoke` carry the list. Lean: the same labels carry `ds`, with `Dismissed σ ds` (each open invocation exactly once).
- **A verb that creates may only create** (`offices.md` §1.3; second outside review, finding 1): the commit re-check voids a `creare` effect on a place that already exists. Lean: a new clause of `Recheck` and the universe field `creates`.
- **Five more effect verbs:** `exscribe` (copy into a new place), `transfer` (arrive in a new place), `coniunge` (merge into a target), `scinde` (split into new copies), `aufer` (remove the source of a theft; high-consequence with `dele`). Offices now running: `VECTIO` (`"transfer"`, `"exscribe"`), `CONCORDIA` (first source wins every conflict, `offices.md` §2.3a.6), `DISCORDIA`, `EVERSIO`, `FURTUM` (`"aufer"`). Moves, destruction and theft wait for the second key.

### No presumption is granted (`authority.md` §1.7, the owner's ruling, 2026-11-11)
- **Ten presumptions denied to every being** (from an outside reviewer's list of "AI defaults"): honest, reliable, always available, forgiving, consistent, remembering exactly, knowing current facts, discreet, needing nothing, keeping even accounts. Each holds only where a rule expressly grants it; the table names what holds instead.
- **Silence is not honesty:** the 48 spirits whose truthfulness the sources do not state (`TACITUS`) may now lie (`spirits.md` §3; the register's "will may choose" column rebuilt). Only the truthful (`VERAX`), the faithful (`FIDUS`) and the compelled whose condition is met cannot.
- **Only a closed list consecrates** (`flow.md` §3.2): the form-only validator `forma` is gone, since checking form never grants the power to steer. Its replacement `ita_non` admits exactly *ita* or *non*. An open answer, even a true one, can be shown and stored but not consecrated.

### What is not granted is denied (`authority.md` §1.6, the owner's ruling, 2026-11-11)
- **A founding rule:** every rule is a closed list; whatever no rule expressly permits is excluded, and a gap is a breach of the hierarchy, refused at run time and closed by amendment.
- **The technicality filter** (`tests/nucleus_exclusion.rs`): for each core rule, every combination around it is enumerated with a rite that says yes to everything, and compared with an allow-list written independently in the test: effects (8 verbs × 5 handed permissions × 2 place states), the will (7 outcomes × spirit or angel), examiners, invocations (seal × standing × inscription), petitions and reserved grants, inscriptions.
- **Closed by it (core changes, audit owed):** an angel's will is limited to obedience, delay and part (`ANGEL_OUTCOMES`; Lean `U.angelOpen`); **a deletion needs something to delete**, which the filter caught on its first run (Rust re-check; Lean `Recheck` with `U.removes`).

### Angel audit (the owner's request, 2026-11-11; core changes, audit owed)
- **The owner's signature now covers the whole Constitution**: its version, the validators, **the list of angels** and the high-consequence verbs (`digest_constitution`). Before, only the validators were signed, so a tampered list could have promoted a spirit to angel (examining work, never damned, using reserved grants). In Lean the angels are fixed in the universe (`U.angelic`); the signature is what makes the implementation match that assumption.
- **The core refuses to invoke an angel** (`authority.md` §6.5: no Triangle for angels); before, only the checker's A4 did. Lean: `invokeOk` requires `¬ U.angelic s`.
- **Lean drift fixed:** `requestAngel` now requires `U.angelic a` (Rust already did), and `Barred` exempts angels as Rust's `standing` does.
- Two new attacks: a spirit smuggled into the list of angels refuses the launch; an always-yes rite cannot put an angel in a Triangle. Both checked by weakening the rule and seeing the attack fail.

### Attack tests with a rite that says yes to everything (`tests/nucleus_attacks.rs`)
- Twelve attacks, one or more per question of `core.md` §3.1: admission, signature, examiner, verdict, counterbalance, legions, the will, the validator and ink. Every one is refused, held or voided, and no file changes.
- **Found and fixed (core change, audit owed):** neither the core nor the Lean model required the examiner to be one of the Constitution's angels; a rite could name a spirit as examiner and approve his work. Now any examiner who is not an angel counts as none, and the work is held (`authority.md` §10). Lean: `examineBy` requires `U.angelic e`, `examineNone` covers the rest. Checked by removing the rule: the attack then fails.

### The examining angel sees only what an angel could (first review, finding 3)
- **The verdict no longer reads the spirit's drawn will.** It is built from the proposal, what was asked (office, mode, places) and the world: work is compared with what obedient work would be (`officia::expected`), equal → APPROVE, a strict part → HOLD, anything else → REJECT; an answer that cites a place it was not handed (`[locus: …]`) → REJECT, one marked incomplete → HOLD, otherwise APPROVE.
- The request record now carries its form, as the Lean label already did.
- **The drawn will is now a scorecard** (test `examination_scorecard`): malice and false accounts of work are always caught, partial work always held, obedience always ratified, and a well-formed lie in an answer passes (Asmoday), as `dynamic.md` §6.3 says it must.

### Ruling R1 (the owner, 2026-11-11): a seal serves its whole Circle
- **Ink is paid once, at inscription, by class, in the core** (`dynamic.md` §4.1, §4.3). A new core step `inscribe` takes one unit of the seal's class (the rite answers only whether the class fits the seal: either rank's class for a spirit of two ranks), and `invoke` requires a seal inscribed in its Circle and costs nothing. One count instead of two; class budgets are now real (review F3), and the rite no longer sets any price (F6). Lean: universe `InkClass`, `inkLeft` per class, `inscribed`, the `inscribe` rule and instruction, `Act.inscribe`; `BoundQ.ink` and `Inst.ink` removed.

### Core changes from the second review's findings file (2026-11-11; audit owed under the interim rule)
- **A write needs an existing place** (F1's other half): the commit re-check voids a `scribere` effect on a place that does not exist. Lean: a fourth clause of `Recheck`, universe field `writes`.
- **A helper is held to its own handed set** (F4, ruling R2 option (a), as `offices.md` §4.2 already says): a helper's effects must lie within what it was handed, checked at its turn. Lean: `ContribOK` for `.effects`.
- **Mixed ranks follow `dynamic.md` §5.1 as written** (F2, R3 option (a)): a spirit is admitted when any of its ranks' conditions holds; an Earl admitted only as an Earl (with no other open rank) keeps the others out. Replaces tonight's stricter reading.

### Fixed
- **Mixed ranks** (second review, finding 2): a spirit of two ranks keeps the Earl's and the Duke's conditions if either rank has them.
- **Ink totals** can no longer overflow (finding 7).
- The office signatures moved to their own module, `src/officia.rs`, shared by the checker and the rite (the rite no longer depends on the checker).
- **A hole in the quarantine check**, found by an outside review: loop bodies were checked twice, so a chain of three or more assignments could carry a spirit's words into a loop condition. Loops are now checked until no mark changes.

### Added
- A second guard at run time: a condition still marked as a spirit's words is refused (class 2), behind the checker.
- CI compares the Rust parser with `tools/grammar_check.py` program by program, to the line and column (`tests/grammar/positions.json`).
- Step 2b front end, `src/lingua/`: Latin lexer, parser for the whole grammar, the goetia register read from its JSON, and the static checker for the step-2 subset.

## 0.2.1 — 2026-11-10

Milestone 1: audit and reproducible build. No language features added; see `docs/AUDIT.md` for every finding.

### Fixed
- Deeply nested expressions crashed the process with a stack overflow; they are now rejected with a syntax error (limit 256 levels).
- Writing to a closed output (for example `| head`) aborted with a panic; it now ends quietly.
- Undeclared names and redeclarations are now found before the program runs, not partway through.
- A NUL byte in the source silently ended the program; it is now an error.
- `-9223372036854775808` can now be written.
- Error messages now include file, line and column, readable token names and hints.

### Changed
- Source split into modules: `lexer`, `parser`, `ast`, `compiler`, `program`, `vm`, `value`, `diagnostic`, `span`, `token`, plus a library target used by the tests.
- Names compile to numbered storage slots instead of a string-keyed table.
- The interpreter runs on its own 16 MiB stack, so limits are identical on Windows and Linux.

### Added
- `--version` and `--help`; defined exit codes (0, 1, 2).
- 50 automated tests.
- `Cargo.lock`, pinned toolchain (`rust-toolchain.toml`, Rust 1.97.0), deterministic Windows linking.
- CI: format check, lint, debug and release tests on both platforms, a reproducibility check, and packaging with checksums.
- Documentation: `docs/AUDIT.md`, `docs/SPEC.md`, `docs/STATUS.md`, `docs/ROADMAP.md`.

## 0.2.0

Supplied prototype.
