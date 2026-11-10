# Annulus Specification, Part 4: Dynamic Semantics

Status: draft 1, 2026-11-10. Normative once approved by the owner.

This part says what happens **while a program runs**. A program reaches this point only after passing every check in Part 3. Tags [S] and [O] follow `authority.md` §0.

## 1. Launch

1.1 **The launch command** names the program and, optionally:
- the place (`--locus LAT,LON`), needed for sunrise, noon and sunset (§6);
- a fixed clock (`--hora` with a date and time), for reproducible runs and tests;
- an entropy seed (`--semen`), to replay a recorded run exactly (§5.4);
- the audit file (`--acta`; default: the program's name with `.acta` added).

1.2 **The faculty is granted first.** The runtime shows the program's `FACULTAS` request. The launching person grants all of it, part of it, or none (`authority.md` §2.4). The grant is the first audit record. Nothing in the program runs before it.

## 2. Evaluation

2.1 **Order.** Statements run in order. In every expression, operands and arguments are evaluated left to right, each exactly once, before the operator or call applies. `ET` and `VEL` stop early: the right side is not evaluated when the left decides the result. An assignment evaluates its right side first, then its place.

2.2 **Numbers.** `NUMERUS` arithmetic is exact or it fails. Overflow, division by zero and remainder by zero are computational errors. `%` takes the sign of the left operand.

2.3 **Sequences.** An index outside the sequence is a computational error. Sequences are values: assigning one copies it.

2.4 **Calls.** Arguments are copied in; the function cannot change the caller's variables. Recursion deeper than the runtime's fixed limit is a computational error.

## 3. Failure classes

`authority.md` §13 names four classes. At runtime they behave as follows:

| Class | Raised by | Can `TENTA` catch it? | If uncaught |
|---|---|---|---|
| 1. Computational error | arithmetic, indexing, recursion depth | yes | the program ends, exit code 1 |
| 2. Denied authority | a missing or exhausted grant, a closed time window, a lapsed seal, an attempt to commit what was not examined | **no** | the program ends, exit code 2 |
| 3. Failed examination | `EXAMINA` | not an error: the verdict is a value | — |
| 4. Judgement | `EXAMINA` | not an error: the destination is a value | — |

3.1 **Leaving a Circle by failure.** When a class 1 or class 2 failure passes out of a Circle, the runtime first dismisses (`DIMITTE`) every invocation still open in it, in reverse order of invocation, and records each. No spirit is ever left bound when the Operator leaves the Circle. [S: the licence to depart is never omitted. O: the automatic dismissal.]

3.2 **Authority is not caught.** A denial cannot be caught and quietly passed over, because the denial is the Constitution's, not the program's. [O]

## 4. Ink, seals and consecration

4.1 **Drawing.** `ATRAMENTUM "classis"` takes one unit from the faculty's grant of that class. When the grant is exhausted, the draw is a denial (class 2).

4.2 **Consecration.** `CONSECRA` turns raw ink into consecrated ink. The act is the Constitution's, not the program's (`authority.md` §7.4): the runtime performs it and records it.

4.3 **Inscription.** `INSCRIBE` spends the consecrated ink and produces the operative seal. A seal lapses when its ink's source is released. Ink drawn from the faculty lasts as long as the program.

## 4a. The apparatus at runtime

`GERE` records the instrument as held, with the time it was taken up; Berith's hour counts from it. `OFFER` spends one unit of the spirit's ink class, or is a denial for Malphas. `CONSTRINGE` performs the constraint (escalation step 3) at once, by the named name, and records it. `APPELLA` and `VETA` are recorded commands. Their effects on the will are those of `spirits.md` §4.4.

**Illusory and veiled work** spends bat's-blood ink, class `"vespertilio"`, one unit per request. Andras's testing is the first such office (`spirits.md` §4.5).

**The kings at runtime.** `INVOCA REGEM r` is refused (class 2) outside the four kings' hours: "à tertia hora usque ad meridiem, à nona hora usque ad vesperas" (Weyer ch. III). Within them, the runtime performs the Invocation of the King in its source form, by the names it cites (El, Iehovah Tetragrammaton, Primeumaton, all in `nomina.md`), and records it. It has three effects for the rest of the Circle:
- a President under that king may be invoked (static rule A13), though still never at nightfall;
- escalation step 4, the invocation of the king, is already done for every spirit under him;
- nothing more. The invocation *asks* the king to send the spirit "without harm" and "to answer truly and faithfully", but no witness says the request is granted, so it closes no outcome of the will. After it, "he will come without doubt, if not at the first or second time of rehearsing" (G): `RECUSATIO` stays open until the Spirits' Chain.

Escalation step 4 for a spirit whose king is not stated is recorded as skipped: "he is bound in chains in hell, and … not in the custody of his King" is the sources' own case for going straight on to the Chain (G).

## 5. Invocation and the will

5.1 **The time window is checked first.** `INVOCA` is refused (class 2) outside the spirit's binding hours by rank, or on a day of the Moon's age not allowed (`authority.md` §8.3). How the runtime reads the conditions:

| Condition (source) | Runtime reading |
|---|---|
| Weyer's canonical hours | **temporal hours** at the declared place: daylight from sunrise to sunset divided into twelve equal hours. *Hora prima* begins at sunrise; *tertia* at the start of the third hour; *meridies* at noon, the end of the sixth; *nona* at the start of the ninth; *vesperae* at sunset. [O: the reading of canonical terms as temporal hours; Sloane's clock hours are the same points at the equinox] |
| *completorium* (compline), *crepusculum noctis* (nightfall) | the end of civil twilight. [O] |
| *finis diei* (Marquises) | the next sunrise, as Sloane 3825 reads it ("till sunrising") |
| sunrise, noon, sunset, dawn | computed for the declared place and date |
| twilight | civil twilight. [O: the sources do not define it] |
| the Moon's age 2, 4, 6, 8, 10, 12, 14 | whole days since the astronomical new moon |
| Earls: "in woods or places where men resort not, or where no noise is" | **[O, for approval]** no other invocation is open in the program |
| Dukes: "in clear weather" | **[O, for approval]** no `TEMPESTAS` effect is active |
| Presidents, at any hour (Weyer, ruling 2026-11-10) | refused unless the king he obeys is invoked, and always at nightfall. A pure President is refused unless `INVOCA REGEM` of his king came first in the Circle. The witnesses name the king for only four spirits (Agares, Asmoday, Gaap, Seere: Amaymon); see `docs/registers/reges.md` |

A spirit of two ranks may be invoked when **either** rank's window is open. [O; the mixed-rank question stays open]

5.2 **Refusal and escalation.** The spirit's will is drawn (§5.4). If it chooses `RECUSATIO`, the runtime performs the next step of the escalation in `spirits.md` §5, records it, and draws again. Each step past the first spends one unit of the seal's ink class. After the seventh step, the greater curse, the spirit is banished (`RELEGA`). The invocation still exists, but it is relegated: requests to it yield nothing, and releasing it again is recorded as already done.

5.3 **Requests.** `PETE` draws the will again, among the outcomes the register leaves open:

| Outcome | What the proposal contains |
|---|---|
| `OBSEQUIUM` | the requested work, complete and true |
| `MORA` | the same, delivered only after a delay the runtime records |
| `PARS` | part of the requested work |
| `MENDACIUM` | a lie, in one of the two forms of `spirits.md` §3.1: a substitute answer of the same form (offices that tell), or a false account of the work (other offices, caught by examination). Only spirits whose compelling condition is unmet can choose it. In Annulus the Triangle is always present, so triangle-compelled spirits (Furfur, Shax, Haures) cannot. Belial, Berith and Asmoday can, until their conditions are met with the apparatus (`spirits.md` §4.4). Valefor always can: no condition that compels him is stated |
| `MALITIA` | the requested work **plus**, unasked, a harm the witnesses give the spirit (`spirits.md` §4.1) |
| `RECUSATIO` | nothing; escalation resumes at the constraint (step 3) |

5.4 **The draw.** The runtime draws uniformly among the outcomes still open, from its entropy source or the launch seed. Uniform is [O]: the sources state no likelihoods, so none is favoured. Every draw is recorded with its result. A run given the same seed, clock and place repeats exactly.

## 6. Examination and judgement

6.1 **`EXAMINA` runs the fixed examiner** (`authority.md` §10) over the proposal and gives a verdict:

| Verdict | When | Destination |
|---|---|---|
| `APPROVE` | complete, within the requested office, within the faculty, every harmful effect's counterbalance met | `PARADISUS` |
| `HOLD` | incomplete (`PARS`), or an answer that cannot be trusted because its spirit's compelling condition was unmet | `PURGATORIUM` |
| `REJECT` | any effect outside the requested office, any `MALITIA`, anything outside the faculty | `INFERNUS` |

6.2 **The destination applies to the proposal**, not to the spirit. Releasing the spirit stays the Operator's act (`VADE IN PACE`, `DIMITTE`, `RELEGA`), as static rule A9 requires.
- `PARADISUS`: the proposal's effects are committed, all at once or not at all, and permanently.
- `PURGATORIUM`: nothing is committed or destroyed. The proposal may be examined again (after a new `PETE`, it may be completed).
- `INFERNUS`: the proposal's sub-tasks are ended and its results are sealed in the Vessel, irreversibly within the program.

A proposal in `PARADISUS` or `INFERNUS` cannot be examined again; trying is a denial.

6.3 **What examination cannot know.** Examination checks declared behaviour and the conditions the register states (`authority.md` §10.3). It cannot detect a well-formed false answer. That is exactly why lying is closed by **constraint**, as in the sources, and not by checking: a spirit whose compelling condition is met cannot choose `MENDACIUM` at all.

## 7. Release

| Act | Effect |
|---|---|
| `VADE IN PACE` | orderly end: temporary capabilities revoked; any proposal still in `PURGATORIUM` is discarded and recorded |
| `DIMITTE` | the same, with the invocation recorded as ended without completion |
| `RELEGA` | forced end: sub-tasks ended, every held proposal sealed in the Vessel |

Seals inscribed in ink drawn from the invocation lapse with it (`authority.md` §7.3).

## 8. Legions

A spirit's helper tasks (`FAMILIARES`, and the sub-tasks of any office) are capped at his legion count. Exceeding it is a denial. [S: legion counts. O: the cap.]

## 9. The audit

9.1 The runtime writes the audit, not the program; it needs no grant. One record per event, in order:
- the faculty requested and granted;
- every draw, consecration and inscription of ink;
- every invocation, with its time-window check and each escalation step;
- every draw of the will and its outcome;
- every proposal, its verdict and its destination;
- every commit;
- every release.

9.2 Each record names the rite, the deciding witness of each attribute it used, and the chain of origin of every capability (`authority.md` §14).

9.3 The audit file can only be appended to. A program cannot read, write or delete it.

## 10. Open items

- The register of divine names that `NOMEN` and `CONSTRINGE … PER NOMEN` must match, transcribed from the Circle of Solomon (Sloane 3825 f112) and the instruments.
- How the instruments are made (materials, days), and the vestments and the Secret Seal (`spirits.md` §4.4).
- The four kings as a register, for the Presidents' twilight rule and for Paimon's attendant kings.
- Signatures for each office kind (what `PETE` passes and what the proposal holds).
- The owner's approval of the two [O] readings of the Earls' and Dukes' conditions (§5.1).
