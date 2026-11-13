# Audit 2: the changes since core audit 1, and the open branches

Done on 2026-11-13 by a reviewer that had not written the work. Range: `2d72b19..1334798` on `main` (22 commits, the first audit's fixes included), and the open branches `acta-signata` (`a84b4c1`), `forma-officii` (`7ac906c`) and `doctrina` (`898c1f6`). Also read: 72Virga `main` and `fenestra-alba`, and the recent commits of Spiri72, 72Spirits and Music72 where they touch Annulus.

Evidence: the code, the specification, the Lean model, and tests. On `main`, 320 Rust tests pass (Linux, Rust 1.97.0). Lean was not available locally; the Lean workflow was run on this branch, and its log is the evidence for the proofs below. Failing tests are written for every finding that could be confirmed. On this branch they are marked `#[ignore]`, so CI stays green until a fix un-ignores them. Nothing else was changed.

## Findings

| # | Severity | Where | Finding | Evidence | Suggested fix |
|---|---|---|---|---|---|
| 1 | Medium | `main`: `mandata::petitiones`; `acta-signata`: `mandata::acta` | **A reading command rewrites a house that another run holds.** `petitiones` (and `acta` on its branch) calls `Domus::open` without taking `annulus.sera`. `open` is not read-only. It runs `recover`, which cuts the stores back to a journal's lengths and deletes the journal. With a real wand it also writes the lineage and the anchor. If it runs while another run is sealing, it can cut back the stores mid-seal. The sealing run then signs a roots line over chains that are no longer on disk, and the house is refused from then on, with no journal left to repair it. | `tests/ritus_cli.rs`, `audit2_petitiones_does_not_touch_a_house_another_run_holds`. It holds the lock, plants a seal in progress and runs `petitiones`, and the journal is gone. | Take the lock in every command that opens a house. Or give `open` a read-only mode that refuses, rather than repairs, while a journal exists. |
| 2 | Medium | `main`: `horae::tenta`, `ritus.rs`, `--tentus` | **A hold is believed on the Operator's word.** `--tentus N@T` makes the hours and the Moon those of `T`, for any `T` in the past at which the spirit could have been called. Nothing records that spirit N was ever called at `T` and kept with `MANE`. `MANE` is written only to the audit, which `main` neither signs nor reads back. The test `a_held_spirit_keeps_the_hours_of_its_calling` admits Gusion two days after a "calling" that never took place in any run. By §1.6 ("what is not granted is denied"), an unrecorded hold grants a calling outside the hours. | Existing tests in `tests/lingua_horae.rs`. Spiri72 passes the flag from its in-memory record, so the window is safe. The command line is not. | Keep each `MANE` in the sealed stores (spirit, Circle, hour of calling), cleared by licence or banishment, and admit `--tentus` only when it matches. Or read it from the previous run's signed audit once `acta-signata` lands. |
| 3 | Medium | `doctrina`: `doctrina.rs`, `gcd` | **A spirit's lesson can abort the run.** `gcd` computes `a % b`, which panics for `i128::MIN % -1` in every build mode. The release profile has `panic = "abort"`, so the process dies before the audit is written. A certificate line `numerus: (0-2)^63*2^64/(0-1) = 1` reaches it. Every step before it is checked arithmetic. This is the same class as the JSON-nesting fix of 2026-11-12. | A test in `tests/audit2_doctrina.rs` (on a copy of `doctrina`). `claim`, `solve` and `verdict` all panic at `doctrina.rs:189`. | Use `checked_rem` in `gcd`. Or refuse `i128::MIN` in `Q::new`. Add the line above as a test. |
| 4 | Medium | `acta-signata`: `digest_acta`, `read_audit` | **An audit is bound to a number, not to a house or a seal.** The signature covers the seal number, the wand and the body. Two audits pass as "bound to seal *n*" when they should not. The first is seal *n* of another house kept by the same wand. The second is the audit of a run whose seal failed: `next_seal` is taken before sealing, so the next run's seal *n* makes the stale audit verify. Once `PRAETERITA` reads "the previous run", either can be shown in its place. | By reading: `digest_acta(seq, master, body)` holds no house identity and no roots hash. | Put the house's identity (the anchor key: the hash of its real path) and the hash of roots line *n*−1 into `digest_acta`. Then check both in `read_audit`. |
| 5 | Medium | `doctrina`: `Goetia::quaestiones` | **The rite's verdict depends on something outside the state.** The question is kept in a map inside the rite (`quaestiones`), which is filled by the evaluator and is not part of `ReqForm`, the request label or the audit. In Lean, `RelRite.verdict` is a relation on `σ`, so the model no longer describes the code. Replaying a run from its audit cannot reproduce the verdict (P9). This is the brief's check 2: something that affects the next step lies outside `State`. | By reading `ritus.rs` and `exsequi.rs` on the branch, against `Step.lean` (`examineBy`, `hv`). | Carry the question in `ReqForm`, so it is recorded with the request. It still chooses nothing that is touched (A14). |
| 6 | Low | `forma-officii`: `step.rs` `turn`; `State.lean` `wrongForm` | **The core's form check lets an empty answer through.** `Idle` with `finish` leaves the content `Empty`. `wrongForm` is false for it, so under a rite that approves, the core gives `RATUM` with nothing done, for either class of office. That is the case finding 4 of core audit 1 named. The goetia rite holds `Empty`, so no run on `main` is affected. The claim "enforced in the core, whatever the rite says" is not yet true. | Two tests in `tests/nucleus_rules.rs` (on a copy of `forma-officii`): both give `Ratum`. | Count `Empty` at completion as wrong form in Rust and in Lean (`wrongForm`, or `misformed` set at `fin` when the content is `.empty`). |
| 7 | Low | Lean: `KeyConfirmed`, `Initial` | **The model allows one confirmation to serve twice.** Rust now refuses a second `confirma` of the same petition. In Lean, confirmations arrive only in the initial `keys`, and nothing there forbids two `.confirmed fp` entries. With two such entries, two `commitKeyed` steps can spend different indices for the same work. No theorem states "one confirmation, one commit". | By reading `State.lean`, `Step.lean` and `Theorems.lean`. | Add the condition to `Initial` (each `.confirmed fp` follows a `.petition fp`, and none is doubled) and state the once-only law. |
| 8 | Low | All four associated repositories | **Commits carry Claude authorship and co-author lines**, against the handoff's rule ("commits carry no co-author or session lines", author `Solomon`). Annulus has 10 commits by `Claude` and 18 with `Co-Authored-By` lines. Every commit on the three open branches is one of them. Spiri72, 72Virga and 72Spirits have the same. | `git log --all --format='%an %b'`. | Before the open branches are merged, rewrite their authors and drop the trailers. Then set the commit identity in every clone, as `HANDOFF.md` says. |
| 9 | Note | `acta-signata`, `forma-officii`: `Cargo.toml` | Annulus pins 72Virga `6d0157b`, which so far exists only on 72Virga's branch `fenestra-alba`. | `git branch -r --contains 6d0157b` in 72Virga. | Merge `fenestra-alba` in 72Virga before these branches are merged here. |
| 10 | Note | 72Virga: `tests/vectors.rs` | Every primitive is hand-written. Ed25519, SHA-2, HMAC and scrypt are checked against RFC and FIPS vectors, and verification refuses non-canonical points, S ≥ L and small-order keys and nonces. ChaCha20-Poly1305 has only an independently generated vector. | Read `ed25519.rs` and `aead.rs`; 24 tests pass. | Add the RFC 8439 §2.8.2 vector. |

## What was found sound

- **The PARS rule:** Rust (`7b7d187`) and Lean (`9278338`) agree. The flag is set by the turn and is never cleared. Examination holds the marked work, and judgement keeps the flag. It is now proved (below).
- **Declared outcomes:** in real mode the record holds only the declaration, a declaration is limited to obedience, part and delay, and `kill` is limited to simulated mode. Rust and Lean agree, and the proofs of 2026-11-12 cover it.
- **`confirm_petition`:** it confirms only the latest pending petition for a fingerprint, so a confirmation cannot be repeated, and a spent key cannot be confirmed again.
- **The anchor, the journal and the lineage:** opening with a real wand checks the anchor, a numbered roots line and the lineage's length and head. The journal can only cut back, so a planted journal can do no more than roll the house back, and the anchor refuses that. A crash at any of the nine points of a seal leaves a house that opens (`virga_cli`).
- **72Virga's lineage:** each act is signed and chained. A revoked wand can no longer act, and diverged copies are refused.
- **`MANE`:** it releases the invocation exactly as licence to depart does. Only the history's word differs, so A9 and the closing of the Circle are unchanged.

## Lean: the commits and the model

| Commit | Touches the core | Lean changed with it | Lean CI |
|---|---|---|---|
| `9ec5b46` declared outcomes | yes | yes (`Will`, `declarable`, `kill` simulated only) | pass |
| `c09f227` core audit 1 fixes | yes (stores, rite) | no: all of it lies outside the model (stores, rite) | pass |
| `d04ee3a` the real wand | yes (`verify`) | no: `verify` is a parameter of the model | pass |
| `9ecb657` `MANE` | yes (`RiteAct::Mane`) | no: acts are rite data | pass |
| `7b7d187` PARS | yes | **no**; Lean followed in `9278338`, 3 minutes later | pass |
| `7ac906c` (`forma-officii`) form of the office | yes | yes (`telling`, `misformed`) | pass; see finding 6 |
| `35e3eb0` (`acta-signata`) signed audit | no (stores) | — | pass |
| `898c1f6` (`doctrina`) | no (rite) | — | not run; see finding 5 |

### Proved on this branch (`formal/Annulus/Proofs.lean`)

All four rest only on Lean's own axioms (`propext`, `Classical.choice`, `Quot.sound`), as the workflow's axiom check confirms.

- `part_turn_marks`: a turn whose outcome is part leaves its proposal marked.
- `pars_examined_held`: an examination of a marked proposal holds it, whatever the examiner says.
- `pars_judged_suspensum`: the judgement after that examination is `SUSPENSUM`, so a commit step, which needs `RATUM`, cannot follow.
- Lemmas: `will_of_outcome`, `afterTurn_props`.

The same three lines that prove `pars_examined_held` prove its counterpart for `misformed` on `forma-officii`, once finding 6 is settled.

### Still owed

These are unchanged from the first audit: P1 (c) and P2–P9, the correspondence of Rust to Lean, and the six drifts of core audit 1, #8.
