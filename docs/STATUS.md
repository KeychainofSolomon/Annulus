# Implementation status

Last updated: 2026-11-12, 18:30 (`main`). **This page is updated with every fix** (the owner's rule).

**Two machines sit side by side until step 2d.** `annulus ritus` runs the step-2 subset of the specification on the new core. `annulus <file>` still runs the v0.2.1 prototype (`docs/PROTOTYPE.md`), which implements none of the specification and is removed in 2d. The specification (`docs/spec/`, thirteen files) is in draft. **The authority properties are mostly tested, not proved:** P1–P9 are stated in Lean (`formal/`); the first proofs are in `formal/Annulus/Proofs.lean`.

## What runs: `annulus ritus` (step 2a–2c)

| Part | State |
|---|---|
| Nucleus (`src/nucleus/`): one method per transition rule of `Step.lean`, commit re-check, marks, release, Index, Vessel, sealed petitions, hash-chained audit, launch checks | Implemented, tested over every audit (P1, P2, P4, P7, damnation law); state private behind read-only views |
| Front end (`src/lingua/`): lexer, parser for the whole grammar, register from `goetia.json`, checker for the step-2 subset | Implemented; agrees with the Python reference on all 69 grammar programs |
| Evaluator (`exsequi`), the `goetia` rite as data, the honest examiner (judges the proposal, never the drawn outcome) | Implemented |
| The apparatus: ring, wand, hexagram, pentagram, offering, naming, forbidding, binding by the Divine Name | Implemented for Paimon, Belial, Asmoday, Beleth, Andras; Astaroth, Focalor, Malphas written but unreachable |
| Stores on disk (`tabularium`): Index, Vessel, sealed petitions, signed roots record, quarantine record (`annulus.signa`), audit per run, granted files read and written | Implemented and tested: tampering, forged or missing roots, another wand, links, escapes from the folder all refuse to launch |
| Command line: `annulus ritus`, `confirma` (second key), `petitiones` | Implemented and tested through the real binary |
| An outside will (`Voluntas`): a local model may supply a spirit's turn | Implemented. The core records each of its turns as **declared, never known** (real-agent mode, `core.md` §9.2); it may declare only obedience, part or delay, and cannot kill. **Test wand only** until a real wand exists (step 2d) |
| **The hours of the sky** (`src/horae.rs`): sunrise, sunset and civil twilight at the declared place; the Moon's age from the true new moon; each rank's window (`dynamic.md` §5.1); `--hora`, `--locus`, `--caelum`; every audit records the sky the run used | Implemented and tested against the 2000, 2017 and 2024 new moons and Berlin's solstices. **Dukes can now be bound.** Readings marked [O] await the owner |
| **`VIRTUTES`**: a spirit reports each handed file's size in bytes and lines; the angel rereads them | Implemented and tested: eight kinds of false account all caught (`offices.md` §2.2). Dates left out until the world records times |
| **`PRAETERITA`** (the present run): a spirit recounts the run's past, one claim per line in the history's words; the angel checks each against the history built from the audit | Implemented and tested: false claims and Berith's lies caught. The previous run (`"prior"`) not yet |
| Seals: all 72 traced from the 1916 plates, with provenance (`docs/sources/goetia-seals/traced/`) | Done; Sloane 3825 not yet compared |

Tests: **281**, debug and release, on Linux here; Windows and Linux in CI.

## Spiri72 (the model side, `KeychainofSolomon/Spiri72`)

Uses this repository as a library, pinned by commit. A local model (llama.cpp, this computer only) is the will of the spirits; Annulus judges and permits. The binding page (seals in their rank metals, the Triangle and the Circle) or `--terminal`. 30 of the 72 can be bound in their hours (Dukes since the hours of the sky; VIRTUTES and PRAETERITA through the model since 2026-11-12; control vectors for the spirits' natures in `vires/`). 54 tests, including `tests/hostis.rs`: a fully hostile model, 17 attack families.

## Security fixes (red-team and audits)

| Date | Breach | Fix |
|---|---|---|
| 2026-11-12 | (Core audit 1, `docs/AUDIT_CORE_1.md`) Rolling the stores back revived a spent key or lifted a damnation; deleting the house started it afresh | An anchor outside the house records the last sealed roots; launch refuses a rolled-back or missing house |
| 2026-11-12 | (Core audit 1) A model was shown files it was never handed to read | It sees only places its request handed it `legere` |
| 2026-11-12 | (Core audit 1) Words for a working office, or work for a telling one, could be approved | The angel rejects the wrong kind of answer |
| 2026-11-12 | (Core audit 1) The petition text shown before `confirma` was unsigned | The effects are stored and checked against the fingerprint before they are shown |
| 2026-11-12 | (Core audit 1) Windows spellings (case, trailing dots, `~` short names) could reach the house | Such names refused; the house compared case-blind |
| 2026-11-12 | (Core audit 1) A petition could be confirmed twice; open instrument list; non-text `PETE` arguments | Closed |
| 2026-11-12 | The angel saw a citation of an unhanded place only as `[locus: x]`; other spellings passed a false source | Every guise read; anything not exactly a handed place refused (`officia::cited`) |
| 2026-11-12 | JSON nesting unbounded: a hostile model server could abort a run before its audit was written | Nesting stops at 64; doubled keys refused (`lingua::json`) |
| 2026-11-12 | A spirit's words reached the terminal raw: clipboard (OSC 52), window title, reversed text, lines forged as Annulus's own | Hidden characters shown as `\u{..}`; every line of an answer marked (`exsequi::visible`) |
| 2026-11-12 | Hidden characters stored raw in the Vessel | Stored as reversible `\u{..}` escapes |
| 2026-11-12 | Quarantine lost between runs (a spirit's file read back as outside text); links in folders on a path; staging could overwrite another file; writes over files changed during a run | `annulus.signa` under the signed roots; every path part checked; fresh staging files; changed files left alone |
| 2026-11-12 | (Spiri72) work items took extra fields; the page server took missing or doubled lengths and cut-off bodies | Exact fields; strict lengths, chunked refused |
| 2026-11-11 | Outside reviews 1–4: mark laundering through loops, overwriting creates, store substitution, struck-out revocations, examiner not required to be an angel | Fixed in the core with tests (see `CHANGELOG.md`) |

## Open

| Area | State |
|---|---|
| Proofs of P1–P9 and their correspondence to the Rust core | **Begun** (`formal/Annulus/Proofs.lean`, checked by CI with no extra axioms): **P1 (a)**, containment at commit; **P1 (b)**, every request's handed places lie in the faculty in every reachable state (an invariant over runs); an outside will's turns are declared; `kill` only in simulated mode. P1 (c) and P2–P9 remain; correspondence to Rust not started. Still the largest gap |
| The agent's seal (model file and settings hash), recorded per rite | Planned (`agens`) |
| Independent audit of the core changes of 2026-11-11 and 2026-11-12 | **Done** (`docs/AUDIT_CORE_1.md`); confirmed findings fixed. The change of the evening (declared outcomes) and the anchor are not yet reviewed |
| The audit files (`.acta`) are not signed or read back | Open (core audit 1, #5); before `PRAETERITA` of the previous run |
| The office's form enforced in the core, not only by the angel | Open (core audit 1, #4) |
| Rust and Lean drift: validator manifest, Circle binding of values, ink without a wand, forced outcomes, the grave store, a refused `CONSECRA` | Open (core audit 1, #8) |
| The anchor can itself be deleted by someone with access to it | Open until a real wand (2d) |
| Kings (`INVOCA REGEM`), so Presidents can come; ink for two-rank seals | Not started; open ruling for the ink |
| Berith's ring hour | Needs `GERE` to record its time (the Prince's and Knight's hours are ruled) |
| `PRAETERITA` of the previous run (`historia "prior" legere`) | Not started: the history must be carried between runs |
| Offices beyond telling and the five working kinds (teaching, transformation, dignities…) | Not in the subset |
| The escalation, so `RECUSATIO` can open; spirit-specific malice | Not started |
| Real validators before step 4; the trust handshake | Not started |
| The seven doubled seals other than Bune | Open ruling |
| Drift watch: a CI step comparing the Python and Rust parsers' rejection positions | Owed |
| Web reading, Gmail drafts, OCR for the agent | Planned; each needs the words-as-words treatment first |
| Step 2d: remove the prototype; a real wand | Not started |
