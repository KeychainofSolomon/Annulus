# Audit brief: Annulus specification, re-audit of the formal security boundary

You are re-auditing one part of the Annulus specification: **the formal security boundary**, after the third audit's ten findings were fixed. Find errors; do not rewrite. The audits and their dispositions are in `docs/AUDIT_SPEC_1.md` to `docs/AUDIT_SPEC_4.md`. Everything outside the scope below has been audited twice; do not report on it.

**Status to keep in mind:** the theorems (now named **P1–P9**; they were T1–T9) are specified for proof, not proved. No security theorem is yet established; the machine-checked proof is roadmap step 3. Judge whether they are stated precisely enough to prove, not whether they are true.

**Rulings to respect** (the owner, 2026-11-11): intruders, and only intruders, are damned irreversibly, by the recorded fact of intrusion and by seal only; P6 is split into rite-parametric safety and trace inclusion for relational rites; grave grants are one-use; each validator's work limit is fixed in its Constitution entry.

## Scope

1. `docs/spec/core.md` in full, especially §3.1, §8 and §9 (the state, transitions and agent states, marks, validators, examiners, grave consecration, non-interference, the rite and P6, the preconditions).
2. Its cross-references, only as far as needed to check them: `flow.md`; `offices.md` §1, §3, §4; `authority.md` §1.5, §2.5, §6.2a, §11, §12.1a; `operator.md` §4, §5, §8; `concurrency.md`; `proscription.md`; `static.md` §3.3 and §6 (A14–A16).

## Check

1. Has each of the third audit's ten findings been resolved, without creating new contradictions?
2. Completeness of the state (§9.1): can anything that affects a run's next step lie outside it?
3. The interface (§3.1): can any rite answer, validator, examiner, agent or library grant authority, skip examination, or lower a mark other than through §9.3a or §9.3c?
4. Each theorem: stated over defined objects, with the precondition §9.6 gives it, and free of hidden assumptions about the rite, the will, proposal content or the order of turns?
5. Damnation: consistent everywhere it is used, irreversible, entered only by the recorded fact of intrusion?
6. Cross-references: every reference in the scope points to a section that says what is claimed.

## Report format

A table with the columns: file and section | problem | evidence | severity (high/medium/low) | suggested fix.
