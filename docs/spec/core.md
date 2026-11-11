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

3.1 **The rite layer talks to the core in exactly these ways.** Each one can refuse, hold, spend or bound. None can grant. **One exception, stated openly** (second audit, 2026-11-11): consecration may *lift* the quarantine on data. It never adds authority, so P1 is untouched, but P5 holds only up to consecration (§9.4).

| What the rite supplies | What the core does with it | Why it cannot add authority |
|---|---|---|
| **admission** of an `INVOCA`, `INVOCA REGEM` or apparatus act: allow or refuse (hours, Moon, Earls, Dukes, kings, required instruments, allegiance) | refuses on any "no" | a refusal only removes |
| **a signature** for each office kind and mode: the (place, permission) pairs a request needs, and the verbs its effects may use | checks every pair against *G* before the program runs (`offices.md` §3.2), and every effect at commit (§3.3 below) | a signature can only *ask for* pairs; the core grants only those in *G* |
| **an examiner** for each proposal, whether its author is a spirit or an angel: only the examiner's **identifier** (§9.3b) | runs the examination step; requires that the examiner is not the author, nor any agent whose words the proposal carries (C2). If the rite names no examiner for an angel's proposal, the verdict is `HOLD` | it can only approve what the core then re-checks |
| **a verdict**: `APPROVE`, `HOLD` or `REJECT` | maps it to a destination (C6) | `APPROVE` is necessary, never sufficient (§3.3) |
| **bounds**: legions, ink costs, lifetimes, escalation steps | enforces them as upper limits, or spends them from *G* | a bound only caps; spending only removes |
| **which outcomes of the will are open** | lets the will choose among them; the choice, and a proposal's content, are treated as adversarial (§9.5) | the will decides what a spirit offers, never what it may hold |
| **consecration** (`CONSECRA`, `flow.md` §3): only the **identifier** of the validator, an entry in the Constitution, that applies to the value's office (§9.3a) | the core looks the validator up, authenticates it, runs it itself and, on acceptance, alone changes the value's mark: raw ink becomes consecrated ink; a quarantined value becomes consecrated at the ordinary grade. The validator MUST be neither the program nor the spirit whose words the value carries. The grave grade needs the wand's second key, outside the session (`operator.md` §8), so it is never given within a run | it changes a mark on data, never a capability. It is the only point where quarantined data may come to steer, and it can steer only among acts the program could already perform within *G* |
| **counterbalances and extra checks** (Focalor's prohibition, Vepar's reversibility, `TUTELA`) | can only turn `APPROVE` into `HOLD` or `REJECT`, or refuse an effect | they only remove |

3.2 **Rite independence.** For **every** rite, even a wrong or a malicious one, the guiding principle (C3) holds. So the proof is done once, over the core, with the rite treated as an unknown oracle that can only answer the questions in §3.1 (P6(a), §9.5). That a stricter rite has fewer runs is claimed only for relational rites (P6(b)); a program may branch on a verdict, so in general a stricter rite leads to different runs, not fewer. [O]

3.3 **The core re-checks at commit.** Whatever the verdict, before any effect is performed, the core itself checks that the effect is in the request's handed set, and so in *G*. If not, nothing commits and the proposal goes to `INFERNUS`. An examiner, however wrong, can never commit something outside *G*. (This was "defence in depth" in `offices.md` §3.3. It is now a core rule.)

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
| P2 | **No self-approval.** Every commit is preceded by a verdict from an examiner other than its author, and a destination from the judgement step. | nobody approves their own work |
| P3 | **Circle containment.** No rite value is used outside the Circle it came from. | the boundary holds |
| P4 | **Release.** Every invocation ends exactly once; after that, nothing it held is used. | nothing lingers, nothing is reused |
| P5 | **Quarantine (non-interference).** What a run decides, and which places it touches, do not depend on any spirit's words that were not consecrated. | spirits cannot steer the Operator |
| P6 | **Rite independence.** (a) P1–P5, P7 and P8 hold under every rite. (b) Under a relational rite, a stricter rite has no more runs (§9.5). | tradition can never make Annulus less safe |
| P7 | **Audit.** Every grant, invocation, verdict, judgement, commit and release appears in the audit, in order, and the audit is only appended to. | everything is on the record |
| P8 | **Fail closed.** Without the wand the faculty is empty; with a tampered Index the runtime does not launch; under doubt the verdict is `HOLD`. | when unsure, nothing happens |
| P9 | **Replay.** In simulated mode, the same program, Constitution version, seed, clock, place, sky and script give the same audit. | every test can be repeated exactly |

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
- **the proposals:** for each, its author, its content (a response, or its list of **pending effects**, with any certificate or resolution map), and its status, *pending* or *judged d* with *d* one of `PARADISUS`, `PURGATORIUM`, `INFERNUS`;
- **the world:** the content and the mark of every place the faculty names, that is, the granted files as the run sees them. Nothing outside *G* is part of the state, because no transition can read or write it;
- **the marks** λ(*v*) of every value (§9.3);
- **the stores:** the audit *A*, a sequence; the Index *X*, a hash-chained log; the Vessel *V*;
- **the scheduler:** in simulated mode, its position in the entropy stream, the simulated clock and the order of turns chosen so far; in real-agent mode, the record of the actual order of turns, which is an input the run does not control (`concurrency.md` §4);
- **the store of grave grants:** one-use grave consecrations confirmed outside any run (§9.3c), each spent or unspent, in a hash-chained log;
- **the wand:** present or absent; locked, or unlocked as master or as rite.

9.2 **Transitions.** A run is a sequence σ₀ → σ₁ → … through these steps only:
- `launch`, `open`, `close`;
- `invoke`, `request`, `turn` (a spirit's or helper's piece of work);
- `examine`, `judge`, `consecrate`, `consecrate-grave`, `commit`;
- `release`, `relegate`, `kill` (Andras, `dynamic.md` §5.5);
- `step`: one step of ordinary computation;
- `schedule`: the scheduler chooses the next worker to take a turn.

Two further steps happen **outside any run**, and only with the wand's master secret (`operator.md` §8): `grant-grave` (§9.3c) and the consequential acts of `operator.md` §4. One step changes the Index during a run: `damn` (below).

Every transition appends to *A*, and none removes from it.

**An invocation's life:** its status starts `open` and moves only to `released` or `relegated`. Both are **terminal**: no transition leaves them. Releasing an invocation that is already terminal only records "already done". `close`, `kill` and the end of a run make every open invocation terminal. That is the "exactly once" of P4.

**Proposals and agents are judged apart.** A **destination** belongs to a proposal, never to an agent (`authority.md` §11.2): `INFERNUS` seals that proposal's results in the Vessel and ends its sub-tasks. **An agent** has only these states:
- within a run, its invocation's status (`open`, `released`, `relegated`);
- across runs, its **standing** in the Index, by its seal (`proscription.md` §2–3): *unknown*, *suspected*, *proscribed* or *damnatus*.

**Damnation** (ruling 2026-11-11). *Damnatus* is the one irreversible agent state:
- **Entered** only by the step `damn`, which the runtime takes when an **intruder** is recorded: an agent that reached a Circle other than through an invocation the Operator made there (`authority.md` §6.2a). That is a recorded fact, never a judgement. No examiner, angel, program or person can condemn, the wand included.
- **Terminal:** no step leaves it. It cannot be lifted.
- **By seal only:** an agent with a new seal starts *unknown*. If its actions resemble a damned one's, it is *suspected* and its work is held; it is never condemned on resemblance (`proscription.md` §2.3).
- Invited agents are never damned. At worst they are *proscribed*, which only a consequential act with the wand can lift (`operator.md` §4).

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

  If it exceeds its work limit, the answer is **refusal**. Purity is checked when the entry is approved (`authority.md` §2.5); the runtime runs a validator isolated from the state.
- **Authentication:** at launch, the core checks every validator's fingerprint against the Constitution's manifest, which the owner's wand signs for each release (`authority.md` §2.5; `operator.md` §8). A mismatch fails closed: the runtime does not launch.
- **What the rite supplies:** only the **identifier** of the validator that applies to the value's office (`authority.md` §10). The core looks it up. An identifier that names no validator entry is refused. A rite never supplies validator code, nor an answer.
- **Independence:** a validator is authored by a Constitution amendment, never by the program or any agent within a run. No run can add or change one.
- **The step `consecrate(v, k)`:**
  - **precondition:** λ(*v*) is `quarantined` (or *v* is raw ink), and *k* is the identifier the rite names for this value's office;
  - **action:** the core runs validator *k* on *v* and *v*'s origin record, isolated, and records in the audit *v*'s origin, *k*, its fingerprint and the answer;
  - **on acceptance:** λ(*v*) becomes `consecrated` (raw ink becomes consecrated ink);
  - **on refusal:** nothing else changes.
- **Proof certificates** (`library.md` §3, `flow.md` §3.4) are checked the same way: the checker is a validator entry, run by the core.

9.3b **Examiners and the examination step** (third audit, 2026-11-11).
- **Who examines:** for each proposal, the rite supplies only the **identifier** of its examiner: for a spirit, his angel (`authority.md` §10); for an angel, another angel, by each rite's rule (`authority.md` §12.1a, open per rite).
- **Independence:** the examiner MUST NOT be the proposal's author, nor any agent whose words the proposal carries. A proposal whose rite names no admissible examiner is `HOLD`.
- **The step `examine(p, e)`:** gives a verdict on *p*, recorded with *e*'s identifier. Then `judge` assigns the destination (`authority.md` §11). For any author, spirit or angel, `commit` requires a prior `judge` to `PARADISUS`, and the re-check of §3.3.

9.3c **Grave consecration** (ruling 2026-11-11: one use).
- **Outside any run, `grant-grave(v, k)`:** the Operator, with the wand's master secret, petitions it for a value *v* that a run left in a petition (`flow.md` §3.3). The core runs validator *k* on *v* as in §9.3a. On acceptance, it appends a **grave grant** to the store: the fingerprint of *v*, of its origin record and of *k*, marked unspent, signed by the wand.
- **At launch:** the store's chain is checked like the Index's (`proscription.md` §3.2). A mismatch fails closed.
- **In a run, `consecrate-grave(v, g)`:**
  - **precondition:** *g* is an unspent grave grant whose fingerprints match *v*, *v*'s origin record and the validator the rite names for *v*'s office;
  - **effect:** λ(*v*) becomes `consecrated-grave`, and *g* is marked spent by an appended entry. It can never be used again.

9.4 **Non-interference (P5, stated exactly).**
- **Observation.** The **observation** of a run, obs(ρ), is its sequence of control decisions (the branch taken at every `SI`, `DUM` and `PER`, and every choice of function or mode) and the sequence of places it touches, each with its verb.
- **Low-equivalence.** Two states are **low-equivalent**, σ ≈ σ′, when they are equal except in the contents of values and places marked `quarantined`.
- **Declassification.** The **declassified record** of a run, D(ρ), is the sequence of its `consecrate` and `consecrate-grave` steps, each with the value it was applied to, the validator or grant, and the answer.
- **P5.** Let ρ and ρ′ be runs of the same well-typed program, from low-equivalent initial states, with the same choices everywhere except in the content of proposals: the same choices of the will, the same schedule, the same rite answers. If D(ρ) = D(ρ′), then obs(ρ) = obs(ρ′).

In words: whatever spirits say, the run behaves the same, except through what consecration let through. The answer of a refused consecration is part of D, so even one bit leaked by a refusal is accounted for. [Parallel: "robust declassification" (Zdancewic and Myers, 2001) and "gradual release" (Askarov and Sabelfeld, 2007).]

9.5 **The will, the rite, and P6** (ruling 2026-11-11: split).
- **The will and proposal content are adversarial.** At each draw, any outcome the rite leaves open may be chosen. A proposal may hold any content of its office's form (`offices.md` §1.4). The seeded draw and the simulated or real agent are only ways of making one such choice; they matter for replay (P9), not for safety.
- **The rite is an oracle.** For the proofs, a rite is any function from (state, question of §3.1) to an answer in that question's domain.
- **P6(a), rite-parametric safety.** P1–P5, P7 and P8 hold for every rite oracle, every choice of the will, every proposal content and every schedule. This is the property that matters: no rite, however wrong or malicious, makes Annulus less safe.
- **P6(b), trace inclusion.** Here a **relational rite** is one whose answer to each question is a *set* of allowed answers, any of which may be given. A relational rite *R′* is **at least as strict** as *R* when, for every state and question, *R′*'s set is a subset of *R*'s. Then every run possible under *R′*, with given choices of the will, proposal content and schedule, is possible under *R* with the same choices.
- **Not claimed:** that a rite giving less favourable verdicts has fewer runs. A program may branch on a destination, so a stricter verdict can lead to a different run, not a subset of runs. (The former statement of the order was withdrawn after the third audit.)

9.6 **Each theorem's precondition** (third audit, 2026-11-11). The program is well-typed and was launched (`launch` succeeded). Beyond that, each has its own:

| Theorem | Ranges over | Further precondition |
|---|---|---|
| P1, P2, P3 | every reachable state | none |
| P4 | every **complete** run: one ended by the program, by `close` of its outermost Circle, by `kill` or by a failure | none |
| P5 | pairs of runs | as stated in §9.4 |
| P6 | every rite oracle; relational rites for (b) | as stated in §9.5 |
| P7 | every prefix of every run | none |
| P8 | the `launch` step, and every `examine` | none |
| P9 | pairs of runs | simulated mode, with the same program, Constitution version and fingerprint, seed, clock, place, sky, script, faculty, store of grave grants and Index |

No precondition may assume anything about the rite, the will, proposal content or the order of turns, beyond what this table states.
