# Audit brief: Annulus specification, third audit (narrow)

You are auditing one part of the Annulus specification: **the formal security boundary**. Find errors; do not rewrite. The earlier audits and their dispositions are in `docs/AUDIT_SPEC_1.md`, `docs/AUDIT_SPEC_2.md` and `docs/AUDIT_SPEC_3.md`. Everything outside the scope below has been audited twice; do not report on it.

**Status to keep in mind:** the theorems are specified for proof, not proved. No security theorem is yet established; the machine-checked proof is roadmap step 3. Judge whether they are stated precisely enough to prove, not whether they are true.

## Scope

1. `docs/spec/core.md` in full, especially:
   - §3.1, the interface between the rite layer and the core, including consecration;
   - §8, theorems T1–T9;
   - §9, the formal model: the state (§9.1), the transitions and agent-level states (§9.2), the marks (§9.3), validators and the consecration step (§9.3a), non-interference (§9.4), the order of rites and the will (§9.5).
2. Its cross-references, read only as far as needed to check them:
   - `docs/spec/flow.md` (marks, quarantine, consecration);
   - `docs/spec/offices.md` §1, §3, §4 (handed sets, containment, helpers);
   - `docs/spec/authority.md` §1.5, §2.5, §6.2a, §11;
   - `docs/spec/operator.md` §4, §5, §8 (the second key and the wand);
   - `docs/spec/concurrency.md` (workers, commits one at a time);
   - `docs/spec/proscription.md` §2–3.

## Check

1. **Completeness of the state:** can anything that affects a run's next step lie outside §9.1?
2. **The interface:** can any rite answer, any validator, any agent or any library grant authority, skip examination, or lift a mark other than through the step of §9.3a?
3. **Each theorem:** is it stated over defined objects, with explicit preconditions, and free of hidden assumptions about the rite, the will, proposal content or the order of turns?
4. **The order of rites:** does §9.5 cover every question of §3.1, and does T6 follow from it as stated?
5. **Non-interference:** is §9.4 well defined given §9.3 and §9.3a, including across files and runs (`flow.md` §1.3)?
6. **Proposals and agents:** is the separation of destinations (proposals) from agent states (invocation status, Index standing) consistent everywhere it is used?
7. **Cross-references:** every reference in the scope points to a section that says what is claimed.

## Report format

A table with the columns: file and section | problem | evidence | severity (high/medium/low) | suggested fix.
