# Session handoff: Annulus (formerly ARS MAGNA)

Written 2026-11-10 for the next Claude session. Read this first, then `docs/STATUS.md`, `docs/ROADMAP.md` and `docs/sources/sloane-3825/angelic-hierarchy.md`.

## The person

The owner. Writes the design handoffs himself.

## The project

A real programming language whose authority and execution model is the Solomonic framework: circles, spirits, angelic examination, licence to depart and banishment, mapped onto capability security, sandboxing and transactional effects.

The governing document is the owner's "Definitive Architecture and Engineering Handoff", 18 sections. **Ask him to re-attach it.** Key points:
- The language is implemented in Rust, but Rust must not leak into the language.
- Bytecode and VM are the reference execution model; a native x86-64 backend comes later.
- Eight foundational laws: ORIGIN, SOVEREIGNTY, DECLARATION, OFFICE, CONTAINMENT, EXAMINATION, RELEASE, JUDGEMENT.
- Axiom: *nulla potestas sibi ipsi oritur* (no authority is self-originating).
- 14 Latin subsystems, from LOGOS to EXAMINATIO.
- 18-step development sequence.
- The implementation source must contain no comments.

## Decisions the owner has made

1. **Name: Annulus**, for the Ring of Solomon. The rename is **not yet applied** in the code: the crate, binaries and `.ars` extension still say ars-magna.
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

## Still open: ask the owner

- Keyword language (English, Latin or mixed) and casing; whether to keep the duplicate spellings `LET`/`DECLARE` and `PRINT`/`MANIFEST`.
- Mixed-type equality: keep `false`, or make it a type error? Recommendation: type error.
- Proposed meaning of the final destinations: PARADISE = fully examined and committed; PURGATORY = held for re-examination; HELL = banished and quarantined. Awaiting approval.
- Licence for the project.
- Seal artwork: the 1904 plates (with lettered double rings) or Sloane 3825's bare sigils. His fidelity rule points to Sloane 3825.
- Do type declarations count as forbidden "annotations"? Recommendation: required on public interfaces.
- What Annulus is ultimately for: real programs, a working model of Solomonic magic, or his Mage setting. Asked twice; no answer yet.
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
