# First audit of the specification (GPT, 2026-11-11)

Brief: `AUDIT_BRIEF.md`. The auditor ran the 64 grammar programs (64/64) and the register build (byte-identical output), and found no rule that lets a spirit or a program grant authority, approve its own work or mint its own ink. It calls the spec "a credible early draft", not yet an unambiguous implementation contract, and recommends a second audit before approval.

Each finding was checked against the text before anything was changed.

| # | Finding | Severity | Checked | Disposition |
|---|---|---|---|---|
| 1 | `dynamic.md` §5.1 and `authority.md` §8.3 still carried the old four-spirit king list, contradicting the quarter ruling in `static.md` §4.5b | High | confirmed | **Fixed:** both now give the quarter rule and the six Presidents with a stated quarter |
| 2 | Examination and judgement not consistently separated: `static.md` §3.3 and `grammar.md` §5 said `EXAMINA` yields a destination; `dynamic.md` §6.1 said it gives a verdict | High | confirmed | **Fixed:** `EXAMINA` performs two acts of the Constitution in order: examination (a verdict, recorded, never a program value), then judgement (the destination, returned). `static.md` §4.10, §3.3; `grammar.md` §5; `dynamic.md` §6.1. **Ruled by the owner 2026-11-11** |
| 3 | `rudd` inherited the Goetia's kings although Rudd's Goetia is a separate work, and `reges.md` has no Rudd entries | Medium | confirmed | **Open; the owner's ruling 2026-11-11: wait for the manuscript.** Marked provisional in `static.md` §4.5b and listed in §7. Needs a reading of Harley 6483, or the owner's ruling that the `goetia` readings apply |
| 4 | `NOMEN`, `SPIRITUS` and `ANGELUS` were not forbidden outside a Circle, though §4.7 binds entries and divine names to their Circle (the audit cited "authority.md §4.7"; the rule is `static.md` §4.7) | Medium | confirmed | **Fixed:** added to `static.md` §4.1; error A1 widened. The grammar tests check syntax only, so `tests/grammar/accept/names_and_entries.annulus` stays valid as syntax |
| 5 | `static.md` §4.6 said the office mapping was still to be written | Low | confirmed | **Fixed:** now points to `spirits.md` §6 and the new `offices.md` |
