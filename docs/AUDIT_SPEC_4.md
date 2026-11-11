# Third audit of the specification: the formal security boundary (2026-11-11)

Narrow brief: `core.md`, with only the cross-references needed to check it. Ten findings: seven high, three medium. As the owner asked, Claude first answered each one (agree, disagree, or needs a ruling) without rewriting. The owner ruled on the open points by questionnaire, then the changes were made in one pass.

**Status:** the theorems are specified for proof, not proved. No security theorem is yet established; the machine-checked proof is roadmap step 3.

| # | Finding | Response | Disposition |
|---|---|---|---|
| 1 | `flow.md` §3.1 said the rite gives the validator's answer; `core.md` §9.3a said the core runs it | agree | **Fixed:** `flow.md` §3.1 defers to §9.3a; the rite supplies only an identifier |
| 2 | Validator purity, input isolation and side effects undefined | agree | **Fixed:** §9.3a. A total function of the value and its origin record only; reads nothing else, changes nothing, same answer for same inputs; a work limit in its Constitution entry (the owner's ruling), and refusal when it is exceeded |
| 3 | No transition for external grave consecration | agree | **Fixed:** §9.3c. `grant-grave` outside any run, with the wand, appends a one-use grave grant (the owner's ruling: one use); `consecrate-grave` in a later run spends it |
| 4 | Mark order against the grave-grade steering rule | agree: the order was coherent, but A16's wording contradicted it | **Fixed:** steering defined by mark in §9.3; A16 reworded in `flow.md` and `static.md` ("a value marked `consecrated` or `quarantined`") |
| 5 | P5 needs explicit low-equivalence and declassification | agree | **Fixed:** §9.4. Observation, low-equivalence, the declassified record, and P5 stated over them; refusals count as declassified |
| 6 | P6's simulation relation ignored rite-answer choices | agree; worse than imprecise, the old statement was false (programs branch on destinations) | **Fixed by split** (the owner's ruling): P6(a), rite-parametric safety under any rite oracle; P6(b), trace inclusion for relational rites only. The verdict-order claim is withdrawn |
| 7 | One universal precondition for every theorem | agree | **Fixed:** §9.6, a precondition for each |
| 8 | Scheduler state omitted | agree | **Fixed:** §9.1 adds the scheduler (entropy position, simulated clock, chosen order; the real order in real-agent mode) and the store of grave grants |
| 9 | The examiner interface did not cover angels' commits | agree | **Fixed:** §3.1 examiner row and §9.3b. The rite supplies the examiner's identifier for any author; independence stated; no admissible examiner means `HOLD` |
| 10 | No irreversible agent-level condemnation, against the intention for Hell | **needed the owner's ruling.** Claude also owned that the "no agent is condemned" rule, written in the previous round, was his own choice, made without a ruling | **Ruled:** intruders only are **damned**, irreversibly; by the recorded fact of intrusion alone; by seal only (look-alikes are suspected, never damned). Changed: `core.md` §9.2; `authority.md` §6.2a, §11.2, §11.3; `proscription.md` §1, §2.3, §3.1, §3.4; `operator.md` §4.1, §8.2; `HANDOFF.md`. The sources' "the magician never condemns" stays true: the Constitution's law acts on a fact |

**Found while fixing:** the theorems were named T1–T9, the same as the type-error codes in `static.md` §6. They are renamed **P1–P9** (proof obligations). The audit records keep the old names as history.
