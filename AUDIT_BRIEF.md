# Audit brief: Annulus specification (step 3)

You are auditing the Annulus language specification. Annulus is a programming language and also a faithful simulation of Solomonic magic. Find errors; do not rewrite.

## The rules the work must follow

1. **Source precedence:** within one work's line of copies, the oldest witness wins. Separate works (rites) never override each other. A later addition stands if it conflicts with nothing older.
2. **Labels:** every rule is tagged **[S]** (sourced, with its witness) or **[O]** (original to Annulus). The owner's rulings are tagged **M**. Witness codes:
   - **W** Weyer 1577
   - **S** Sloane 3825
   - **G** the 1904 edition
   - **F** Folger V.b.26
   - **A** Ad. 36674
   - **K** BnF Ital. 1524 (1446)
3. **Analogues:** a traditional detail is kept only if it has a mechanical analogue in the language.
4. **Rulings in force** (recorded in `HANDOFF.md`):
   - rank and seal metal come from Sloane 3825;
   - binding hours come from Weyer ch. III;
   - a spirit's king is decided by his quarter;
   - the Ring of Virtues counts over the 1904 "invincible";
   - Andras serves as a tester, by command only;
   - all keywords are Latin;
   - no comments in code;
   - the licence is the Unlicense.

## Read in this order

1. `docs/spec/authority.md`: the authority model (Part 1)
2. `docs/spec/spirits.md`: offices, will, malice, apparatus (Part 1a)
3. `docs/spec/grammar.md`: lexical grammar and EBNF (Part 2)
4. `docs/spec/static.md`: compile-time rules (Part 3)
5. `docs/spec/dynamic.md`: runtime rules (Part 4)
6. `docs/registers/*.md`: generated from the `.json` files by `tools/build_register.py`
7. `docs/sources/LOG.md` and `docs/spec/provenance.md`: evidence and witnesses
8. `docs/sources/occultist-explorer/CROSSCHECK.md`

## Check

1. **Citations:** every [S] claim is supported by the cited witness, and nothing invented is passed off as [S].
2. **Precedence:** the oldest-wins rule is applied consistently, and the rulings above are obeyed everywhere.
3. **Consistency:** the five parts agree with each other. Pay particular attention to: section references; error codes A1 to A13; the outcomes of the will; destinations applying to proposals; ink classes.
4. **Grammar:** it is LL(1) and matches `tools/grammar_check.py`. Run it with `python3 tools/grammar_check.py tests/grammar` (64 programs).
5. **Registers:** they match their sources. In particular the 8 legion corrections, the ranks, the quarters, and the divine names.
6. **Laws:** no rule lets a spirit or program grant, approve or mint for itself (the axiom, `authority.md` §1).
7. **Gaps:** rules that cannot be implemented as written.

## Known open items (do not report)

- the signatures of the office kinds;
- mixed-rank seal metal;
- quarters for Buer, Glasya-Labolas, Haagenti, Caim, Amy, Valac, Botis and Marax;
- the making of the instruments and the Secret Seal;
- the `heptameron` sigils;
- the Harley 6483 seals.

## Report format

A table with the columns: file and section | problem | evidence | severity (high/medium/low) | suggested fix.
