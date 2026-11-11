# Fourth audit: the formal security boundary, re-audited (2026-11-11)

Verdict: not yet ready for machine-checked proof. The third audit's ten findings were substantially addressed, and the auditor found no route to escalate authority, only definitions not yet sharp enough, plus one genuine contradiction. Ten findings: five high, five medium. Claude checked each against the text: all ten hold, and none needs a constitutional ruling.

| # | Finding | Severity | Disposition (`core.md` unless stated) |
|---|---|---|---|
| 1 | Proof certificates gave a second way to lower a mark (`flow.md` §3.4 implied the angel consecrates) | High | **Fixed:** checking a certificate *is* the step `consecrate(v, k)`, with the checker as validator *k*; no verdict lowers a mark (`flow.md` §3.4, `library.md` §3.1, §9.3a) |
| 2 | A proposal's lifecycle incomplete: completion, races, cancellation | High | **Fixed:** §9.2, statuses `pending`, `complete`, `judged d`, `abandoned`; `examine` only on complete (or Purgatory); the Operator's worker waits; unjudged work is abandoned when its invocation ends |
| 3 | P4: "held" and "used" undefined; repeated release | High | **Fixed:** §9.2 defines Hold(*i*), the single terminal step and "use"; a later release is a no-op, not a second ending |
| 4 | P9: initial conditions incomplete | High | **Fixed:** equal initial states, including the world, captured at launch by fingerprints in the audit's first record (§9.6) |
| 5 | Wand identity and revocation missing from the state | High | **Fixed:** §9.1, the wand's fingerprint, the revocation log, the manifest check and the Index root; checked at launch, constant afterwards |
| 6 | P5's observation silent on termination, failure and divergence | Medium | **Fixed:** how a run ends is observed; runs that never end are compared by finite prefixes (§9.4) |
| 7 | Grave grant: no atomicity | Medium | **Fixed generally:** every transition is atomic with its records; recovery at the next launch from the audit, the Index and the store (§9.2) |
| 8 | Validator work limit undefined | Medium | **Fixed:** fuel, counting evaluation steps, memory and called validators against one budget; validators call only validators (§9.3a) |
| 9 | P6(b) needed a trace definition | Medium | **Fixed:** the trace is the observation plus commits, without rite answers; same initial state and non-rite choices (§9.5) |
| 10 | Intrusion and damnation needed one operational definition | Medium | **Fixed:** the step `intrude(a, c)`, on the **attempt** detected at the entrance, atomically refuses, seals, records and damns; `authority.md` §6.2a now says "reaches, or attempts to reach". Interpretation, flagged to the owner: since messages are refused at the entrance, nothing uninvited ever *reaches* a Circle, so the attempt is the only fact the ruling can apply to |

**Next, proposed by Claude:** rather than a fifth prose audit, write the core's formal model in a proof assistant (Lean) as the last part of step 1, and audit that model against the spec. Awaiting the owner's choice.
