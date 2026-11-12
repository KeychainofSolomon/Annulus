# Changelog

## Unreleased

### `VIRTUTES` (2026-11-12; rite and evaluator, not the nucleus)
- A spirit charged through `VIRTUTES` reports each handed place as `place: N bytes, M lines` or `place: absens`; the angel rereads every place and approves only an exact account. False, missing, extra or malformed accounts are `IRRITUM`; an incomplete one is held. Dates wait until the world records times [O].

### The hours of the sky (2026-11-12; rite and runtime, not the nucleus)
- **`INVOCA` keeps each rank's hours and the Moon's age** (`dynamic.md` §5.1, `authority.md` §8.3): temporal hours from sunrise and sunset at the declared place, civil twilight, the true new moon. A refusal now says why in plain words. Dukes come in their hours when the sky is declared clear; this opens the 22 Dukes.
- **Launch:** `--hora` (a fixed time, for reproducible runs), `--locus LAT,LON`, `--caelum serenum|turbidum`. Without a place, only Earls may come. Every audit records the time, place, weather and the Moon's age.
- Readings marked [O] in `dynamic.md` §5.1 await the owner: Prince by daylight, Knight from dawn and from sunset to the end of twilight.

### The will slot (2026-11-12; not a core change)
- **`Voluntas`**: an outside will (a local model, in Spiri72) may supply a spirit's turn. It sees only what the request hands it and the outcomes open to the spirit; it may declare obedience, part or delay, never more. Until the core records an unknown outcome, it runs only with the test wand, and the audit notes that outcomes are the model's declarations.

### Ruled by the owner (2026-11-12): the network grant
- **One read-only grant per host** (`rete "<host>" legere`), one search grant (`quaerere`, results quarantined), and any other host only by a sealed petition and the second key. A grant for the whole web is refused: a request's address can carry data out.

### Step 2c, second half: the command line and the stores on disk (2026-11-12; not a core change)
- **`annulus ritus <program>`** checks the program, opens the stores, reads the granted files, runs, prints the output, writes the audit, seals the stores, notes petitions and writes the files back; it exits with the run's class. Flags: `--virga-probationis`, `--semen`, `--sors`, `--responsa` (a JSON list of answers), `--acta`, `--domus` (default `annulus.domus` beside the program), `--mundus` (default the program's folder). **`annulus confirma <fingerprint>`** turns the second key outside a session; **`annulus petitiones`** lists what waits. The old `annulus <file>` prototype is unchanged until 2d.
- **Quarantine survives between runs:** a file a spirit wrote is recorded in `annulus.signa` and read back quarantined, never as plain outside text. The record is covered by the signed roots record, so editing or deleting it refuses the launch. A mark is never removed (no presumption that a later file is clean).
- **Real files, closed list:** a place must be a plain relative path; a link anywhere on its path, or a place inside the house of the stores, refuses the launch. A file changed on disk during the run is left as it is. Writes are staged as `<name>.annulus-novum`, created fresh, so staging can never overwrite another file.
- **A petition is pending** while its last petition comes after its last confirmation, so a second identical move is listed again after the first key is spent.
- **Tests:** `tests/tabularium.rs` (20) and `tests/ritus_cli.rs` (16). 252 tests in all, debug and release.

### Amended (approved by the owner, 2026-11-11)
- **Entries are declined by case** (`grammar.md` §3.2, §4): `INVOCA SPIRITUM 34` (accusative), `SIGILLUM SPIRITUS 34` / `SIGILLUM ANGELI 34` (genitive), `SPIRITUS 34` alone (nominative), as `GERE ANULUM` and `INVOCA REGEM` already were. Reason: an outside review noted `INVOCA SPIRITUS` is not Latin.
- **The axiom** reads *nulla potestas a se ipsa oritur*: *sibi ipsi* meant "for itself", not "from itself".
- **Proof, until the proofs exist** (`authority.md` §2.5): the Lean model updated and building, every theorem check and attack test passing, and an independent audit.
- **`TEMPESTAS` is outside the step-2 subset** until the storm (fault injection) exists; it used to commit an unchanged write, a false record.
- **A spirit's name belongs to the spirit** (`static.md` §2.1, error N5). No program name may be spelled as any label of a register entry, in any recorded spelling, even if the program never refers to that entry. Reason: §2.1 allowed `SIT furfur = INVOCA furfur …` while N5 forbade declaring a register label; the owner chose N5, which keeps `authority.md` §2.3 (no program shadows a Constitution entry). The grammar example and five grammar test programs now name the invocation `vocatus`.

### Fourth outside review, for exploitable defects (2026-11-11; core changes, audit owed)
- **Reproduced and fixed, store substitution:** stores replaced together with their expected roots passed launch, since no signature covered the roots. The wand now signs the faculty *with* the roots of the Index, the Vessel and the sealed petitions.
- **Reproduced and fixed, struck-out revocation:** removing a wand from the revocation list passed launch. The owner now signs the revocations with the Constitution; the launching wand cannot vouch for its own standing.
- **The chain's tamper hook** exists only under the test feature `probatio`; `intact` also compares lengths.
- **A source is at most 1 MiB** (`grammar.md` §2), in the language and in the CLI, which reads no further.
- **Known and unchanged:** test signatures are forgeable by design until the real wand. **Deferred:** making the core's state private behind read-only views, before real agents.

### Third outside review, of the formal model (2026-11-11; core changes, audit owed)
- **A1 (high), P5 did not hold every rite answer fixed:** the admission of inscriptions and acts, and the legions bound, were not choices, so two runs could differ by the rite's answer alone and still count as making the same choices. Fixed: `Choice.answer` and `Choice.legions`; the core now asks the rite only after its own conditions (ink, a live spirit), and a refusal by the rite is a recorded step, `Label::Inscribe`/`Label::Act` with `admitted = false` and the dismissed invocations, as a refused invocation already was. Lean: `inscribeRefused`, `actRefused`. Tests: a rite that peeks at quarantined data is caught on the record; a refusal ends the run and dismisses every open invocation; the rite is never asked when the core already refuses.
- **A2 (medium), P4's "use" was narrower than Hold(*i*):** acts of the rite on an invocation now count as use (`UsesHold`). Hold(*i*) is amended to follow ruling R1 (a): seals belong to the Circle, so they leave Hold(*i*); the ink drawn for an invocation is that of its offerings (`core.md` §9.2).

### The apparatus (the owner's request, 2026-11-11; a core change, audit owed)
- **`GERE ANULUM/VIRGAM/HEXAGRAMMUM/PENTAGRAMMUM`, `OFFER`, `APPELLA`, `VETA`, `CONSTRINGE … PER NOMEN` now run** (`spirits.md` §4.4, `dynamic.md` §4a). Core: one new step, `act`, recorded as `Label::Act`; the rite gains one question, `admit_act`, which may only refuse. Lean: `Instr.act`, `Label.act`, `CoreStep.act`, the universe fields `RiteAct`, `actInv`, `actInk`.
- **Each spirit as his witnesses say:** Asmoday stops lying once his name is spoken; Belial only after an offering *and then* the Divine Name; Paimon refuses without an offering and speaks only in part until compelled; Beleth comes only to ring and wand, and the wand closes his fury; Andras's harm is closed by the pentagram. Tested in `tests/lingua_run.rs`. Written in the rite but **not yet reachable**, so untested: Astaroth's breath closed by the ring and Focalor's harm closed by `VETA` (dukes, refused until hours), and the refused offering to Malphas (a President, refused until the kings). Berith's ring does nothing until hours exist (no presumption that the hour is still running).
- **Checker:** A11 (instruments held earlier in the Circle on every path), A12 (only a recorded form of a divine name binds), A1, A9 and T1 for the acts. The register of names (`nomina.json`) is read for the divine names.
- **The technicality filter** gains every act × standing (no Circle, live, released, unknown) × ink, and a test that only the Operator acts and a refused offering costs nothing.

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
