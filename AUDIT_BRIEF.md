# Audit brief: Annulus specification, second audit

You are auditing the Annulus language specification. Annulus is a programming language and also a faithful simulation of Solomonic magic. Find errors; do not rewrite. The first audit's findings and their disposition are in `docs/AUDIT_SPEC_1.md`; check that its fixes hold.

## The rules the work must follow

1. **Source precedence:** within one work's line of copies, the oldest witness wins. Separate works (rites) never override each other. A later addition stands if it conflicts with nothing older.
2. **Labels:** every rule is tagged **[S]** (sourced, with its witness) or **[O]** (original to Annulus). The owner's rulings are tagged **M** or "ruling" with a date. Witness codes:
   - **W** Weyer 1577
   - **S** Sloane 3825
   - **G** the 1904 edition
   - **F** Folger V.b.26
   - **A** Ad. 36674
   - **K** BnF Ital. 1524 (1446)
3. **Analogues:** a traditional detail is kept only if it has a mechanical analogue in the language.
4. **The guiding principle:** no program's authority can exceed its grant, at any depth of delegation (`authority.md` §1.5). Where tradition and the principle collide, the principle wins and the collision is raised with the owner.
5. **Rulings in force:** recorded in `HANDOFF.md`, including those of 2026-11-11 (office signatures, the core/rite split, angels' reserved grants, Andras's danger, simulated spirits, information flow, the powerbox, actors, the Index, the wand, the `.annulus` extension).

## Read in this order

1. `docs/spec/core.md`: the core, the rite layer, the interface, the proof obligations (Part 0)
2. `docs/spec/authority.md`: the authority model (Part 1)
3. `docs/spec/spirits.md`: offices, will, malice, apparatus (Part 1a)
4. `docs/spec/operator.md`: the discipline of the Operator, and the wand (Part 1b)
5. `docs/spec/offices.md`: office signatures and containment (Part 1c)
6. `docs/spec/grammar.md`: lexical grammar and EBNF (Part 2)
7. `docs/spec/flow.md`, `library.md`, `concurrency.md`, `proscription.md` (Parts 2a–2d)
8. `docs/spec/static.md`: compile-time rules (Part 3)
9. `docs/spec/dynamic.md`: runtime rules (Part 4)
10. `docs/registers/*.md`: generated from the `.json` files by `tools/build_register.py`
11. `docs/sources/LOG.md`, `docs/spec/provenance.md`, `docs/sources/occultist-explorer/CROSSCHECK.md`

## Check

1. **Citations:** every [S] claim is supported by the cited witness, and nothing invented is passed off as [S].
2. **Precedence:** the oldest-wins rule is applied consistently, and the rulings are obeyed everywhere.
3. **Consistency:** the parts agree with each other: section references, error codes A1 to A16, the outcomes of the will, destinations, ink classes, grant kinds, office signatures.
4. **The core interface:** no rite rule anywhere grants authority, skips examination or reaches past `core.md` §3.1. Look especially for rules that let data, a spirit, an angel or a library act outside the faculty.
5. **Theorems:** whether T1–T9 (`core.md` §8) are stated precisely enough to prove, and whether any rule in the spec contradicts one.
6. **Grammar:** it is LL(1) and matches `tools/grammar_check.py`. Run `python3 tools/grammar_check.py tests/grammar` (69 programs).
7. **Registers:** they match their sources: the 8 legion corrections, the ranks, the quarters, the divine names.
8. **Gaps:** rules that cannot be implemented as written.

## Known open items (do not report)

- mixed-rank seal metal;
- quarters for Buer, Glasya-Labolas, Haagenti, Caim, Amy, Valac, Botis and Marax;
- Rudd's kings (waiting for Harley 6483);
- which angel examines an angel, per rite;
- the making of the instruments and the Secret Seal;
- the `heptameron` sigils and the Harley 6483 seals;
- `LONGAEVITAS`'s time grant; the network;
- the angels' channel (`ADVOCA`).

## Report format

A table with the columns: file and section | problem | evidence | severity (high/medium/low) | suggested fix.
