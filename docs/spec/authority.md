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

1.5 **No amplification: the guiding principle** (ruling 2026-11-11). *The type system makes it impossible to construct a program whose authority exceeds the authority granted to it, including through multiple levels of delegation.*

Stated precisely: let *G* be the faculty granted at launch. For every program that passes Part 3, and every run of it:
- every capability ever exercised is one that *G* contains;
- every delegated capability (a seal handed to a spirit, a helper task started by a spirit, an inner Circle, a function call) is contained in the capability it was delegated from;
- containment composes, so authority at depth *n* is contained in authority at depth *n − 1*, and so in *G*.

This is a property **of the type system**, checked before the program runs. It is not a runtime hope. Every rule in this specification MUST preserve it. A rule that cannot be shown to preserve it is a defect, whatever its source. Where tradition and the principle collide, the conflict is raised with the owner, never silently resolved.

**How the rules already serve it:**

| Rule | How it narrows authority |
|---|---|
| Faculty (§2.4) | the single root: nothing exists outside *G* |
| No self-granting, approval or minting (§1.2–1.4) | no entity adds authority from itself |
| Rite values bound to their Circle (`static.md` §4.7) | capabilities cannot leak sideways through functions, sequences or outer variables |
| Inner Circles only narrow (`static.md` §4.12) | nesting never widens |
| Legion cap (`dynamic.md` §8) | a spirit's helpers are bounded in number |
| `DIGNITATES` proposes only; `TUTELA` and `LONGAEVITAS` never exceed the faculty (`spirits.md` §6) | offices that sound like grants are proposals bounded by *G* |
| Intruders hold nothing (§6.2a) | authority cannot enter from outside |

**Proof obligations, still open:**
1. **Helper tasks.** Each spirit's helper tasks (`FAMILIARES`, and the sub-tasks of any office) MUST hold only capabilities contained in the seals given to that spirit. The office signatures (`static.md` §7) must be written to make this checkable.
2. **The Constitution's own acts.** Consecration, examination, the invocation of a king and the escalation steps are the Constitution's acts, not the program's. They MUST spend only what *G* provides (ink, time) and grant nothing beyond it.
3. **A machine-checked proof.** The principle MUST be proven over a formal model of Part 3's typing rules, and tested by Andras's `probatio` against the real runtime (roadmap: EXAMINATIO).

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

2.5 **Amendment** (draft 1, 2026-11-11, for the owner's approval). The Constitution changes only between releases, never while a program runs (§2.1).
- **The version.** Every release carries a Constitution **version**, and a **fingerprint**: a hash of all its contents (§2.2, with the core spec, `core.md`). A program's first audit record, beside the faculty, names both.
- **Checked under one version, run under one version.** A program checked under one version MUST be checked again before it runs under another, because the static rules read the registers. Replaying a run (`dynamic.md` §5.4) needs the same version.
- **Who amends.** Only the owner approves a change. No program, spirit, angel or agent can amend the Constitution, and no AI session either: drafts from Claude, GPT or any other assistant are **petitions**, as a spirit's `CONSECRATIO` candidate is (`offices.md` §2.4). [O] Honestly stated, as with the faculty (§2.4), the owner is the real-world source of the Constitution's authority.
- **Three kinds of change, by layer** (`core.md`):

  | Kind | What it touches | What it needs before approval | Version step |
  |---|---|---|---|
  | **Core** | any core element C1–C9, or the core mechanics (effect verbs, grant kinds, ink budget, taint, failure classes), or the interface of `core.md` §3.1 | a new proof over the formal model (roadmap step 3), the attack tests passing, and an audit | major |
  | **Rite** | registers, offices and their signatures, hours, conditions, seals, ink classes, pairings, divine names | a source check under the precedence rule (`provenance.md`), recorded with its deciding witness in the cross-check or source log, and the registers rebuilt from the JSON | minor |
  | **Editorial** | wording only, with no change of meaning | a reading by the owner | patch |

- **A rite change may not cross the line.** If a rite change would need anything the interface of `core.md` §3.1 does not already offer (a new way to grant, or to skip examination), it is a core change and needs the proof.
- **The record of a change.** Each release lists every changed entry, the reason, the deciding source or the proof, and the date of approval (`CHANGELOG.md`).
- **What survives an upgrade.** The Index of the proscribed and the audit persist across versions. An upgrade never lifts a proscription (`operator.md` §4). The history of the previous run (`offices.md` §2.2a) stays readable, marked with the version it ran under.

[S, model: the *Liber iuratus* opens with a council of masters who decide together how the art is kept, and the book may pass only to a successor who has been tried. Changes to the law come by deliberate council and trial, never mid-work. O: versions, fingerprints and the three kinds.]

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
| `goetia` | Lemegeton Goetia, within the *Liber officiorum spirituum* line (ruling 2026-11-10) | for names, offices, legions and directions: the oldest *Officium* witness (Oberon c. 1577-83, Wellcome 110, Weyer); for seals, **ranks** and seal metals: Sloane 3825 (1641+) (ruling 2026-11-10: rank goes with the seal and the metal table, from one book; Weyer's ranks are kept as variants) |
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
| Divine and sacred names | 71 | `docs/registers/nomina.md`: the Circle of Solomon, Triangle, hexagram, pentagram and Ring as drawn in Sloane 3825 f112-113, and the names of the First Conjuration; the 1904 edition fills only what the manuscript leaves illegible |
| Kings of the quarters | 4 per rite | `docs/registers/reges.md`. `goetia`: Amaymon E, Gorson (Corson) S, Zymymar (Ziminiar) N, Goap W (Weyer 1577, ch. III, the oldest witness to pair them; Sloane 3825 names the same four without pairing). `clavicula`: Oriens E, Paimon W, Amaymon S, Egin N (BnF 1524, ruling). The Oberon and Wellcome 110 set (the four 'kings of the firmament') is recorded as a variant |

5.3 **Identity is the number.** Numbers never change. Names are labels and MAY be corrected without breaking stored data. [O; this follows Occultist Explorer's key derivation by number.]

## 6. Circle and Triangle

6.1 **The Circle protects the Operator.** The Circle is the Operator's protected jurisdiction: the bounded space in which authority-sensitive work happens. [S: in every Solomonic rite the operator stands within the circle.]

It defines permissions, prohibitions and resource limits [Architecture §9].

6.2 **One entrance.** A Circle has exactly one declared entrance: the only path by which capabilities and messages cross into it.
- [S: BnF 1524 p246, the Book II circle with an open corridor into the centre, and p191 L.]
- [O: the single-entry rule.]

6.2a **The uninvited go to Hell** (ruling 2026-11-10). Any agent that reaches, or **attempts** to reach, a Circle other than through an invocation the Operator made there is an **intruder** (fourth audit, 2026-11-11: since its messages are refused at the entrance, the recorded fact is the attempt; the step is `intrude`, `core.md` §9.2): a spirit not invoked, or an outside program or AI agent acting on the program without a grant. It is not examined, because it has nothing to examine. No capability it holds traces to the faculty (law ORIGIN), and whether it was invoked is a recorded fact, not a judgement, so no examiner's error can condemn it. Its messages are refused at the entrance, everything it sent is sealed in the Vessel (`INFERNUS`), it is barred from the program for the rest of the run, and all of it is recorded. [S: the Circle "to preserve himself from the malice of those evill Spirits" (Sloane 3825 f112); Orobas and Haures suffer no other spirit to tempt the Operator (G, W). O: the mechanism.] (Ruling 2026-11-11: "go to Hell" means that everything they produced is sealed in `INFERNUS`, and that the intruder itself is **damned**, irreversibly, by its seal, on the recorded fact of intrusion alone: `core.md` §9.2, `proscription.md` §1.)

**Proscription** (ruling 2026-11-11). An intruder is banned **on every level at which it sought to enact change**: the program, every Circle it reached, and every resource it tried to alter (each file, place, grant or invocation it aimed at). The ban is entered in the **Index of the proscribed** (*index proscriptorum*), which the Constitution keeps outside every program. It holds:
- every identity the intruder presented;
- every target it aimed at;
- the audit records proving the intrusion.

The ban **persists across runs**. Each later launch refuses those identities at those targets before any program code runs. No program and no agent can read, alter or lift the Index. Only the person who launches programs on that installation may lift an entry, by an explicit, recorded act outside any program. [O]

**Why lifting stays possible:** an attacker could present someone else's identity in order to have them proscribed. Lifting is therefore kept, as the one remedy against being framed. Detection itself is never weakened.

**The Vessel is never opened for use.** What is sealed in it is never released, committed, run or reused, by anyone, the Operator included. The Operator may *read* it, to learn what was attempted; reading changes nothing. [S: Sloane 3825 f113, the warning: the Babylonians broke the vessel "hoping to find great treasure", and the spirits fled to their former places, save Belial, who "entered into a certain Image". O: read-only access.]

Invited agents keep the rule of §11 for what they do through their invocation: Hell applies to their *proposals*, and their release stays the Operator's act. An invited agent that attempts to reach a Circle where it holds no open invocation, or acts after its release, is an intruder there (ruling 2026-11-11, `core.md` §9.2).

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
| by rank of spirit | the seal metal of the rank (table 7.5a) | a spirit's operative seal requires the ink class of its rank |

**7.5a Seal metals by rank.** [S: Sloane 3825 f111 (119R), the "Observations": "The seals of those 72 kings are to be made in Mettalls"; the 1904 edition agrees.]

| Rank | Metal (planet in Sloane 3825) | Ink class |
|---|---|---|
| King (*rex*) | gold (☉) | `"rex"` |
| Marquis (*marchio*) | silver (☽) | `"marchio"` |
| Duke (*dux*) | copper (♀) | `"dux"` |
| Prelate (*praelatus*), read as Prince | tin (♃) | `"princeps"` |
| Knight (*eques*) | lead (♄) | `"eques"` |
| President (*praeses*) | mercury (☿) | `"praeses"` |
| Earl (*comes*) | copper and silver "equally alike" (♀ & ☽) | `"comes"` |

- Sloane 3825 names **Prelates**, not Princes. The 1904 edition writes "Princes and Prelates", and Peterson's edition reads Prelates as Princes. The spirit list uses "prince" (Vassago, Sitri, Stolas), so the reading is adopted, labelled as a reading.
- Iron (♂) is absent from the list. [S, as an absence; noted by Peterson.]
- **Mixed ranks** (for example Botis, president and earl): the sources give no rule. Open item.

7.6 **Tempering: binding at creation.** An instrument's limits are fixed when it is made and can never be widened.
[S: the knife "Artano" tempered in goose blood (1446 II.7). O: the immutability rule.]

## 8. Invocation

8.1 **For a declared office only.** A spirit is invoked for one of its declared offices only.

8.2 **The invocation steps:**
1. The Operator declares intent.
2. The Circle is closed.
3. The Operator inscribes the spirit's operative seal (section 7).
4. The spirit is invoked into its Triangle, bound by that seal, for the declared office.
5. The spirit proposes effects.
6. The paired angel examines them (section 10).
7. The proposal is judged (section 11).
8. The spirit is released (section 9).

[Architecture §9; S: the Key, Goetia and Arbatel sequences.]

8.3 **Rank limits.**
- **Legions** cap the number of concurrent sub-tasks a spirit may start. [S: legion counts. O: the cap.]
- **Attendants:** a king's attendants start with him and are released with him. Paimon comes with the kings Bebal and Abalam (Sloane 3825). [S]
- **Time windows:** the binding hours by rank limit *when* each rank may be invoked under `goetia` and `rudd`. **Weyer 1577 (ch. III) governs** (ruling 2026-11-10: the oldest witness of the line). Sloane 3825 f111 is the later copy: it restates Weyer's canonical hours as clock hours, and where it only restates them, its wording is the reading of Weyer's terms.

  | Rank | Weyer 1577 (governs) | Sloane 3825 (later copy) |
  |---|---|---|
  | King | "à tertia hora usque ad meridiem, à nona hora usque ad vesperas" | from 9 till 12 at noon, and from 3 till sunset |
  | Marquis | "à nona usque ad completorium, vel à completorio usque ad finem diei" | 3 in the afternoon till 9 at night, and 9 at night till sunrise |
  | Duke | from the first hour until noon, in clear weather | sunrise till noon, in clear weather |
  | Prelate (Prince) | any hour of the day | any hour of the day |
  | Knight | "ab aurora usque ad ortum solis, vel à vesperis usque ad finem solis" | dawn till sunrise, or 4 o'clock till sunset |
  | President | **"in aliqua hora diei non potest constringi, nisi rex cui paret, invocaretur, & nec in crepusculo noctis"**: never unless the king he obeys is invoked, and never at nightfall | any time except twilight, unless the king is invoked. **Conflicts; Weyer prevails** |
  | Earl | any hour, in woods or fields where men do not go | the same, and "where no noise is" (a later addition that does not conflict, so it stands) |

  The same page adds the **Moon's age:** the work is done when the Moon is 2, 4, 6, 8, 10, 12 or 14 days old, "as Salomon sayeth". [S]

  `arbatel` requires the spirit's own day at sunrise, and holding a spirit beyond one hour is forbidden (Aph. XXI). [S]
- **The four kings over the 72 (`goetia`):** the same page puts the 72 under Amaymon, Corson, Ziminiar and Goap, "kings ruling in ye 4 quarters East, West, North & South", not to be called "except it be upon great occasions". [S] Whether the two lists pair in order (Amaymon East, Corson West, Ziminiar North, Goap South) is not stated; it is recorded as unresolved. Weyer (ch. III) gives the pairing: Amaymon East, Gorson South, Zymymar North, Goap West. They may be bound "from the third hour till noon, and from the ninth hour till evening" (*à tertia hora usque ad meridiem, à nona hora usque ad vesperas*). [S] See `docs/registers/reges.md`.
- **Presidents need their king.** A President's king is the `goetia` king of the quarter the witnesses give him (ruling 2026-11-10: by quarter, not by a name from the other king-set). Presidents with a stated quarter: Gaap and Marbas (East, Amaymon); Foras, Malphas and Osé (South, Gorson); Zagan (West, Goap). A President without a stated quarter cannot be invoked (`static.md` §4.5b, A13). `INVOCA REGEM` (`static.md` §4.5b) invokes the king. Spirits of two ranks (Botis, Marax, Gaap, Zagan) may still be invoked within their other rank's window.

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
| `VADE IN PACE` (licence to depart) | orderly completion; temporary capabilities revoked | 1446 p216 L; Arbatel XXI, "vade in pace ad ordines tuos"; Goetia licence |
| `DIMITTE` (dismiss) | ends the invocation without completion; uncommitted effects are discarded | [Architecture §12] |
| `RELEGA` (banish) | forced termination: capabilities revoked, child tasks terminated, uncommitted effects discarded, results quarantined in the **Vessel** | fire curse (1446 p215 R); "Spirits chaine" (Sloane 3825); the Brass Vessel (Sloane 3825) |

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

11.1 **Only the Constitution's judgement step assigns a destination,** on the basis of the examiner's verdict. No agent assigns a destination to its own work. The core's commit step may then move an approved proposal to `INFERNUS` (its effects fall outside what was handed) or to `PURGATORIUM` (it awaits the second key), never upwards (`core.md` §9.2).

11.2 **The destinations** apply to the proposal judged, not to the spirit who made it; releasing the spirit stays the Operator's act (`dynamic.md` §6.2). An agent's own states are separate (`core.md` §9.2). Only an intruder can be damned, and only by the recorded fact of intrusion:

| Destination | Mechanics |
|---|---|
| PARADISE | examined, approved, committed: final and permanent |
| PURGATORY | held: not committed, not destroyed; awaits re-examination |
| HELL | banished: capabilities revoked, tasks terminated, results quarantined in the Vessel and sealed; irreversible within the program |

11.3 **Real states, symbolic claim.** The states are real and have real consequences. The claim is symbolic: the runtime never claims to determine actual divine judgement [Architecture §4; JUDGEMENT law].
- [S] In the sources the magician binds and banishes but never condemns, and some spirits' ends are left open: several hope to return "after 1200 years".
- [O] So the Operator never condemns, in Annulus either. Damnation (`core.md` §9.2) is the Constitution's law acting on a recorded fact, intrusion, never a judgement by anyone. Nothing a spirit does through his invocation can damn him; an attempt outside it, in another Circle or after his release, is intrusion (ruling 2026-11-11).

## 12. Angels

12.1 **Angels hold authority from the Constitution only,** never from the Operator [Architecture §11].

12.1a **Standing from God, reach from the grant** (ruling 2026-11-11). An angel's **standing** comes from the Constitution: the Operator cannot command, compel or bind an angel, only petition. An angel's **reach** in any program is the faculty *G*, like everything else (`core.md` C1). Angels have **extended privileges**, which the faculty grants to them alone:
- **Reserved grants.** A grant ending in `angelis` ("to the angels") is reserved: `liber "x" delere angelis;`. It is part of *G*, so the guiding principle holds. But only an angel may exercise it. No spirit's handed set may contain it, the program cannot use it directly, and no angel can pass it on (§8.5). [O: the word and the mechanism]
- **What angels also hold, without a reserved grant:** examination (§10), and advice through their channel (§6.5, `ADVOCA`), which is a response like any other: untrusted data until judged.
- **Only at the Operator's petition** (ruling 2026-11-11). An angel exercises a reserved grant only when the Operator petitions it through its channel. Nothing in a spirit's proposal can move an angel to act: when an angel examines a proposal, its text is data under judgement, never a request (`operator.md` §2.2). [S, in principle: only the Operator prays; angels stand against the spirits, never for them (Rudd's pairings); a loyal office's power never passes to the fallen (§8.5). O: the rule as a mechanism, closing the "confused deputy" attack, where a less-trusted party tricks a trusted one into using its power]
- **An angel's effects are examined too,** by an examiner other than that angel (law EXAMINATION, `core.md` C2), and the core re-checks them at commit (`core.md` §3.3). Which angel examines an angel is set by each rite. **Open:** the `iuratus`, `janua` and `arbatel` rules for it.

[S: in the sources the Operator only petitions angels, and they act by God's will (*Liber iuratus*; Arbatel Aph. XV). O: the reserved grant.] The ministries are NUNTIATIO, MINISTERIUM and MILITIA [O: from the Architecture].

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

- **Transcriptions from Sloane 3825:** frame 41L (the 72 angel names: if the *Janua* and Rudd count as one work, 41L decides the spellings), and frame 40 (the angels of the lunar mansions).
- **The `heptameron` rite:** verify the sigils against an early printing; Tuesday's angel is Samael.
- **Amy's seal:** compare with Peterson's g58.jpg.
- **Mixed ranks:** which metal and which hours apply to a spirit of two ranks.
- **Possibly older seals:** Sloane 3853 ff. 257-259 and the Folger V.b.26 originals.
- **Ganell, the four kings and their *ydea*** (SSM L.3.f.21, 1346): the oldest witness of the kings. It belongs to the `iuratus` line, so its directions apply within that rite.
