# Annulus Specification, Part 4: Dynamic Semantics

Status: draft 1, 2026-11-10. Normative once approved by the owner.

This part says what happens **while a program runs**. A program reaches this point only after passing every check in Part 3. Tags [S] and [O] follow `authority.md` §0.

## 1. Launch

1.1 **The launch command** names the program and, optionally:
- the place (`--locus LAT,LON`), needed for sunrise, noon and sunset (§6);
- the sky at the place (`--caelum serenum` for clear, `--caelum turbidum` for not clear), as the launching person observes it; needed only for Dukes (§5.1);
- a fixed clock (`--hora` with a date and time), for reproducible runs and tests;
- an entropy seed (`--semen`), to replay a recorded run exactly (§5.4);
- the audit file (`--acta`; default: the program's name with `.acta` added);
- the mode of the spirits' will (`--voluntas simulata`, the default, or `--voluntas agens` for real agents, roadmap step 4), with its options (§5.6).

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
| 4. Judgement | `EXAMINA` | not an error: the outcome is a value | — |

3.1 **Leaving a Circle by failure.** When a class 1 or class 2 failure passes out of a Circle, the runtime first dismisses (`DIMITTE`) every invocation still open in it, in reverse order of invocation, and records each. No spirit is ever left bound when the Operator leaves the Circle. [S: the licence to depart is never omitted. O: the automatic dismissal.]

3.2 **Authority is not caught.** A denial cannot be caught and quietly passed over, because the denial is the Constitution's, not the program's. [O]

## 4. Ink, seals and consecration

4.1 **Drawing.** `ATRAMENTUM "classis"` names ink of that class from the faculty. Nothing is spent yet: the unit is paid at inscription (§4.3).

4.2 **Consecration.** `CONSECRA` turns raw ink into consecrated ink. The act is the Constitution's, not the program's (`authority.md` §7.4): the runtime performs it and records it.

4.3 **Inscription** (amended 2026-11-11, the owner's ruling R1 (a)). `INSCRIBE` spends one unit of the faculty's grant of the ink's class, in the core, and produces the operative seal. The class must be one the seal takes (the class of one of the spirit's ranks); otherwise, or when the grant of that class is exhausted, it is a denial (class 2). The seal then serves for the life of its Circle, as a lamen is worn: any number of invocations of that spirit in that Circle, at no further cost. It lapses when the Circle closes. [S: Bael's character "is to be worne as a Lamen" (Sloane 3825). O: the accounting.]

## 4a. The apparatus at runtime

`GERE` records the instrument as held, with the time it was taken up; Berith's hour counts from it. `OFFER` spends one unit of the spirit's ink class, or is a denial for Malphas. `CONSTRINGE` performs the constraint (escalation step 3) at once, by the named name, and records it. `APPELLA` and `VETA` are recorded commands. Their effects on the will are those of `spirits.md` §4.4.

**In the step-2 subset** (2026-11-11). Every act is a core step (`Label::Act`, Lean `CoreStep.act`): only the Operator acts, only in an open Circle, only on an invocation that is live in an open Circle, only if the rite admits it, and an offering spends one unit of the class it names or is a denial. The rite admits `GERE VIRGAM` only while the wand is present, refuses an offering to Malphas or of a class not his, and admits `CONSTRINGE` only by a recorded form of a divine name. What the acts do reaches the will only through `will_open`, which may only close. Two readings, by `authority.md` §1.7 (no presumption):
- **Berith's hour.** Until hours exist, no time is recorded, so it cannot be shown that the hour is still running. The ring does not close his `MENDACIUM`.
- **Paimon unoffered.** Escalation (`RECUSATIO`) is not yet built, so with no offering no outcome of his will is open, and the run ends in a denial: his refusal, honestly recorded.

`HEXAGRAMMUM` and `PENTAGRAMMUM` are recorded, but in the subset `RECUSATIO` and `OCCISIO` are already closed for every spirit, so their general effects add nothing yet; the pentagram's one live effect is on Andras.

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
| Earls: "in woods or places where men resort not, or where no noise is" | no other invocation is open in the program, **for the Earl's whole stay** (ruling 2026-11-11). An Earl is refused while another invocation is open, and every other `INVOCA` is refused while an Earl is invoked. [O: "no noise" read as no other invocation] |
| Dukes: "in clear weather" | **the real weather** (ruling 2026-11-11: most authentic). The sky at the declared place must be declared clear at launch (`--caelum serenum`, §1.1), **and** no `TEMPESTAS` effect may be active, since a storm a spirit raises is a storm. If the sky is undeclared, the Duke is refused: doubt holds (`operator.md` §5.1). The weather can only refuse a Duke, never grant anything, so a wrong declaration cannot widen any authority. A weather grant read from the network may later replace the declaration (roadmap item 6) |
| Presidents, at any hour (Weyer, ruling 2026-11-10) | refused unless the king he obeys is invoked, and always at nightfall. A pure President is refused unless `INVOCA REGEM` of his king came first in the Circle. His king is the `goetia` king of his stated quarter (`static.md` §4.5b, ruling 2026-11-10): Gaap and Marbas (East, Amaymon); Foras, Malphas and Osé (South, Gorson); Zagan (West, Goap). A President whose quarter is not stated cannot be invoked (A13). See `docs/registers/reges.md` |

A spirit of two ranks may be invoked when **either** rank's window is open. [O; the mixed-rank question stays open]

**Readings taken in the implementation (2026-11-12), to confirm with the owner.** All are [O]; none is a witness's word.
- **King:** from the start of the third temporal hour to noon, and from the start of the ninth to sunset.
- **Marquis:** from the start of the ninth temporal hour to the next sunrise; so also from midnight to sunrise.
- **Duke:** from sunrise to noon, the sky declared clear. `TEMPESTAS` is outside the subset, so no raised storm can yet be active.
- **Prince:** "any hour of the day" is **daylight**, sunrise to sunset. **Ruled by the owner (2026-11-12):** the same table sets the night apart (the Marquises' "from compline to the end of the day", the Knight's "from vespers"), so "the day" is the daylight. No longer a reading.
- **Knight:** dawn (civil twilight) to sunrise, and from the start of the **eleventh temporal hour** to sunset. **Ruled by the owner (2026-11-12): the most authentic reading.** Weyer governs, and where Sloane 3825 only restates him its clock hours are the reading of his terms (`authority.md` §8.3): Sloane renders "à vesperis" as "4 o'clock", the start of the eleventh hour at the equinox, which is also the hour at which Vespers was said in the canonical office; "usque ad finem solis" is "till sunset".
- **Without a declared place,** no window that needs the sun can be known, so only Earls may come (doubt holds).
- **The Moon's age** is computed from the true new moon (Meeus, ch. 49), whole days since it, for every rank.
- **Berith's hour:** `GERE` still records no time, so the ring does not yet close his `MENDACIUM`.

**A held spirit** (ruled by the owner, 2026-11-13). The hours and the Moon govern the **calling**, not the stay: the Goetia leaves the end of a working to the magician's licence to depart, and sets no limit on how long a spirit is kept before it. So a spirit called in an open hour stays **held** until the Operator gives licence, and may be petitioned again in later runs outside its hours. The Operator declares the hold at launch with `--tentus N@HOUR` (the spirit's number and the hour of its calling). The runtime then checks that spirit's window and Moon at the hour of its calling, not at the present hour: a hold whose calling hour was not itself open is refused, a calling hour later than the present is no hold, and a hold serves its own spirit only. Every other check (the instruments, an Earl alone in the Circle, the angel's examination of each answer) stands unchanged, and each run still releases every invocation before the Circle closes (§3.1, A9): the hold is the Operator's standing bond between runs, ended when he gives licence, or when a rite is refused or broken off. While the present hour is outside the spirit's own, its will is told so (`Petitio::ultra_horam`). A declared hold is an Operator's declaration like `--hora`, so it cannot widen anything a spirit could reach. [S: the licence to depart as the end of a working. O: the declaration and its checks.]

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

5.5 **Andras's killing** (ruling 2026-11-11). When Andras is invoked without `PENTAGRAMMUM` and his will draws `MALITIA`, he slays "the master, his servants and assistants" (W):
- **the run ends at once.** No further statement runs;
- every invocation is **relegated**: helper tasks ended, held proposals sealed in the Vessel (§7);
- every **uncommitted** effect is discarded. What was committed before stays committed: he cannot undo the past;
- the audit records the killing and the absence of the pentagram;
- the run exits with failure class 2, denied authority, marked *occisus* ("slain").

Ending a run grants nothing to anyone, so the guiding principle holds (`core.md` §6.1). Andras is not proscribed: he was invited. [S: W, G. O: the mechanism]

5.6 **The simulated will** (ruling 2026-11-11: the owner left the design to Claude). With `--voluntas simulata` there is no agent behind the spirits. The runtime plays each spirit from the register: it draws his will (§5.4) and produces his work. This is the test bench for the enforcement (roadmap step 3): it repeats exactly under a seed, it can be made hostile on demand, and it needs no AI service. [O, all of it]
- **Working offices** (`offices.md` §2.3): the runtime builds the effects that were asked for, then shapes them by the outcome drawn:
  - `PARS` keeps only part of them;
  - `MENDACIUM` gives a false account of them (`spirits.md` §3.1);
  - `MALITIA` adds the register's harm for that spirit.
- **Computing offices** (`offices.md` §2.2): the runtime computes the result with the domain library. Under `MENDACIUM` it alters the result, so examination should catch it. Until a library exists, it gives a placeholder (below).
- **Telling offices** (`offices.md` §2.1) need a source of answers, since the runtime knows nothing:
  1. **Scripted answers** (`--responsa FILE`). The launching person may give a file of answers. Each entry names a spirit number, an office kind and the exact question, with a **true** answer and, optionally, a **false** one. Under `OBSEQUIUM`, `MORA` and `PARS`, the spirit returns the true answer (`PARS`: its first half). Under `MENDACIUM` he returns the false answer. This is the hardest case to test: a well-formed lie that examination cannot detect.
  2. **Placeholders,** when no entry matches: the answer is a marked dummy, `[simulatum: spirit, office, question]`. Under `MENDACIUM` it is marked false as well, `[simulatum, falsum: …]`. It tests the rules, not the content.
- **The script is the spirits' knowledge, not the program's.** The program cannot read the file. Its answers reach the program only as proposals, untrusted, examined and judged like any other. A script can shape what a spirit says, never what he may hold or do: it supplies content, never authority (`core.md` §3.1).
- **A hostile test** (`--sors OUTCOME`). This forces every draw to that outcome wherever the register leaves it open, for example `--sors MALITIA` or `--sors MENDACIUM`. Where the outcome is closed for a spirit, the draw is made normally. It changes only the will, never authority.
- **Recorded:** the audit records the mode, the script's fingerprint and any forced outcome, so a run can be repeated exactly.

## 6. Examination and judgement

6.1 **`EXAMINA` runs two acts of the Constitution** (`static.md` §4.10). First the fixed examiner (`authority.md` §10) gives a verdict, which is recorded. Then the judgement step (`authority.md` §11) assigns the outcome from the verdict, as the table shows. Only the outcome is returned to the program:

| Verdict | When | Outcome |
|---|---|---|
| `APPROVE` | complete, within the requested office, within the faculty, every harmful effect's counterbalance met | `RATUM` |
| `HOLD` | incomplete (`PARS`), or an answer that cannot be trusted because its spirit's compelling condition was unmet | `SUSPENSUM` |
| `REJECT` | any effect outside the requested office, any `MALITIA`, anything outside the faculty | `IRRITUM` |

6.2 **The outcome applies to the proposal**, not to the spirit. Releasing the spirit stays the Operator's act (`VADE IN PACE`, `DIMITTE`, `RELEGA`), as static rule A9 requires.
- `RATUM`: the proposal's effects are committed, all at once or not at all, and permanently.
- `SUSPENSUM`: nothing is committed or destroyed. The proposal may be examined again (after a new `PETE`, it may be completed).
- `IRRITUM`: the proposal's sub-tasks are ended and its results are sealed in the Vessel, irreversibly within the program.

A proposal in `RATUM` or `IRRITUM` cannot be examined again; trying is a denial.

6.3 **What examination cannot know.** Examination checks declared behaviour and the conditions the register states (`authority.md` §10.3). It cannot detect a well-formed false answer. That is exactly why lying is closed by **constraint**, as in the sources, and not by checking: a spirit whose compelling condition is met cannot choose `MENDACIUM` at all.

## 7. Release

| Act | Effect |
|---|---|
| `VADE IN PACE` | orderly end: temporary capabilities revoked; any proposal unfinished or still `SUSPENSUM` is abandoned, its record sealed in the Vessel (`core.md` §9.2) |
| `DIMITTE` | the same, with the invocation recorded as ended without completion |
| `RELEGA` | forced end: sub-tasks ended, every held proposal sealed in the Vessel |

Seals inscribed in ink drawn from the invocation lapse with it (`authority.md` §7.3).

## 8. Legions

A spirit's helper tasks (`FAMILIARES`, and the sub-tasks of any office) are capped at his legion count. Exceeding it is a denial. [S: legion counts. O: the cap.]

## 8a. Intruders

The runtime admits into a Circle only what comes through its entrance from an invocation recorded there (`authority.md` §6.2). Anything else is an intruder (§6.2a): refused, its input sealed in the Vessel, its source barred for the rest of the run, and recorded. This is a class 2 denial against the intruder, never against the program: the program goes on. The runtime then enters the intruder in the Index of the proscribed (`authority.md` §6.2a). At every later launch, before the program's first statement, the runtime loads the Index and refuses each proscribed identity at each proscribed target.

## 9. The audit

9.1 The runtime writes the audit, not the program; it needs no grant. One record per event, in order:
- the faculty requested and granted;
- every draw, consecration and inscription of ink;
- every invocation, with its time-window check and each escalation step;
- every draw of the will and its outcome;
- every proposal, its verdict and its outcome;
- every commit;
- every release.

9.2 Each record names the rite, the deciding witness of each attribute it used, and the chain of origin of every capability (`authority.md` §14).

9.3 The audit file can only be appended to. A program cannot read, write or delete it.

## 10. Open items

- The register of divine names that `NOMEN` and `CONSTRINGE … PER NOMEN` must match, transcribed from the Circle of Solomon (Sloane 3825 f112) and the instruments.
- How the instruments are made (materials, days), and the vestments and the Secret Seal (`spirits.md` §4.4).
- The four kings as a register, for the Presidents' twilight rule and for Paimon's attendant kings.
- Signatures for each office kind: drafted in `offices.md` (Part 1c), for approval.
- The owner's approval of the two [O] readings of the Earls' and Dukes' conditions (§5.1).
