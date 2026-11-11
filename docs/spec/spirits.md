# Annulus Specification, Part 1a: The Spirits: Offices, Will, Malice and Wards

Status: draft 1, 2026-11-10. Normative once approved by the owner. The register this part governs is `docs/registers/goetia.md`, generated from `docs/registers/goetia.json` by `tools/build_register.py`.

## 1. Witnesses and precedence (`goetia` and `rudd` rites)

| Attribute | Decided by | Then |
|---|---|---|
| offices, legions, names, directions | Weyer, *Pseudomonarchia* (1577) | where Weyer is silent or lacks the spirit: Sloane 3825; then the 1904 edition |
| rank, seal, seal metal | Sloane 3825 (ruling 2026-11-10) | Weyer's ranks are recorded as variants |
| additions | a later witness's office or warning stands if it does not contradict an older one | |

Every line of the register carries its witness: **W** Weyer 1577 (Latin, with Scot's English of 1584), **S** Sloane 3825, **G** the 1904 edition. **M** marks the owner's rulings, which are original to Annulus and never tradition.

Contradictions resolved by the oldest witness: Naberius (loss of dignities, not their restoring), Paimon (face north), Sitri (secrets of women), Zagan (blood and oil), Osé (one hour). One is resolved by internal evidence: Haures lies when **not** in a triangle. Weyer's "si fuerit in triangulo, mentitur" contradicts Weyer's own Furfur and Shax, and Sloane 3825 reads it with the negation.

## 2. The will

2.1 **Spirits have will.** The fall was volitional, and binding constrains a spirit's conduct, never its will. [S: the spirits "fell" and some still hope to return (Marchosias, Phenex, Focalor, Amy); the whole apparatus of constraint, bonds, chain and curse presumes a will that resists.] So no spirit is a function. Every request is answered by the spirit's choice among the outcomes its nature and its constraints still allow.

2.2 **The outcomes.** [O: the mechanism.]

| Outcome | Meaning | Open to |
|---|---|---|
| `OBSEQUIUM` | obeys fully and truly | every spirit |
| `MORA` | obeys, but late | every spirit (Amdusias's music comes "not soon or immediately") |
| `PARS` | obeys only in part | every spirit (Shax "promises obedience but does not fully keep it") |
| `MENDACIUM` | answers falsely | only spirits the sources call liars, and only while the condition that compels them is unmet (§3) |
| `MALITIA` | performs, unasked, a harm the witnesses give him (in his offices or his warnings) | only spirits whose offices the sources list as harmful, and only while the counterbalance is unmet (§4) |
| `RECUSATIO` | refuses: will not appear, enter the Triangle, or answer | every spirit, until constrained (§5) |

2.3 **How the will chooses.** Each choice is drawn from the runtime's entropy source, among the outcomes still open. It is recorded in the audit, so a run can be replayed exactly. [O] The register records which outcomes are open, never how likely they are, because no witness states likelihoods. The runtime therefore draws uniformly among the open outcomes (`dynamic.md` §5.4).

2.4 **Even the faithful are examined.** A spirit the sources call faithful or of good nature (Vassago, Orobas, Marchocias, Phenex, Seere) has `MENDACIUM` and `MALITIA` closed. Its work is still a proposal and is still examined, because the examination law admits no exemption.

## 3. Truth and its compulsion

The register gives each spirit one veracity class. The class names are [O]; the condition is quoted from the witness.

| Class | Meaning | Examples |
|---|---|---|
| `VERAX` | the witnesses say he answers truly; no condition is stated | Gusion, Botis, Bune, Asmoday (when named) |
| `COACTUS` | he answers truly only under a stated constraint; `MENDACIUM` is open until it is met | Furfur (triangle), Shax (triangle), Haures (triangle), Belial (divine power, after offerings) |
| `MENDAX` | a liar or deceiver; any stated constraint compels him only for a limited time, or none is stated | Berith (the ring compels him for one hour, W); Valefor (deceives by familiarity, W) |
| `FIDUS` | faithful, deceives none | Orobas ("deceives none, permits no tempting", W), Marchocias (W) |
| `TACITUS` | the witnesses say nothing of his truthfulness | the rest; `MENDACIUM` is closed, but his answers stay unverified free acts (`authority.md` §8.4) |

**The Triangle in Annulus is always present** (`authority.md` §6.4). So triangle-compelled spirits answer truly whenever properly invoked. Their lies become possible only where a rite's apparatus is incomplete, for instance where a spirit's own required figure (Furfur's triangle, Wellcome 110) is missing.

### 3.1 What a lie is

A lie takes one of two forms, depending on the kind of office. [O: the mechanism.]

1. **Lies of information.** These arise in the offices that tell, listed in `offices.md` §2.1, the one place that list is kept. The true answer is replaced by a different well-formed value of the same type, drawn like any choice of the will (`dynamic.md` §5.4). Nothing in its form betrays it, so examination cannot catch it. Only constraint closes it, as in the sources.
2. **Lies of work.** These arise in every other office. A lie can only be a false account of what the proposal does. The examiner inspects the proposal's actual effects, so the false account is caught: `REJECT`, and the proposal goes to `INFERNUS`.

### 3.2 Belial and Berith

- **Berith** tells things past, present and to come. His lies about the present and the future are of the first kind. His lies about the past are now caught, because `PRAETERITA` is checked against the history (`offices.md` §2.2a). The ring of art closes `MENDACIUM` for **one hour** from its use; after the hour it is open again. [S: "compelled for an hour by a ring of art", W.]
- **Belial's** offices all do work (dignities, favour, familiars), so his lies are of the second kind, and examination catches them. `MENDACIUM` closes only when **both** conditions are met: offerings, sacrifices and gifts, and then constraint by divine power. [S: W and G agree on both.]
- **Until a program meets their conditions** with the apparatus (§4.4), every answer from either is held in `PURGATORIUM`. It is never committed, and never readable by the program, whether it happens to be true or not.

## 4. Malice and its counterbalances

4.1 **Malice is sourced, not invented.** A spirit has `MALITIA` open only for harms the witnesses give him: Focalor slays and drowns; Vepar and Sabnock rot wounds; Andras "will slay both him and his fellows"; Valefar draws his familiars on "until they are brought to the gallows" (W); Malphas "deceives all sacrificers" (W).

4.2 **Every malice has its counterbalance, as stated by the witness.** Where the witnesses give one, it is recorded with it:

| Spirit | Danger | Counterbalance (source) |
|---|---|---|
| Focalor | slays, drowns, sinks ships | "harms no one if so commanded" (W, G): a declared prohibition closes `MALITIA` |
| Vepar | wounds that rot in three days | "all can be healed with care" (W): every such effect must be reversible |
| Astaroth | noisome breath that harms the Operator | the magical ring held to the face (G; silver ring, W) |
| Beleth | fury at his first appearance | hazel wand, triangle, the Bonds and Charges, the silver ring (W, G) |
| Asmoday | Amaymon deceives the Operator | stand, abroad, head uncovered; ask "Art thou Asmoday?" (G); speak his name (W) |
| Berith | lies | the ring compels him for one hour (W) |
| Belial | no true answers | offerings; then truth only under divine power; if he refuses, read the bond of spirits (W, G) |
| Andras | slays Operator and fellows | "if the Exorcist have not a care" (G): no counterbalance stated beyond the apparatus, so `RELEGA` is the only remedy |
| Malphas | accepts sacrifice and deceives the giver (W, G) | none stated. [O] Offerings to Malphas are refused by the runtime |

4.3 **The examiner is the standing ward.** Under `rudd`, Shem angel *n* examines spirit *n* (`authority.md` §10). Any `MALITIA` or `MENDACIUM` that reaches a proposal is what examination is there to catch:
- an effect not of the requested office is `REJECT`;
- a harmful effect without its counterbalance met is `REJECT`;
- a rejected proposal goes to `INFERNUS`, and its results are sealed in the Vessel.

[S: Rudd, Harley 6483, which pairs every Goetic spirit with his angel. The older principle that each demon has an angel "by whom he is frustrated" is the *Testament of Solomon*'s, a separate work; it is cited for context, not as law.]

### 4.4 The apparatus

The Operator takes up the instruments inside a Circle (`grammar.md` §3.3). Each piece does what its witness says it does, and nothing more.

**General instruments:**

| Instrument | Effect for every spirit | Source |
|---|---|---|
| `GERE ANULUM` | protects the Operator from spirits' harmful breath and fumes | S f113; G |
| `GERE HEXAGRAMMUM` | once the spirit has appeared, `RECUSATIO` is closed for its requests: "compelled to be obedient". Delay (`MORA`) and partial work (`PARS`) stay open: obedience is not completeness | S f112; G |
| `GERE PENTAGRAMMUM` | "to preserve thee from danger": harm aimed at the Operator himself is closed | G (preserve); S f112 (to command) |
| `GERE VIRGAM` | no general effect stated | |

**What each spirit requires or allows:**

| Spirit | Rule | Source |
|---|---|---|
| 13 Beleth | `INVOCA` requires `ANULUM` and `VIRGAM`; his fury (`MALITIA`) is closed by the wand | G "he must hold a Hazel Wand", "thou must have always a Silver Ring"; W |
| 28 Berith | `INVOCA` requires `ANULUM`; the ring closes `MENDACIUM` for **one hour** from its taking up | G "thou must make use of a Ring"; W "compelled for an hour" |
| 29 Astaroth | `INVOCA` requires `ANULUM`; it closes his `MALITIA` (the noisome breath) | G "Thou must in no wise let him approach … hold the Magical Ring"; W |
| 32 Asmoday | `APPELLA` closes `MENDACIUM` | W "unless the exorcist speaks his name"; G "Art thou Asmoday?" |
| 68 Belial | `MENDACIUM` closes only after `OFFER` **and then** `CONSTRINGE … PER NOMEN`, in that order | G "must have Offerings … unless he be constrained by Divine Power"; W |
| 9 Paimon | without `OFFER`, every request is `RECUSATIO`; without `CONSTRINGE`, his answers are only `PARS`, since "his speech is such that the Magician cannot well understand unless he can compel him" | G; W (offering) |
| 41 Focalor | `VETA` closes his `MALITIA` | W "harms no one if so commanded"; G |
| 63 Andras | `PENTAGRAMMUM` closes his harm to the Operator. **[O link]** The witnesses state no counterbalance for him; this applies the pentagram's general effect | G (pentagram) |
| 39 Malphas | `OFFER` is refused (a denial): he "deceives all sacrificers" | W; G |
| 33 Gaap | `OFFER` is allowed; no effect is stated | W "receives offerings" |
| 42 Vepar | his `MORBUS` effects must be reversible | W "all can be healed with care" |

**What is recorded but not enforced:** these conditions have no mechanical analogue yet, so the register keeps them as notes:
- Asmoday's other conditions: called abroad, standing, head uncovered, lest Amaymon deceive.
- The vestments: sceptre or sword, mitre or cap, white linen robe, lion-skin girdle.
- The making of the Secret Seal: Tuesday or Saturday at midnight, the Moon increasing in Virgo, a black cock's blood.

The instruments themselves are part of the Constitution's apparatus, like the Circle and the Triangle. A program takes them up but does not make them. [O] How they are made (gold or silver ring, calf-skin hexagram) is recorded for a later part.

**An offering costs** one unit of the spirit's ink class from the faculty. [O: an offering is a gift of the Operator's own substance.]

### 4.5 Andras as the tester (`PROBATIO`, ruling 2026-11-10)

1. **Only when commanded.** Andras tests the program only when invoked `AD OFFICIUM probatio`. The office belongs to no other spirit.
2. **Both of his natures are used.** His sourced office, discord (W, G), and his sourced danger, killing "the master, his servants and assistants" (W), are both turned on **copies** of the Operator's own program. They test whether its parts hold under conflict, and whether they survive being ended. [S: the office and the danger. M: their use for testing.]
3. **The pentagram protects the Operator from his killing for as long as it is worn,** that is, until the Circle in which it was taken up closes. It is required to invoke him for `probatio` (static rule A11). Without it, his killing reaches the real program, as the sources warn.
4. **Copies only.** Testing is an **illusory** effect (`authority.md` §7.5): nothing it does to the copies can commit. Each `PETE` for `probatio` spends one unit of `"vespertilio"` ink (bat's blood) from the faculty, besides the marquis's ink for his seal.
5. **What comes back** is a proposal holding only his **report** of the failures he found in the copies. Only the report can reach `PARADISUS`.
6. **He keeps his will.** `MORA`, `PARS` and `RECUSATIO` stay open, and a partial report is held in `PURGATORIUM`. Any attempt on the real program is outside the office: examination rejects it, and it goes to `INFERNUS`.

## 5. Refusal and the escalation of constraint

`RECUSATIO` is met by the Goetia's own sequence. Each step is stronger than the last, and each is recorded:

1. **Coniuratio:** the conjuration to call forth.
2. **Coniuratio secunda:** the second conjuration.
3. **Constrictio:** the constraint.
4. **Invocatio regis:** the invocation of the king he is under (`INVOCA REGEM`; skipped where his king is not stated).
5. **Catena spirituum:** "the General Curse, called the Spirits' Chain, against all Spirits that rebel".
6. **Ignis:** the conjuration of the fire. The spirit's seal, bound in a box, is held over the fire.
7. **Maledictio maior:** the greater curse, which is `RELEGA`: banishment, with results to the Vessel.

[S: the sequence and its headings, 1904 edition; "Spirits chaine", Sloane 3825; Weyer for Belial, "the bond of spirits must be read". O: steps as escalating retries, each spending ink and time.] The curse is named for rebellion: refusal is the will's act, and constraint answers it.

Beleth is the model case: if he will not enter the triangle, "rehearse the Bonds and Charms before him, and then he will yield Obedience" (G).

## 6. Offices as computer work

6.1 **One mapping per kind of office, applied to every spirit alike.** The sources' offices fall into recurring kinds, and each kind maps to one kind of computer work. The mapping is [O], but it is fixed and systematic: a spirit's computer task follows from what the source says he does, never from a free reading of his character.

| Kind | Source office (examples) | Computer work |
|---|---|---|
| `PRAETERITA` | tells things past | reads the **history**: the present run and the one before it, never the audit itself (`offices.md` §2.2a, ruling 2026-11-11). Its answers can be checked, so a lie about the past is caught |
| `PRAESENTIA` | tells things present | inspects current state |
| `FUTURA` | tells things to come | forecasts and estimates; always a free act, never certain |
| `OCCULTA` | discovers things hidden, secrets | searches for unindexed or concealed data |
| `THESAURI` | discovers treasure; treasure "kept by spirits" (Amy) | locates valuable records; data held by other invocations |
| `AMISSA` | recovers things lost | recovers missing or deleted data |
| `RESPONSA` | true answers to demands | answers a query |
| `DOCTRINA` | teaches an art (astronomy, logic, rhetoric, geometry, arithmetic, grammar, philosophy, mechanics) | runs the matching domain library (dates and ephemerides, validation, text, geometry, arithmetic, parsing, build tooling) |
| `LINGUAE` | teaches tongues; the voices of birds, beasts and waters | translates between encodings, formats and protocols |
| `VIRTUTES` | virtues of herbs, stones, woods, stars | looks up the properties (metadata) of things |
| `CONCORDIA` | reconciles friends and foes; causes love | merges, links and resolves conflicts between records |
| `DISCORDIA` | sows discord, hatred, battles | creates conflict and contention: splits, races, load |
| `DIGNITATES` | gives, confirms or takes away dignities and prelacies | **proposes** role and privilege changes; never grants them (axiom 1.2) |
| `FAMILIARES` | gives good familiars; frees familiars from other magicians | starts helper tasks, capped by legions; frees helpers from other invocations, or from another program only under the launching person's oversight (`offices.md` §4.6) |
| `VECTIO` | carries men or things from place to place | moves or copies data between locations |
| `FURTUM` | steals treasure; brings back thief and goods | takes data out of a place / detects and reverses unauthorized taking |
| `INVISIBILITAS` | makes a man go invisible | conceals data: a **veiled** effect, bat's-blood ink (`authority.md` §7.5) |
| `ILLUSIO` | similitudes, seeming candles, noises without water, delusions | produces **illusory** effects that can never commit: dry runs, test doubles; bat's-blood ink |
| `TRANSMUTATIO` | metals to gold or coin; water, wine, blood, oil; changes shapes | converts data from one form to another |
| `AEDIFICATIO` | builds towers, houses, cities; furnishes them | creates structures: files, directories, indices |
| `EVERSIO` | destroys cities, walls, dignities; earthquakes | deletes structures |
| `TEMPESTAS` | storms, thunder, lightning, rough waters, great noises | disrupts on purpose: load and fault injection |
| `NEX` | slays, drowns, sinks ships | terminates tasks |
| `MORBUS` | causes and cures disease; rots wounds; heals | corrupts or repairs data |
| `MORTUI` | brings souls of the dead to answer; moves dead bodies | reads and relocates archived or ended data |
| `MENS` | knows and changes thoughts; takes away sight, hearing, understanding; makes witty, bold, insensible | inspects or alters another task's inputs and state |
| `CUSTODIA` | guards treasure; suffers no other spirit to tempt the Operator | monitors other invocations on the Operator's behalf, without approving anything |
| `CONSECRATIO` | teaches to consecrate the things of Amaymon's dominion (Gaap) | proposes validators for consecration; the Constitution still signs (`authority.md` §7.4) |
| `TUTELA` | gives the Ring of Virtues (Asmoday, W) | protects a thing against deletion (`EVERSIO`) and termination (`NEX`). It is proposed and examined like any effect, and it never outranks the faculty: the person who launched the program can always lift it. The sources name the ring without saying what it does, so this mapping is [O] (ruling 2026-11-10) |
| `LONGAEVITAS` | makes a man live long (Foras, W) | lengthens a lifetime: how long data is kept before it expires, or how long a task stays alive. Never beyond the faculty's budget, for no spirit gives time any more than it mints ink |
| `PROBATIO` | **none: the owner's ruling (M), for Andras only.** His danger, to "kill the master, his servants and assistants" (W), turned under command into an office | attacks **copies** of the Operator's own program to find where it fails (§4.5) |
| `APPELLATIO` | gives a good name (Forneus, W) | names and labels things: file names, titles, descriptions. **Partial:** "good name" means reputation, and its other half, trust, is not offered, because a fallen spirit cannot vouch for anything (axiom 1.1) |

**Signatures:** what `PETE` passes and what the proposal holds for each kind are in `offices.md` (Part 1c).

6.2 **Harmful kinds stay harmful.** `DISCORDIA`, `EVERSIO`, `TEMPESTAS`, `NEX`, `MORBUS`, `FURTUM` and `MENS` are real, destructive computer work: deletion, termination, corruption, disruption. They act only on what the faculty grants and only after examination. They are not softened: a spirit invoked for `NEX` really ends tasks.

6.3 **Offices the computer cannot hold.** Some offices have no honest computer analogue: making one "invincible" (Asmoday, G). It is the only one: by ruling 2026-11-10 the Ring of Virtues, from the older witness, holds the protective office instead. The register keeps them, marked `NULLUM`. Invoking a spirit for them is an error (law OFFICE): the office exists in the tradition, but Annulus does not pretend to perform it.
