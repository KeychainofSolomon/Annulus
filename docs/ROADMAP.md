# Roadmap

Annulus is both a general-purpose programming language and a faithful simulation of the Solomonic tradition. Its mechanics must be grounded in the sources. Every cosmological element is labelled either as sourced, with its witness and date, or as original to Annulus. The source rule is in `docs/spec/provenance.md`.

The order follows the Definitive Architecture handoff (section 17). Each milestone delivers:
- the source tree;
- the updated specifications;
- build instructions;
- working examples;
- automated tests and their actual results;
- implemented and outstanding feature lists.

Nothing is reported as built or tested unless it was.

| # | Step | State |
|---|---|---|
| 1 | Audit the v0.2 source | **Done** (v0.2.1, `docs/AUDIT.md`) |
| 2 | Reproducible Rust build | **Done** (v0.2.1; pinned toolchain, CI double build) |
| 3 | Formal language specification | **In progress**: authority model first (`docs/spec/authority.md`), then grammar and semantics |
| 4 | Instruction set | |
| 5 | Intermediate representation | |
| 6 | Compiler | |
| 7 | Bytecode generation and verification | |
| 8 | Virtual machine | |
| 9 | Type system and general-purpose features | |
| 10 | Memory and resource management (including ink budgets) | |
| 11 | Circles and capabilities | |
| 12 | Transactional execution | |
| 13 | Examination and judgement | |
| 14 | Invocation lifecycle | |
| 15 | Angelic authority and hierarchy | |
| 16 | Remaining cosmological subsystems (HISTORIA, REVELATIO, registers) | |
| 17 | Native x86-64 compilation | |
| 18 | Windows and Linux releases | |

## Step 3: the formal specification

1. **Authority model:** the constitution, the axiom, the eight laws, seals and ink, registers, examination, destinations. File: `docs/spec/authority.md`.
2. **Provenance register:** the source-precedence rule, plus a sourced or original label on every element. File: `docs/spec/provenance.md`.
3. **Lexical grammar:** comments, literals, the keyword set and casing. Keywords are all Latin (ruled 2026-11-10). File: `docs/spec/grammar.md` (draft 1).
4. **Syntax in EBNF**, in the same file (draft 1). Checked LL(1): every choice is decided by the next token. The reference recognizer `tools/grammar_check.py` runs the accept/reject programs in `tests/grammar/`, and CI runs it.
5. **Static semantics:** scopes, name resolution, types, and authority analysis. File: `docs/spec/static.md` (draft 1).
6. **Dynamic semantics:** evaluation order, the effect model, failure classes.

**Exit criterion:** the owner approves the specification.

## The first end-to-end working

This is the first slice through steps 4 to 14, with tests at each stage:
1. One circle.
2. Furfur (Goetia 34) is invoked into the triangle for his office.
3. Lehahiah (Shem angel 34) examines what Furfur proposes.
4. The work is committed (PARADISE), held (PURGATORY), or banished and quarantined (HELL).

Furfur is chosen because the Goetia says he speaks falsely unless he is confined in the triangle, so his output is untrusted by default.

## Data still pending (blocks nothing above)

- Traced seal artwork from Sloane 3825. Until then, seals are stored as manuscript references.
- Sloane 3825 frame 41L (the 72 angel names) and frame 40 (the angels of the lunar mansions).
- Rudd's 72 angel seals (Harley 6483).
- Amy's seal, by comparison with Peterson's g58.jpg.
- The *Heptameron* day-angel sigils, checked against an early printing.
- The ring lettering of the 1446 pentacles, from Gallica colour images.
