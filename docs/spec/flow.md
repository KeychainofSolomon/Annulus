# Annulus Specification, Part 2a: Information Flow

Draft 1, 2026-11-11, from the owner's rulings of the same day. Roadmap step 1, item 4. Layer: core (`core.md` §1, taint).

## 1. Two marks, kept apart

1.1 **Outside data is tainted** (`authority.md` §7.4). This covers data the program reads itself from what the faculty grants it, such as a file it reads. It MUST be consecrated before it is used as ink. It MAY steer the program: the launching person granted it, so it carries no spirit's word.

1.2 **Spirits' words are quarantined** (ruling 2026-11-11). This covers every response (`RESPONSUM p`), and every value computed from one. In the rite, a spirit's words stay in the Triangle and never command the magician (`operator.md` §2). [S, model. Modern parallel: CaMeL (Google DeepMind, 2025), which defends AI agents against hidden instructions by never letting untrusted text decide what the program does.]

1.3 **The mark follows the data.** It survives storage: a file written with quarantined content stays quarantined, and so does anything read back from it, across runs. The runtime keeps the mark beside the file and records it in the audit. A file with no recorded mark, which the audit shows a spirit wrote, is quarantined. [O]

## 2. Quarantine: what spirits' words may and may not do

2.1 **Allowed:**
- **shown** with `MANIFESTA`, marked with the spirit's name and veracity class (`operator.md` §2.2);
- **stored** in a granted file, which keeps the mark;
- **computed with:** the result is quarantined too.

2.2 **Forbidden until consecrated** (§3). The type checker rejects each of these before the program runs:
- **steering:** being, or deciding, the condition of `SI` or `DUM`, the range of `PER`, or a choice between functions (A14). Because no quarantined value can be a condition, no decision can depend on one even indirectly;
- **choosing what is touched:** being a place, a slot or a mode in `PETE`, or an index that picks a place (A14);
- **reaching another spirit:** being passed to any spirit, even as the text of a question (A15). Otherwise one spirit could speak to another through the Operator, the same reason the history never holds a proposal's words (`offices.md` §2.2a);
- **becoming ink, a seal or a grant** (A14).

2.3 **Labels are inferred.** Programs write no labels. The type checker works out each value's mark; a function is checked once for each mark it is called with. [O]

## 3. Consecration by gravity (ruling 2026-11-11)

3.1 **`CONSECRA v`** lifts the quarantine from a value, when a validator other than the program accepts it (`authority.md` §7.4). It yields a value marked **consecrated**, at one of two grades.

3.2 **Ordinary grade.** The validator is the one the examining angel's rite assigns for that office (`authority.md` §10). An ordinary-grade value may steer the program, except where the code it steers contains a grave act.

3.3 **Grave grade.** This is needed to steer code containing a grave act: `dele`, `aufer`, `neca`, freeing a familiar from another program, or any high-consequence commit (`operator.md` §5.2). It needs both keys: the validator, **and** the launching person's confirmation, given outside the session (`operator.md` §4). So within a run, a grave-grade consecration can only be asked for, as a petition; it takes effect in a later run. Steering a grave act with an ordinary-grade value is error A16.

3.4 **A proof consecrates.** A computing office's result whose certificate the angel has verified (`offices.md` §2.2) is consecrated at the ordinary grade, provided all of its inputs were clean or consecrated. A verified proof needs no second validator.

## 4. Errors added

| Code | Error |
|---|---|
| A14 | a quarantined value steering the program, choosing a place, slot or mode, or used as ink, a seal or a grant |
| A15 | a quarantined value passed to a spirit |
| A16 | a grave act steered by a value not consecrated at the grave grade |
