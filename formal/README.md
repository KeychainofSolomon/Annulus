# The formal model of the core (Lean 4)

Draft 1, 2026-11-11. The last part of roadmap step 1: `docs/spec/core.md` §9 written as a Lean 4 model, with P1–P9 **stated** as Lean propositions. **Nothing is proved.** The proofs are roadmap step 3. No security theorem of Annulus is yet established.

**Type-checks** (2026-11-11): GitHub Actions builds it on every push (`.github/workflows/lean.yml`), and fails if any proof, `sorry` or axiom appears.

## Checking it

1. Install VS Code, then the extension **"Lean 4"** (by leanprover). It offers to install Lean itself (elan); accept.
2. Open the `formal` folder in VS Code. The pinned version, `leanprover/lean4:v4.19.0`, installs on first open.
3. Open `Annulus.lean`. Lean checks all four files. Red underlines are errors; the "Lean Infoview" panel shows the messages. Or, in a terminal in `formal`: `lake build`.

The model uses Lean's core library only, no Mathlib, so the check takes seconds. Yellow underlines (unused variables) are harmless.

## Files

| File | Contents | `core.md` |
|---|---|---|
| `Annulus/Basic.lean` | carriers (`Universe`), marks and their order, verdicts, destinations, statuses, powers, effects, proposals' content, the program's instructions, transition labels, the Index, grave grants, the language and mechanics interfaces | §9.1–9.3, C1–C8 |
| `Annulus/State.lean` | the state σ, the rite oracle (functional and relational), and the helper definitions the transitions use | §9.1, §3.1, §9.5 |
| `Annulus/Step.lean` | the 29 transition rules, the atomic step with its audit record, runs, reachability | §9.2, §9.3a–c |
| `Annulus/Theorems.lean` | P1–P9, the damnation law, and `AllObligations` | §8, §9.4–9.6 |

## What is concrete and what is a parameter

- **Concrete (the core):** the state, every transition and its enabling condition, marks, the commit re-check, release and abandonment, the Index and its standings, grave grants, launch checks, the audit.
- **Assumed, not modelled:** cryptography. `Mech.verify`, the fingerprints and the hash chains are parameters; signature checks are real conditions of `launch`, but the model assumes, as every proof of this kind does, that signatures cannot be forged.
- **Parameters:** the carriers (places, seals, values…), the program's semantics and its type system (`Lang`, with `WellTyped` standing for `static.md`), the cryptography and the validator interpreter (`Mech`), the simulated will (`Sim`), and the rite (`RelRite`/`Rite`, any function at all).
- **Consequence for step 3.** P1, P2, P4, P7, P8, P9 and the damnation law should be provable for **any** language, since the core alone enforces them. P3 and P5 depend on the language: P3 because the program names the invocations it uses (`static.md` §4.1, §4.7), P5 because the typing rules (`flow.md` §2) and an honest report of each value's inputs are what keep quarantined data from steering. Neither can be proved until `Lang` is instantiated with a model of `static.md` §3–4. P3 as stated covers invocations and the proposals made to them, not every Circle-bound value.

## How the spec maps to Lean

| Spec | Lean |
|---|---|
| the state σ (§9.1) | `State` |
| faculty *G*, reserved *G*ᵃ | `State.faculty`, `State.reserved` |
| a request's handed set H(*q*) | `ReqInfo.handed` |
| marks and their order (§9.3) | `Mark`, `Mark.rank`, `Mark.join` |
| a proposal's life (§9.2) | `PStatus` |
| an invocation's life | `InvStatus`, `terminate` |
| Hold(*i*) and its use | `UsesHold` |
| the rite as an oracle (§9.5) | `Rite`; relational: `RelRite`; `Rite.toRel` |
| at least as strict | `AtLeastAsStrict` |
| the validator and its fuel (§9.3a) | `Validator`, `Mech.runValidator`, `accepts` |
| `consecrate`, `consecrate-grave` | `CoreStep.consecrateOk/Refused`, `CoreStep.graveOk/Refused`, `GraveMatch` |
| examination and judgement (§9.3b) | `CoreStep.examineBy/examineNone`, `CoreStep.judge`, `finalVerdict` |
| the re-check at commit (§3.3) | `Recheck`, `CoreStep.commitDone/Outside/Await` |
| `intrude` and damnation | `CoreStep.intrude`, `standing`, `DamnationLaw` |
| atomicity, the audit | `Step` (one relation per transition, record appended) |
| obs, ≈, D (§9.4) | `Exn.obs`, `LowEq`, `Exn.declass` |
| equal initial states (§9.6) | `x.st 0 = y.st 0` in `P9` |

## Modelling choices

Each is a reading of the spec where it was silent. The auditor should check each.

1. **A failed precondition of the program's own request is a denial** (class 2): the run ends with a recorded failure (`CoreStep.deny`, `invokeRefused`), as `dynamic.md` §3 says. Consecration is the exception: its refusal is an answer the program continues with. (Before the Lean audit this choice wrongly let a refused `invoke` be caught.)
2. **Steps alternate with `schedule`.** Every worker step clears the current worker, so the scheduler chooses before each step. This makes the order of turns an explicit, recorded choice.
3. **The examiner's verdict** is `HOLD` when the author is *suspected*; counterbalances combine with the verdict by taking the lower one.
4. **Committed work marks every place it touches `quarantined`** (`flow.md` §1.3: what a spirit wrote is quarantined).
5. **Effects on invocations** (inspect, narrow, warn…) commit without a modelled change of state: only the check that they were handed is modelled.
6. **A helper's handed set** must be a subset of the handed set of the request it serves (`offices.md` §4.2), not of everything its spirit holds.
7. **Only the spirit's own worker may finish a request**, not a helper.
8. **The program performs no writes of its own**; its only touches are reads. All writes are commits.
9. **The program sees only the places it may read** (`visibleWorld`). The marks of computed values are joined by the core from the inputs the language reports (`Made.ins`, `Made.places`); that the report is honest is part of what a model of `static.md` must supply.
10. **Not modelled:** what an effect writes (effects carry no data); `carries` growing beyond the author; the signature's list of allowed verbs and a counterbalance's veto of a single effect (both only restrict, so leaving them out makes the model more permissive, not less safe); `GERE VIRGAM`; the clock, ink per invocation and escalation as values that change. Forced releases and abandonments are recorded inside the label of the step that forces them, not as records of their own.

## What formalising found, and how it was settled

Ranked most severe first. The owner ruled on 1 and 4 (2026-11-11); the rest are spec fixes made the same day in `core.md` §9 and `authority.md` §6.2a, §11, unless marked open.

| # | Where | Problem | Severity | Settled |
|---|---|---|---|---|
| 1 | §9.4, P5 | P5 was false: the observation includes the places approved work touches, which the spirit chose; and verdicts depend on content but were assumed equal | High | **Ruling: judgement releases.** Every `examine`, `judge` and `commit` is in D, as consecration is (`Exn.declass`) |
| 2 | §9.2 | A proposal judged `PARADISUS` whose invocation ended before commit would use Hold(*i*) afterwards | High | `commit` is the very next step after a `judge` to `PARADISUS` (`CommitOwed`) |
| 3 | §9.1, §3.3 | Two-key commits had no store for the confirmation | High | **Ruling: sealed petitions.** Awaiting work is sealed by the fingerprint of its effects (`commitAwait`); a confirmation given outside any run lets exactly that work commit once in a later run (`commitKeyed`, `KeyConfirmed`, `State.keys`) |
| 4 | §9.2, §6.2a | `intrude` caught an invited agent attempting delivery into another Circle or after release, against "invited agents are never damned" | High | **Ruling: yes, damned.** Only an open invocation in that Circle is an invitation (`Invited`) |
| 5 | §9.2 | Nothing defined between `examine` and `judge` | Medium | `judge` is the very next step (`JudgeOwed`) |
| 6 | §3.3, §11.1 | A failed re-check assigned `INFERNUS` after `judge` | Medium | §11.1 names the commit step's core-only moves, downwards only |
| 7 | §9.2 | Angels' work had no life | Medium | `request` covers petitions; an angel's unjudged work is abandoned at the end of the run (`endRun`) |
| 8 | §9.2, §9.6 | No ending steps; forced status unsaid; outermost Circle unclear | Medium | `step` may end the run; forced status `released`; closing the outermost Circle does not end the run |
| 9 | §9.1 | The wand was constant, yet `GERE VIRGAM` unlocks the rite secret | Medium | Exception: locked ↔ rite only, never master. Not in the model (no `GERE VIRGAM` step yet) |
| 10 | §9.2, `proscription.md` §2.3 | Suspicion arises in a run but has no field and no step | Medium | **Ruling: appended to the Index**, by the examination step, with verdict `HOLD` (`NewlySuspect`, `noteSuspicion`) |
| 11 | §9.6, P9 | Replay needs a functional rite and no intrusions | Medium | Added to P9's precondition (`P9`, `NoIntrusion`) |
| 12 | §9.3a | Raw ink has no value to act on | Low | Clause withdrawn until ink values are defined (open) |
| 13 | §9.3c | Grave consecration could raise a `clean` value's mark | Low | Precondition: `quarantined` or `consecrated` (`GraveMatch`) |
| 14 | §9.2 | `relegate` of a terminal invocation unsaid | Low | Applies only to an open invocation |
| 15 | §9.6, P8 | "Doubt" undefined | Low | No admissible examiner, or a suspected author |
| 16 | §9.5, P6(b) | Almost immediate in the model | Note | Correct as it stands |

## The Lean audit (2026-11-11)

An independent Claude agent, which had not seen the work, audited the model against `core.md` (`AUDIT_BRIEF.md`). Fourteen findings; Claude checked each against the text and all hold. The owner ruled on two. The model passes `lake build` after the fixes.

| # | Finding | Severity | Settled |
|---|---|---|---|
| A1 | **P3 was false whatever the language:** a failed commit counted the invocations its (adversarial) effects named, even closed or nonexistent ones | High | Only a commit that was **done** uses its targets (`usesInv`) |
| A2 | The core handed the program the whole world, and trusted its report of a value's inputs | Medium | The program sees only readable places (`visibleWorld`); honest reporting is named as part of the language model (choice 9) |
| A3 | A refused `invoke` could be caught, against `dynamic.md` §3 | Medium | Ends the run as a denial (`invokeRefused`) |
| A4 | The program was told `PARADISUS` before the commit's outcome | Medium | The commit step gives the final destination (`commitDone/Keyed/Outside/Await`) |
| A5 | Failed preconditions left the run stuck and unrecorded; an empty rite answer could halt it | Medium | `CoreStep.deny` (`Denied`); relational rites must be total (`RelRite.Total`, in P6) |
| A6 | A damned agent kept its open invocations in other Circles | Medium | **Ruling: cast out everywhere.** `intrude` relegates every open invocation of that seal; doubt also covers proscribed and damned authors |
| A7 | Circle identifiers could be reused, weakening P3 | Medium | Each `open` needs a fresh identifier (`openC`) |
| A8 | `wandLock` never read | Low | `launch` refuses a wand unlocked as master |
| A9 | Audit records thinner than the spec | Low | Launch records each granted place's fingerprint **and** mark; the rest noted (choice 10) |
| A10 | `carries` never grows; effects carry no data | Low | Noted (choice 10) |
| A11 | `INFERNUS` from the re-check did not end the proposal's helpers | Low | It does (`commitOutside`) |
| A12 | A held proposal outlived its spirit and could be examined after release | Low | **Ruling: abandoned at release** (`Lingers`); examination counts as use (`UsesHold`) |
| A13 | Signature verbs and counterbalance vetoes left out | Low | Noted (choice 10) |
| A14 | Dead definitions and fields | Low | Dead definitions removed; unchanging fields noted (choice 10) |

## The second Lean audit (2026-11-11)

By a different AI (ChatGPT), from `annulus-lean-audit.zip`, which had no Lean installed and took the CI log as evidence. Six findings; Claude checked each against the text.

| # | Finding | Severity | Settled |
|---|---|---|---|
| B1 | P3 covered invocations and proposals, not every Circle-bound value; `circleRite` unused | High | **Accepted in part.** The core now tracks each value's Circle (`valueCircle`, `madeCircle`, `Made.rite`), and P3 covers values used by computation and consecration (`usesVal`). `circleRite` stays: it is rite data the rite oracle reads (instruments, `TEMPESTAS`); no step changes it, since apparatus acts are not modelled |
| B2 | `UsesHold` ignored a commit's effects on invocations | High | **Accepted.** A done commit uses every invocation it targets |
| B3 | The faculty's signature was a bare proposition, not a check | High | **Accepted.** `launch` verifies the faculty's signature against the wand's key and the manifest's against the owner's (`Mech.verify`, `digestFaculty`, `digestManifest`); cryptography stated as an assumption (above) |
| B4 | P5 held part of a proposal's content fixed (a helper's handed set, the shape of a turn) | Medium | **Accepted.** P5 now fixes only the will's outcome per turn (`Choice.will`); everything else the agent does is content |
| B5 | Examination does not check the rite's pairing rules | Medium | **Declined.** Pairings are rite (`core.md` §2: "who examines"); the core checks only independence, and re-checks every commit. `core.md` §9.3b now says so explicitly |
| B6 | P9 gives prefix compatibility, not equality, for unfinished runs | Medium | **Accepted as a clarification.** Equality is impossible for runs stopped at different lengths; `core.md` §9.6 now states both parts |

## Outcomes and destinations (ruling 2026-11-11)

Work is never sent to Heaven, Purgatory or Hell. A judged proposal is `RATUM` (ratified), `SUSPENSUM` (suspended) or `IRRITUM` (void), with the same mechanics the old names had (`WorkOutcome`). Paradise, Purgatory and Hell are the standing of beings, read from the Index (`authority.md` §11.2a). Angels are in Paradise by nature: `Universe.angelic`, `damnIfMortal`, and the last clause of `DamnationLaw`.

## For step 3

Prove `AllObligations M` for every `M` whose `Lang` is a model of `static.md`. Before then: the owner's approval of step 1.
