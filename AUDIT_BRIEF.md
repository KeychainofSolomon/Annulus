# Audit brief: the Lean model of the Annulus core

You are auditing `formal/`, a Lean 4 model of `docs/spec/core.md` §9, against the specification. Find errors; do not rewrite. The model states P1–P9; it proves nothing, and you should not try to prove anything. Judge whether the Lean says what the spec says, and whether each statement is one that could be proved.

**Before you start:** the model must type-check (`lake build` in `formal/`). If it does not, report the errors and stop.

**Read first:** `formal/README.md`, especially "Modelling choices" and "What formalising found". Those findings are known; do not report them again, but do report if one is wrong.

## Scope

1. `formal/Annulus/*.lean` in full.
2. `docs/spec/core.md` §3.1, §8 and §9, as the reference. Other spec files only where `core.md` points to them.

## Check

1. **Fidelity.** Does each transition rule in `Step.lean` have the enabling condition and effect §9.2–9.3c give it? Is anything in the spec missing from the model, or in the model without a source in the spec or the modelling choices?
2. **The state.** Can anything that affects the next step lie outside `State`? Is any field never read or never written, and does that matter?
3. **The interface.** Does the rite (`RelRite`) reach the core only through the questions of §3.1, and can any answer grant authority, skip examination or lower a mark other than through the consecration steps?
4. **The statements.** Is each `Pn` over the right objects, with the precondition of §9.6 and no more? Is any vacuous (true because a hypothesis can never hold) or trivially false?
5. **Proof readiness.** Which statements depend on the abstract language (`Lang`)? Is the split claimed in the README (P3 and P5 need the typing rules, the rest do not) right?

## Report format

A table with the columns: file and definition | problem | evidence | severity (high/medium/low) | suggested fix.
