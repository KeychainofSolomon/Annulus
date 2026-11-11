# Annulus Specification, Part 3: Static Semantics

Status: draft 1, 2026-11-10. Normative once approved by the owner.

This part says what is checked **before a program runs**. A program that fails any rule here is refused whole: no statement of it executes, and nothing is granted, drawn or invoked. Provenance tags ([S], [O]) follow `authority.md` §0. Most rules here are mechanism and so [O]; where a rule enforces a sourced practice, the source is given.

## 1. Phases

The checks run in this order. Each phase reports every error it finds, then the program is refused if there were any.

1. **Lexing and parsing:** `grammar.md`.
2. **Name resolution:** §2.
3. **Type checking:** §3.
4. **Authority analysis:** §4, the compile-time half of the eight laws.

Every error names its phase, a code (§6), and the source position.

## 2. Names and scopes

2.1 **Two separate namespaces.**
- **Program names:** variables, functions (`OPUS`), forms (`FORMA`) and parameters.
- **Register labels:** the names of Constitution entries, such as `furfur` or `lehahiah`.

An identifier in an **entity position** (after `INVOCA` or `SIGILLUM`) is a register label. Everywhere else it is a program name. So `SIT furfur = INVOCA furfur AD OFFICIUM …;` is lawful: the variable holds the invocation; the label names the spirit.

A program cannot declare, assign or shadow a register label (`authority.md` §2.3).

2.2 **Scopes.**
- Functions and forms are declared at the top level and are visible everywhere in the program, so functions may call each other in any order.
- Every block (`{ … }`) opens a scope. A variable is visible from its declaration to the end of its block.
- A `PER` variable is visible in its loop body. A `CAPE` variable is visible in its handler block.
- A function body sees its parameters, the top-level functions and forms, and nothing else. Top-level variables are not visible inside functions.

2.3 **No shadowing.** A name MUST NOT be declared again while an outer declaration of it is still visible. Two sibling blocks may reuse a name. [O: a hidden name is a misread name.]

2.4 **Immutability.** A variable declared without `MUTABILIS` MUST NOT be assigned. `PER` and `CAPE` variables and parameters are immutable.

2.5 **Use before declaration** is an error.

## 3. Types

3.1 **Every expression has one type, known before the program runs.** A `SIT` without a type label takes the type of its expression.

3.2 **Plain types:** `NUMERUS`, `VERITAS`, `VERBUM`, `NIHIL`, `SERIES[T]`, and each declared `FORMA`.

3.3 **Rite types.** The Constitution supplies these types. Except for the outcome, a program can never write their names: it can only receive them from the rite.

| Type | Produced by | Notes |
|---|---|---|
| invocation | `INVOCA` | a spirit in its Triangle |
| proposal | `PETE` | untrusted output of a spirit |
| outcome | the judgement step inside `EXAMINA` (§4.10); the literals `RATUM`, `SUSPENSUM`, `IRRITUM` | a plain value; may leave the Circle |
| seal (exemplar) | `SIGILLUM e` | data; grants nothing |
| seal (operative) | `INSCRIBE … ATRAMENTO …` | a capability |
| ink (raw) | `ATRAMENTUM "classis"` | drawn from the faculty; not yet usable |
| ink (consecrated) | `CONSECRA` on raw ink | usable for one inscription |
| **marks** on plain values | inferred, never written (`flow.md` §2.3) | `clean`, `outside`, `consecrated-grave`, `consecrated`, `quarantined` (`core.md` §9.3). `CONSECRA` on a quarantined value gives the same type marked `consecrated`. The grave grade is never given within a run (`flow.md` §3.3) |
| entry | `SPIRITUS n`, `ANGELUS n` | a register reference |
| divine name | `NOMEN "…"` | a Constitution reference |

3.4 **Operators.**

| Operator | Operands | Result |
|---|---|---|
| `+ - * / %` | `NUMERUS`, `NUMERUS` | `NUMERUS` |
| `+` | `VERBUM`, `VERBUM` | `VERBUM` (joined) |
| unary `-` | `NUMERUS` | `NUMERUS` |
| `< <= > >=` | two `NUMERUS`, or two `VERBUM` (by code point) | `VERITAS` |
| `== !=` | two values of the **same** type (§3.5) | `VERITAS` |
| `ET VEL` | `VERITAS`, `VERITAS` | `VERITAS` |
| `NON` | `VERITAS` | `VERITAS` |

3.5 **Equality (ruling 2026-11-10).** Comparing values of different types is an error. Equality is defined for the plain types (sequences and forms compare element by element) and for outcomes and entries. It is **not** defined for invocations, proposals, seals, ink or divine names: capabilities are used, not compared.

3.6 **No hidden truth.** Conditions of `SI` and `DUM` MUST be `VERITAS`. A number or a string is never "true".

3.7 **Sequences.**
- All elements of a literal `[a, b, c]` have one type.
- An empty `[]` takes its type from a type label (`SIT s: SERIES[NUMERUS] = [];`) or from the place it is assigned to; with neither, it is an error.
- `s[i]` requires a `NUMERUS` index and gives the element type.
- `PER x IN s` requires a `SERIES`; `x` has the element type.

3.8 **Forms.** A form is built by calling its name with one argument per field, in declared order: `punctum(1, 2)`. `p.x` reads a field; `p.x = …` writes one if `p` is mutable.

3.9 **Functions.**
- A call has exactly as many arguments as the function has parameters, each of the parameter's type.
- A function with a result type MUST end in `REDDE value` on every path. A function without one returns `NIHIL`, and `REDDE value` inside it is an error.
- Rite types other than outcome cannot be parameters or results (§4.7).

3.10 **Output.** `MANIFESTA` accepts the plain types, outcomes, entries (shown as their number and label) and exemplar seals (shown in their revealed reading). Other rite values cannot be shown. [O]

3.11 **Errors caught by `CAPE`.** The `CAPE` variable has type `VERBUM`: the error's message. Which failures `TENTA` can catch is set in Part 4. Authority failures will not be catchable, so a denial can never be quietly swallowed.

## 4. Authority analysis

These rules are checked before the program runs. Where a law can only be checked while running (budgets, time windows, verdicts), Part 4 covers it.

4.1 **CONTAINMENT and SOVEREIGNTY: rite operations stay inside a Circle.** `GERE`, `OFFER`, `CONSTRINGE`, `APPELLA`, `VETA`, `INVOCA`, `PETE`, `SIGILLUM`, `ATRAMENTUM`, `CONSECRA`, `INSCRIBE`, `EXAMINA`, `RESPONSUM`, `VADE IN PACE`, `DIMITTE` and `RELEGA` are errors outside a `CIRCULUS` block. So are the expressions that produce Circle-bound values: `NOMEN`, `SPIRITUS` and `ANGELUS` (audit 2026-11-11). Outside a Circle there is no rite in which an entry could resolve (§4.3), and §4.7 binds entries and divine names to their Circle. A function body counts as outside unless the operation is in a Circle inside that body. [S: the operator works only within the circle.]

4.2 **The rite must exist.** `RITU` names one of the rites in `authority.md` §4.1. The intention string MUST NOT be empty. [O]

4.3 **Registers by rite.** What `SPIRITUS n` and `ANGELUS n` mean depends on the rite of the innermost Circle:

| Rite | `SPIRITUS n` | `ANGELUS n` | `INVOCA` allowed |
|---|---|---|---|
| `goetia` | Goetic spirit, 1–72 | none | yes |
| `rudd` | Goetic spirit, 1–72 | Shem angel, 1–72 | yes |
| `arbatel` | Olympic spirit, 1–7 | none | no: Olympic spirits are called by prayer, through their channel (`authority.md` §6.5) |
| `heptameron` | none | day-angel, 1–7 | no |
| `clavicula`, `iuratus`, `janua` | not yet defined: these registers await transcription | | |

An entry outside the range, a label not in the rite's register, or a reference the rite does not define is an error. A label MAY be written in any spelling the register records for that rite.

4.4 **No Triangle for angels.** `INVOCA` applies only to entities that have a Triangle: spirits in `goetia` and `rudd`. Invoking an angel or an Olympic spirit is an error. Calling through a channel uses `ADVOCA`, defined in a later part. [S: `authority.md` §6.5.]

4.5 **The seal comes first.** `INVOCA` of a spirit requires that the spirit's operative seal was inscribed earlier in the same Circle, on every path. [S: Sloane 3825, the Goetia, e.g. Bael: his character "is to be worne as a Lamen before him who calleth him forth, or else he will not doe you homage". O: the path rule.]

4.5a **The apparatus.** `GERE`, `OFFER`, `CONSTRINGE`, `APPELLA` and `VETA` are rite operations: only inside a Circle (§4.1). An instrument taken up lasts until its Circle closes. Where a spirit **requires** an instrument (`spirits.md` §4.4: Beleth, Berith, Astaroth), it MUST be taken up before `INVOCA`, on every path (A11). What `PETE` to Andras for `probatio` takes is set by his signature (`offices.md` §2.5). The name in `CONSTRINGE … PER NOMEN` follows the rule for `NOMEN` (`grammar.md` §5, item 4).

4.5b **The kings.** `INVOCA REGEM r` names a king of the Circle's rite (`docs/registers/reges.md`). It is defined for `goetia`; in other rites it is an error (A3). It yields `NIHIL`: the king does not appear, and nothing is released. [S: the kings "are not to be called forth except it be upon Great Occasions; but are to be Invocated and Commanded to send" (G).]

**`rudd` (open, audit 2026-11-11):** Rudd's Goetia is a separate work (`provenance.md`), so it cannot silently inherit the Goetia's kings. By ruling 2026-11-11, `INVOCA REGEM` under `rudd` stays provisional until Harley 6483 is read for Rudd's own kings. Until then, a pure President under `rudd` follows the provisional reading.

**A President needs his king.** `INVOCA` of a spirit whose only rank is President requires `INVOCA REGEM` of the king he obeys, earlier in the same Circle, on every path (A13). [S: Weyer ch. III, "non potest constringi, nisi rex cui paret, invocaretur", ruling 2026-11-10.] The king a spirit obeys is **the `goetia` king of his quarter** (ruling 2026-11-10): the quarter is the direction the witnesses give for the spirit (`goetia.md`), and the king is Weyer's for that quarter (`reges.md`). A President whose quarter is not stated cannot be invoked (A13). Presidents with a quarter: Gaap and Marbas (East, Amaymon); Foras, Malphas and Osé (South, Gorson); Zagan (West, Goap). Spirits of two ranks are not bound by this rule when invoked within their other rank's hours.

4.6 **OFFICE.** The identifier after `AD OFFICIUM` MUST name one of the spirit's office kinds in the rite (`docs/registers/goetia.md`); an office marked `NULLUM` cannot be invoked. Each office has a fixed signature: what `PETE` may pass, and what the proposal will contain. The arguments to `PETE` MUST match it (A5). The mapping from each traditional office to computer work is in `spirits.md` §6; the signatures are in `offices.md` (Part 1c, draft 1, for approval).

4.7 **Rite values are bound to their Circle.** Invocations, proposals, seals, ink, entries and divine names MUST stay in the Circle that produced them. They MUST NOT be:
- assigned to a variable declared outside that Circle;
- stored in a sequence or form;
- passed to, or returned from, a function.

Outcomes are plain values and may leave. [O; enforces SOVEREIGNTY.]

4.8 **Seals and ink.**
- `INSCRIBE s ATRAMENTO a`: `s` MUST be an exemplar seal and `a` MUST be consecrated ink.
- **All drawn ink is raw** and MUST pass `CONSECRA` before use. [S: the blood is exorcised and blessed before any writing (*Liber iuratus* CXXXVII; Mathers II.16).]
- **One inscription per ink.** Consecrated ink is spent by one `INSCRIBE`; using the same ink value twice is an error. [O]
- **The ink class must match the seal** (`authority.md` §7.5). A spirit's seal takes the ink of the spirit's rank: Furfur, an Earl (*comes*), takes `ATRAMENTUM "comes"`. Where the class is known before running, a mismatch is an error. The ink classes are named by rank, per `authority.md` table 7.5a (Sloane 3825 f111). A spirit of two ranks is an open item.
- **No self-minted ink.** Ink comes only from `ATRAMENTUM "classis"`, which draws from the faculty. No expression creates ink from program data (`authority.md` §1.4).

4.9 **DECLARATION: the faculty.**
- `ATRAMENTUM "classis"` requires a grant `atramentum "classis" n;` in the `FACULTAS` block.
- Any other external effect requires a grant of its kind. The kinds so far (ruling 2026-11-11 for the last three items):
  - `liber`: a file, with `legere` (read), `scribere` (write), `creare` (create) and `delere` (delete);
  - `locus`: a **slot**, a file the launching person chooses at launch, with the same permissions as `liber` (`library.md` §2);
  - `atramentum`: ink;
  - `historia "prior" legere`: the history of the previous run (`offices.md` §2.2a);
  - `familiaris "programma" liberare`: freeing a familiar from that program (`offices.md` §4.6).
  - **reserved for angels:** any grant may end in `angelis` (ruling 2026-11-11, `authority.md` §12.1a). Such a grant MUST NOT appear in a `PETE` request's handed set (A5), and no program operation may use it (A8).

  Every request needs its grants checked against the office signatures (`offices.md` §3.2). The standard library that performs file work will be specified with its grants.
- `MANIFESTA` needs no grant: the console is part of the default faculty.
- The same grant MUST NOT appear twice. Unknown grant kinds or permissions are errors.
- Whether the budget suffices is checked while running, because loops make it unknowable beforehand.
- The binding hours and the Moon's age (`authority.md` §8.3) are checked while running, against the clock.

4.10 **EXAMINATION and JUDGEMENT.** `EXAMINA p` requires a proposal. It performs two separate acts of the Constitution, in order (ruling 2026-11-11, after the audit):
1. **Examination** (`authority.md` §10): the fixed examiner gives a **verdict**, `APPROVE`, `HOLD` or `REJECT`. The verdict is recorded in the audit. It is not a program value and has no type a program can hold.
2. **Judgement** (`authority.md` §11): the Constitution's judgement step assigns the **outcome** from the verdict (`dynamic.md` §6.1). The outcome is the value of `EXAMINA p`.

The examiner is not written and cannot be: the Constitution's pairing decides it (`authority.md` §10). No agent and no program performs either act.

4.10a **RESPONSUM** (ruling 2026-11-11). `RESPONSUM p` reads the answer of a proposal `p` whose office gives a response (`offices.md` §1.4). Otherwise it is an error (A5). It has type `VERBUM`, and the value is **quarantined** (`flow.md` §1.2): it cannot steer the program or reach another spirit until consecrated. It is a rite operation, so only inside a Circle (§4.1). It succeeds only if `p` was judged `RATUM`. Before that, or after any other outcome, it is a class 2 denial. Authority failures cannot be caught (§3.11), so no program can quietly read a held or condemned answer. There is no expression that produces an outcome except `EXAMINA` and the three literals, and a literal outcome cannot commit anything: it is only a value to compare with.

4.11 **RELEASE: every invocation ends.** For each variable holding an invocation:
- on **every path** through the Circle, it MUST be released exactly once, by `VADE IN PACE`, `DIMITTE` or `RELEGA`, before the Circle closes;
- after release it MUST NOT be used;
- `DESINE`, `PERGE` and `REDDE` MUST NOT leave a Circle, or an iteration of a loop inside it, while an invocation made there is unreleased;
- an invocation made inside a loop body MUST be released within that same iteration.

[S: every operation ends with the licence to depart (1446 p216; Arbatel XXI; the Goetia). O: the path rule.]

4.12 **Nested Circles.** An inner Circle sees the outer Circle's variables. Rite values stay bound to the Circle that made them (§4.7). An inner Circle of another rite is a cross-rite operation and so declares both rites by its nesting (`authority.md` §4.3). An inner Circle never gains a grant the outer lacks. The faculty is program-wide, and Circles only narrow it.

4.13 **Divine names.** The string in `NOMEN "…"` and in `CONSTRINGE … PER NOMEN "…"` MUST equal, exactly, one recorded form of an entry of kind `deus` in `docs/registers/nomina.md`. Angels, orders, sephiroth, spheres and the unexplained names on the figures cannot be cited as divine names (A12). [S: the constraint is "by Divine Power", by God's names; O: the exact-match rule.]

## 5. Changes to the grammar made by this part

- `ATRAMENTUM , string` is a primary expression: it draws raw ink of that class.
- A faculty grant may end with an integer: `atramentum "comes" 1;`.

Both are applied in `grammar.md` and in `tools/grammar_check.py`.

## 6. Error codes

| Code | Rule |
|---|---|
| N1 | unknown name |
| N2 | use before declaration |
| N3 | shadowing |
| N4 | assignment to an immutable variable |
| N5 | declaring or assigning a register label |
| T1 | operand types do not fit the operator |
| T2 | equality between different types |
| T3 | condition is not `VERITAS` |
| T4 | wrong number or type of arguments |
| T5 | missing `REDDE` on some path |
| T6 | empty sequence without a type |
| T7 | value cannot be shown by `MANIFESTA` |
| A1 | rite operation, entry or divine name outside a Circle |
| A2 | unknown rite, or empty intention |
| A3 | entry or label not in the rite's register |
| A4 | invocation of an entity with no Triangle |
| A5 | office not held by the spirit, or `PETE` arguments not matching it |
| A6 | rite value leaving its Circle |
| A7 | seal or ink misuse: raw ink, spent ink, wrong class, non-exemplar seal |
| A8 | faculty: missing, duplicate or unknown grant |
| A9 | invocation not released on some path, or used after release |
| A10 | invocation without the spirit's operative seal |
| A11 | invocation without a required instrument |
| A12 | a name cited as divine that is not a recorded form of a divine name |
| A13 | a President invoked without his king, or whose king is not stated |
| A14 | a quarantined value steering the program, choosing a place, slot or mode, or used as ink, a seal or a grant (`flow.md` §2.2) |
| A15 | a quarantined value passed to a spirit (`flow.md` §2.2) |
| A16 | a grave act steered by a value marked `consecrated` or `quarantined` (`flow.md` §3.3) |

## 7. Open items

- **Signatures for `PETE`:** drafted in `offices.md`; its §7 lists what waits for the owner.
- **Mixed ranks:** which ink class a spirit of two ranks takes.
- **Rudd's kings** (audit 2026-11-11; ruling 2026-11-11: wait for the manuscript): establish from Harley 6483 whether Rudd gives the four kings (§4.5b). Then add `rudd` entries to `reges.json`.
- **Registers for `clavicula`, `iuratus` and `janua`.**
- **Modules (`LIBER`, `ADHIBE`):** how files are found and what a module exports; deferred to the part on the standard library.
- **Outcome as a written type:** for passing outcomes to functions, a type name is needed. The proposal is `DESTINATIO`.
