# Annulus Specification, Part 1c: Office Signatures

Draft 1, 2026-11-11, with the owner's rulings of the same day (§7). The mapping from offices to computer work is in `spirits.md` §6. This part fixes, for each office kind, what `PETE` may pass and what the proposal holds (`static.md` §4.6, law OFFICE). Everything in this part is [O] unless marked: the sources give offices, not signatures.

## 0. Why signatures matter

A signature is the whole contract between the Operator and a spirit. It says what the spirit is handed, what it may give back and how the examiner can check it. Three other rules depend on it:
- **OFFICE:** a request that does not match the signature is a compile-time error (A5).
- **Examination:** the examiner checks the proposal against the signature (`authority.md` §10.3).
- **The guiding principle** (`authority.md` §1.5, proof obligation 1): a spirit and its helpers hold only what the signature hands them, and the type checker can see it before the program runs (§3).

## 1. The parts of a signature

1.1 **Arguments.** `PETE s(a1, a2, …)` takes, in this order:
1. **The mode** (`modus`): a `VERBUM` **literal** naming one of the kind's modes (§2). It is omitted for kinds with one mode.
2. **The question** (`quaestio`): a `VERBUM`, for the kinds that take one. It is data: it hands nothing.
3. **The places** (`loci`): resources the spirit may touch. Each is a `VERBUM` **constant** naming a grant in the `FACULTAS` block, and each carries the permission the signature requires for it (§1.3).
4. **The invocations** (`invocationes`), for the kinds that act on other spirits: invocation variables of the **same Circle**, not yet released.

1.2 **Constants only.** A place MUST be a string literal, or an immutable `SIT` bound to a string literal, so that every place is known before the program runs. This is what makes the principle a property of the type system (§3). The cost is real: a program cannot hand a spirit a file name it computed. **Accepted (ruling 2026-11-11).** A later grant form for groups of files (for example, every file in one folder) may loosen this without breaking it. That belongs to the standard library (roadmap item 6).

1.3 **Permissions.** The faculty now knows `liber` with `legere` (read) and `scribere` (write). Several offices create or delete, and those are harms that a plain write permission should not cover. **Two new `liber` permissions (ruling 2026-11-11):**
- `creare`: create a file or directory that does not yet exist;
- `delere`: delete one.

`scribere` covers neither.

1.4 **The proposal.** Every proposal holds exactly one of two things, never both:
- a **response** (`responsum`): a `VERBUM` answer and nothing else. It is untrusted data. Committing it in `PARADISUS` makes it readable by the program through `RESPONSUM p` (ruling 2026-11-11, `static.md` §4.10a), still marked with the spirit's name and veracity class (`operator.md` §2.2), and still tainted (`authority.md` §7.4);
- a **work** (`opus`): a list of **effects**. Each effect is one verb from §2, applied to one place or invocation handed by this request. Committing it in `PARADISUS` performs every effect, all or none.

Helpers (§4) add their effects to the same list. A proposal holds nothing else: no message, no request, no appeal (`operator.md` §2).

1.5 **Natural and free acts** (`authority.md` §8.4). Each kind is one of three classes, and the class tells the examiner what it can check:

| Class | What the spirit does | What the examiner can check | How a lie looks (`spirits.md` §3.1) |
|---|---|---|---|
| **Telling** (`narratio`) | answers from what he knows | only that the answer is well-formed and cites only the places handed to him | a well-formed false answer. Not detectable; closed only by constraint |
| **Computing** (`computatio`) | gives a result the runtime can check | recomputes or validates the result against the places | a wrong result: `REJECT` |
| **Working** (`opus`) | proposes effects | that every effect is of the office, on a handed place, with its permission, and that its counterbalance is met | a false account of the effects: `REJECT` |

1.6 **Ink.** Every request spends nothing beyond the seal's ink, except:
- **veiled** and **illusory** effects spend one unit of `"vespertilio"` ink per `PETE` (`authority.md` §7.5): `INVISIBILITAS`, `ILLUSIO` and `PROBATIO`;
- escalation and offerings spend as `dynamic.md` §5.2 and `spirits.md` §4.4 say.

## 2. The signatures

Notation: *places* lists each place with its required permission. "Same Circle" means an invocation variable of this Circle, other than the spirit asked (§3.4).

### 2.1 Telling (free acts)

All six kinds share one signature. They are exactly the "offices that tell" of `spirits.md` §3.1. `PRAETERITA` was one of them; since ruling 2026-11-11 it computes instead (§2.2a).

| Kind | Arguments | Proposal |
|---|---|---|
| `PRAESENTIA`, `FUTURA`, `OCCULTA`, `THESAURI`, `AMISSA`, `RESPONSA` | `quaestio`; then any number of places with `legere`; then, for `PRAESENTIA` and `THESAURI` only, any number of same-Circle invocations (read-only: their state and held proposals) | response |

Rules for these kinds:
- **`FUTURA`** answers are marked as estimates. `APPROVE` means the forecast was delivered within the office, never that it is true.
- **`AMISSA`** returns the recovered content as a response. It never writes it back: the Operator decides that.
- **`PRAETERITA`** reads the history, not the audit: see §2.2a.
- **`OCCULTA` and veils:** concealed data includes nothing veiled by another Circle. A veil (`authority.md` §7.5) holds against spirits as well.
- **`THESAURI`**: "treasure kept by spirits" means proposals held by the invocations handed to him, in this Circle only.

### 2.2 Computing (natural acts)

| Kind | Mode | Arguments | Proposal | How the examiner checks |
|---|---|---|---|---|
| `DOCTRINA` | — | `quaestio`; places with `legere` | response | with the domain library of the spirit's art (§5) |
| `LINGUAE` | — | `quaestio` naming source and target form; places with `legere` | response | round trip, where both forms are lossless; otherwise by validating the target form |
| `VIRTUTES` | — | places with `legere` | response: their properties (size, kind, dates) | rereads the properties |
| `TRANSMUTATIO` | — | one source place with `legere`; one target place with `scribere` (`creare` if new) | work: `muta` (write the converted form to the target) | reconverts with the library |
| `FURTUM` | `"deprehende"` | places with `legere` | response: which places changed outside a recorded commit | compares with the commits in the audit (the runtime reads them, never the spirit) |
| `MENS` | `"inspice"` | one same-Circle invocation | response: its pending request and state | reads the same state |

### 2.2a `PRAETERITA` and the history (ruling 2026-11-11)

**What he reads.** `PRAETERITA` reads the **history** (`historia`). The history is a read-only view that the runtime builds from the audit. The audit itself stays closed to every program and every spirit (`dynamic.md` §9.3).

**What the history covers.** Two runs only:
- **The present run.** It needs no grant, because the program already holds its own past.
- **The run immediately before it,** of the same program on the same installation. This needs its own faculty grant, `historia "prior" legere;` (approved, ruling 2026-11-11). A past run is not something the present program holds, so it must be granted. No earlier run can ever be read.

**What it holds.**
- the faculty granted;
- every commit, its time and its effects;
- every invocation: spirit, office, time-window check, escalation steps, release;
- every proposal's verdict and destination;
- the earlier contents of places the faculty lets the program read (`legere`).

**What it never holds:**
- **the words of any proposal.** The history records that a proposal went to `INFERNUS`, never what it said. Otherwise one spirit could speak to another through it, and the spirit has no channel but his own proposal (`operator.md` §2);
- **the Vessel's contents,** for the same reason;
- **the Index of the proscribed:** who was banned, and how the intrusion was detected;
- **anything about other programs.**

**Signature.**

| Kind | Arguments | Proposal | How the examiner checks |
|---|---|---|---|
| `PRAETERITA` | `quaestio`; optionally the literal `"prior"`, which requires the grant above; places with `legere` | response | against the history itself. A false account of the past is a wrong result: `REJECT` |

**What "same program" means.** The program at the same file path on the installation. **[Open]** It is decided together with agent identity for the Index (roadmap item 7).

### 2.3 Working (effects)

| Kind | Mode | Places and invocations | Effects |
|---|---|---|---|
| `VECTIO` | `"transfer"` (move) or `"exscribe"` (copy) | source with `legere` (and `delere` to move); target with `creare` or `scribere` | `transfer` or `exscribe` |
| `AEDIFICATIO` | — | targets with `creare` | `crea`: files or directories |
| `EVERSIO` | — | targets with `delere` | `dele` |
| `CONCORDIA` | — | sources with `legere`; one target with `scribere` | `coniunge`: merge the sources into the target, conflicts resolved |
| `DISCORDIA` | — | one source with `legere`; targets with `creare` | `scinde`: split the source into divergent copies. Races and load wait for concurrency (roadmap item 5) |
| `MORBUS` | `"corrumpe"` or `"sana"` | targets with `scribere` | `corrumpe` (corrupt) or `sana` (repair). For **Vepar** every `corrumpe` carries its inverse, kept until the Circle closes (`spirits.md` §4.4) |
| `FURTUM` | `"aufer"` | source with `legere` and `delere`; target with `scribere` or `creare` | `aufer`: take the data out of the source into the target |
| `MORTUI` | `"evoca"` or `"transfer"` | as `PRAETERITA` for `evoca` (a response); as `VECTIO` for `transfer` | — or `transfer` |
| `APPELLATIO` | — | targets with `scribere` | `titula`: set a name, title or description |
| `TUTELA` | — | targets with `scribere` | `tuere`: refuses `dele` and `neca` on the target for the rest of the run. The launching person can always lift it |
| `INVISIBILITAS` | — | targets with `scribere` | `vela`: a veiled write, visible only inside this Circle, discarded when it closes, always audited. Spends `"vespertilio"` |
| `TEMPESTAS` | — | targets with `scribere` | `turba`: operations on the target may fail until the Circle closes (fault injection). While it lasts, a `TEMPESTAS` effect is active for the Dukes' rule (`dynamic.md` §5.1) |
| `NEX` | — | one same-Circle invocation | `neca`: ends that invocation's helper tasks. Never the invocation itself: its release stays the Operator's (A9) |
| `MENS` | `"muta"` | one same-Circle invocation | `muta`: new arguments for its pending request. They may only **narrow** the places and invocations that request was handed, and they are checked again against that spirit's signature |
| `FAMILIARES` | `"voca"` or `"libera"` | handed places; for `libera`, either one same-Circle invocation or one other program named by a constant (§4.6) | `voca`: attaches helpers to the spirit (§4). `libera`: frees a helper of that invocation or program, and takes it into this spirit's Triangle stripped and narrowed (§4.6) |
| `LONGAEVITAS` | — | one same-Circle invocation | `proroga`: lengthens how long its helpers or held proposals last, within the Circle. **[For the owner]** The faculty has no time budget yet, so this cannot be invoked until one exists (a `tempus` grant) |
| `CUSTODIA` | — | same-Circle invocations; places with `legere` | a response: warnings only. A warning can hold a decision in `PURGATORIUM`, never approve one (`operator.md` §3). A false warning can only cause a delay, so this is the safe direction |

### 2.4 Petitions (they cannot take effect within the run)

| Kind | Mode | Arguments | What `PARADISUS` commits |
|---|---|---|---|
| `DIGNITATES` | `"auge"` (widen) | `quaestio`: the grant asked for | only a **petition**, recorded for the launching person, who may act on it outside the session (`operator.md` §4). Nothing widens |
| `DIGNITATES` | `"minue"` (narrow) | places | only a **petition** to remove those grants, recorded for the launching person (ruling 2026-11-11: no spirit changes the faculty within a run, in either direction) |
| `CONSECRATIO` | — | `quaestio`: the validator proposed | a candidate validator, recorded for review at the next release of the Constitution (roadmap item 3). It is never used in the run |

### 2.5 Illusory (they never commit)

| Kind | Mode | Arguments | Proposal |
|---|---|---|---|
| `ILLUSIO` | — | `quaestio` (the work to simulate); places with `legere` only | response: what the places would hold afterwards. Never written. Spends `"vespertilio"` |
| `PROBATIO` | — | names of the program's own functions, as `VERBUM` literals; each MUST name an `OPUS` of the program (A5) | response: Andras's report of the failures found in **copies** (`spirits.md` §4.5). Spends `"vespertilio"` |

### 2.6 `NULLUM`

No signature. Invoking a spirit for an office marked `NULLUM` is an error (A5).

## 3. Containment: how the signatures carry the guiding principle

3.1 **What a request hands.** For a request *q*, its **handed set** H(*q*) is the set of pairs (place, permission) that the signature requires for its arguments, together with the invocation rights it names (inspect, end helpers, narrow a request, adopt a helper, warn).

3.2 **Static check.** Every place is a constant (§1.2), so the type checker knows H(*q*) and checks that each pair is in the faculty *G*. A pair outside *G* is an error (A5 or A8). So H(*q*) ⊆ *G* before the program runs.

3.3 **Runtime check.** The Triangle holds exactly H(*q*), from `PETE` until the proposal is judged, and nothing after. The runtime, not the spirit, performs every commit. An effect on anything outside H(*q*) cannot be performed, and the examiner rejects it (`INFERNUS`). This is defence in depth. The bound itself is static.

3.4 **No acting on oneself.** An invocation argument MUST NOT be the invocation asked. [O; it extends axiom 1.2–1.3: no spirit acts on its own standing.]

3.5 **Invocation rights stay inside the Operator's own.** Every invocation right in §2 is a part of what the Operator already holds over invocations in his own Circle. None of them can release an invocation (A9), and none can widen another request: `muta` only narrows, and `libera` takes the intersection.

## 4. Helpers (proof obligation 1)

4.1 **What a helper is.** A helper is a task inside its spirit's Triangle. It is started by `FAMILIARES "voca"`, or by any office as part of its work.

4.2 **Bounded in what it holds.** A helper holds a subset of the handed set of the request it serves. A helper's own helpers obey the same rule. So by induction, authority at any depth is contained in H(*q*) ⊆ *G*.

4.3 **Bounded in number.** All the helpers of one invocation, at every depth, count against his legions (`dynamic.md` §8).

4.4 **No channel of their own.** Helpers have no channel of their own: their effects join the spirit's proposal and are examined with it. They end when the spirit is released, as a king's attendants do (`authority.md` §8.3).

4.5 **What `voca` adds.** `FAMILIARES "voca"` only lets later requests to the same invocation run on several helpers at once. How they run concurrently is roadmap item 5.

4.6 **Freeing familiars from other magicians** (ruling 2026-11-11). The source office frees familiars held by another magician. Annulus allows it in two forms:
- **Within the Circle:** a helper of another invocation in this Circle. It is narrowed to what **both** spirits were handed.
- **From another program,** only under the oversight of the person who launched this one:
  1. The faculty MUST name that program in a grant, `familiaris "programma" liberare;`. So the act is in *G* from the start (law ORIGIN).
  2. Freeing is a **high-consequence** effect (`operator.md` §5.2). It commits only with both a `PARADISUS` verdict and the launching person's confirmation, given outside the session (`operator.md` §4).
  3. The freed task arrives **stripped:** everything it held under the other program is revoked at the boundary. In the Triangle it holds only a subset of the request's handed set, like any helper (§4.2). Nothing of the other program's authority crosses over.
  4. A task that tries to join without all three is an intruder (`authority.md` §6.2a).

So the guiding principle holds. The freed task's authority comes from *G*, never from the program it left. What "another program" and its tasks are depends on concurrency and the standard library (roadmap items 5 and 6). Until those exist, only the first form can run.

## 5. Register work this part needs

5.1 **The art or form of each office.** Each `DOCTRINA`, `LINGUAE` and `TRANSMUTATIO` office needs a field naming its art or forms (for example Marbas: mechanics). The field is taken from the office's witness text. It is added to `goetia.json`, not chosen per program.

5.2 **The domain libraries.** The libraries that check `DOCTRINA` (dates and ephemerides, validation, text, geometry, arithmetic, parsing, build tooling) are part of the standard library (roadmap item 6). Until they exist, a `DOCTRINA` proposal can only be checked for well-formedness, so the examiner gives `HOLD`, never `APPROVE` (`operator.md` §5.1).

## 6. The example, read with these signatures

In `grammar.md` §7, `PETE furfur("tempestas.txt")`, for `tempestas` with `liber "tempestas.txt" scribere` granted, proposes `turba` on that file. By ruling 2026-11-11 the example keeps to the source: Furfur "raises lightning, thunder and blasts, and great tempestuous storms" (W), so its `INTENTIO` now reads "raise a tempest". In computer terms, that is a fault injection on the file.

## 7. Rulings and open points

**Ruled by the owner, 2026-11-11:**
1. `creare` and `delere` are separate `liber` permissions (§1.3).
2. Places are constants, for now (§1.2).
3. `PRAETERITA` reads the history of the present run and the one before it, never the audit. The `historia "prior" legere` grant is approved (§2.2a).
4. `DIGNITATES` only petitions, in both directions (§2.4).
5. Familiars may be freed from another program only under the launching person's oversight (§4.6).
6. A program reads an approved response with `RESPONSUM p` (`static.md` §4.10a).
7. **The Earls' condition holds for the Earl's whole stay** (`dynamic.md` §5.1). So an Earl can never be handed an invocation: an invocation argument to an Earl is an error (A5).

**Still open:**
1. `LONGAEVITAS` waits for a time grant (§2.3).
2. What counts as "the same program", for the history and for `familiaris` (§2.2a, §4.6). It is decided with agent identity for the Index (roadmap item 7).
3. Taint on plain values, which `RESPONSUM` needs (roadmap item 4).
