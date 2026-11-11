# Second audit of the specification (GPT, 2026-11-11)

Brief: `AUDIT_BRIEF.md`. The auditor confirmed the first audit's fixes (Rudd's kings correctly still open), ran the 69 grammar programs (69/69) and the register build (no changes), and found the guiding principle respected in the reserved angel grants, `DIGNITATES`, helpers, freed familiars, runtime-only commits and Andras's danger. Verdict: not yet ready for approval; the remaining concerns concern the formal security boundary and precision.

Each finding was checked against the text before anything was changed.

| # | Finding | Severity | Checked | Disposition |
|---|---|---|---|---|
| 1 | Consecration was not part of the core interface; the types covered only consecrated ink | High | confirmed | **Fixed:** a row in `core.md` §3.1, with the exception stated openly: consecration may lift the quarantine (never add authority), so T5 holds up to consecration. Marks added to the types (`static.md` §3.3); `flow.md` §3.1 points to the interface |
| 2 | T1–T9 not formal enough to prove | High | confirmed | **Fixed:** `core.md` §9 defines the state, the transitions, the terminal states of an invocation (T4), the order of marks, non-interference with declassification (T5), the order of rites (T6), and the form of every theorem |
| 3 | Bethor: 42 in the spec, 32/186 in the handoff | Medium | **partly**: the spec, the source log and the provenance rule agree on 42. The handoff's old "corrections" note was stale and wrong about the printed total (the page prints 196) | **Fixed:** the handoff note now records what is printed and what is intended. No source recheck needed |
| 4 | `CONCORDIA`'s merge not deterministic | Medium | confirmed | **Fixed:** `offices.md` §2.3a. The proposal holds the whole merged content and a resolution map; agreement kept; conflicts mapped; simulated mode: first source wins |
| 5 | The LL(1) claim exceeds what the tool checks | Medium | confirmed | **Fixed:** the claim is limited in `grammar.md`; an independent FIRST/FOLLOW analysis is carried to step 2 (`ROADMAP.md`) |
