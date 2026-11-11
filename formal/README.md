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
| `Annulus/Step.lean` | the 27 transition rules, the atomic step with its audit record, runs, reachability | §9.2, §9.3a–c |
| `Annulus/Theorems.lean` | P1–P9, the damnation law, and `AllObligations` | §8, §9.4–9.6 |

## What is concrete and what is a parameter

- **Concrete (the core):** the state, every transition and its enabling condition, marks, the commit re-check, release and abandonment, the Index and its standings, grave grants, launch checks, the audit.
- **Parameters:** the carriers (places, seals, values…), the program's semantics and its type system (`Lang`, with `WellTyped` standing for `static.md`), the cryptography and the validator interpreter (`Mech`), the simulated will (`Sim`), and the rite (`RelRite`/`Rite`, any function at all).
- **Consequence for step 3.** P1, P2, P4, P7, P8, P9 and the damnation law should be provable for **any** language, since the core alone enforces them. P3 and P5 depend on the typing rules: they cannot be proved until `Lang` is instantiated with a model of `static.md` §3–4 and `flow.md` §2.

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

1. **A failed precondition is a recorded refusal**, not a stuck step, for `invoke`, `consecrate` and `consecrate-grave`. The program continues on its refusal branch.
2. **Steps alternate with `schedule`.** Every worker step clears the current worker, so the scheduler chooses before each step. This makes the order of turns an explicit, recorded choice.
3. **The examiner's verdict** is `HOLD` when the author is *suspected*; counterbalances combine with the verdict by taking the lower one.
4. **Committed work marks every place it touches `quarantined`** (`flow.md` §1.3: what a spirit wrote is quarantined).
5. **Effects on invocations** (inspect, narrow, warn…) commit without a modelled change of state: only the check that they were handed is modelled.
6. **A helper's handed set** must be a subset of the handed set of the request it serves (`offices.md` §4.2), not of everything its spirit holds.
7. **Only the spirit's own worker may finish a request**, not a helper.
8. **The program performs no writes of its own**; its only touches are reads. All writes are commits.

## What formalising found (for the auditor and the owner)

Ranked most severe first. "Model" says what the Lean does meanwhile.

| # | Where | Problem | Severity | Suggested fix | Model |
|---|---|---|---|---|---|
| 1 | §9.4, P5 | **P5 as stated is false.** The observation includes the places commits touch. A work proposal's effects are its content. Two runs alike except that an approved proposal writes file A in one and file B in the other (both handed) meet every hypothesis, and their observations differ. Second, verdicts depend on content, but P5 assumes "the same rite answers", so the verdict channel is assumed away rather than accounted for in D | High | Count examination as declassification: add each verdict and each committed effect list to D (the "robust declassification" reading), or restrict obs to the program's own decisions and touches. Needs the owner's ruling | `P5` states the spec's version; finding stands |
| 2 | §9.2, proposal's life | A proposal judged `PARADISUS` whose invocation ends before it commits: the life says nothing. Committing it later would use Hold(*i*) after the terminal step (P4) | High | Commit immediately follows a `judge` to `PARADISUS`, as `judge` follows `examine` | `CommitOwed` forces it |
| 3 | §9.1, §3.3, `operator.md` §5.2 | Two-key (high-consequence) commits need a confirmation "outside a session", but §9.1 has no store for such confirmations and no step uses one. So within a run they can never commit | High | A store of one-use commit confirmations, like grave grants, or rule that such proposals wait in `PURGATORIUM` for a later run | commit result `awaitsKey` → `PURGATORIUM` |
| 4 | §9.2 damnation, `authority.md` §6.2a | `intrude`'s condition, "other than through an invocation the Operator made there", also catches an invited agent attempting delivery into **another** Circle, or after its release. That contradicts "invited agents are never damned" | High | the owner rules: is an invited agent's out-of-channel attempt intrusion? | literal reading: not invited **in that Circle**, live |
| 5 | §9.2, `judge` | Status between `examine` and `judge` undefined; "immediately after" does not say what happens if anything else intervenes | Medium | Say that nothing else may occur between them | `JudgeOwed` forces it |
| 6 | §3.3, `authority.md` §11.1 | A failed re-check sends the proposal to `INFERNUS` after `judge` gave `PARADISUS`, yet only `judge` assigns destinations | Medium | Name the re-check's outcome in §11 as a second, core-only assignment, or have the commit step record a refusal without a destination | `commitOutside` sets `judged INFERNUS` |
| 7 | §9.2 | Angels' work has no life: no transition creates an angel's proposal (there is no petition step), and nothing ends it when a Circle closes. P4 does not cover angels | Medium | Add `petition` to §9.2 and say when an angel's work ends | `request` to an angel; never abandoned |
| 8 | §9.2, §9.6 | No transition ends a run normally or by failure; which terminal status (`released` or `relegated`) `close`, `kill` and the end of a run force is unsaid; whether closing the outermost Circle ends the run is unclear | Medium | Add the ending steps; name the forced status | ends are results of `step`; forced status `released`; closing does not end the run |
| 9 | §9.1, `operator.md` §8.2 | The wand's state is "constant for the rest of the run", yet `GERE VIRGAM` needs the rite secret unlocked during the rite | Medium | Allow exactly one change, locked ↔ rite, and keep master unreachable | constant; `GERE VIRGAM` not modelled |
| 10 | §9.2, `proscription.md` §2.3 | Suspicion is produced during a run (a new seal whose actions match a mark), but no field holds it and no step records it; §9.2 says only `intrude` changes the Index in a run | Medium | Add suspicion to the state, or a step that appends it | read only from entries made outside runs |
| 11 | §9.6, P9 | Replay needs (a) a functional rite, not a relational one; (b) intrusions excluded or scripted, since they are outside input; (c) a fixed rule for when core steps (`judge`, `commit`) happen | Medium | State these in P9's precondition | `P9` over `Rite`, with `NoIntrusion` |
| 12 | §9.3a | "Raw ink becomes consecrated ink": ink is a budget (a number), and no value is raw ink, so the step has nothing to act on | Low | Define ink values, or drop the clause | omitted |
| 13 | §9.3c | `consecrate-grave`'s precondition does not require the value's mark to be above `consecrated-grave`; on a `clean` value it would raise the mark | Low | Require `quarantined` or `consecrated` | required |
| 14 | §9.2 | A later `release` of a terminal invocation is a no-op, but a later `relegate` is unsaid | Low | Say it is a no-op too, or not enabled | not enabled |
| 15 | §9.6, P8 | "Under doubt" is not defined | Low | Define doubt: no admissible examiner, a suspected author, or an examiner's `HOLD` | the first two |
| 16 | §9.5, P6(b) | In the model it is almost immediate: the same run works under the laxer rite. That is correct, and shows how little (b) claims | Note | none | `P6b` |

## For step 3

Prove `AllObligations M` for every `M` whose `Lang` is a model of `static.md`. Before then: the owner's rulings on findings 1 and 4, the spec fixed for 2–3 and 5–15, and this model audited by someone who reads Lean.
