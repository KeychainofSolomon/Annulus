# Annulus Specification, Part 1: The Authority Model

Status: draft 1, 2026-11-10. Normative once approved by the owner.

Annulus is both a programming language and a simulation of the Solomonic tradition. This part defines who may do what, by whose authority, and with what consequence.

## 0. Conventions

**Normative words.** MUST, MUST NOT and MAY carry their usual normative meaning.

**Provenance tags.** Every rule carries one of two tags:
- **[S: witness]:** sourced. The rule models something attested in the named witness. Witnesses and their dates are listed in `provenance.md`.
- **[O]:** original to Annulus. A mechanical analogue invented here, labelled so that it is never mistaken for tradition.

**Source precedence.** Precedence follows `provenance.md`:
- Within one work's line of copies, the oldest witness wins.
- Separate works never override each other; each is a separate **rite** (section 4).

**Ordinary computation.** Arithmetic, logic, control flow and data structures need no authority beyond the program's own execution, and never require a spirit [Architecture §14]. This part governs authority-sensitive operations only:
- external effects (files, network, devices, persistence);
- effects on data the program does not own;
- invocation of agents.

## 1. The axiom

> **NULLA POTESTAS SIBI IPSI ORITUR**: no authority originates in itself. [O: the owner's formulation; its scriptural ancestor is Romans 13:1, *non est enim potestas nisi a Deo*. The sources share the principle: Arbatel Aph. XV, where God alone gives the Olympic spirits power; the *Liber iuratus*, where spirits are bound by God's names and Seal; the Clavicula, where every conjuration works by divine names.]

1.1 **No authority without a source.** No entity MAY hold a capability that does not trace, through recorded grants, to the Constitution (section 2).

1.2 **No self-granting.** No entity MAY grant a capability to itself, directly or through another entity it controls.

1.3 **No self-approval.** No entity MAY examine, approve or judge its own proposed effects.

1.4 **No self-minted ink.** No entity MAY mint ink (section 7) from its own budget to empower its own seals.
- [O] The sources contain no pact signed in the magician's own blood. Blood ink is always animal blood, consecrated before use (*Liber iuratus* ch. CXXXVII; Mathers II.16). Annulus reads that absence as the axiom in material form. This reading is interpretive.

## 2. The Constitution (the Absolute)

2.1 **The root of trust.** The Absolute is represented by the **Constitution**: a fixed body of law and registers, built into each release of the language. [Architecture §4] It MUST NOT be an executable object, a user, an administrator or a superuser. It never runs, never signs anything at runtime, and cannot be logged into.

2.2 **Contents.** The Constitution contains:
- the eight laws (section 3);
- the registers (section 5);
- the pairings of spirits with examining angels (section 10);
- the divine names (section 6.4);
- the judgement rules (section 11).

2.3 **Divine names are references, not property.** A program MAY cite a divine name; the authority belongs to the Constitution's entry, not to the speaker. No program MAY extend, rewrite or shadow a Constitution entry. [S: the Clavicula, Lemegeton and *Liber iuratus*, where every binding works "by" God's names.]

2.4 **The faculty (the launch grant).** A program starts with no capabilities. Its first and only external grant is the **faculty** (`FACULTAS`), given by the person who launches it. It is not called *licentia*: in the sources the licence (*licentia*) is the licence to depart (§9).
- **What it lists:** the resources the program may touch (files, network, devices) and the budget of ink.
- **How it is treated:** the licence is recorded, bounded and auditable.
- **How it is declared:** the program declares the faculty it requests; the launching person grants all of it, part of it, or none.
- [O] Honestly stated, the launching person is the real-world source of the Operator's standing. The specification says so rather than hiding it.
- [S, model] In the sources the magician acts only by permission. In the *Liber iuratus*, permission for the vision is granted from above; the Key requires confession and fasting before any work.

## 3. The eight laws

Each law is enforced by the compiler's authority analysis, by the VM, or by both.

| Law | Rule | Enforcement |
|---|---|---|
| ORIGIN | every capability has a recorded chain of grants ending in the Constitution or the faculty | VM; audit |
| SOVEREIGNTY | every capability is valid only inside a named jurisdiction (a Circle) | compiler and VM |
| DECLARATION | every entity, office and effect class used MUST be declared; programs MUST NOT add register entries | compiler |
| OFFICE | an agent MAY act only within its declared office; asking a spirit for work outside its office is an error | compiler and VM |
| CONTAINMENT | every authority-sensitive operation runs inside a Circle; every invocation runs inside a Triangle | VM |
| EXAMINATION | no proposed protected effect is committed without examination by an entity other than its author | VM |
| RELEASE | every invocation terminates (enforced by budget) and releases its capabilities | VM |
| JUDGEMENT | only the Constitution's judgement step assigns a destination; no subordinate overturns it | VM |

**[S]** for the laws as a group: the operator in the sources works only inside the circle, by declared names and offices, with examination ("true answers" only under constraint), a licence to depart, and judgement reserved to God.
**[O]** the enforcement mechanisms.

## 4. Rites: separate works are separate jurisdictions

4.1 **A rite is the law of one work.** Every Circle MUST declare the **rite** whose law it follows. A rite is one work together with its line of copies. Defined rites:

| Rite | Work | Governing witness for the rite |
|---|---|---|
| `clavicula` | Key of Solomon | BnF Ital. 1524 (1446); later Key copies as additions |
| `goetia` | Lemegeton Goetia, within the *Liber officiorum spirituum* line (ruling 2026-11-10) | for names, offices, legions and directions: the oldest *Officium* witness (Oberon c. 1577-83, Wellcome 110, Weyer); for seals: Sloane 3825 (1641+) |
| `iuratus` | *Liber iuratus Honorii* | the text, as edited by Peterson; for the Seal drawing, Ganell SSM (L.4.f.22) |
| `arbatel` | *Arbatel de magia veterum* | Basel 1575 |
| `janua` | *Janua Magica Reserata* and the nine Keys | Sloane 3825 |
| `rudd` | Rudd's Goetia with the Shem angels | Harley 6483, via Skinner & Rankine |
| `heptameron` | *Heptameron* (the day-angels) | an early printing, still to be verified |

4.2 **Attributes resolve within the rite.** Inside a Circle, an attribute of an entity (direction, ruling planet, office, legion count, seal) resolves according to that Circle's rite. [O mechanism; S rule: the owner's within-one-work precedence.]

Example: the Sun's angel resolves to Raphael under `iuratus`, Michael under `janua` and Michael under `heptameron`. None of these overrides another.

4.3 **Cross-rite operations.** An operation involving entities from two rites MUST declare both. The audit records which rite decided each attribute.

## 5. Entities and registers

5.1 **The entity record.** Every celestial entity has an identity record with six separate properties [Architecture §4]:

| Property | Content |
|---|---|
| origin | where the entity comes from |
| allegiance | loyal or fallen, and at what point in the history |
| nature | what kind of being it is |
| office | what it does |
| jurisdiction | where its authority runs |
| judgement | its final symbolic state |

Each property records its source, the witness's date, and its **attestation**: scripture, narrator, or the entity's own unverified claim. Example: Barbatos "was of the Order of Virtues, of which some part he retaineth still" is the narrator's claim; Amy's hope of return is recorded as his own.

5.2 **Registers are fixed, keyed by number, with names as labels.** The Constitution contains these registers:

| Register | Entries | Source of the entries |
|---|---|---|
| Spirits | 72 | Goetia numbering; spellings and seals from Sloane 3825; names, offices, legions and directions resolved within the *Officium* line |
| Shem angels | 72 | names computed from Exodus 14:19-21, reading v.19 forward, v.20 reversed and v.21 forward; each verse has exactly 72 letters, and the result is checked against Kircher's *Oedipus Aegyptiacus* II (1652-54); the tool is `tools/shem_names.py` |
| Olympic spirits | 7 | Arbatel 1575: provinces 49 / 42 / 35 / 28 / 21 / 14 / 7 = 196 (the printed XXXII for Bethor is a misprint) |
| Day-angels | 7 | *Heptameron*: Michael, Gabriel, Samael, Raphael, Sachiel, Anael, Cassiel |
| Orders | 9 | per rite: `iuratus` order and ban (section 12); `janua` Scale of Ten |
| Kings of the quarters | 4 | per rite: `clavicula` and the *Officium* line give Oriens E, Paymon W, Amaymon S, Egin N |

5.3 **Identity is the number.** Numbers never change. Names are labels and MAY be corrected without breaking stored data. [O; this follows Occultist Explorer's key derivation by number.]

## 6. Circle and Triangle

6.1 **The Circle protects the Operator.** The Circle is the Operator's protected jurisdiction: the bounded space in which authority-sensitive work happens. [S: in every Solomonic rite the operator stands within the circle.]

It defines permissions, prohibitions and resource limits [Architecture §9].

6.2 **One entrance.** A Circle has exactly one declared entrance: the only path by which capabilities and messages cross into it.
- [S: BnF 1524 p246, the Book II circle with an open corridor into the centre, and p191 L.]
- [O: the single-entry rule.]

6.3 **The form follows the rite.** A Circle's structure (its rings, names and figures) is drawn from its rite:
- `clavicula`: the 1446 Circle of the Art (p194): a square with 8 crosses, 4 hexagrams and the ring of names.
- `goetia`: the Circle of Solomon with its double ring of names (Sloane 3825, 120R).

The names in the form are references to Constitution entries (section 2.3).

6.4 **The Triangle is the spirit's sandbox.**
- **Placement:** a spirit is always invoked into a Triangle **outside** the Circle. [S: Sloane 3825, where the triangle is 2 ft from the circle and 3 ft across, with Anephexeton, Primeumaton and Michael.]
- **Isolation:** the Triangle is an isolated task. It has no access to the Operator's memory. Its only powers are the sealed capabilities handed to it. It communicates only by messages across the boundary.
- **Untrusted output:** everything a spirit produces is a **proposal**, and is untrusted by default. [S: Goetia; Furfur "will never speak truth unless he be compelled, or brought within a triangle"; Arbatel, where governors act "naturally or of their own free will".]
- **Spirit-specific triangles:** where a rite gives one (Furfur's triangle, Wellcome 110 image 221; Andromalius's circle, image 200), that figure is the spirit's required containment form.

6.5 **Angels have no Triangle.** Angels are not confined, compelled or guarded against. They act only through a designated **channel**:
- `iuratus`: the raised mound inside the circle, with the operator standing outside it (CXII-CXV);
- `janua`: the crystal of the Keys;
- `arbatel`: prayer at sunrise on the spirit's day.

[S as cited; O: the channel as a mechanism.]

## 7. Seals and ink

7.1 **Exemplar and operative.** A seal exists in two forms:
- **Exemplar:** the seal as data, "in colours". It can be printed, displayed, type-checked and revealed. It grants nothing.
- **Operative:** the seal **inscribed** with ink, through the `INSCRIBE` operation. Only an operative seal is a capability.

[S: *Liber iuratus*, where the Seal of God's colours describe how it appears in the exemplars, but for operation it MUST be written in blood on virgin parchment. O: the mechanism.]

7.2 **Seals by register.**

| Register | Seal |
|---|---|
| Spirits | the accepted seal in Sloane 3825, never a struck-out one. Bune: the first (left) seal (ruling 2026-11-10). Amy: provisional 1904 inner sigil until compared with Peterson's reading (g58). The 1904 lettered rings are later additions |
| Shem angels | the angel's Hebrew name, with its verse in `rudd`. No original drawn set exists; Rudd's (Harley 6483) is the oldest seal made for each angel. A drawn sigil MAY be **derived** from the name by Agrippa's method (*De occulta philosophia* III.30) and MUST be labelled as derived |
| Olympic spirits | the 1575 woodcut characters, including Phaleg's inset sign. Rudd's ringed "Sigillum" versions are later additions |
| Day-angels | the *Heptameron* sigils (provisional, section 4.1) |

7.3 **Ink is a finite budget drawn from a living source.** Inscribing spends ink from a budget owned by a living source: the faculty (section 2.4), or a running invocation.
- **Provenance:** ink records its source.
- **Lapse:** when the source is released or banished, every seal inscribed in its ink lapses.

[S: blood carries life, "the life is in the blood", Leviticus 17:11 (general knowledge, not stated in the sources). O: the budget and the lapse rule.]

7.4 **Raw ink is tainted until consecrated.** Data or resources from outside the program arrive **tainted**. Tainted material MUST NOT be used as ink until it has passed a consecration step: validation, signed by an authority other than the one that will use it.

[S: the blood is exorcised and blessed before use (*Liber iuratus* CXXXVII, "creature of blood"; Mathers II.16). O: taint tracking.]

7.5 **Typed ink: the material matches the work.** Every effect class requires a matching ink class; a mismatch is a compile-time error where it can be detected, and a runtime denial otherwise. [S: the correspondence principle, below. O: the type rule.]

| Ink | Source | Effect class |
|---|---|---|
| bat's blood | 1446 I.5, I.10; Mathers I.16 | **veiled** effects (visible only inside their Circle, always still audited) and **illusory** effects (they appear but can never commit: dry runs and test doubles) |
| by rank of spirit | the seal metal of the rank (Goetia Observations; table to be transcribed from Sloane 3825) | a spirit's operative seal requires the ink class of its rank |

7.6 **Tempering: binding at creation.** An instrument's limits are fixed when it is made and can never be widened.
[S: the knife "Artano" tempered in goose blood (1446 II.7). O: the immutability rule.]

## 8. Invocation

8.1 **For a declared office only.** A spirit is invoked for one of its declared offices only.

8.2 **The invocation steps:**
1. The Operator declares intent.
2. The Circle is closed.
3. The spirit is invoked into its Triangle.
4. The spirit receives the operative seals for the declared office.
5. The spirit proposes effects.
6. The paired angel examines them (section 10).
7. The proposal is judged (section 11).
8. The spirit is released (section 9).

[Architecture §9; S: the Key, Goetia and Arbatel sequences.]

8.3 **Rank limits.**
- **Legions** cap the number of concurrent sub-tasks a spirit may start. [S: legion counts. O: the cap.]
- **Attendants:** a king's attendants start with him and are released with him. Paimon comes with the kings Bebal and Abalam (Sloane 3825). [S]
- **Time windows:** the binding hours by rank (Goetia Observations) limit *when* each rank may be invoked. `arbatel` requires the spirit's own day at sunrise, and holding a spirit beyond one hour is forbidden (Aph. XXI). [S] The precise Goetic table is to be transcribed from Sloane 3825.

8.4 **Spirits may not tell the truth.** A spirit's output carries a provenance mark and is never trusted until examined. Two kinds of act are distinguished:
- **natural acts:** predictable functions of the spirit's office;
- **free acts:** where an output may be wrong, partial or illusory.

[S: Arbatel general precepts. O: the distinction as a type.]

8.5 **Allegiance.** Fallen spirits act within the fallen domain, under Lucifer's derivative sovereignty [Architecture §4].
- A capability belonging to a loyal office MUST NOT be passed to a fallen spirit.
- Effects proposed by fallen spirits are examined under the stricter rule set.

## 9. Release

| Operation | Effect | Source |
|---|---|---|
| `LICENSE_TO_DEPART` | orderly completion; temporary capabilities revoked | 1446 p216 L; Arbatel XXI, "vade in pace ad ordines tuos"; Goetia licence |
| `DISMISS` | ends the invocation without completion; uncommitted effects are discarded | [Architecture §12] |
| `BANISH` | forced termination: capabilities revoked, child tasks terminated, uncommitted effects discarded, results quarantined in the **Vessel** | fire curse (1446 p215 R); "Spirits chaine" (Sloane 3825); the Brass Vessel (Sloane 3825) |

**[O]** the Vessel is the quarantine store named after Solomon's brass vessel.

**[S]** Every invocation MUST end in one of these three; RELEASE is enforced by the ink budget.

## 10. Examination

10.1 **The examiner is fixed by the Constitution.** In `rudd`, spirit *n* is examined by Shem angel *n*. Example: Furfur (34) is examined by Lehahiah (34, להח).
- [S: Rudd, Harley 6483, via Skinner & Rankine pp. 366-376.]
- Examiners are assigned by the Constitution. Programs MUST NOT choose, replace or impersonate them.

10.2 **Other rites.** Rites without a fixed pairing use the Constitution's angelic examiners by office (ANGELICUS, MILITIA), never an agent written by the same author as the examined work. [O]

10.3 **What examination checks:** the declared office, the ink and seal classes, the budget, the jurisdiction, the time window, and the rite's specific rules.
- Examination checks **declared** behaviour. It does not claim to judge intent; that is a proven limit of computation, not an omission.

10.4 **The verdicts:**
- `APPROVE`;
- `REJECT`;
- `HOLD`: insufficient grounds to decide.

Examination is not judgement (section 11). [Architecture §10]

## 11. Judgement and the three destinations (ruling 2026-11-10)

11.1 **Only the Constitution's judgement step assigns a destination,** on the basis of the examiner's verdict. No agent assigns a destination to its own work.

11.2 **The destinations:**

| Destination | Mechanics |
|---|---|
| PARADISE | examined, approved, committed: final and permanent |
| PURGATORY | held: not committed, not destroyed; awaits re-examination |
| HELL | banished: capabilities revoked, tasks terminated, results quarantined in the Vessel and sealed; irreversible within the program |

11.3 **Real states, symbolic claim.** The states are real and have real consequences. The claim is symbolic: the runtime never claims to determine actual divine judgement [Architecture §4; JUDGEMENT law].
- [S] In the sources the magician binds and banishes but never condemns, and some spirits' ends are left open: several hope to return "after 1200 years".

## 12. Angels

12.1 **Angels hold authority from the Constitution only,** never from the Operator [Architecture §11]. The ministries are NUNTIATIO, MINISTERIUM and MILITIA [O: from the Architecture].

12.2 **The orders, per rite:**
- `iuratus`: the nine orders serve God alone and MUST NOT be invoked (ch. III), in the order Cherubim, Seraphim, Thrones, Dominations, Virtues, Principalities, Powers, Archangels, Angels.
- `janua`: the nine Keys invoke the orders to visible appearance through the crystal, each with its seal pair (Sloane 3825, frames 57-99).
- Neither rite overrides the other (ruling 2026-11-10).

12.3 **No supernatural authentication.** The runtime authenticates software identities, signed records and Constitution entries, never supernatural revelation [Architecture §11].

## 13. Failure classes

The runtime MUST distinguish four classes [Architecture §8]:
1. **Computational error:** for example, division by zero.
2. **Denied authority:** a missing or lapsed capability, a wrong ink, outside the jurisdiction, outside the office or time window.
3. **Failed examination:** the verdict was REJECT or HOLD.
4. **Judgement:** the work was assigned to a destination.

## 14. Audit

Every grant, inscription, invocation, examination, verdict, judgement and release is recorded. Each record names:
- the rite;
- the deciding source for each attribute;
- the chain of origin of every capability used.

## 15. Open items

- **Keywords:** Latin, English or mixed (the owner to decide).
- **Transcriptions from Sloane 3825:** the seal metals and binding hours by rank (Observations), frame 41L (the 72 angel names: if the *Janua* and Rudd count as one work, 41L decides the spellings), and frame 40 (the angels of the lunar mansions).
- **The `heptameron` rite:** verify the sigils against an early printing; Tuesday's angel is Samael.
- **Amy's seal:** compare with Peterson's g58.jpg.
- **Possibly older seals:** Sloane 3853 ff. 257-259 and the Folger V.b.26 originals.
- **Ganell, the four kings and their *ydea*** (SSM L.3.f.21, 1346): the oldest witness of the kings. It belongs to the `iuratus` line, so its directions apply within that rite.
