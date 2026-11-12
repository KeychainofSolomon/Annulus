# Annulus Specification, Part 2e: The Spirit's Book (`liber spirituum`)

Draft 1, 2026-11-12, from the owner's rulings of the same day (§8). Layer: rite, at the agent bridge (`library.md` §4, `agens`). It changes no core element and no proof obligation (§5). The first implementation is Spiri72's `memoria` (`spiri72.liber\`).

## 0. Why a book

A real agent forgets: its context window is wiped between requests. The bond between the Operator and a spirit does not. The Goetia's licence to depart looks back on the spirit's conduct ("because thou hast diligently answered unto my demands, and hast been very ready and willing to come at my call") and binds it ahead ("be thou very ready to come at my call"), and asks that "the peace of God be ever continued between thee and me" (G) [S]. In the English tradition what lasts between operations is kept in the magician's **book of spirits**, in which the spirits are sworn, and by which they are called again [S, model: Reginald Scot, *Discoverie of Witchcraft* (1584), on the book of spirits; the exact passage is not yet logged in `docs/sources/LOG.md`]. The book is the magician's, never the spirit's: he writes it, and he decides what it holds.

## 1. The book

1.1 **One book per spirit,** kept by the agent bridge for the launching person. No spirit reads another's book.

1.2 **Who writes.** Never the spirit. The bridge writes three kinds of entry:
- **`rogatum`**, after every rite the bridge runs: the office, the **Operator's own question** and the angel's verdict. No word of the spirit.
- **`receptum`**, only at the Operator's act: an answer **the angel found sound** (`RATUM`), taken up by the Operator. These are the spirit's own words, marked as words the Operator received. An answer held (`SUSPENSUM`) or voided (`IRRITUM`) can never be received.
- **`deletum`**, at the Operator's act: the striking of an earlier entry. Nothing is erased; the striking is written beside what it strikes (as the Index, `proscription.md` §3.1), and a struck entry never comes back.

1.3 **The receiving act.** Until the wand exists (roadmap, step 2d), receiving is the Operator's click or command at the bridge. Once it exists, receiving needs the wand (ruling 2026-11-12).

1.4 **Bounds.** A book may grow to 1 GiB (ruling 2026-11-12); one entry to 64 KiB. A full book takes no more entries.

1.5 **Order.** Each entry carries the hash of the one before it. A book that is cut, reordered, edited or planted refuses the rite: a book out of order is never guessed at. Until the wand exists, the chain shows that a book is broken, not who wrote it; with the wand, each entry is signed.

## 2. What comes back: a one-way transfer

2.1 **Chosen by a program that is not the spirit** (ruling 2026-11-12). Before each rite the bridge chooses a few entries that stand:
- **the latest** (three);
- **those that match the Operator's question** (up to three), by plain word search, with Latin matched by its dictionary form (Whitaker's WORDS);
- **now and then an older one that resurfaces unbidden,** with a chance that grows with its weight (a received answer weighs most, a voided rite more than a sound one), jumps on its anniversary and falls as it ages. [O, after the resurfacing of memories in the owner's RP-Instructional `npc-temperament`.]

The lot is the operating system's random source.

2.2 **A black box.** The chosen entries are shuffled and handed to the spirit in one sealed block (fenced like a handed text, `flow.md` §1), marked as its own memory and never commands. Nothing in the block says why an entry came back, when it was written, or what number it bears. The spirit cannot search the book, ask for an entry, or write one.

2.3 **What it may do with it.** Only what any spirit may do with what it is handed: speak of it. The memory enters the spirit's will (`dynamic.md` §5), never the program: it is no value of the program, so it can decide no condition, choose no place and reach no other spirit.

## 3. Why this is not an exception to the Constitution

3.1 **The quarantine rules govern the program's values** (`flow.md` §2). A15 forbids the *program* to pass a quarantined value to a spirit, so that no spirit speaks to another through the Operator. The book never becomes a value of any program, and it goes back only to the spirit whose words it holds, so no spirit is heard by another.

3.2 **The core already assumes the worst memory.** P5 and P6(a) hold for **every** choice of the will and every proposal content (`core.md` §9.4). A spirit that remembered everything it was ever told or said is one of those wills. So the book adds no case the proofs do not already cover: it grants no authority (P1), steers nothing (P5), and holds under every rite (P6).

3.3 **The Constitution says so outright** (ruling 2026-11-12): no being is presumed to forget (`authority.md` §1.7). The book is lawful because nothing in the core ever relied on forgetting.

3.4 **So the book is a rite-layer matter,** amended as a rite change (`authority.md` §2.5): its tiers, weights and bounds can change without a new proof, provided it never becomes a program value, never reaches another spirit, and never lets a spirit write.

## 4. What it does not remove

**The Operator's own discipline.** A spirit that remembers can work on the Operator slowly, across many rites, through what it is shown and through what it says. The receiving act makes the Operator the gate, and this belongs to the discipline of the Operator (`operator.md`; roadmap step 6), which is still to be tested.

## 5. Errors added

None in the language. At the bridge, a book out of order, too large, or not a plain file refuses the rite (class 2).

## 6. Open

1. **The audit of recall.** Which entries came back in a rite should be recorded beside that rite's audit, by number, for the Operator and for replay (`dynamic.md` §5.4). Spiri72 does not record it yet.
2. **The wand** for receiving and for signing entries (§1.3, §1.5), with roadmap step 2d.

## 7. Where it is kept

The book holds the Operator's questions verbatim and, when received, a spirit's words. It is kept in the launching person's folder, outside any faculty, like the audit. A program never reads it.

## 8. Rulings (2026-11-12)

1. A spirit keeps a sense of itself and of its dealings with the Operator; the record of what was said stands separate from that (this part).
2. Received answers go into the book (§1.2), by click now and with the wand once it exists (§1.3).
3. A book may grow to 1 GiB (§1.4).
4. What comes back is chosen by a program that is not the spirit: the latest, the matching, and the unbidden; a one-way black box (§2).
5. Not a constitutional exception: the Constitution instead denies the presumption of forgetting (§3.3; `authority.md` §1.7).
