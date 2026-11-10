# Roadmap

This roadmap follows the development order in the engineering handoff (section 12). Each milestone ends with: source tree, build instructions, real test results, working examples, an updated specification, both binaries when built, and an implemented/unimplemented list.

Sizes are rough relative effort, not dates.

| # | Milestone | Size | State |
|---|---|---|---|
| 1 | Audit and reproducible build | S | **Done in v0.2.1** |
| 2 | Formal grammar and semantics | M | Next |
| 3 | Compiler and intermediate representation | M | |
| 4 | Versioned bytecode format and virtual machine | L | |
| 5 | Types, functions, control flow, modules | XL | |
| 6 | Circles, capabilities and containment | L | |
| 7 | Transactional execution and examination | L | |
| 8 | Invocation lifecycle and termination | M | |
| 9 | Angelic hierarchy and delegated authority | M | |
| 10 | Historical cosmological entities and states | M | |
| 11 | Remaining subsystems | L | |
| 12 | Build, test and package release binaries | S | |

## Milestone 2 — Formal grammar and semantics

Write a normative specification (`docs/spec/`) covering the whole language, not just what exists.

- Complete lexical grammar: comments, string escapes, numeric literal forms, keyword set and casing rule.
- Complete syntax in EBNF, checked to be unambiguous; the parser's tests are derived from it.
- Static semantics: scopes, name resolution, the type system, which errors are compile-time.
- Dynamic semantics: evaluation order, the effect model, failure behaviour.
- The authority model in prose: origin, jurisdiction, capability derivation, examination, commit, release.
- A provenance register (`docs/spec/provenance.md`) that labels every cosmological element as either **historical tradition**, with its source, or **original to ARS MAGNA**.

Exit criteria: spec reviewed and approved by the language owner; every decision in section 4 of the audit settled.

## Milestone 3 — Compiler and intermediate representation

- Split compilation into resolve → type-check → lower → generate.
- A typed, scoped intermediate representation with explicit effects, so later passes (containment, examination) can see every effect statically.
- Multiple diagnostics per run instead of stopping at the first.

## Milestone 4 — Versioned bytecode and virtual machine

- A binary bytecode format: magic number, format version, constant pool, function table, code, source-position table.
- A bytecode verifier that rejects malformed or unsafe code before execution.
- Jumps, call frames, return.
- `ars-magna build` (source → bytecode file) and `ars-magna run` (bytecode or source).
- Fuel (an instruction budget) so execution is bounded and deterministic.
- Identical bytecode from identical source on both platforms, tested.

## Milestone 5 — Types, functions, control flow, modules

- Primitive types: integer, boolean, string, plus a decision on rationals or fixed-point numbers for NUMERUS.
- Composite types: records, lists, maps, tagged unions.
- Functions and procedures, closures if the spec requires them.
- Conditionals, loops, pattern matching.
- Error handling as values (result types), not exceptions.
- Modules with explicit imports and exports.

## Milestone 6 — Circles, capabilities and containment

Laws: ORIGIN, SOVEREIGNTY, DECLARATION, CONTAINMENT; axiom NULLA POTESTAS SIBI IPSI ORITUR.

Proposed technical basis (to be confirmed in milestone 2): **object capabilities**.

- An authority is an unforgeable runtime value. No instruction creates one from nothing; new authorities are only derived from, and narrower than, an existing one. This is the axiom enforced by the VM.
- Every authority records its origin chain. The root of every chain is a fixed marker for THE ABSOLUTE that programs can refer to but never hold, call or impersonate.
- A CIRCLE is a bounded execution region: the set of authorities, the fuel, memory and effect budget code inside it receives. Nothing inside can reach outside except through granted authorities.

## Milestone 7 — Transactional execution and examination

Laws: EXAMINATION, JUDGEMENT.

- Effects inside an operation are written to a journal, not performed.
- An examiner with a different identity from the operator approves or rejects the journal. The VM refuses self-approval by comparing identities.
- Approved journals are committed; rejected ones are discarded entirely.
- Ordinary examination is kept separate from the symbolic final judgement.

## Milestone 8 — Invocation lifecycle

INVOKE, EXAMINE, LICENSE TO DEPART, DISMISS and BANISH as distinct VM operations with a defined state machine.

- LICENSE TO DEPART: orderly end; temporary authorities revoked; outstanding work allowed to finish and be examined.
- BANISH: forced end; execution halted, authorities revoked, child tasks terminated, uncommitted effects discarded, results quarantined.
- Release is guaranteed: an invocation that ends any other way, including by error or fuel exhaustion, is released by the runtime (law RELEASE).

## Milestone 9 — Angelic hierarchy and delegated authority

- The nine orders (Seraphim to Angels) as a typed hierarchy.
- The three ministries (NUNTIATIO, MINISTERIUM, MILITIA) as offices, not ranks; an angel may hold several.
- Origin, allegiance, nature, office, jurisdiction and judgement as separate fields on an entity.
- Angelic oversight as bounded, independently terminable runtime interventions.

## Milestone 10 — Historical cosmological entities and states

- HISTORIA: the War in Heaven and the Fall as recorded state transitions of allegiance.
- Fallen entities with per-entity retained, inverted or lost offices, not a single rule for all.
- LUCIFER's domain as a jurisdiction whose authority is real but derived.
- PARADISE, PURGATORY and HELL as the language's symbolic final destinations, with no claim to determine actual judgement.

## Milestone 11 — Remaining subsystems

LOGOS, NUMERUS, MEMORIA, POTESTAS, OFFICIUM, MANIFESTATIO, IUDICIUM, HISTORIA, ANGELICUS, REVELATIO, CIRCULUS, IDENTITAS, INVOKATIO, EXAMINATIO: each as a real runtime component with tests. Several are delivered by earlier milestones (for example POTESTAS and CIRCULUS in 6, IUDICIUM and EXAMINATIO in 7, INVOKATIO in 8). This milestone completes and audits the rest.

## Milestone 12 — Release packaging

- Tagged releases with both binaries and checksums attached.
- Windows reproducibility verified in CI (two builds compared).
- Optional fully static Linux build (musl) so the binary runs on any distribution regardless of glibc version.

## Decisions needed from the language owner

These shape milestone 2 and should be settled before it is written:

1. **Keyword language.** Keep English (`LET`, `PRINT`), move to Latin to match the subsystem names (`DECLARA`, `MANIFESTA`), or mix?
2. **Casing.** Keep uppercase-only keywords?
3. **Duplicate spellings.** Keep both `LET`/`DECLARE` and `MANIFEST`/`PRINT`, or choose one of each?
4. **Mixed-type equality.** Should `1 == "1"` stay `false` or become a compile-time type error? (Recommended: type error.)
5. **Final destinations.** Proposed mapping, for approval: PARADISE = an invocation fully examined and committed; PURGATORY = held for re-examination; HELL = banished and quarantined. These would be outcome labels only.
6. **Licence** for the project.
