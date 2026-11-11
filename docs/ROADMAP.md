# Roadmap

Annulus is both a general-purpose programming language and a faithful simulation of the Solomonic tradition. Its mechanics must be grounded in the sources. Every cosmological element is labelled either as sourced, with its witness and date, or as original to Annulus. The source rule is in `docs/spec/provenance.md`.

**Order (adopted 2026-11-11).** Prove enforcement first, with the simplest machine that can run it; optimise later. The earlier 18-step plan built the full compiler pipeline before the authority subsystems. Its steps survive as the later steps 7-9 below.

Each milestone delivers:
- the source tree;
- the updated specifications;
- build instructions;
- working examples;
- automated tests and their actual results;
- implemented and outstanding feature lists.

Nothing is reported as built or tested unless it was.

| # | Step | State |
|---|---|---|
| 0 | Audit and reproducible build of the v0.2 prototype | **Done** (v0.2.1; `docs/AUDIT.md`; pinned toolchain, CI double build) |
| 1 | **Complete the formal specification** | **In progress**: Parts 1, 1a, 1b, 2, 3 and 4 drafted (`docs/spec/`); outstanding work below |
| 2 | **Minimal executable interpreter** (Rust): parser, type checker with the authority rules, direct interpreter | |
| 3 | **Demonstrate enforcement**: programs written to break every law, each refused; Andras's `probatio`; begin the no-amplification proof | |
| 4 | **Practical demonstration**: a real AI agent invoked as a spirit, in a Triangle, for one office, under a narrow faculty | |
| 5 | **Evaluate** against conventional languages, permission systems and capability frameworks: security, clarity, complexity, performance | |
| 6 | **The discipline of the Operator** (`docs/spec/operator.md`): test whether a persuasive agent can win a wider faculty, with and without the discipline, against conventional systems | |
| 7 | Instruction set, intermediate representation, compiler, verified bytecode, virtual machine | |
| 8 | Remaining subsystems: angelic channels (`ADVOCA`), the `clavicula`/`iuratus`/`janua`/`heptameron` rites, HISTORIA, REVELATIO | |
| 9 | Native x86-64 backend; Windows and Linux releases | |

## Step 1: what the specification still needs

1. **Office signatures:** for each office kind (`spirits.md` §6), what `PETE` passes and what the proposal holds. This is the biggest gap: the OFFICE law, examination and helper-task containment all depend on it.
2. **The two-layer split:** a small **core** (capabilities, the grant, no amplification, the Circle boundary, commits, audit, Vessel, Index) that is proven, separate from the **rite layer** (registers, offices, hours, seals, ink, apparatus), which is data and policy judged by the source rule. Tradition decides what the rite means; the core decides what is possible.
3. **Amendment procedure:** the Constitution is versioned; changes come only through reviewed releases; every program records the Constitution version it ran under. The sources check the rite layer's fidelity; proof and attack testing check the core's safety.
4. **Information flow:** rules beyond taint, including when tainted data may be released; no ambient authority anywhere in the standard library.
5. **Concurrency semantics.**
6. **Standard library** for files and the network, with its grants.
7. **Persistent proscription details:** agent identity, storage of the Index, and how an entry is lifted.
8. **The guiding principle** (`authority.md` §1.5, ruling 2026-11-11): no program's authority can exceed its grant, at any depth of delegation. Its proof obligations bind later steps:
   - the office signatures must make helper tasks checkable;
   - the type checker (step 2) must enforce the principle;
   - the bytecode verifier (step 7) must reject anything that breaks it;
   - a machine-checked proof over a formal model of the typing rules, plus Andras's `probatio`, must show it (step 3).
9. **The GPT audit:** fold in its findings (`AUDIT_BRIEF.md`).
10. **The owner's approval** of the whole, including the [O] readings of the Earls' and Dukes' conditions (`dynamic.md` §5.1).

**Exit criterion:** the owner approves the specification.

## Specification parts written so far

1. **Authority model:** the constitution, the axiom, the eight laws, seals and ink, registers, examination, destinations. File: `docs/spec/authority.md`.
2. **Provenance register:** the source-precedence rule, plus a sourced or original label on every element. File: `docs/spec/provenance.md`.
3. **Lexical grammar:** literals, the keyword set and casing; no comment syntax. Keywords are all Latin (ruled 2026-11-10). File: `docs/spec/grammar.md` (draft 1).
4. **Syntax in EBNF**, in the same file (draft 1). Checked LL(1): every choice is decided by the next token. The reference recognizer `tools/grammar_check.py` runs the accept/reject programs in `tests/grammar/`, and CI runs it.
5. **Static semantics:** scopes, name resolution, types, and authority analysis. File: `docs/spec/static.md` (draft 1).
6. **Dynamic semantics:** evaluation order, the effect model, failure classes. File: `docs/spec/dynamic.md` (draft 1).
7. **The spirits:** offices as computer work, will, malice, counterbalances, apparatus, Andras's testing. File: `docs/spec/spirits.md` (Part 1a, draft 1).
8. **The discipline of the Operator:** commitment before the work, no free channel, guardians, preparation, two keys, real-agent mode. File: `docs/spec/operator.md` (Part 1b, draft 1).
9. **Registers** (generated from JSON by `tools/build_register.py`): the 72 spirits, divine names, the four kings. Files: `docs/registers/`.

## The first end-to-end working (target of steps 2-3)

1. One circle.
2. Furfur (Goetia 34) is invoked into the triangle for his office (storms: `TEMPESTAS`).
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
- Quarters (and so kings) for Buer, Glasya-Labolas, Haagenti, Caim, Amy, Valac, Botis and Marax: search Rudd's Harley 6483 and the rest of Wellcome 110.
- The seal metal for spirits of two ranks.
- The registers of the `clavicula`, `iuratus` and `janua` rites.
