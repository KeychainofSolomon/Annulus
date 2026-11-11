# Annulus Specification, Part 0: The Core and the Rite Layer

Draft 1, 2026-11-11. For the owner's approval. Roadmap step 1, item 2.

## 0. Why the split

Annulus has two kinds of rule, and they are checked in two different ways:
- **The core** says what is *possible*. It is small, and it is **proven**: a machine-checked proof over a formal model (roadmap step 3), plus attack testing. Tradition does not decide it.
- **The rite layer** says what the work *means*: registers, offices, hours, seals, ink classes, apparatus, the will. It is data and policy, judged by the **sources** under the owner's rule (`provenance.md`). The proof does not cover it.

The split is what keeps the project safe as it grows. A mistake in the rite layer, such as a misread manuscript or a wrong office, can make Annulus less faithful. It can never make it less safe, because the rite layer has no way to add authority (§3). [O]

## 1. The core

The core is these nine things and nothing else:

| # | Core element | Defined in |
|---|---|---|
| C1 | **Capabilities and the faculty** *G*: the single root of authority, granted at launch; no authority without a recorded chain to *G* (law ORIGIN). Grants reserved for angels are part of *G* (`authority.md` §12.1a) | `authority.md` §1.1, §2.4 |
| C2 | **No self-granting, self-approval or self-minting** | `authority.md` §1.2–1.4 |
| C3 | **No amplification** (the guiding principle): every capability exercised is in *G*; every delegation is contained in what it came from, at any depth | `authority.md` §1.5 |
| C4 | **The Circle boundary**: rite values stay in their Circle; one entrance; inner Circles only narrow; intruders hold nothing and go to Hell | `static.md` §4.1, §4.7, §4.12; `authority.md` §6.1–6.2a |
| C5 | **The Triangle and the handed set**: a spirit holds exactly what its request handed it (`offices.md` §3), its helpers a subset of that (`offices.md` §4) | `authority.md` §6.4; `offices.md` §3–4 |
| C6 | **Commits**: only the runtime performs effects; all at once or not at all; nothing commits without a verdict from an examiner other than its author, and a destination from the judgement step | `authority.md` §3 (EXAMINATION, JUDGEMENT), §11; `dynamic.md` §6 |
| C7 | **Release**: every invocation ends exactly once; its capabilities are revoked; seals in its ink lapse | `static.md` §4.11; `dynamic.md` §7 |
| C8 | **The three stores**: the audit (append-only, written by the runtime), the Vessel (never opened for use) and the Index of the proscribed (persists across runs) | `dynamic.md` §9; `authority.md` §6.2a |
| C9 | **The Operator's discipline over grants**: no widening while any spirit is invoked; consequential acts only outside the session; doubt holds; two keys | `operator.md` §1, §4, §5 |

The core also owns the **mechanics** that the rite fills with content:
- the effect verbs the runtime can commit (`offices.md` §2: `crea`, `dele`, `transfer` …) and the permissions they need (`static.md` §4.9);
- ink as a **budget** drawn from *G* and spent, never created;
- taint, and consecration by a validator other than the user (`authority.md` §7.4);
- the failure classes (`authority.md` §13), and the rule that authority failures cannot be caught (`static.md` §3.11).

## 2. The rite layer

Everything else is rite:
- **the registers:** spirits, ranks, legions, offices, seals, veracity, malice, counterbalances, quarters, kings, divine names (`docs/registers/`);
- **the rites themselves and their precedence** (`authority.md` §4–5; `provenance.md`);
- **the mapping of offices to computer work** (`spirits.md` §6) and of each office kind to its signature (`offices.md` §2);
- **the will** and its outcomes (`spirits.md` §2–3; `dynamic.md` §5.3–5.4);
- **the conditions of invocation:** hours, the Moon's age, Earls, Dukes, Presidents and kings, the apparatus (`authority.md` §8.3; `dynamic.md` §5.1, §4a; `spirits.md` §4.4);
- **refusal and escalation** (`spirits.md` §5);
- **seals and ink classes:** exemplar and operative forms, metals by rank, bat's blood (`authority.md` §7.1–7.2, §7.5);
- **who examines:** Rudd's pairings, examiners by office (`authority.md` §10.1–10.2), and the rite's own checks (counterbalances, malice);
- **the guardians** (`operator.md` §3) and **real-agent mode** (`operator.md` §6);
- **angels** (`authority.md` §12).

## 3. The interface: the rite can only restrict

3.1 **The rite layer talks to the core in exactly these ways.** Each one can refuse, hold, spend or bound. None can grant.

| What the rite supplies | What the core does with it | Why it cannot add authority |
|---|---|---|
| **admission** of an `INVOCA`, `INVOCA REGEM` or apparatus act: allow or refuse (hours, Moon, Earls, Dukes, kings, required instruments, allegiance) | refuses on any "no" | a refusal only removes |
| **a signature** for each office kind and mode: the (place, permission) pairs a request needs, and the verbs its effects may use | checks every pair against *G* before the program runs (`offices.md` §3.2), and every effect at commit (§3.3 below) | a signature can only *ask for* pairs; the core grants only those in *G* |
| **an examiner** for each spirit | requires that it is not the author (C2) | it can only approve what the core then re-checks |
| **a verdict**: `APPROVE`, `HOLD` or `REJECT` | maps it to a destination (C6) | `APPROVE` is necessary, never sufficient (§3.3) |
| **bounds**: legions, ink costs, lifetimes, escalation steps | enforces them as upper limits, or spends them from *G* | a bound only caps; spending only removes |
| **the will's draw** | gives the proposal its content | the will decides what a spirit offers, never what it may hold |
| **counterbalances and extra checks** (Focalor's prohibition, Vepar's reversibility, `TUTELA`) | can only turn `APPROVE` into `HOLD` or `REJECT`, or refuse an effect | they only remove |

3.2 **Monotonicity.** Formally: for any two rites *R* and *R′*, where *R′* adds restrictions to *R*, every run allowed under *R′* is allowed under *R*. And for **every** rite, even a wrong or a malicious one, the guiding principle (C3) holds. So the proof is done once, over the core, with the rite treated as an unknown that can only answer the questions in §3.1. [O]

3.3 **The core re-checks at commit.** Whatever the verdict, before any effect is performed, the core itself checks that the effect is in the request's handed set, and so in *G*. If not, nothing commits and the proposal goes to `INFERNUS`. An examiner, however wrong, can never commit something outside *G*. (This was "defence in depth" in `offices.md` §3.3. It is now a core rule.)

3.4 **What the core does not promise.** The core guarantees that no program exceeds its grant. It does **not** guarantee that a program does no harm *within* its grant. A spirit granted `delere` on a file, and approved, really deletes it. Stopping harm within the grant is the rite's work: examination, counterbalances, two keys. This is stated so that no one reads the proof as more than it is.

## 4. Proof obligation 2, restated

The Constitution's own acts (consecration, examination, judgement, the invocation of a king, escalation) are rite **content** run by core **mechanics**. Under §3 they can spend only from *G* (ink, time) and can only refuse, hold or bound. So proof obligation 2 (`authority.md` §1.5) reduces to checking that each of them uses only the interface of §3.1.

## 5. What this changes for the next steps

- **Roadmap step 2, the minimal interpreter:** build the core first, as its own module, with the rite reaching it only through the interface of §3.1. The registers are loaded as data. This is what keeps the code out of spaghetti: tradition can grow without touching the core.
- **Step 3, the proof:** covers the core and §3 only.
- **Item 3, amending the Constitution:** a change to the core needs a new proof; a change to the rite needs a source check. The amendment procedure can follow this line.

## 6. Collisions found while splitting (for the owner)

Writing the interface brought up two rite rules that, read literally, would act outside it. The guiding principle wins until the owner rules (`authority.md` §1.5).

1. **Andras without the pentagram.** `spirits.md` §4.5 says that without the pentagram "his killing reaches the real program". Under the core, nothing reaches the real program without examination and a commit inside the handed set (C6, §3.3). The case also cannot be written, since A11 requires the pentagram. **Proposed reading:** the sources' warning is kept as the reason for A11. In Annulus, an attempt on the real program is outside his office, so examination rejects it and it goes to `INFERNUS`, as §4.5 item 6 already says. `spirits.md` is updated to say so.
2. **Angels' authority. Ruled 2026-11-11:** standing from God, reach from the grant, with extended privileges that the grant gives to angels alone (`authority.md` §12.1a). A grant ending in `angelis` is in *G* but reserved: no spirit and no program operation may use it. So the angels' privileges are wider than any spirit's or the program's own, and still inside *G*. C1 and C3 hold.

## 7. Map: which layer each section belongs to

C = core, R = rite, C+R = core mechanics with rite content.

| File | Section | Layer |
|---|---|---|
| `authority.md` | §0 conventions | — |
| | §1 axiom, §2 Constitution, §3 laws | C |
| | §4 rites, §5 entities and registers | R |
| | §6.1–6.2a Circle, entrance, intruders, Index, Vessel | C |
| | §6.3 Circle forms, §6.4 placement and figures, §6.5 angels' channels | R |
| | §6.4 isolation and untrusted output | C |
| | §7.1–7.2 seals, §7.5 ink classes, §7.6 tempering | R |
| | §7.3 ink budget, §7.4 taint and consecration | C+R |
| | §8 invocation | C+R (steps C; hours, ranks and kings R) |
| | §9 release | C |
| | §10 examination | C+R (independence C; pairings R) |
| | §11 judgement | C |
| | §12 angels | R; §12.1a reserved grants C |
| | §13 failure classes, §14 audit | C |
| `static.md` | §1 phases, §2 names, §3 types | C |
| | §4.1, §4.7, §4.9–4.12 containment, faculty, examination, release, nesting | C |
| | §4.2–4.6, §4.8, §4.13 registers, apparatus, kings, office, seals, divine names | R |
| | §6 error codes | C+R |
| `dynamic.md` | §1–3 launch, evaluation, failures | C |
| | §4, §4a ink, apparatus | C+R |
| | §5 invocation and the will | R |
| | §6 examination and judgement, §7 release | C |
| | §8 legions | C+R (the cap C; the numbers R) |
| | §8a intruders, §9 audit | C |
| `spirits.md` | all | R |
| `operator.md` | §1, §2, §4, §5 | C |
| | §3 guardians, §6 real agents | R |
| `offices.md` | §1, §3, §4 | C |
| | §2, §5, §6 | R |
| `grammar.md` | all | C (syntax carries no authority) |
| `provenance.md` | all | R |
| `docs/registers/` | all | R |
