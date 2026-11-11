# Review before the third audit (2026-11-11)

The owner's review of the second audit's corrections. He reran the 69 grammar programs and the register build independently (identical). Verdict: the second audit's corrections are accepted provisionally; a third audit is required; **no security theorem is yet established**.

| # | Matter | Disposition |
|---|---|---|
| 1 | Is the state of `core.md` §9.1 complete? | **Fixed:** §9.1 now states it is complete, and adds the execution state of every worker (control position, call stack, environment, memory, by reference to the ordinary evaluation rules), the workers' queues and turns, proposals' pending effects, the world (content and marks of granted places; nothing outside *G* is in the state, since nothing can reach it), launch parameters and Circle effects |
| 2 | §9.5 does not compare the will's draw | **Fixed:** the will and proposal content are adversarial choices in the model; the rite's part is only which outcomes are open, and the relation compares those as subsets. T6 is stated for the same choices under both rites |
| 3 | Validator identity, independence, authentication, and the consecration step | **Fixed:** `core.md` §9.3a. Validators are deterministic Constitution entries, never agents; the rite supplies only an identifier; the core authenticates the fingerprint against the wand-signed manifest, runs the validator itself, and records it. The step `consecrate(v, k)` has an exact precondition and effect |
| 4 | A proposal sent to Hell versus an agent condemned to Hell | **Fixed:** destinations belong to proposals only. Agents have only invocation states and Index standing; no agent is irreversibly condemned, which follows the sources (`authority.md` §11.3). `authority.md` §6.2a's "go to Hell" is defined accordingly |
| 5 | Specified for proof is not proved | **Fixed:** `core.md` §8 opens with a status statement: specified, not proved; nothing may be cited as a guarantee until step 3 |
