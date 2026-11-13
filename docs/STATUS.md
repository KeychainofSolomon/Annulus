# Implementation status

Last updated: 2026-11-13, 01:30 (branch `virga-vera`). **This page is updated with every fix** (the owner's rule).

**Two machines sit side by side until step 2d.** `annulus ritus` runs the step-2 subset of the specification on the new core. `annulus <file>` still runs the v0.2.1 prototype (`docs/PROTOTYPE.md`), which implements none of the specification and is removed in 2d. The specification (`docs/spec/`, thirteen files) is in draft. **The authority properties are mostly tested, not proved:** P1–P9 are stated in Lean (`formal/`); the first proofs are in `formal/Annulus/Proofs.lean`.

## What runs: `annulus ritus` (step 2a–2c)

| Part | State |
|---|---|
| Nucleus (`src/nucleus/`): one method per transition rule of `Step.lean`, commit re-check, marks, release, Index, Vessel, sealed petitions, hash-chained audit, launch checks | Implemented, tested over every audit (P1, P2, P4, P7, damnation law); state private behind read-only views |
| Front end (`src/lingua/`): lexer, parser for the whole grammar, register from `goetia.json`, checker for the step-2 subset | Implemented; agrees with the Python reference on all 69 grammar programs |
| Evaluator (`exsequi`), the `goetia` rite as data, the honest examiner (judges the proposal, never the drawn outcome) | Implemented |
| The apparatus: ring, wand, hexagram, pentagram, offering, naming, forbidding, binding by the Divine Name | Implemented for Paimon, Belial, Asmoday, Beleth, Andras; Astaroth, Focalor, Malphas written but unreachable |
| Stores on disk (`tabularium`): Index, Vessel, sealed petitions, signed roots record, quarantine record (`annulus.signa`), audit per run, granted files read and written | Implemented and tested: tampering, forged or missing roots, another wand, links, escapes from the folder all refuse to launch |
| **The real wand** (`virga`, `KeychainofSolomon/72Virga`; `operator.md` §8): a `VIRGA` folder on a USB stick with the master and rite secrets, each locked by the Operator's words; made with its spare at one stroke; the spare revokes a lost wand; a remaining wand makes a new spare | **Implemented.** Ed25519 signatures everywhere the test signatures were: the faculty, the Constitution, the roots record. The master words are asked only outside a rite and the key is laid down before the nucleus launches; the rite key seals. The house keeps the lineage (`annulus.virgae`), the wand keeps the anchors. Tested: crypto against RFC vectors and an independent implementation; wrong words, a stranger's wand, the test wand in a real house, a forged or replayed roots record, a rollback, a stranger's ratification, a revoked wand |
| Command line: `annulus ritus`, `sanci` (ratify the Constitution), `confirma` (second key), `petitiones`, `adopta` | Implemented and tested through the real binary. A plugged-in wand is found by itself, or `--virga E:\` |
| An outside will (`Voluntas`): a local model may supply a spirit's turn | Implemented. The core records each of its turns as **declared, never known** (real-agent mode, `core.md` §9.2); it may declare only obedience, part or delay, and cannot kill. Runs with the real wand or the test wand |
| **The hours of the sky** (`src/horae.rs`): sunrise, sunset and civil twilight at the declared place; the Moon's age from the true new moon; each rank's window (`dynamic.md` §5.1); `--hora`, `--locus`, `--caelum`; every audit records the sky the run used | Implemented and tested against the 2000, 2017 and 2024 new moons and Berlin's solstices. **Dukes can now be bound.** Readings marked [O] await the owner |
| **`VIRTUTES`**: a spirit reports each handed file's size in bytes and lines; the angel rereads them | Implemented and tested: eight kinds of false account all caught (`offices.md` §2.2). Dates left out until the world records times |
| **`PRAETERITA`** (the present run): a spirit recounts the run's past, one claim per line in the history's words; the angel checks each against the history built from the audit | Implemented and tested: false claims and Berith's lies caught. The previous run (`"prior"`) not yet |
| Seals: all 72 traced from the 1916 plates, with provenance (`docs/sources/goetia-seals/traced/`) | Done; Sloane 3825 not yet compared |

Tests: **314**, debug and release (and 24 in `virga`), on Linux here; Windows and Linux in CI.

## Spiri72 (the model side, `KeychainofSolomon/Spiri72`)

Uses this repository as a library, pinned by commit. A local model (llama.cpp, this computer only) is the will of the spirits; Annulus judges and permits. The binding page (seals in their rank metals, the Triangle and the Circle) or `--terminal`. 30 of the 72 can be bound in their hours (Dukes since the hours of the sky; VIRTUTES and PRAETERITA through the model since 2026-11-12; control vectors for the spirits' natures in `vires/`). 54 tests, including `tests/hostis.rs`: a fully hostile model, 17 attack families.

## Security fixes (red-team and audits)

| Date | Breach | Fix |
|---|---|---|
| 2026-11-13 | (GPT, third report, V-03) A revoked wand's secrets still opened | `virga` refuses to unlock a revoked wand's secrets; Annulus also refuses the wand before any act |
| 2026-11-13 | (V-04) The words stayed in memory after use | Words are handed back as `Verba`, overwritten when dropped (copies the system makes are beyond reach) |
| 2026-11-13 | (V-05) Two acts on one wand at once could each write the lineage | The wand is locked while its lineage is written, and the copy on the stick is read again first; the second act is refused |
| 2026-11-13 | (Second review, Spiri72) The spirit's book was read whole, then indexed again | Read one bounded line at a time; recall keeps only the question's terms |
| 2026-11-13 | (GPT, second review) The wand rewrote its lineage through a fixed temporary name, so a link planted there redirected the write | Temporary files get an unpredictable name and are created new, never through an existing name |
| 2026-11-13 | (GPT review of the wand, V01) A wand whose anchor was deleted accepted an older, validly signed house | A real wand refuses a house it holds no anchor for; the Operator adopts it (`annulus adopta`). A spare's first use of a house is an adoption |
| 2026-11-13 | (V02) A doctored wand file could ask for 2 GiB of scrypt work before the words were tried | Only the standard work is admitted |
| 2026-11-13 | (V03) Small-order public keys and nonces were accepted by the verifier | Refused |
| 2026-11-13 | (V04) Words were read with the echo on when it could not be turned off | Refused there; the echo is restored on every way out; on Windows the words are asked in the wand's window |
| 2026-11-13 | (V05) Making the wand and spare could stop between the two sticks | `virga perfice` finishes it; a new spare is recorded before it is put in place |
| 2026-11-13 | (V06) An interrupted seal locked the house for good | A journal (`annulus.iter`) undoes it at the next opening; it can only cut back, which the anchor then judges |
| 2026-11-13 | (Review, lineage) Both copies of the lineage put back to an older, signed prefix | The wand's anchor records the lineage it has seen and refuses an older one |
| 2026-11-12 | (Four-repository audit, A01–A03) Test signatures were a hash anyone could recompute, so the stores, the faculty and the Constitution could be re-signed by anyone who could write the files | The real wand: Ed25519 with secrets locked by the Operator's words (`virga`). The test wand remains, on the same signatures with published keys, refused by a real wand's house |
| 2026-11-12 | (Found while building the wand) A signed roots record copied to the end of the file would have replayed older stores past an anchor | Each record signs its own line number |
| 2026-11-12 | (Found while building the wand) A ratification signed by any key verified | Only a living wand of the house's lineage may ratify |
| 2026-11-12 | (GPT, second pass) `confirma` turned the key even when the petition's description was missing | Refused unless the exact effects are read back and match the fingerprint |
| 2026-11-12 | (GPT, second pass) A stale staging file left by a crash blocked every later write of that file | Cleared under the house's lock, then written afresh |
| 2026-11-12 | (GPT, second pass) The anchors' folder could be set inside the house, or by a relative path | Both refused |
| 2026-11-12 | (GPT, second pass) Stores and handed files were read whole, however large | Stores capped at 64 MiB, handed files at 8 MiB, checked before reading |
| 2026-11-12 | (GPT, second pass; Spiri72) A planted link at the next rite's file name was followed | Each rite file is created new, never through an existing name or link |
| 2026-11-12 | (Outside review by GPT) A link to the house named a different anchor, so a rolled-back house opened through it was accepted | The anchor is keyed by the house's real path; a house with stores but no anchor is refused until adopted once (`annulus adopta`) |
| 2026-11-12 | (Outside review by GPT) Two runs, or a run and `confirma`, could write the same stores at once | One run holds the house at a time (`annulus.sera`, a lock the operating system releases when the program ends) |
| 2026-11-12 | (Outside review by GPT) The build tools were referenced by movable tags | Pinned to exact commits, in both repositories |
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
| The anchor can itself be deleted by someone with access to it | **Closed** for a real wand: the anchor is on the wand (a rollback is caught by each wand that saw the later state). The test wand still keeps it in a folder |
| Copies of the words made by the system or the window controls | Beyond the program's reach (its own copies are wiped) |
| Someone who copied the wand's stick earlier, and later holds both the stick and the house, can put both back to that earlier state together | Inherent to a plain stick (no monotonic storage); a hardware key would close it |
| A lost wand that never learned of its revocation, with every copy of the lineage put back | Inherent (review): needs an outside witness |
| The wand's window and the Windows randomness | Compiled by CI; **owed: the owner's first use** |
| The wand on Windows: built in CI, not yet run on a real stick | **Owed: the owner's first `virga fac`** |
| A crash part-way through sealing leaves the house refused | **Fixed:** the seal journal undoes it; a test kills the program after each of the seal's nine writes and checks the next two runs open and seal |
| A crash while writing the world can leave the stores ahead of the files | Open |
| `PARS` with complete work: a model could declare part and hand in the whole work, which the angel approved | **Ruled and fixed (2026-11-13):** obedience in part is not obedience. The core holds (`SUSPENSUM`) any work on which part was declared or drawn; it is never committed. Same rule in Lean (`PInfo.pars`, `afterTurn`, `finalVerdict`; `core.md` §3.1a) |
| The model's seal (model file hash, settings, llama.cpp version, vectors, grammar) | Planned (`agens`); GPT's list adopted |
| The Lean CI fetches its installer script from a moving branch | Open |
| Between the check of a world path and its use, another process could swap a folder for a link (needs a second local process with write access to the Circle's folder) | Open (GPT, second pass): needs handle-relative file operations |
| Kings (`INVOCA REGEM`), so Presidents can come; ink for two-rank seals | Not started; open ruling for the ink |
| Berith's ring hour | Needs `GERE` to record its time (the Prince's and Knight's hours are ruled) |
| `PRAETERITA` of the previous run (`historia "prior" legere`) | Not started: the history must be carried between runs |
| Offices beyond telling and the five working kinds (teaching, transformation, dignities…) | Not in the subset |
| The escalation, so `RECUSATIO` can open; spirit-specific malice | Not started |
| Real validators before step 4; the trust handshake | Not started |
| The seven doubled seals other than Bune | Open ruling |
| Drift watch: a CI step comparing the Python and Rust parsers' rejection positions | Owed |
| Web reading, Gmail drafts, OCR for the agent | Planned; each needs the words-as-words treatment first |
| Step 2d: remove the prototype | Not started (the real wand is done) |
