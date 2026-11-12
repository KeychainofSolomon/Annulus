# Session handoff: Annulus (formerly ARS MAGNA)

Written 2026-11-10 for the next Claude session. Read this first, then `docs/STATUS.md`, `docs/ROADMAP.md` and `docs/sources/sloane-3825/angelic-hierarchy.md`.

## The person

The owner writes the design handoffs himself. **Privacy rule (the owner, 2026-11-11):** never write the owner's name or any personal detail about him (background, experience, devices, location) in this repository, its commit messages or anything else on GitHub. Refer to him only as "the owner".

## The project

A real programming language whose authority and execution model is the Solomonic framework: circles, spirits, angelic examination, licence to depart and banishment, mapped onto capability security, sandboxing and transactional effects.

The governing document is the owner's "Definitive Architecture and Engineering Handoff", 18 sections. He has ruled that re-attaching it is not needed. Key points:
- The language is implemented in Rust, but Rust must not leak into the language.
- Bytecode and VM are the reference execution model; a native x86-64 backend comes later.
- Eight foundational laws: ORIGIN, SOVEREIGNTY, DECLARATION, OFFICE, CONTAINMENT, EXAMINATION, RELEASE, JUDGEMENT.
- Axiom: *nulla potestas a se ipsa oritur* (no authority arises from itself). Corrected 2026-11-11 from *sibi ipsi*, which reads "for itself".
- 14 Latin subsystems, from LOGOS to EXAMINATIO.
- 18-step development sequence.
- The implementation source must contain no comments.

## Decisions the owner has made

1. **Name: Annulus**, for the Ring of Solomon. Applied in the code. File extension `.annulus` (ruled 2026-11-11, replacing `.ann` of 2026-11-10).
2. **Zero annotations in Annulus programs.** No comment syntax exists; the lexer rejects it.
3. **"Fully obfuscated, like the real world until you look closer."** Agreed design:
   - opaque surface, fully lawful depth;
   - obscure but never ambiguous;
   - a compiler-generated "reveal" reading (the REVELATIO subsystem);
   - examination and audit always operate on the revealed form.

   The owner explicitly commits to holding this.
4. **True sigils, only if they faithfully replicate Solomon's seals.** Otherwise the feature is moot.
5. **Seals cover only the authority layer.** Ordinary code needs a different veil.
   - Planetary signs were rejected for this: in Sloane 3825 they belong to the angelic hierarchy.
   - The replacement veil is still undecided.
6. **The most authentic source wins:** the oldest manuscript.
   - Sloane 3825 is the authority; it is Peterson's base text, first half of the 1600s.
   - For the doubled seals, use one version from the most authentic period. Bune is settled: figure 31, "the least is best" per Sloane 3648 and 2731.
7. **Stay true to the source material, no matter what.**
   - No sanitising of names, seals or offices.
   - The protections (circle, triangle, examination, licence to depart) are equally mandatory.
   - Disagreements between sources are kept, with attribution.
   - Inventions are always labelled as original.

## Rulings of 2026-11-10 ("use the most intuitive option")

- Keywords: all Latin, uppercase (see `docs/spec/grammar.md`). The duplicate spellings are dropped.
- Mixed-type equality is a type error.
- Type labels are syntax, not annotations: required on function interfaces and record fields, optional elsewhere.
- Pentacles: the 1446 Key's set is the foundation; Mathers' 44 are later additions where they don't conflict, each labelled. 1446 wins where they disagree.
- Licence: public domain (the Unlicense). Anyone may use the code; no credit needed.
- File extension: `.annulus` (programs); `.acta` (audit files).
- `DIMITTE` (dismiss) and `RELEGA` (banish) are kept.
- The "Definitive Architecture" re-attachment is not needed.
- **Offices once marked impossible:** the Ring of Virtues (Asmoday, Weyer) = `TUTELA`, protection against deletion and termination, taking precedence over the 1904 'invincible', which stays `NULLUM`. Foras's long life = `LONGAEVITAS`. Forneus's good name = `APPELLATIO`, naming only (partial).
- **Rank of a Goetic spirit comes from Sloane 3825** (with the seals and the metal table). Weyer's ranks are variants. Legions, names, offices and directions still come from the oldest *Officium* witness (Weyer where it speaks). See `docs/sources/occultist-explorer/CROSSCHECK.md` for the 8 legion corrections.

## Spec progress (step 3)

- Part 1 `authority.md`, Part 2 `grammar.md`, Part 3 `static.md`: draft 1 each, awaiting the owner's approval.
- The grammar is LL(1). `tools/grammar_check.py` checks it against `tests/grammar/` (57 programs) and runs in CI.
- Part 1a `spirits.md` and the 72-spirit register `docs/registers/goetia.json` (rendered to `goetia.md` by `tools/build_register.py`): offices from Weyer first, then Sloane 3825, then 1904; legions corrected; will, malice, counterbalances; Rudd examiner per spirit.
- The apparatus (`GERE ANULUM/VIRGAM/HEXAGRAMMUM/PENTAGRAMMUM`, `OFFER`, `CONSTRINGE … PER NOMEN`, `APPELLA`, `VETA`) is in the grammar and in `spirits.md` §4.4, from Sloane 3825 ff. 112-113 and the spirit entries.
- **Andras as tester** (ruling 2026-11-10): office `PROBATIO`, only when commanded; pentagram required; attacks copies only (illusory, bat's-blood ink); only his failure report can commit. See `spirits.md` §4.5.
- Registers of divine names (`nomina.json`, 71 entries, Sloane 3825 ff. 112-113 + First Conjuration) and four kings (`reges.json`: goetia per Weyer ch. III, clavicula per 1446).
- **Binding hours: Weyer ch. III governs** (ruling 2026-11-10). Presidents can never be bound unless their king is invoked (`INVOCA REGEM`), and never at nightfall. **A spirit's king is decided by quarter** (ruling 2026-11-10): the goetia king of the quarter his witnesses give (Folger, Ad. 36674, Weyer, Sloane 3825), never a name from the Oriens/Paymon/Amaymon/Egyn set. Presidents now invocable: Gaap, Marbas (Amaymon); Foras, Malphas, Osé (Gorson); Zagan (Goap). Still blocked: Buer, Glasya-Labolas, Haagenti, Caim, Amy, Valac, Botis, Marax (no quarter found; Rudd's Harley 6483 and the rest of Wellcome 110 are unsearched).
- **Uninvited agents go to Hell** (ruling 2026-11-10; `authority.md` §6.2a): not examined, because invitation is a recorded fact; input sealed in the Vessel; barred for the run. Invited agents: Hell applies only to their proposals. **Proscription** (ruling 2026-11-11): intruders are banned at every level they sought to change, in a persistent Index the Constitution keeps outside all programs; only the launching person can lift an entry (the safeguard against identity framing). The Vessel is readable by the Operator, but never released or reused. **Ruling 2026-11-11:** an intruder is **damned** (*damnatus*), irreversibly, by its seal, on the recorded fact of intrusion alone; invited agents are never damned, only proscribed, which the wand can lift.
- **Guiding principle** (ruling 2026-11-11, `authority.md` §1.5): the type system makes it impossible to construct a program whose authority exceeds its grant, at any depth of delegation. Every rule must preserve it; it needs a machine-checked proof.
- Part 4 `dynamic.md`: draft 1. The whole step-3 specification is now drafted and awaits the owner's approval; after it, step 4 (instruction set) begins.

## Still open: ask the owner

- Seal artwork: the 1904 plates (with lettered double rings) or Sloane 3825's bare sigils. His fidelity rule points to Sloane 3825.
- How the 72 spirits' offices map to computational jobs. His Occultist Explorer project already gave all 72 real functions; it may be reusable.

## State of the code (v0.2.1, Milestone 1 complete)

- The repository is this folder (git). The first commit is the untouched v0.2 import; the second is Milestone 1.
- **Milestone 1 delivered:**
  - code split into modules with source positions;
  - all audited defects fixed;
  - 50 tests, all passing;
  - pinned Rust 1.97.0, `Cargo.lock`;
  - Linux build verified byte-identical across folders;
  - CI workflow written but **never run**;
  - Windows .exe **never built**.
- **Local build note:** the environment had no network and no Windows target. Use `RUSTUP_TOOLCHAIN=stable cargo … --offline`.
- **Next engineering step:** the formal spec (grammar plus authority model), then one end-to-end slice: one circle, one spirit with one office, invoke, propose, angel examines, commit or banish, running in the VM with tests. Recommend small verified steps, never one big pass.

## Proposed demon and history model

Discussed with the owner, not built:
- Spirits are untrusted workers: they do authority-sensitive effects inside the Triangle of Art, which is the sandbox.
- Angels examine. The 72 Shem HaMephorash angels are the natural counterpart examiners.
- No agent approves its own work.
- Ranks bound authority; legions bound the number of child tasks.
- **HISTORIA** is an immutable chronicle built into the runtime (Creation, War, Fall), read-only to programs.
  - Origin never changes, even for the fallen.
  - Allegiance and jurisdiction changed at the Fall.
  - Former orders come from the Goetia: Barbatos "of the Order of Virtues, of which some part he retaineth still"; Paimon and Beleth "is of"; Agares, Gaap and Raum "was of"; Camio, Murmur; Belial "created next after Lucifer… fell first"; Marchosias, Phenex and Focalor hope to return "to the Seventh Throne".
  - Each fact records who attests it: scripture, the narrator, or the spirit's own unverified claim.
- MILITIA, under Michael, owns banishment.
- Optional invention, to be labelled as such: a spirit is examined by the angelic order it fell from.

## Sources

**The owner holds these; ask him to re-attach them.**

| Source | What it gives | Status |
|---|---|---|
| 1904/1916 *Goetia* plates (Project Gutenberg 72679; 7 images) | figures 1–80 = the 72 seals plus 8 doubles; 81–152 Rudd's Hebrew table; 153–184 circle, triangle, hexagram, pentagram, Ring of Solomon (fig. 157), vessel, Secret Seal | received |
| Sloane 3825 microfilm PDF (`Janua-Magica-Reserata-Sloane-3825.pdf`, 194 frames) | frame 8: Trinity seals; frames 57–99: nine angelic Keys; frames ~109–119: the Goetia with bare seals; frames 120–153: other Lemegeton parts | studied for angels; Goetia frames **not yet** studied |
| Mathers, *The Key of Solomon the King* (132-page PDF) | the 44 planetary pentacles, for capabilities | **received at the end of this session, not yet examined** |
| Seals of the 72 Shem HaMephorash angels (Rudd: Harley 6482 / Sloane 3824) | examiner seals | not obtained |

## Seal tracing

- Pipeline: `tools/sigils/trace.py` (OpenCV contours to SVG, plus a comparison sheet).
- Grid detection: `grid.py` and `grid2.py`. Doubled-seal sheet: `pairs.py`.
- Bael sample traced at 97.5 % match with the plate (`tools/sigils/samples`).
- Known issues: the double outer ring merges where the two lines nearly touch, and lines come out slightly bold.
- No potrace or fontforge available. fontTools is untested; check whether it is installed.
- Plate grid coordinates are in `pairs.py`.
- Plan: trace all seals, build an "Annulus" font in which the spirit's name in source renders as its seal, and record provenance per glyph.

## Deferred tasks

- Research fuller transcriptions of every Sloane 3825 fragment (the owner asked for this "later").
- Read the remaining unread mottos and names listed in the angelic-hierarchy document.
- Add the Goetia former-order data to a sourced demonic-hierarchy document. Offered to the owner, not yet confirmed.
- Apply the Annulus rename. Write founding principles 2–7 into the spec. Update `ROADMAP.md` to the 18-step sequence of the definitive handoff, which adds: an instruction-set spec, authority analysis, audit records, four error classes, and a native backend.

## Update: Key of Solomon PDF examined

- The PDF is the revised Mathers edition with figures replaced from manuscripts.
- **All 44 planetary pentacles** are present: figures 11–54 on PDF pages 55–99 (Saturn 7, Jupiter 7, Mars 7, Sun 7, Venus 5, Mercury 5, Moon 6).
- **Source manuscript stated for 16:**
  - Saturn 3–7: M276 and Kings 288;
  - Jupiter 1–7: all M276;
  - Mars 1–4: Lansdowne 1202, Harley 3981, Sloane 3091 ×2.
- **Not stated for 28:** Saturn 1–2, Mars 5–7, and all of Sun, Venus, Mercury and Moon. These are probably Mathers' own redrawings.
- Some figures are manuscript photographs (grey, faint, e.g. Saturn 1); others are clean line drawings (e.g. Mars 1).
- Image sizes are 300–640 px. The geometry is traceable. The ring lettering (Hebrew and Latin) is roughly 10 px tall, so faithful lettering is the weak point.
- Extract images with `pdfimages -f N -l N -png`. Page-to-figure table: figure = page − 44, except page 55–56 (both figure 11).
- Open questions for the owner:
  - Is the stated-source subset enough?
  - Which Key of Solomon manuscript counts as "most authentic"? Not yet researched.

## Update: Olympic spirits (Arbatel, 1575)

- The owner recalled an old occult site listing the Olympic spirits' sigils. It is probably **graveworm.com/arbatel/arb06.html**, a hand-coded 1990s-style site with `.gif` characters for Aratron, Bethor, Phaleg, Och, Hagith, Ophiel and Phul. Its translation is unnamed and the fidelity of its sigils is unverified.
- Primary sources to use instead: the 1575 Basel first edition (a Wellcome Collection download exists: wellcomecollection.org/works/r4hhxh28) and Peterson's Latin/English edition (esotericarchives.com/solomon/arbatel.htm).
- The *Arbatel* (1575) predates the Lemegeton manuscripts.
- Where the Olympic spirits fit in Annulus is undecided.

## Update: old-web sources found (deep search)

- **Twilit Grotto / Esoteric Archives, original .GIFs** (esotericarchives.com/gifs/index.html), Joseph Peterson's 1990s collection:
  - a 72-seal Goetic table;
  - Goetia triangle, hexagram, pentagram, ring and seal of Solomon, brass vessel;
  - magic circles credited per manuscript (Add. 10862, Sloane 3847, M276, Kings 288, Lansdowne 1202, Sloane 3645);
  - Hebrew *Mafteah Shelomoh* pentacles (Or. 14759);
  - Lansdowne 1202/1203 seals;
  - the Olympic spirits (Arbatel) and the "juratus" planetary seals;
  - a 1620 magical calendar;
  - Bruno's planetary images (1591).
  - Per-image provenance is uneven: the Goetic table cites no manuscript.
- **fromoldbooks.org/Mathers-Goetia/**: scans of the 1904 Goetia, possibly higher resolution than the Gutenberg plates.
- **sacred-texts.com/grim/** (1999): Goetia, Arbatel, Key of Solomon texts.
- **tarrdaniel.com Thelemagick / Hermetic Magick library**: an old-style mirror site.
- **Shem HaMephorash seals:** no primary source online yet; 360headsoftheearthzone.com and a wizardforums thread point toward Rudd's manuscripts.
- **Graveworm Arbatel**: see above.
- Wayback Machine snapshots of GeoCities and Angelfire are not searchable with the available tools. The owner can browse them himself, or the built-in browser can be used.

## Update: the oldest Key of Solomon examined

- BnF Ital. 1524 (1446) is received; see `docs/sources/LOG.md`.
- It has about 9 "Pentacoli/Candarie di Salomone" on PDF p. 195 and the great circle on p. 196.
- It does **not** have the 44 planetary pentacles.
- Ask the owner which set Annulus uses.
- Research method: use the `occult-source-research` skill (proposed this session) and the log.

## Decision: layered source authority (the owner, 2026-11-10)

- **BnF Ital. 1524 (1446) is foundational for everything it covers.** Where a later source covers the same material and differs, the 1446 version supersedes it. Examples: the same pentacle drawn differently, or the same circle.
- **Later additions stand where they don't clash.** Examples: the 44 planetary pentacles beyond the 1446 set, the Goetia's 72 spirits (Sloane 3825), the angelic Keys (Sloane 3825).
- The same rule applies down the chain: within the later material, the oldest witness still wins (Sloane 3825 over the 1904 edition).
- **Open task:** match the ~9 pentacles of 1446 against the 44 to find which later pentacles are versions of a 1446 one (1446 wins) and which are new (they stand).

## Decision (the owner, 2026-11-10): older supersedes, applied to Paimon

- Paimon's late-16th-century character in Wellcome MS 110 (image 27, among the four kings) **supersedes** his Goetic seal from Sloane 3825.
- The general rule is confirmed: when an older witness gives a different sign for the same entity, the older sign wins.
- Check the other three kings (Oriens, Amaymon, Egin) and any other Goetic spirits that appear in MS 110 or Folger V.b.26 in the same way.

## Full source reading and rulings (the owner, 2026-11-10)

- All nine source PDFs were read in full.
- They are condensed into three skills, ordered by date: `key-of-solomon-tradition`, `solomonic-spirits-tradition` and `solomonic-angelic-tradition`.

**Rulings:**
- **The Paimon ruling is WITHDRAWN.** Wellcome MS 110 image 27 shows the Tetragrammaton letters of a ring, not king characters. No Goetic spirit has a seal in MS 110. Paimon keeps his Sloane 3825 seal. The general rule that the older sign supersedes still stands.
- **Bune: Sloane 3825's "the first is best" wins**, so use the first (left) seal. This replaces fig. 31 and the "least is best" note from Sloane 3648 and 2731, which is kept as a disagreement.
- **The four kings follow BnF Ital. 1524 (1446), the oldest witness:** Oriens East, Paimon West, Amaymon South, Egin North.

**Corrections to the notes above:**
- **Mathers PDF:** figure = page − 45 for pp. 56–99. A source is stated for 42 of the 44 pentacles, mostly in captions inside the images.
- **Sloane 3825:** dated 1641 or later. The 1904 "doubles" may be copies of seals the scribe struck out.
- **Arbatel 1575:** the edition *prints* Bethor XXXII and a total of CLXXXXVI (196); the printed numbers sum to 186. Its intended reading, by its own total and the series falling by 7, is **42**, total 196 (`docs/sources/LOG.md`; `provenance.md` rule 6). (Corrected 2026-11-11 after the second audit: this note used to say "total 186".)
- **Book of Oberon:** all its figures are modern redrawings.

## Decision (the owner, 2026-11-10): "oldest wins" applies only within one work

- **The rule works inside a single work's line of copies.** The oldest copy of that work wins (for example, the 1446 Clavicula over Mathers' Key, or the 1575 Arbatel over Rudd's copy of it).
- **Separate works do not override each other.** The *Liber iuratus*, the Clavicula, the *Liber officiorum spirituum* line, the Goetia, the Arbatel and the *Janua* each keep their own readings, attributed. Annulus can treat them as separate jurisdictions.

**Consequences:**
- **The four kings' directions** are per work:
  - Clavicula 1446, Oberon, the *Janua* and Rudd: Amaymon South, Egin North.
  - The Goetia: its own four kings (Amaymon East, Corson, Ziminiar, Goap).
  - Wellcome MS 110 disagrees with itself.
- **Sun/Mercury angels** are per work:
  - *Liber iuratus*: Raphael is the Sun, Michael is Mercury.
  - *Janua* Keys and Scale: Michael is the Sun, Raphael is Mercury.
- **The *Liber iuratus* ban on invoking the nine orders** binds only the *Liber iuratus*. The *Janua*'s nine Keys stand.
- **The "Paymone angelus Mercurii" question is moot.** It is a different work from the Goetia.
- **Open:** does the Goetia count as a later copy of the *Liber officiorum spirituum* (via Weyer), or as a separate work? This decides whether 16th-century offices and legion counts override the Goetia's.

## Ruling (the owner, 2026-11-10): the Goetia counts as part of the *Liber officiorum spirituum* line

- **The Goetia belongs to the same work** as the *Officium* tradition: Weyer, Oberon's Officium, Wellcome 110's Offitium, Sloane 3853 ff. 257-259.
- **So within that line, the older witness wins** on name forms, offices, legion counts and directions, where the identification is secure.
  - Example: Morax has 36 legions (Wellcome 110 and Weyer), against 30 in the Goetia.
  - Example: Naberius appears as a raven, not a crane.
  - Example: Beleth carries men from region to region (Wellcome 110); that ability stands.
- **The four kings in this line:** Oberon's Officium (c. 1577-83) gives Oriens E, Paymon W, Amaymon S, Egin N. **The Goetia's Amaymon-in-the-East is superseded.**
  - The Goetia's Corson, Ziminiar and Goap are recorded as a later variant.
- **Seals are unchanged.** No older witness of this line has seals: Oberon's figures are modern redrawings, and Weyer has none. So Sloane 3825's seals stay in force.
- **Amy (58):** the image received is the 1904-style seal, with the A-M-Y lettered double ring. The ring and lettering are 1904 additions. Use the inner sigil provisionally, labelled 1904, until Peterson's reading of Sloane 3825 (g58.jpg) is compared.

## Ruling (the owner, 2026-11-10): the three final destinations, APPROVED

| Destination | Mechanical meaning |
|---|---|
| PARADISE | examined, approved, committed: final and permanent |
| PURGATORY | held: not committed, not destroyed; awaits re-examination |
| HELL | banished: capabilities revoked, tasks terminated, results quarantined and sealed; irreversible within the program |

- **The states are real; the claim is symbolic.** The runtime never claims to determine actual divine judgement (JUDGEMENT law, *nulla potestas*).
- **Separation of roles:** angelic examination produces the verdict that routes work to a destination, and only the constitution's judgement step assigns the destination. No agent assigns one to its own work.

## Purpose (the owner, 2026-11-10)

"Authenticity to the source material is key. This is as much a simulation as it is a coding language."

- **The question is answered:** Annulus is both a general-purpose language and a faithful simulation of the Solomonic tradition.
- **Keeping a sourced detail:** it is kept only if it has a mechanical analogue (ruling of the same day).
- **Specification started:** `docs/spec/authority.md` and `docs/spec/provenance.md`.
- **Roadmap:** now the 18 steps (`docs/ROADMAP.md`).

## Next session: start here (2026-11-11, second session)

- **Read first:** `docs/ROADMAP.md`. The order is: finish the spec → minimal interpreter → demonstrate enforcement → AI-agent demo → evaluation → discipline of the Operator → VM and native backend.
- **Current step:** step 1, completing the specification.
- **Done this session:**
  - **Office signatures**, `docs/spec/offices.md` (Part 1c, draft 1): every office kind's arguments, proposal and check, and the containment argument for helper tasks (proof obligation 1). Its §7 lists eight points for the owner.
  - **Ruled by the owner:** `PRAETERITA` reads a **history** of the present run and the one before it, never the audit (`offices.md` §2.2a). The earlier run needs a `historia "prior" legere` grant, still to be approved.
  - **The GPT audit**, folded in: `docs/AUDIT_SPEC_1.md`. Four findings fixed, Rudd's kings left open.
- **Ruled by the owner this session** (all written in): `EXAMINA` is examination then judgement, returning only the destination; Rudd's kings wait for Harley 6483; `creare`/`delere` permissions; constant places; `DIGNITATES` petitions only; familiars freed from another program only under the launcher's oversight; `historia "prior" legere`; `RESPONSUM p` (added to the grammar, the checker and three tests); Earls' condition for the whole stay; Dukes need real clear weather (`--caelum serenum`) and no active storm; the grammar example now "raise a tempest". The owner's guide for readings: **the most authentic option that does not compromise the Solomonic principle**.
- **Two-layer split done:** `docs/spec/core.md` (Part 0). Nine core elements; the rite can only refuse, hold, spend or bound, never grant (§3); the core re-checks every effect at commit. **Build the interpreter core first, with the rite reaching it only through §3.1.**
- **Pending from the owner:** (Andras ruled: no pentagram required, unprotected he may end the run, `dynamic.md` §5.5; angels ruled: `authority.md` §12.1a, reserved grants `… angelis;`, used only at the Operator's petition, since only the Operator prays); then approval of the whole spec, after a second audit.
- **Amendment procedure drafted:** `authority.md` §2.5 (version and fingerprint; only the owner approves; AI drafts are petitions; core, rite and editorial changes; for approval).
- **Simulated spirits drafted:** `dynamic.md` §5.6 (`--voluntas simulata`; scripted answers via `--responsa`, placeholders otherwise; `--sors` forces an outcome for hostile tests; the script is never readable by the program).
- **Items 4–7 drafted from the owner's rulings** (asked as design questions pairing Solomonic principles with modern or forgotten techniques):
  - `flow.md`: spirits' words quarantined, CaMeL-style; consecration by gravity; outside data tainted for ink but may steer;
  - `concurrency.md`: each Triangle an actor (E vats), only the Operator sends, deterministic simulation;
  - `library.md`: no ambient authority, a powerbox of slots chosen at launch, proof-carrying answers;
  - `proscription.md`: identity by seal then by scope of actions (a match on scope only means suspected and held), an append-only hash-chained log (`.proscriptio`).
- **Libraries to build** (approved): `library.md` §4, in order: hours, registers, calendar, proof checker, names, squares and seals, audit tools, the wand, agent bridge.
- **The wand** (`operator.md` §8, ruling): a plain USB stick (the owner's choice) with a master secret and a rite secret, each locked by a passphrase; it signs the faculty, the second key, Constitution releases, and is needed for `GERE VIRGAM`. The master secret is never unlocked while a spirit is invoked; a spare wand can revoke a lost one. Hours and registers come first, since the interpreter needs them for any invocation.
- **The owner's working style for design questions:** ask by prompt (multiple choice), be creative, keep to the Solomonic principles, and draw on current technology or older ideas that never caught on.
- **Step 1's writing is complete.** Item 8 drafted as proof obligations P1–P9 (named T1–T9 until the third audit) (`core.md` §8). `AUDIT_BRIEF.md` is rewritten for the **second audit**.
- **Second audit received and folded in:** `docs/AUDIT_SPEC_2.md`. All five findings fixed (Bethor was a stale handoff note). New: `core.md` §9, the formal model; consecration in the core interface; `offices.md` §2.3a, the merge rule.
- **The owner's review before the third audit** (`docs/AUDIT_SPEC_3.md`): the second audit's corrections are accepted provisionally, and four matters are fixed: the complete state, the will in the rite order, validators as authenticated Constitution entries, and proposals versus agents (no agent is irreversibly condemned). **No security theorem is yet established; proof is step 3.**
- **Third audit received** (ten findings: seven high, three medium; `docs/AUDIT_SPEC_4.md`). Claude answered each first, without rewriting; the owner ruled on the open points (damnation for intruders only, by recorded fact, by seal; P6 split; one-use grave grants; validator work limits in their Constitution entries); then all ten were fixed in one pass. The theorems are renamed **P1–P9**, because `static.md` already used T1–T9 for type errors.
- **Fourth audit received and fixed** (`docs/AUDIT_SPEC_5.md`): ten findings, no ruling needed, no route to escalate authority found. Proof certificates now consecrate only through `consecrate`; proposal lifecycle; Hold(*i*) and P4; P9 from equal initial states; wand and authentication in the state; atomic transitions; validator fuel; P6(b) traces; the `intrude` step (damnation on the **attempt**).
- **Next: the core's formal model in Lean 4** (the owner's choice, 2026-11-11), the last part of step 1. It replaces a fifth prose audit.
  - **What:** a Lean 4 model of `core.md` §9: the state (§9.1), the transitions and their enabling conditions (§9.2), marks (§9.3), validators and consecration (§9.3a), examiners (§9.3b), grave grants (§9.3c), and P1–P9 *stated* as Lean propositions (§8, §9.4–9.6). **Definitions and statements only: no proofs, which are step 3.** Put it in `formal/` (for example `formal/Annulus/Core.lean`, with a `lakefile` and `lean-toolchain`).
  - **Checking it:** Lean is not installed in the cloud workspace, and package installs are blocked there. Options: (1) run the session from the Claude desktop app linked to the owner's Windows PC, and use its shell if the link offers one; (2) the owner installs VS Code with the "Lean 4" extension, which installs Lean, opens the file and reports what is underlined; (3) the owner allows package installs in his network settings and starts a new chat.
  - **Then:** audit the Lean model against the spec (an auditor who reads Lean), fold in the findings, and the owner's approval ends step 1. **No theorem is proved yet; that is step 3.**
- **Standing instruction from the owner (2026-11-11):** say so plainly, and early, when the project reaches a point where it would pay to bring the core up to standard before building more on it, so it never ends in spaghetti. Watch for: the same rule written in two places, the two parsers (`tools/grammar_check.py` and the Rust one) drifting apart, and new features built on unapproved spec. **Known now:** the Rust prototype implements none of the spec. It should be replaced (step 2) once the spec is approved, never extended.
- **Data scripts:** the registers' JSON is the source of truth. Edit the JSON, then run `python3 tools/build_register.py`; CI checks that the generated `.md` files match.
- **Tests:** `RUSTUP_TOOLCHAIN=stable cargo test --offline` (prototype, 50 tests); `python3 tools/grammar_check.py tests/grammar` (69 programs).

## Next session: start here (2026-11-11, third session)

- **Done:** the core's formal model in Lean 4, `formal/` (draft 1): the state, 27 transition rules, marks, validators, examiners, grave grants, the Index, and P1–P9 plus the damnation law **stated** as Lean propositions. No proofs. Plain Lean core, no Mathlib, toolchain `v4.19.0`.
- **Type-checks.** The project is now on GitHub, private: `KeychainofSolomon/Annulus`. The `lean` workflow builds `formal/` on GitHub's servers on every push; read its result with `env -u GH_TOKEN gh run list -R KeychainofSolomon/Annulus`. Lean reserves the word `seal`: the invocation's field is `spiritSeal`.
- **Authorship:** commits are authored as `Solomon <KeychainofSolomon@users.noreply.github.com>` (history rewritten 2026-11-11 so that no personal name or address is in it). Keep it so.
- **Formalising found 15 problems in the spec** (`formal/README.md`, last table). Two need the owner's ruling:
  - **Finding 1, high:** P5 as stated in §9.4 is false. Committed effects are proposal content and are in the observation; verdicts depend on content but are assumed equal. Proposed: count examination as declassification (verdicts and committed effect lists go into D), or narrow the observation to the program's own acts.
  - **Finding 4, high:** `intrude`'s condition also catches an *invited* agent that attempts delivery into another Circle, or after its release, contradicting "invited agents are never damned". Ruling needed: is that intrusion?
  - The rest (two-key commits have no store, commit must follow judge, angels' work has no life, run endings, the wand during `GERE VIRGAM`, suspicion in the state, P9's preconditions, …) are spec fixes with a suggested fix each.
- **Then:** fold the fixes into `core.md`, update the Lean to match, audit the Lean against the spec (`AUDIT_BRIEF.md` is rewritten for that), and the owner's approval ends step 1.
- **Watch (standing instruction):** the Lean model and `core.md` §9 now state the same rules in two places. Treat `core.md` as normative and change both together; say so if they drift.
- **Ruled by the owner (2026-11-11, later):** P5, **judgement releases** (verdicts, destinations and commits go into D); intrusion, **an invited agent outside its open invocation in that Circle is an intruder, damned**. Both written into `core.md` §9 and `authority.md` §6.2a, §11, with the other formalisation fixes; the Lean model matches and CI passes. Still open: the store for second-key confirmations, and where suspicion is recorded during a run (`formal/README.md`, findings 3 and 10).
- **Next:** the Lean audit (`AUDIT_BRIEF.md`), then findings 3 and 10, then the owner's approval of step 1.
- **Ruled by the owner (2026-11-11, 15:15), now written into `core.md` §9 and the Lean:** findings 3 and 10.
  - **Two keys: sealed petition.** At the end of the run, approved work awaiting the second key is sealed as a petition: the fingerprint of its exact effects. The owner confirms it with the wand outside the session; a later run may commit that exact work once (a one-use grant, stored like grave grants).
  - **Suspicion: appended to the Index.** A second step that may change the Index during a run, `suspect`, appends *suspected* for the seal; it persists across runs until the owner clears it outside a session.
- **Lean audit 1 done (2026-11-11):** an independent Claude agent; 14 findings, all confirmed, all settled (`formal/README.md`, "The Lean audit"). The owner ruled: **damnation casts out everywhere** (every open invocation of that seal relegated) and **held work is abandoned when its spirit is released**. P3 was false as stated (a failed commit counted its targets); fixed. Denials now end the run, as `dynamic.md` §3 says. CI passes.
- **Next:** a second Lean audit by a different AI (the owner pastes `AUDIT_BRIEF.md`, `docs/spec/core.md` and `formal/` into ChatGPT or Gemini), then the owner's approval ends step 1.
- **Lean audit 2 done (2026-11-11):** ChatGPT, six findings; five accepted (Circle-bound values in P3, invocation targets in Hold, real signature checks at launch, P5 fixes only the will, P9 clarified), one declined (rite pairings are rite, not core). CI passes. **Step 1 awaits only the owner's approval.** Then step 2: the minimal interpreter, a fresh core module built from the spec, replacing the Rust prototype.
- **Ruled by the owner (2026-11-11, 15:45): outcomes and destinations.** Heaven, Purgatory and Hell are for beings only. Judged work is `RATUM` / `SUSPENSUM` / `IRRITUM` (same mechanics; keywords changed in the grammar, `tools/grammar_check.py` and the tests). A being's destination is its standing: unknown = Paradise, suspected/proscribed = Purgatory, damnatus = Hell; programs other than the launched one have one by seal. **Angels are in Paradise by nature**, outside the Index. Claude assumed the names and the standing map (the owner skipped those two questions); confirm. Open: whether void work's record stays sealed in the Vessel (as now) or is deleted.
- **Ruled by the owner (2026-11-11, 15:50): the Vessel** persists per installation, hash-chained, root checked at launch; void work's record stays in it. It begins with three records: Solomon's sealing, then (afterwards, no year claimed) the Babylonians' breaking, Belial into the image, then the making of this vessel, signed with the wand. **Ruled: brass**, cast into a deep lake or hole (Sloane 3825 ff. 110v–111r), overriding the older-witness rule for this item; Weyer's glass vessel in a well is recorded as the differing witness in `authority.md` §6.2a. Records only: no spirit or capability in them; the spirits stay invocable. Confirmed by the owner (16:03): the names `RATUM`/`SUSPENSUM`/`IRRITUM`, what `SUSPENSUM` does, and the standing map.
- **Ruled by the owner (2026-11-11, 16:05):** **only a proven seal is damned**; a claimed seal is recorded and never damns its owner (`core.md` §9.2, Lean `intrude a claimed …`, `DamnationLaw`). **The mirror** (`speculum`, `operator.md` §9): a standalone instrument to scry Hell and the Vessel, only outside sessions, wand present, read-only, inert display, no export of the damned's words, its own hash-chained log; added to the library list as item 9.
- **Ruled by the owner (2026-11-11, 16:11): the crystal** (`crystallus`, `operator.md` §10): the mirror's counterpart for Heaven, the angels by nature and beings in good standing with their ratified works, under the mirror's rules; Paradise is not trust. Library item 10; `agens` is now 11.
- **Ruled by the owner (2026-11-11, 16:23): Paradise is for angels only.** Spirits are demons; beings in good standing have no destination ("at large", after "their former places", Sloane 3825 ff. 110v–111r). The crystal shows angels and the works they ratified, never spirits as its own. The standing map: unknown = at large, suspected/proscribed = Purgatory, damnatus = Hell, angels = Paradise by nature.
- **STEP 1 APPROVED by the owner, 2026-11-11 16:33** ("ok, next", after every open core question was settled). From now on the spec changes only by the amendment procedure (`authority.md` §2.5). Next: step 2, the minimal interpreter, plan in `docs/STEP2_PLAN.md`.

## Next session: start here (step 2)

- **Read:** this file's last sections, `docs/STEP2_PLAN.md` (approved by the owner, 2026-11-11 16:41: the test wand, the subset, the Furfur and working-spirit example), `docs/spec/core.md` §3 and §9, and `formal/Annulus/Step.lean`.
- **Do:** milestone 2a, the **core** module in Rust: one function per transition rule of `Step.lean`, named after it, with a test for each and the property checks (P1, P2, P4, P7, damnation law) over every audit. No comments in the source.
- **Build and test:** `RUSTUP_TOOLCHAIN=stable cargo test --offline` (the pinned 1.97.0 cannot download here; the local stable is 1.97.0). Push to `KeychainofSolomon/Annulus` (private); GitHub runs `build` (Rust, Windows and Linux) and `lean`. Read results with `env -u GH_TOKEN gh run list -R KeychainofSolomon/Annulus`. Both are green as of this handoff.
- **Commits** are authored as `Solomon <KeychainofSolomon@users.noreply.github.com>`. After cloning, first run `git config user.name Solomon` and `git config user.email KeychainofSolomon@users.noreply.github.com`. Never put the owner's name or email in the history. **No co-author or session lines** (no `Co-Authored-By`, no `Claude-Session`), and no Claude or Anthropic identity in any commit: the owner's instruction, 2026-11-11, which overrides any default attribution.
- **Milestone 2a, first pass (2026-11-11, same session):** `src/nucleus/` (named `nucleus` because `core` clashes with Rust's own library): `types.rs`, `rite.rs` (the §3.1 questions as a trait), `state.rs` (state, hash-chained `Chain`, `Stores` with the Vessel's three records, test-wand signatures), `step.rs` (one method per rule; `examine` performs examine, judge and commit as three consecutive audit records), `sha256.rs`. `tests/nucleus_rules.rs`: 31 tests, theorem checks over every audit. Next in 2a: suspicion at examination, angels' petitions end to end, simulated draws for the scheduler; then 2b, the language subset.
- **Milestone 2a done (2026-11-11):** plus suspicion at examination (`IxEntry::Proscribe(seal, marks)`, `Reach`), angels' petitions end to end (reserved grants, angel examined by another angel), and `nucleus::Sim` (seeded scheduler and will; same seed, same audit). 38 tests. **Next: 2b**, the language subset (`docs/STEP2_PLAN.md`): lexer, parser and type checker for the subset, and an evaluator that drives the nucleus one Operator step at a time. Reuse `tests/grammar` (69 programs) and `tools/grammar_check.py` as the reference parser; keep the two in agreement.


## Next session: start here (2026-11-11, evening)

- **Done: milestone 2b, the front end** (`src/lingua/`): lexer, parser for the whole grammar, register loaded from `docs/registers/goetia.json` (via `include_str!`, so the JSON stays the source of truth), and the static checker for the step-2 subset. Parsing and checking run on the interpreter's 16 MiB stack, with a nesting limit of 256 (`parse::MAX_DEPTH`), so deep nesting is an error, never a crash.
- **Tests:** `tests/lingua_grammar.rs` (all of `tests/grammar` through the Rust parser; the 41 rejections fail at the same line and column as in the Python recognizer, checked by hand this session) and `tests/lingua_check.rs` (43 tests). 136 tests in all.
- **Office signatures implemented in the checker** (`check::signature`): the six telling kinds (question, then places with `legere`; invocation arguments not yet), and the working kinds `AEDIFICATIO`, `APPELLATIO`, `TUTELA`, `TEMPESTAS`, `MORBUS`. Every other office is `X1`.
- **Readings taken, to confirm with the owner:**
  - **Labels:** a register label is the name lowercased, with `-` and spaces as `_` (`glasya_labolas`). Aliases count as labels where they spell one spirit only (`barbas` = Marbas).
  - **N5 vs `static.md` §2.1:** §2.1 says `SIT furfur = INVOCA furfur …` is lawful (two namespaces), and the grammar example does exactly that, yet N5 says declaring a register label is an error. The checker follows §2.1, so N5 never fires. Needs an amendment either way.
  - **A13 always refuses** in step 2 (pure Presidents), because the kings are outside the subset. **A11 is now checked** (the apparatus, 2026-11-11): Beleth comes with ring and wand taken up on every path; Berith and Astaroth are dukes, still refused while the sky is undeclared.
  - **Copies of rite values** (`SIT b = a;` for ink or an invocation) share the original: spending or releasing through either counts for both.
  - **A rite value in a `MUTABILIS` variable** is `X1` for now.
- **Drift watch (standing instruction):** there are now two parsers, Python and Rust, plus the old prototype's. They agree on all 69 programs today, but nothing in CI compares their rejection positions. When 2c adds the command line, add a CI step that runs both and diffs. The prototype (`src/lexer.rs`, `parser.rs`, `compiler.rs`, `vm.rs`, …) still uses the English keywords; it is removed in 2d.
- **Next: 2c**, the evaluator (driving `nucleus` one Operator step at a time, as `Lang.next` does), the `goetia` rite as data, the test wand and the command line, so `annulus` runs the Furfur example.
- **Ruled by the owner (2026-11-11, 20:00):** the label reading stands (lowercase, `-` as `_`, unique aliases count), and Beleth, Berith, Astaroth and the pure Presidents stay refused (A11, A13) until the apparatus and the kings are built. **N5 ruled (20:26): a spirit's name belongs to the spirit.** No program name may be spelled as any register label; `static.md` §2.1 amended, examples renamed to `vocatus`, recorded in `CHANGELOG.md`, checked (test `n5_…`).

## 2c, first half (2026-11-11, night)

- **Runs end to end, in memory:** `lingua::run_source` parses, checks, launches the nucleus with the test wand and runs. `src/ritus.rs` is the `goetia` rite; `src/lingua/exsequi.rs` the evaluator. Tests: `tests/lingua_run.rs` (14). 151 tests in all, debug and release.
- **The will (simulated):** OBSEQUIUM, MORA (one idle turn, then the work), PARS always open; MENDACIUM for `MENDAX`, and for `COACTUS` unless his condition is the triangle (so Furfur, Shax, Haures cannot lie, `dynamic.md` §5.3); MALITIA unless `FIDUS` or no malice recorded. `--sors` forces an outcome only where it is open. Renamed the nucleus's `Obedientia` to `Obsequium`, the spec's word.
- **Examination in `goetia`:** by `constitutio:angelicus`, the Constitution's angelic examiner (`authority.md` §10.2); Rudd's angel-per-spirit pairing is for `rudd`. The simulated verdict reads the drawn outcomes that examination can see: MALITIA and a false account of work → REJECT, PARS → HOLD; a well-formed lie in a telling office passes, as §6.3 says it must.
- **Runtime conditions enforced:** the Earl's stay (no other invocation open, either way), Dukes refused while the sky is undeclared, Presidents refused (no kings yet). Ink budgets per class; running out is a class 2 denial.
- **A consecration the validator refuses is a class 2 denial** [reading, to confirm]. The checker already let the value steer, so it cannot be left quarantined.
- **Gaps, to tell the owner:**
  - **RECUSATIO is closed** for now: the escalation (`spirits.md` §5) is not built.
  - **Hours and the Moon are not checked** (out of the subset by the approved plan).
  - **Ink acts are not in the audit:** the core has no label for a draw, a consecration of ink or an inscription. Adding one is a **core** change (`authority.md` §2.5: proof, attack tests, audit).
  - **`TEMPESTAS`** commits an unchanged write; the storm itself (fault injection) waits for the file library.
  - **`DIMITTE`** is recorded as an ordinary release; the core has no separate label.
- **Next:** the command line (`annulus` running a `.annulus` file with `--virga-probationis`, `--semen`, `--sors`, `--responsa`), the stores and audit written to disk and checked at launch, granted files read from and written to disk with their marks. Then 2d.

## Outside review and the owner's rulings (2026-11-11, late evening)

- **An outside review** (another Claude, from the handover report) found: a real hole in mark inference (fixed: loops checked to a fixpoint, test `a14_a_chain_of_assignments_cannot_launder_quarantine`); the report's "nothing widens authority" was false (hours unchecked admit more; `TEMPESTAS` made a false record, now set aside; ink budgets live outside the core); the simulated examiner reads the drawn outcome, which no real examiner sees; the trusted base includes the checker and evaluator; an abnormal end does not record each release (`dynamic.md` §3.1).
- **Ruled by the owner ("go intuitive"):** entries declined by case (`SPIRITUM`, `ANGELI`); the axiom *a se ipsa*; the interim meaning of proof (`authority.md` §2.5); copyrighted files removed from the history (the 2020 Shem-angel PDF and the website's day-angel images) with the pdfcoffee mention. All in `CHANGELOG.md`.
- **Next, in order:**
  1. **The examiner sees only what an angel could:** the proposal, its effects and the office. The drawn outcome becomes a separate test score of what examination caught and missed. Needed before step 3 measures enforcement.
  2. **Record each release at an abnormal end** (a core change: Lean model, tests, audit, per the interim rule).
  3. **Ink in the audit** (core change, same rule); then hours.
  4. **Attack tests with a rite that says yes to everything** (one per question of `core.md` §3.1), a mutation fuzzer over the grammar programs, and a check that Lean rule names match the core's functions.
  5. **Research:** is Sloane 3825 the oldest Goetia witness or only the best? Is the *Livre des Esperitz* (15th c.) an older witness of the *Officium* line than Weyer? Current rules stand until evidence.
  6. Then the second half of 2c (command line, stores on disk).
- **The repository is to become public.** Keep it free of the owner's identity (privacy rule above) and of copyrighted modern material: cite such sources, never commit them.

## Second outside review and the core changes (2026-11-11, night)

- **Done at the owner's request:** release records on every run's end; five effect verbs and the offices `VECTIO`, `CONCORDIA`, `DISCORDIA`, `EVERSIO`, `FURTUM` (`"aufer"`); a create can no longer overwrite (the reviewer's finding 1, the only route found by which a spirit exceeded its grant); mixed ranks keep the Earl's and Duke's conditions; ink totals saturate. 165 tests. Lean updated (labels carry `ds`, `Dismissed`, `Recheck` with `creates`).
- **Still owed under the interim proof rule:** an independent audit of these core changes.
- **Open from the second review:**
  - Finding 3, **needs a ruling:** ink is counted twice, differently (per class at each draw in the evaluator; one class-blind unit per invocation in the core). Does one inscribed seal serve one invocation or many?
  - Finding 4: a helper's narrower handed set is checked at spawn but not on its effects; matters for freed familiars (`offices.md` §4.6). Spec gap.
  - Finding 5: `forma` is only a format check; consecrated words can then be passed to another spirit. Needs real validators before the agent demo.
  - Finding 6: the rite sets the ink cost; a rite returning 0 makes invocations free. Add to the always-yes attack tests.
  - Note: Furfur's inability to lie hangs on the word "triangle" in a prose field of the register; give the register an explicit field.
- **Not yet in the subset:** `TRANSMUTATIO` (needs the conversion library), `MORBUS`, `APPELLATIO`, `TUTELA`, `INVISIBILITAS`, `TEMPESTAS` (their effects need state the core does not have yet: a title, protection, a veil, a storm).

## The findings file of the second review (2026-11-11, 22:10)

- Reviewed at `e962db6`; F1, F7 and the layering note were already fixed in `708a533`. The Lean check passed on `708a533`, so the Lean side of tonight's core changes is verified.
- **Done now:** F1's other half (a write needs an existing place), F4 (helpers held to their own set, R2(a)), F2 by the written rule (R3(a)). 168 tests.
- **R1 ruled by the owner: (a).** A seal serves its whole Circle; ink is paid once at inscription, by class, in the core (`inscribe` step, Rust and Lean). F3 and F6 are closed by it. 173 tests. F5 (validators) before step 4. The ink class of a two-rank seal is still an open item.
- The repository is now **public**.

## The honest examiner (2026-11-11, 22:40)

- Done: the angel judges only the proposal, the request's form and the world (`ritus.rs` `verdict`, `officia::expected`); the will is a test scorecard (`examination_scorecard`). This was next-step 1 of the outside review. 174 tests.
- **Attack tests done** (`tests/nucleus_attacks.rs`, 12, a rite that says yes to everything): all held; they found that the core did not require an angelic examiner, now fixed in Rust and Lean. 186 tests.
- **Next, in order:** spirit-specific malice (each spirit's recorded harm, not a stock one); the escalation, so RECUSATIO can open; real validators (F5) before step 4; then the second half of 2c (command line, stores on disk). An independent audit of tonight's core changes is still owed.

## Angel audit (2026-11-11, 23:00)

- Fixed: the angel list is now under the owner's signature; the core refuses to invoke an angel; Lean's angel petition and barring now match Rust. 188 tests.
- **Questions for the owner (not holes):** (1) may an angel's will ever lie or do harm, if a rite opens it? The core does not close it; examination still catches harm in work, but a lie in an angel's answer would pass. (2) Angels may be petitioned outside a Circle (prayer needs no Circle); confirm. (3) An angel's answer is quarantined like a spirit's until consecrated; confirm or exempt.

## What is not granted is denied (2026-11-11, 23:15)

- **The owner's ruling, now `authority.md` §1.6:** every rule is a closed list; a gap is a breach. Weigh every test by what it fails to exclude. Every new core rule needs an exclusion test in `tests/nucleus_exclusion.rs` (the technicality filter) before it is finished.
- Applied: angels' will closed to obedience, delay, part (this settles the first angel question); deletion of nothing excluded (caught by the filter). The other two angel questions stand as before: angels may be petitioned outside a Circle (prayer), and their answers stay quarantined (nothing is trusted unless consecrated). 194 tests.
- **Keep in mind always (the owner):** the Solomonic principles decide every design question.

## No presumption, and deception (2026-11-11, 23:20)

- `authority.md` §1.7: ten presumptions denied to every being. Silence in the sources is not honesty (TACITUS may lie). Only closed-list validators consecrate (`ita_non`); `forma` is gone. 195 tests.
- **Before real agents (step 4):** make the core's `State` fields private behind read-only views (fourth outside review, D4). In simulated mode `ritus::launch` signs the roots of the stores it is given; a real launch must take them from the last run's signed record, never recompute them.
- **Deception threats still open:** lies in answers pass examination until constraints exist (the apparatus); test-wand signatures are forgeable (real wand); seals are claimed, not proven, until the step-4 handshake.
- **Step 4 design note (the owner's friend):** a trust handshake before any agent's work: seal, name-challenge (`APPELLA`, "Art thou …?"), oath by the Divine Names, and a written proof record the **core** verifies. Never run a script an agent supplies.

## 2026-11-12: step 2c in progress, and the agent bridge (`agens`) planned

Branch `step-2c`. Owner's decisions this session:
- **The Goetic SLM is Annulus's real-agent mode** (`agens`, library item 11, roadmap step 4): a local model (Qwen3 8B class, low-refusal "abliterated" variant, run with llama.cpp Vulkan on the owner's AMD Windows PC, 16GB RAM total, kept within it) as the will of the spirits. The LLM is subject to the Constitution from its foundation. No model alignment layer: authenticity comes from the binding, not from the model's own filters.
- **Every act is an Annulus rite.** The model's petitions are JSON for now, shaped exactly like `PETE` (spirit, `modus`, `quaestio`, `loci`); each becomes a small rite checked by `lingua::check` and run by `exsequi` with the model supplying only the spirit's turn (`--voluntas agens`). Later the model writes Annulus directly.
- **Functions wanted:** web reading and search, Gmail drafts (sent only by the owner: a sealed petition, the second key, e.g. a prefilled Gmail compose window), PDF/image OCR (Tesseract, runtime-side as `TRANSMUTATIO`), a Latin/English lexicon (`LINGUAE`, e.g. Whitaker's Words), file work in a Circle folder, training on reading material as an index with passage-citation certificates (`library.md` §3).
- **Planned ground-floor features:** quarantine of web/PDF text (it can never choose a place, URL or recipient); the agent's seal is `goetia:N:label@agens:<hash of model file + config>` so the Index tracks each model (Goetia::spirit must strip the suffix); legions cap helpers, ink caps tokens; Andras `PROBATIO` as the red team for roadmap step 6.
- **Open rulings:** the network grant, (a) one `rete` grant per host or (b) one read-only grant for the whole web chosen at launch, everything fetched quarantined; the hours (`horae`) deferred.

Done:
1. **D4:** every `State` field is crate-private; read-only accessors; test-only mutators (`auth_mut`, `faculty_mut`, `world_mut`, `stores_mut`) under `probatio`. All 216 tests pass.
2. `ritus::launch` takes the roots from the last signed record (`Optiones.radices`), recomputing only for a fresh installation.
3. `src/tabularium.rs` (compiles, **untested**): the Index (`annulus.proscriptio`), Vessel (`annulus.vas`), sealed petitions (`annulus.claves`) on disk, one hash-chained line per entry, re-verified line by line on load; `annulus.radices`, the signed record of the roots after each run, checked at launch (fail closed); `annulus.petitiones`, a readable list of what each pending petition would do; the audit written once per run (`<program>.acta`, then `.2.acta` …, never overwritten, first line marks the test wand); real files: `read_world` / `write_world` for granted places only, plain relative paths, no links, staged writes.

Next (rest of 2c):
- Command line, keeping the prototype until 2d: `annulus ritus <program> [--virga-probationis] [--semen N] [--sors X] [--responsa FILE] [--acta FILE] [--domus DIR] [--mundus DIR]`: check, open the stores, read the world, run, print output, write the audit, seal the stores, note petitions, write the world, exit with the run's class. `annulus confirma <fingerprint>` (second key, outside a session) and `annulus petitiones`.
- Tests: round trip of every entry kind; a tampered line, a missing or forged roots record and another wand all refuse to launch; a store keeps an Index entry across two runs; a move waits for the key, `confirma` then lets it commit in a later run; places with `..`, absolute paths or links refused.
- Then `agens`: the will trait in `exsequi`, the llama.cpp client (std only, HTTP on localhost), the GBNF grammar for petitions, spirit personas from `goetia.json`, the trust handshake (seal, `APPELLA`, oath) from the step-4 note above.

Commits: author `Solomon <KeychainofSolomon@users.noreply.github.com>`, no co-author lines (the owner's rule).

**Standing rule (the owner, 2026-11-12):** update `docs/STATUS.md` in the same commit as every fix, in Annulus and in Spiri72's README where it applies.

## 2026-11-12, later: the rest of 2c done

- **Done:** `annulus ritus`, `confirma`, `petitiones` (`src/mandata.rs`; `main.rs` only dispatches). `src/tabularium.rs` is now tested: `tests/tabularium.rs` (20: round trip of every entry kind with tabs, newlines and backslashes; tampered, removed, appended and truncated lines; a missing or forged roots record; another wand; no wand) and `tests/ritus_cli.rs` (16, through the real binary: Furfur end to end, two runs keep an Index entry, a move waits, `confirma`, then commits; a spent key does not serve a second move; unlawful and linked places; bad flags). 252 tests, debug and release, clippy and fmt clean.
- **Holes found in the first half and closed:** (1) a file a spirit wrote was read back next run as `outside`, which may steer, so quarantine was laundered between runs; now `annulus.signa`, its hash in the signed roots record (the record line now has six fields). (2) Links were checked only on the last part of a path. (3) The staged write `a.txt` → `a.annulus-novum` could overwrite another file; now `a.txt.annulus-novum`, created fresh. (4) Nothing stopped a write over a file changed during the run, the audit included; now refused. (5) `petitiones` hid a fingerprint forever once it had been confirmed once.
- **Readings taken, to confirm with the owner:** marks are never removed, even when the program deletes the file or the owner rewrites it by hand (there is no way yet to clear one; that would be a consecration of a file). Store and launch refusals exit 2, the same code as a usage error and as class 2.
- **Order at the end of a run:** audit, then seal (which records a key's spending), then petitions, then the files. A failure after sealing leaves the files unwritten and the key spent: the safe direction.
- **Still owed:** the drift-watch CI step (diff the Python and Rust parsers' rejection positions over `tests/grammar`), now that the command line exists. An independent audit of the core changes of 2026-11-11.
- **Next:** `agens`, as planned above.

## Ruling (the owner, 2026-11-12, 14:06): the network grant

The owner: "whichever is most authentic and doesn't compromise security; spirits should be useful." Taken as **(a), one grant per host**, with two additions for usefulness:
- **Per host, read-only:** `rete "<host>" legere;` in `FACULTAS`. Within a granted host the spirit may fetch any page. Authentic: a spirit is charged with a named task in named bounds, and what is not granted is denied (`authority.md` §1.6). (b) is refused because a read-only fetch still leaks: the address of a GET request can carry the Circle's data to any host on the web.
- **One search grant:** `rete "<search host>" quaerere;` lets a spirit search; results are quarantined text.
- **Reaching beyond the grant is a petition:** a spirit that wants a host it was not granted (a link from a search result, say) makes a sealed petition naming the host; the owner turns the second key (`confirma`) and the next run may fetch it. Quarantined text never chooses a host or an address by itself.
- **Kept, under the same rule:** quarantine marks are never removed until the operator can consecrate a file through a closed-list validator; refusals keep exit code 2.

## 2026-11-12, afternoon: the will slot, and Spiri72

- **The owner's decision:** the model side lives in its own repository, `KeychainofSolomon/Spiri72`, which uses Annulus as a library. Annulus keeps everything that judges and permits; Spiri72 can only propose.
- **Done in Annulus:** `exsequi::Voluntas` (`nomen`, `age(&Petitio) -> (Outcome, Actum)`), `run_with`, `lingua::run_source_with`, `mandata::ritus_with`. The will is shown the spirit, office, question, mode, handed places, the contents of handed places the faculty lets it read, and the outcomes open to the spirit now (of `DECLARABLE`: obedience, part, delay). A declared outcome that is not open, a second delay or a failing model is a class 2 denial. Working offices still pass only with exactly the office's expected work (the honest examiner), so a model has freedom only in telling offices. `registra::GOETIA` is public for personas. Tests: `tests/lingua_will.rs` (7). 259 tests.
- **Interim, a presumption avoided:** the core records each turn's outcome, but a model's real outcome (a lie, malice) is not knowable. Until the core can record an unknown outcome, a model's will runs only with the test wand, and its audit says each outcome is the model's own declaration. **Owed: a core change, `Outcome` unknown for a real will** (Lean, exclusion tests, audit).
- **Not yet:** the agent's seal `goetia:N:label@agens:<hash>`; the trust handshake; the model as petitioner (writing `PETE`s).

- **Spiri72 (2026-11-12, 14:40):** `KeychainofSolomon/Spiri72`, pinned to Annulus `13058f1`. Started with no program, it asks which spirit is bound, shows the register entry, and makes each question its own rite (`spiri72.ritus/<label>-<k>.annulus`): ring worn, seal inscribed, the spirit's restraints applied (Paimon, Belial: offering then the Name; Asmoday: `APPELLA`; Beleth: wand; Andras: pentagram), examined, dismissed. 14 spirits bindable today; Presidents, Dukes, two-rank spirits and spirits with no telling office are refused in plain words. When Annulus's interface changes, update the pinned `rev` in Spiri72's `Cargo.toml`.
- **Red-team of a hostile spirit (2026-11-12, 15:00, the owner's request):** `Spiri72/tests/hostis.rs` attacks through the real binary with a hostile model: closed outcomes, changed answer shapes, injected commands and forged output, citing unhanded places, destruction and escape beyond the grant, floods, endless delay, poisoned stores, crashes, guises. **Two breaches found and closed in Annulus:** (1) the angel's citation check (`officia::cited`) saw only `[locus: x]` exactly; `[locus:x]`, `[LOCUS: x]` and the like slipped a false source past examination. Now every guise is read, and anything not exactly a handed place is refused. (2) `lingua::json` recursed without limit: a hostile model server could abort the process mid-run, leaving no audit. Now nesting stops at 64 (`MAX_DEPTH`), and doubled keys are refused.
- **Second red-team round (2026-11-12, 15:20):** the spirit's words reached the operator's terminal raw: OSC 52 could write the clipboard (Windows Terminal honours it), retitle the window, reverse text, and newlines let it print lines that looked like Annulus's own (`IRRITUM`, `audit:`). Fixed: `exsequi::visible` shows every hidden control, direction and invisible character as `\u{..}`; every line of a quarantined answer carries the spirit's mark; outside text is shown the same way; run messages too. The stores now write hidden characters as reversible `\u{..}` escapes, so opening `annulus.vas` in a terminal is safe.
- **The hours of the sky (2026-11-12, 16:00):** `src/horae.rs` and `Goetia::refusal`; tests `tests/horae.rs`, `tests/lingua_horae.rs`. Dukes now bindable. Tests use a fixed moment, `Caelum::probatio()` (2024-04-13 16:30 Rome, Moon 4 days). Readings for the owner in `dynamic.md` §5.1. Next in the core: VIRTUTES (the angel rereads file properties), then PRAETERITA against the history.
- **VIRTUTES (2026-11-12, 16:10):** `officia::virtutes`, `same_account`; the angel's check in `ritus.rs` `verdict`; simulated will in `exsequi::work`. Tests `tests/lingua_virtutes.rs`. Next: PRAETERITA against the history.
- **PRAETERITA, present run (2026-11-12, 16:30):** `Goetia::historia` builds the claim lines from the audit; `officia::account_of_past`; check in `ritus.rs` `verdict`; the checker refuses `"prior"` until the history is carried between runs. Knight's hours ruled (eleventh hour). Tests `tests/lingua_praeterita.rs`. Next: carry the history between runs for `"prior"`; then Spiri72's PRAETERITA form.

## 2026-11-12, evening: `step-2c` merged; the unknown outcome

- `step-2c` fast-forwarded into `main` (281 tests). Work now goes on `main` through short branches, merged after CI (build and Lean) passes.
- **Done, core change:** a turn records a `Will`: `Known` in simulated mode, `Declared` in real-agent mode (a run with an outside will). Real-agent mode declares only obedience, part or delay and cannot `kill`. Rust, Lean (`Will`, `Will.of`, `declarable`), exclusion tests, `core.md` §9.2. 284 tests. The test-wand gate on a model remains only because there is no real wand (2d).
- **Order agreed with the owner, next:** independent audit of the 11–12 November core changes (and this one); start the proofs of P1–P9 with their correspondence to Rust; step 2d (remove the prototype, a real wand).

## 2026-11-12, night: core audit 1 and the first proofs

- **Core audit 1** (`docs/AUDIT_CORE_1.md`): rollback and deletion of the house closed by an anchor outside the house (`ancorae`), the model's view bounded by its request, the wrong kind of answer rejected, petition text checked against its fingerprint, Windows spellings refused, smaller closures. Open items in `STATUS.md`.
- **Proofs** (`formal/Annulus/Proofs.lean`): P1 (a), P1 (b) (invariant over runs, with frame lemmas for every step) and three facts about declared outcomes and `kill`. CI admits theorems only there and checks their axioms.
- **Next:** P1 (c) (helpers) by the same method, then P2; the open audit items; step 2d (remove the prototype; a real wand, which also holds the anchor).

- **The wand is on hold** (the owner, 2026-11-12): do not start the real wand until he asks. Options laid out for him: a key file on a USB stick behind a passphrase, the Windows protected store, or a passphrase only.
