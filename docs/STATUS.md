# Implementation status

Last updated: 2026-11-12, 15:30 (branch `step-2c`). **This page is updated with every fix** (the owner's rule).

**Two machines sit side by side until step 2d.** `annulus ritus` runs the step-2 subset of the specification on the new core. `annulus <file>` still runs the v0.2.1 prototype (`docs/PROTOTYPE.md`), which implements none of the specification and is removed in 2d. The specification (`docs/spec/`, thirteen files) is in draft. **The authority properties are tested, not proved:** P1–P9 are stated in Lean (`formal/`), with no proofs yet.

## What runs: `annulus ritus` (step 2a–2c)

| Part | State |
|---|---|
| Nucleus (`src/nucleus/`): one method per transition rule of `Step.lean`, commit re-check, marks, release, Index, Vessel, sealed petitions, hash-chained audit, launch checks | Implemented, tested over every audit (P1, P2, P4, P7, damnation law); state private behind read-only views |
| Front end (`src/lingua/`): lexer, parser for the whole grammar, register from `goetia.json`, checker for the step-2 subset | Implemented; agrees with the Python reference on all 69 grammar programs |
| Evaluator (`exsequi`), the `goetia` rite as data, the honest examiner (judges the proposal, never the drawn outcome) | Implemented |
| The apparatus: ring, wand, hexagram, pentagram, offering, naming, forbidding, binding by the Divine Name | Implemented for Paimon, Belial, Asmoday, Beleth, Andras; Astaroth, Focalor, Malphas written but unreachable |
| Stores on disk (`tabularium`): Index, Vessel, sealed petitions, signed roots record, quarantine record (`annulus.signa`), audit per run, granted files read and written | Implemented and tested: tampering, forged or missing roots, another wand, links, escapes from the folder all refuse to launch |
| Command line: `annulus ritus`, `confirma` (second key), `petitiones` | Implemented and tested through the real binary |
| An outside will (`Voluntas`): a local model may supply a spirit's turn | Implemented; **test wand only**, until the core can record an outcome it does not know |
| Seals: all 72 traced from the 1916 plates, with provenance (`docs/sources/goetia-seals/traced/`) | Done; Sloane 3825 not yet compared |

Tests: **263**, debug and release, on Linux here; Windows and Linux in CI.

## Spiri72 (the model side, `KeychainofSolomon/Spiri72`)

Uses this repository as a library, pinned by commit. A local model (llama.cpp, this computer only) is the will of the spirits; Annulus judges and permits. The binding page (seals in their rank metals, the Triangle and the Circle) or `--terminal`. 14 of the 72 can be bound today. 49 tests, including `tests/hostis.rs`: a fully hostile model, 16 attack families.

## Security fixes (red-team and audits)

| Date | Breach | Fix |
|---|---|---|
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
| Proofs of P1–P9 and their correspondence to the Rust core | Not started; the largest gap |
| An unknown outcome for a real will, so a model may run without the test wand | Owed (core change) |
| The agent's seal (model file and settings hash), recorded per rite | Planned (`agens`) |
| Independent audit of the core changes of 2026-11-11 and 2026-11-12 | Owed |
| Kings (`INVOCA REGEM`), so Presidents can come; hours of the sky, so Dukes can come; ink for two-rank seals | Not started; open ruling for the ink |
| Offices beyond telling and the five working kinds (teaching, transformation, dignities…) | Not in the subset |
| The escalation, so `RECUSATIO` can open; spirit-specific malice | Not started |
| Real validators before step 4; the trust handshake | Not started |
| The seven doubled seals other than Bune | Open ruling |
| Drift watch: a CI step comparing the Python and Rust parsers' rejection positions | Owed |
| Web reading, Gmail drafts, OCR for the agent | Planned; each needs the words-as-words treatment first |
| Step 2d: remove the prototype; a real wand | Not started |
