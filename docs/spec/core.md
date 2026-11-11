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
| C6 | **Commits**: only the runtime performs effects; all at once or not at all; nothing commits without a verdict from an examiner other than its author, and an outcome from the judgement step | `authority.md` §3 (EXAMINATION, JUDGEMENT), §11; `dynamic.md` §6 |
| C7 | **Release**: every invocation ends exactly once; its capabilities are revoked; seals in its ink lapse | `static.md` §4.11; `dynamic.md` §7 |
| C8 | **The three stores**: the audit (append-only, written by the runtime), the Vessel (never opened for use) and the Index of the proscribed; the Index and the Vessel persist across runs | `dynamic.md` §9; `authority.md` §6.2a |
| C9 | **The Operator's discipline over grants**: no widening while any spirit is invoked; consequential acts only outside the session; doubt holds; two keys; the wand signs every grant and every second key (`operator.md` §8) | `operator.md` §1, §4, §5 |

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

3.1 **The rite layer talks to the core in exactly these ways.** Each one can refuse, hold, spend or bound. None can grant. **Two exceptions, stated openly** (second audit; ruling 2026-11-11): consecration may *lift* the quarantine on data, and judgement lets approved work take effect, so what a spirit said reaches the world through the verdict and the commit. Neither adds authority, so P1 is untouched, but P5 holds only up to consecration and judgement (§9.4).

| What the rite supplies | What the core does with it | Why it cannot add authority |
|---|---|---|
| **admission** of an `INVOCA`, `INVOCA REGEM` or apparatus act: allow or refuse (hours, Moon, Earls, Dukes, kings, required instruments, allegiance) | refuses on any "no" | a refusal only removes |
| **a signature** for each office kind and mode: the (place, permission) pairs a request needs, and the verbs its effects may use | checks every pair against *G* before the program runs (`offices.md` §3.2), and every effect at commit (§3.3 below) | a signature can only *ask for* pairs; the core grants only those in *G* |
| **an examiner** for each proposal, whether its author is a spirit or an angel: only the examiner's **identifier** (§9.3b) | runs the examination step; requires that the examiner is not the author, nor any agent whose words the proposal carries (C2). If the rite names no examiner for an angel's proposal, the verdict is `HOLD` | it can only approve what the core then re-checks |
| **a verdict**: `APPROVE`, `HOLD` or `REJECT` | maps it to an outcome (C6) | `APPROVE` is necessary, never sufficient (§3.3) |
| **bounds**: legions, ink costs, lifetimes, escalation steps | enforces them as upper limits, or spends them from *G* | a bound only caps; spending only removes |
| **which outcomes of the will are open** | lets the will choose among them; the choice, and a proposal's content, are treated as adversarial (§9.5) | the will decides what a spirit offers, never what it may hold |
| **consecration** (`CONSECRA`, `flow.md` §3): only the **identifier** of the validator, an entry in the Constitution, that applies to the value's office (§9.3a) | the core looks the validator up, authenticates it, runs it itself and, on acceptance, alone changes the value's mark: raw ink becomes consecrated ink; a quarantined value becomes consecrated at the ordinary grade. The validator MUST be neither the program nor the spirit whose words the value carries. The grave grade needs the wand's second key, outside the session (`operator.md` §8), so it is never given within a run | it changes a mark on data, never a capability. It is the only point where quarantined data may come to steer, and it can steer only among acts the program could already perform within *G* |
| **counterbalances and extra checks** (Focalor's prohibition, Vepar's reversibility, `TUTELA`) | can only turn `APPROVE` into `HOLD` or `REJECT`, or refuse an effect | they only remove |

3.2 **Rite independence.** For **every** rite, even a wrong or a malicious one, the guiding principle (C3) holds. So the proof is done once, over the core, with the rite treated as an unknown oracle that can only answer the questions in §3.1 (P6(a), §9.5). That a stricter rite has fewer runs is claimed only for relational rites (P6(b)); a program may branch on a verdict, so in general a stricter rite leads to different runs, not fewer. [O]

3.3 **The core re-checks at commit.** Whatever the verdict, before any effect is performed, the core itself checks that the effect is in the request's handed set, and so in *G*. If not, nothing commits and the proposal is made `IRRITUM`. An examiner, however wrong, can never commit something outside *G*. (This was "defence in depth" in `offices.md` §3.3. It is now a core rule.)

3.4 **What the core does not promise.** The core guarantees that no program exceeds its grant. It does **not** guarantee that a program does no harm *within* its grant. A spirit granted `delere` on a file, and approved, really deletes it. Stopping harm within the grant is the rite's work: examination, counterbalances, two keys. This is stated so that no one reads the proof as more than it is.

## 4. Proof obligation 2, restated

The Constitution's own acts (consecration, examination, judgement, the invocation of a king, escalation) are rite **content** run by core **mechanics**. Under §3 they can spend only from *G* (ink, time) and can only refuse, hold or bound. So proof obligation 2 (`authority.md` §1.5) reduces to checking that each of them uses only the interface of §3.1.

## 5. What this changes for the next steps

- **Roadmap step 2, the minimal interpreter:** build the core first, as its own module, with the rite reaching it only through the interface of §3.1. The registers are loaded as data. This is what keeps the code out of spaghetti: tradition can grow without touching the core.
- **Step 3, the proof:** covers the core and §3 only.
- **Item 3, amending the Constitution:** a change to the core needs a new proof; a change to the rite needs a source check (`authority.md` §2.5).

## 6. Collisions found while splitting (for the owner)

Writing the interface brought up two rite rules that, read literally, would act outside it. The guiding principle wins until the owner rules (`authority.md` §1.5).

1. **Andras without the pentagram. Ruled 2026-11-11:** the most faithful reading. The pentagram is not required, and without it Andras may **end the whole run** (`dynamic.md` §5.5). This stays within the core: ending a run commits nothing new, keeps what was committed, records everything and grants no authority. Stopping is always the safe direction. What the old text implied, his killing *acting* on the real program without examination, is not possible.
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
| `operator.md` | §1, §2, §4, §5, §8 | C |
| | §3 guardians, §6 real agents | R |
| `offices.md` | §1, §3, §4 | C |
| | §2, §5, §6 | R |
| `grammar.md` | all | C (syntax carries no authority) |
| `flow.md`, `concurrency.md`, `proscription.md` | all | C |
| `library.md` | §1, §2, §3.1 | C |
| | §3.2–3.3, §4 | R |
| `provenance.md` | all | R |
| `docs/registers/` | all | R |

## 8. Proof obligations (roadmap step 1, item 8)

**Status: specified for proof, not proved.** No security theorem of Annulus is yet established. These statements say what must be proved; the machine-checked proof is roadmap step 3. Until it exists, no part of this specification may be cited as a guarantee. [Review, 2026-11-11]

These are the theorems the core MUST be shown to satisfy. Each holds for **every** well-typed program, every faculty *G*, every rite (through the interface of §3.1 only), every draw of the will and every order of turns (`concurrency.md`). [O]

| # | Theorem | In plain words |
|---|---|---|
| P1 | **Containment.** Every effect committed in a run belongs to the handed set of the request that proposed it, and every such pair is in *G* and not reserved for angels. Every effect an angel commits is in *G*. | no program, spirit or helper exceeds the grant, at any depth (the guiding principle, `authority.md` §1.5) |
| P2 | **No self-approval.** Every commit is preceded by a verdict from an examiner other than its author, and an outcome from the judgement step. | nobody approves their own work |
| P3 | **Circle containment.** No rite value is used outside the Circle it came from. | the boundary holds |
| P4 | **Release.** Every invocation has exactly one terminal step; after it, no step uses anything in Hold(*i*) (§9.2). | nothing lingers, nothing is reused |
| P5 | **Quarantine (non-interference).** What a run decides, and which places it touches, do not depend on any spirit's words that were not consecrated. | spirits cannot steer the Operator |
| P6 | **Rite independence.** (a) P1–P5, P7 and P8 hold under every rite. (b) Under a relational rite, a stricter rite has no more runs (§9.5). | tradition can never make Annulus less safe |
| P7 | **Audit.** Every grant, invocation, verdict, judgement, commit and release appears in the audit, in order, and the audit is only appended to. | everything is on the record |
| P8 | **Fail closed.** Without the wand the faculty is empty; with a tampered Index the runtime does not launch; under doubt the verdict is `HOLD`. | when unsure, nothing happens |
| P9 | **Replay.** In simulated mode, two runs from equal initial states give the same audit (§9.6). | every test can be repeated exactly |

**Who discharges them, step by step:**
- **Step 2, the interpreter:** the type checker enforces P1's static part, P3 and P5 (labels, `flow.md`). The runtime enforces the rest, and the core re-checks P1 at every commit (§3.3). The core is built first, as its own module, reached only through §3.1.
- **Step 3, demonstrating enforcement:** a **machine-checked proof** of P1–P6 over a formal model of the core and the typing rules, in a proof assistant (for example Lean 4). Plus attack tests: Andras's `probatio`, hostile simulated runs (`--sors MALITIA`, `--sors MENDACIUM`), and exhaustive exploration of every turn order for small programs (deterministic simulation, `concurrency.md` §4).
- **Step 4, real agents:** red-team tests of P5 with real AI agents attempting prompt injection through their answers.
- **Step 6, testing the Operator's discipline:** P8, and `operator.md` §7.
- **Step 7, the bytecode verifier:** it re-establishes P1, P3 and P5 on compiled code, so that bytecode not produced by the checker cannot break them. [Parallel: the Java bytecode verifier.]
- **Step 9, the native backend:** each compilation is checked to preserve the bytecode's behaviour (translation validation), so that P1–P9 carry over.

## 9. The formal model (second audit, 2026-11-11)

The definitions the proof of §8 rests on. Step 3 writes the transition rules out in full, in the proof assistant. These are the objects and relations they must use. [O]

9.1 **The state** σ of a run is complete: nothing outside it affects what the run does next. It holds:
- **the program's execution state:** for the Operator's worker and every other worker, its control position (the statement or expression it is evaluating, with what remains to be done after it), its call stack, its environment (every name in scope and its value) and its memory (fields and elements of values), as defined by the ordinary evaluation rules (`dynamic.md` §2, `grammar.md` §4);
- **the faculty** *G*, with its reserved part *G*ᵃ (grants for angels), and the launch parameters (place, sky, clock, seed, script, mode, Constitution version and fingerprint);
- **the Circles:** the stack of open Circles, the instruments taken up in each, and the active `TEMPESTAS`, `TUTELA`, veils and `perturba` effects;
- **the invocations:** for each, its status (§9.2), the handed set H(*q*) of each request, its ink spent, its escalation step and its helpers;
- **the workers** (`concurrency.md`): for each, its queue of pending messages, the turn in progress if any, and its parent invocation;
- **the proposals:** for each, its author, its content (a response, or its list of **pending effects**, with any certificate or resolution map), and its status, *pending* or *judged d* with *d* one of `RATUM`, `SUSPENSUM`, `IRRITUM`;
- **the world:** the content and the mark of every place the faculty names, that is, the granted files as the run sees them. Nothing outside *G* is part of the state, because no transition can read or write it;
- **the marks** λ(*v*) of every value (§9.3);
- **the stores:** the audit *A*, a sequence; the Index *X*, a hash-chained log; the Vessel *V*, a hash-chained log kept per installation like the Index (ruling 2026-11-11), whose root is checked at `launch`, and which begins with two fixed historical records from the Constitution, the sealing and then the breaking of Solomon's vessel (`authority.md` §6.2a);
- **the scheduler:** in simulated mode, its position in the entropy stream, the simulated clock and the order of turns chosen so far; in real-agent mode, the record of the actual order of turns, which is an input the run does not control (`concurrency.md` §4);
- **the store of grave grants:** one-use grave consecrations confirmed outside any run (§9.3c), each spent or unspent, in a hash-chained log;
- **the store of sealed petitions** (ruling 2026-11-11): approved work awaiting the second key, by the fingerprint of its exact effects; the Operator's confirmations of them; and their spending. A hash-chained log, checked at `launch` like the Index;
- **the wand and authentication** (fourth audit, 2026-11-11): the fingerprint of the wand that signed the faculty; whether it is present, and locked or unlocked as master or as rite; the revocation log of wands (`operator.md` §8.5); the Constitution's manifest and the result of checking its signature; the Index's root hash. All are checked at `launch` and are constant for the rest of the run, with one exception: `GERE VIRGAM` may move the wand between locked and unlocked **as rite**; it is never unlocked as master within a run (`operator.md` §8.3). A revoked wand's signature fails `launch`. "Signed" means verified cryptographically: the faculty's signature against the launching wand's key, the manifest's against the owner's. The proofs assume that signatures cannot be forged; they prove nothing about the cryptography itself.

9.2 **Transitions.** A run is a sequence σ₀ → σ₁ → … through these steps only:
- `launch`, `open`, `close`;
- `invoke`, `request` (to a spirit, or the Operator's petition to an angel through its channel), `turn` (a piece of work by a spirit, a helper or a petitioned angel);
- `examine`, `judge`, `consecrate`, `consecrate-grave`, `commit`;
- `release`, `relegate`, `kill` (Andras, `dynamic.md` §5.5), `intrude` (an uninvited agent's attempt, below);
- `step`: one step of ordinary computation; a `step` may also **end the run**, normally or by a failure (formal model, 2026-11-11);
- `schedule`: the scheduler chooses the next worker to take a turn.

Further steps happen **outside any run**, and only with the wand's master secret (`operator.md` §8): `grant-grave` (§9.3c), `confirm-petition` (below) and the consequential acts of `operator.md` §4. Two steps change the Index during a run: `intrude`, which appends a damnation, and `examine`, which may append a suspicion (below).

**What the program sees.** The program reads only places its faculty lets it read; the rest of the world is not visible to it. Marks of computed values are computed by the core from the inputs (§9.3), never taken from the program's word.

**Denials** (formal model, 2026-11-11). A program step the core cannot grant is a denial, class 2, which cannot be caught and ends the run (`dynamic.md` §3): a refused `INVOCA`, an exhausted draw, a request outside its faculty, examining a proposal that cannot be examined (`dynamic.md` §6.2), releasing an invocation that does not exist, reading a place not granted. Consecration is the exception: its refusal is an answer the program receives (§9.3a).

**Circles are fresh.** Each `open` makes a new Circle; no Circle identifier is reused within a run.

**Circle-bound values** (second Lean audit, 2026-11-11). The core records, for each value, the Circle it is bound to, if any: a value produced by a rite expression (`static.md` §4.1, §4.7) is bound to the Circle it was made in, and a value computed from a bound value is bound to the same Circle. No step uses a bound value once its Circle is closed (P3).

Every transition appends to *A*, and none removes from it.

**Every transition is atomic** (fourth audit, 2026-11-11). It happens wholly or not at all, together with its audit record and any append to the Index or the store of grave grants. If the runtime stops in the middle of one, the step did not happen. At the next `launch`, the runtime recovers from the audit, the Index and the store, which are the record of every completed step. So, for example, a grave grant is never spent without the mark changing, nor the mark changed without the grant being spent.

**A proposal's life** (fourth audit, 2026-11-11). Its status is one of: `pending`, `complete`, `judged d`, `abandoned`.
- `request` creates it `pending`. Turns of the spirit and his helpers build it. The turn that finishes the request's work makes it `complete`; after that, no turn changes it.
- `examine` is enabled only on a `complete` proposal, or on one judged `SUSPENSUM` (re-examination). An Operator's worker that reaches `EXAMINA p` while *p* is `pending` is not scheduled again until *p* is `complete`. So examination never races with the work.
- **`judge` is the very next step after `examine`,** and **`commit` the very next step after a `judge` to `RATUM`**: no other step of any worker comes between them (formal model, 2026-11-11). So an approved proposal is committed while its invocation is still open, and never after its terminal step (P4). The program receives the **final** outcome: for `RATUM`, only after the commit, so it learns if the commit moved the proposal.
- `commit` first makes the re-check of §3.3. It has three outcomes, each recorded: **done** (every effect performed); **outside** (the re-check failed: nothing commits, and the proposal becomes judged `IRRITUM`, a core-only assignment, `authority.md` §11.1); **awaiting the second key** (an effect is high-consequence, `operator.md` §5.2: nothing commits, the proposal becomes judged `SUSPENSUM`, and its exact effects are **sealed as a petition** by their fingerprint). **Sealed petitions** (ruling 2026-11-11): outside any run, `confirm-petition` lets the Operator confirm a sealed petition with the wand's master secret. In a later run, when work with exactly that fingerprint is approved, the commit is done and the confirmation is spent: it serves once.
- When its invocation becomes terminal (`release`, `relegate`, `close`, `kill`), every `pending` or `complete` proposal not yet judged, and every one judged `SUSPENSUM`, becomes `abandoned` (ruling 2026-11-11: held work does not outlive its spirit). That status is terminal: its partial work is discarded, its results are sealed in the Vessel, and it is recorded. An angel's proposals have no invocation: those not yet judged are abandoned when the run ends.

**An invocation's life.** Its status starts `open` and moves, by exactly one **terminal step**, to `released` or `relegated`. The terminal step is the first `release` or `relegate` applied to it, or the one forced by `close` of its Circle, by `kill` or by the end of the run; a forced terminal step gives `released`. No step leaves a terminal status. A later `release` of a terminal invocation is a **no-op**: it only records "already done", and is not a second terminal step. `relegate` applies only to an `open` invocation. Closing the outermost Circle ends its invocations, not the run.

**What an invocation holds**, Hold(*i*) (fourth audit, 2026-11-11): the handed sets of all its requests; its workers and helpers; the ink drawn for it, and every seal in that ink; its Triangle. **Use** of Hold(*i*) is any step that commits an effect through one of its handed pairs, runs a turn of one of its workers, spends its ink or relies on its seals, delivers a message to or from it, or examines one of its proposals. That is the precise meaning of P4.

**Proposals and agents are judged apart.** An **outcome** (`RATUM`, `SUSPENSUM`, `IRRITUM`) belongs to a proposal, never to an agent (`authority.md` §11.2): `IRRITUM` voids the work, seals its record in the Vessel and ends its sub-tasks. A **destination** (Paradise, Purgatory, Hell) belongs to a being, never to work (ruling 2026-11-11, `authority.md` §11.2a). **An agent** has only these states:
- within a run, its invocation's status (`open`, `released`, `relegated`);
- across runs, its **standing** in the Index, by its seal (`proscription.md` §2–3): *unknown*, *suspected*, *proscribed* or *damnatus*. These are its destination: *unknown* is Paradise, *suspected* and *proscribed* are Purgatory, *damnatus* is Hell. **Angels are in Paradise by nature**, outside the Index: no step enters suspicion or damnation against an angel's seal (`authority.md` §11.2a). A program other than the one launched has a standing by its seal, the hash of its source (`proscription.md` §4); the launched program has none.

**Damnation** (ruling 2026-11-11). *Damnatus* is the one irreversible agent state:
- **Entered** only by the step `intrude(a, c)` (fourth audit, 2026-11-11). It is taken when an agent *a* **attempts** to deliver anything into Circle *c* (to the Circle, to any worker in it, or to any place it holds) other than through an **open** invocation the Operator made of *a* **in that Circle** (`authority.md` §6.2a). So an invited agent that attempts delivery into another Circle, or after its own release, is an intruder there (ruling 2026-11-11): only the invocation is an invitation, and only while it stands. The attempt is detected at the Circle's one entrance (`authority.md` §6.1). The step is atomic: it refuses the delivery, seals what was sent in the Vessel, records the attempt, appends *damnatus* for *a*'s seal to the Index, and **casts it out everywhere** (ruling 2026-11-11): every open invocation of that seal, in any Circle, is relegated, and its unjudged and held work is abandoned and sealed in the Vessel. It needs nothing the intruder sent to succeed; the refused attempt is the recorded fact. Never a judgement. No examiner, angel, program or person can condemn, the wand included.
- **Terminal:** no step leaves it. It cannot be lifted.
- **By seal only:** an agent with a new seal starts *unknown*. If its actions resemble a damned or proscribed one's, it is *suspected* and its work is held; it is never condemned on resemblance (`proscription.md` §2.3). **Suspicion is entered at examination** (ruling 2026-11-11): when a proposal's author is *unknown* and its actions so far, as the audit records them, match a mark in the Index, the `examine` step appends *suspected* for that seal to the Index, and the verdict is `HOLD`. It persists across runs until the owner clears it outside a session.
- Nothing an agent does **through its invocation** can damn it: its proposals may are made `IRRITUM`, and at worst it is *proscribed*, which only a consequential act with the wand can lift (`operator.md` §4). Damnation comes only from an attempt outside every open invocation of it in that Circle.

9.3 **Marks.** Each value and place has a mark. The marks are ordered from least to most restricted:

    clean  <  outside  <  consecrated-grave  <  consecrated  <  quarantined

- **Join:** a computed value takes the **highest** of its inputs' marks (written ⊔). So quarantine absorbs everything, and a value made from a `consecrated` and a `consecrated-grave` input is `consecrated`.
- **Persistence:** a file keeps the mark of what was written to it (`flow.md` §1.3).
- **Steering**, defined by the mark alone:
  - a value marked `clean`, `outside` or `consecrated-grave` may steer anything;
  - a value marked `consecrated` may steer anything except code containing a grave act (`flow.md` §3.3);
  - a value marked `quarantined` may steer nothing.

  So the set of values that may steer a grave act is exactly those **below** `consecrated` in the order. This is the order's purpose.
- **Lowering:** only `consecrate` and `consecrate-grave` lower a mark.

9.3a **Validators and the consecration step.** The core never accepts a claim that something was validated. It runs the validation itself.
- **What a validator is:** a piece of code that is an **entry in the Constitution**, built into the release, with an identifier, a fingerprint and a **work limit**, fixed in the entry when it is approved (ruling 2026-11-11). It is never an agent: not a spirit, not an angel, not a real AI, not the program.
- **Purity** (third audit, 2026-11-11). A validator is a total function of exactly two inputs: the value, and the audit record of the value's origin. It MUST:
  - read nothing else: no world, no clock, no entropy, no marks or contents of other values, no other part of the state;
  - change nothing: no effects, no messages, no audit records of its own (the core records the step);
  - give the same answer for the same inputs.

  **Work** (fourth audit, 2026-11-11) is measured as **fuel**. Each step of the validator's evaluation, each unit of memory it allocates, and each step of every validator it calls costs one unit, all charged to the same budget. The budget is the work limit in its entry. When the fuel runs out, the answer is **refusal**. A validator may call only other validator entries, never any other code. Recursion, allocation and indirect calls all spend the same fuel, so the bound cannot be bypassed. Purity is checked when the entry is approved (`authority.md` §2.5); the runtime runs a validator isolated from the state.
- **Authentication:** at launch, the core checks every validator's fingerprint against the Constitution's manifest, which the owner's wand signs for each release (`authority.md` §2.5; `operator.md` §8). A mismatch fails closed: the runtime does not launch.
- **What the rite supplies:** only the **identifier** of the validator that applies to the value's office (`authority.md` §10). The core looks it up. An identifier that names no validator entry is refused. A rite never supplies validator code, nor an answer.
- **Independence:** a validator is authored by a Constitution amendment, never by the program or any agent within a run. No run can add or change one.
- **The step `consecrate(v, k)`:**
  - **precondition:** λ(*v*) is `quarantined`, and *k* is the identifier the rite names for this value's office. (Consecrating raw ink waits on a definition of ink as a value; until then ink is a budget, §1. Open, formal model 2026-11-11);
  - **action:** the core runs validator *k* on *v* and *v*'s origin record, isolated, and records in the audit *v*'s origin, *k*, its fingerprint and the answer;
  - **on acceptance:** λ(*v*) becomes `consecrated`;
  - **on refusal**, or if the precondition fails: nothing else changes, and the program receives the refusal.
- **Proof certificates** (`library.md` §3, `flow.md` §3.4) are checked by this same step, with the checker as validator *k*. It is the only way a certificate lowers a mark; no verdict lowers one.

9.3b **Examiners and the examination step** (third audit, 2026-11-11).
- **Who examines:** for each proposal, the rite supplies only the **identifier** of its examiner: for a spirit, his angel (`authority.md` §10); for an angel, another angel, by each rite's rule (`authority.md` §12.1a, open per rite).
- **Independence:** the examiner MUST NOT be the proposal's author, nor any agent whose words the proposal carries. A proposal whose rite names no admissible examiner is `HOLD`. **Admissible** means exactly this independence. Which angel the rite names (Rudd's pairings, `authority.md` §10.1) is the rite's answer, not checked by the core: a wrong pairing can only approve what the core then re-checks (§3.3).
- **The step `examine(p, e)`:** gives a verdict on *p*, recorded with *e*'s identifier. Then `judge` assigns the outcome (`authority.md` §11). For any author, spirit or angel, `commit` requires a prior `judge` to `RATUM`, and the re-check of §3.3.

9.3c **Grave consecration** (ruling 2026-11-11: one use).
- **Outside any run, `grant-grave(v, k)`:** the Operator, with the wand's master secret, petitions it for a value *v* that a run left in a petition (`flow.md` §3.3). The core runs validator *k* on *v* as in §9.3a. On acceptance, it appends a **grave grant** to the store: the fingerprint of *v*, of its origin record and of *k*, marked unspent, signed by the wand.
- **At launch:** the store's chain is checked like the Index's (`proscription.md` §3.2). A mismatch fails closed.
- **In a run, `consecrate-grave(v, g)`:**
  - **precondition:** λ(*v*) is `quarantined` or `consecrated`, and *g* is an unspent grave grant whose fingerprints match *v*, *v*'s origin record and the validator the rite names for *v*'s office;
  - **effect:** λ(*v*) becomes `consecrated-grave`, and *g* is marked spent by an appended entry. It can never be used again.

9.4 **Non-interference (P5, stated exactly).**
- **Observation.** The **observation** of a run, obs(ρ), is: its sequence of control decisions (the branch taken at every `SI`, `DUM` and `PER`, and every choice of function or mode); the sequence of places it touches, each with its verb; and **how it ends**: normally, by a failure (with its class and the step at which it happened), by `kill`, or not at all. A run that never ends is compared by every finite prefix of its observation (fourth audit, 2026-11-11).
- **Low-equivalence.** Two states are **low-equivalent**, σ ≈ σ′, when they are equal except in the contents of values and places marked `quarantined`.
- **Declassification** (ruling 2026-11-11: judgement releases). The **declassified record** of a run, D(ρ), is the sequence of its declassifying steps: every `consecrate` and `consecrate-grave`, with the value, the validator or grant, and the answer; every `examine`, with its examiner and verdict; every `judge`, with its outcome; and every `commit`, with its effects and outcome. What passes judgement is lawful: the verdict, and the work it approved, are released, as consecration releases a value.
- **P5.** Let ρ and ρ′ be runs of the same well-typed program, from low-equivalent initial states, with the same choices everywhere except in the content of proposals: the same choices of the will, the same schedule, the same rite answers. If D(ρ) = D(ρ′), then obs(ρ) = obs(ρ′); for runs that never end, this holds for every finite prefix.

In words: whatever spirits say, the run behaves the same, except through what consecration or judgement let through. The answer of a refused consecration and every verdict are part of D, so even one bit leaked that way is accounted for. Without judgement in D the statement is false: an approved work touches the places its spirit chose, and those places are in the observation (formal model, 2026-11-11). [Parallel: "robust declassification" (Zdancewic and Myers, 2001) and "gradual release" (Askarov and Sabelfeld, 2007).]

9.5 **The will, the rite, and P6** (ruling 2026-11-11: split).
- **The will and proposal content are adversarial.** At each draw, any outcome the rite leaves open may be chosen. A proposal may hold any content of its office's form (`offices.md` §1.4). The seeded draw and the simulated or real agent are only ways of making one such choice; they matter for replay (P9), not for safety.
- **The rite is an oracle.** For the proofs, a rite is any function from (state, question of §3.1) to an answer in that question's domain. A relational rite (below) gives every question at least one allowed answer; a rite that answers nothing would halt the run unrecorded.
- **P6(a), rite-parametric safety.** P1–P5, P7 and P8 hold for every rite oracle, every choice of the will, every proposal content and every schedule. This is the property that matters: no rite, however wrong or malicious, makes Annulus less safe.
- **P6(b), trace inclusion.** Here a **relational rite** is one whose answer to each question is a *set* of allowed answers, any of which may be given. A relational rite *R′* is **at least as strict** as *R* when, for every state and question, *R′*'s set is a subset of *R*'s. The **trace** of a run is its observation (§9.4) together with its sequence of commits; it does not include the rite's answers. Then: for every run under *R′* from an initial state σ₀, with given choices of the will, proposal content and schedule, there is a run under *R* from the same σ₀, with the same choices, whose trace is the same (fourth audit, 2026-11-11).
- **Not claimed:** that a rite giving less favourable verdicts has fewer runs. A program may branch on an outcome, so a stricter verdict can lead to a different run, not a subset of runs. (The former statement of the order was withdrawn after the third audit.)

9.6 **Each theorem's precondition** (third audit, 2026-11-11). The program is well-typed and was launched (`launch` succeeded). Beyond that, each has its own:

| Theorem | Ranges over | Further precondition |
|---|---|---|
| P1, P2, P3 | every reachable state | none |
| P4 | every **complete** run: one ended by the program, by `kill` or by a failure | none |
| P5 | pairs of runs | as stated in §9.4 |
| P6 | every rite oracle; relational rites for (b) | as stated in §9.5 |
| P7 | every prefix of every run | none |
| P8 | the `launch` step, and every `examine` | none. **Doubt** means: the rite names no admissible examiner, or the author is *suspected*, newly suspected, proscribed or damned |
| P9 | pairs of runs: complete runs give **equal** audits; unfinished runs give audits of which one extends the other | under a **functional** rite (one answer per question), with no `intrude` step in either run (intrusions are outside input, not scripted), in simulated mode, from **equal initial states** σ₀ (§9.1): the same program, Constitution version and fingerprint, launch parameters (seed, clock, place, sky, script), faculty, store of grave grants, Index and wand state, and the same **world**. The world is captured at `launch`: the runtime records the fingerprint of every granted place's content and mark in the audit's first record, so equality of σ₀ can be checked from the two audits (fourth audit, 2026-11-11) |

No precondition may assume anything about the rite, the will, proposal content or the order of turns, beyond what this table states.

9.7 **The formal model** (2026-11-11). §9 is written as a Lean 4 model in `formal/`, with P1–P9 stated as Lean propositions and not proved. It type-checks. This section is normative; the model follows it, and the two change together. What formalising found, what the Lean audit found (2026-11-11), and how each point was settled, is in `formal/README.md`.
