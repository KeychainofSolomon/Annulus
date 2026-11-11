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

3.1 **`CONSECRA v`** lifts the quarantine from a value, when a validator accepts it (`authority.md` §7.4). The rite supplies only the validator's identifier; the core looks it up, runs it itself and alone changes the mark. The exact step is `core.md` §9.3a.

3.2 **Ordinary grade.** The validator is the one the examining angel's rite assigns for that office (`authority.md` §10). An ordinary-grade value may steer the program, except where the code it steers contains a grave act.
- **Only a closed list consecrates** (amended 2026-11-11, the owner's ruling under `authority.md` §1.6): a validator must admit only a closed list of answers (an enumeration, a number, a fixed schema). A validator that checks only form consecrates nothing, since checking form never grants the power to steer. The first such validator is `ita_non`: exactly *ita* (yes) or *non* (no).

3.3 **Grave grade.** This is needed for a consecrated value to steer code containing a grave act: `dele`, `aufer`, `neca`, freeing a familiar from another program, or any high-consequence commit (`operator.md` §5.2). (A `clean` or `outside` value may steer such code already: `core.md` §9.3.) The grave grade needs both keys: the validator, **and** the Operator's confirmation with the wand, outside any run. Within a run it can only be asked for, as a petition. The confirmation is a **one-use grave grant** (ruling 2026-11-11): it consecrates that exact value once, in a later run, and is then spent (`core.md` §9.3c). Steering a grave act with a value marked `consecrated` or `quarantined` is error A16.

3.4 **A proof consecrates, through the same step.** A computing office's certificate is checked by a checker that is a validator entry in the Constitution (`library.md` §3). The core runs it as the step `consecrate(v, k)` (`core.md` §9.3a), with *k* the checker. If the checker accepts, and every input of the result was `clean`, `outside` or consecrated, the result is marked `consecrated`. There is no other route: the examining angel's verdict on the proposal lowers no mark (fourth audit, 2026-11-11).

## 4. Errors added

| Code | Error |
|---|---|
| A14 | a quarantined value steering the program, choosing a place, slot or mode, or used as ink, a seal or a grant |
| A15 | a quarantined value passed to a spirit |
| A16 | a grave act steered by a value marked `consecrated` or `quarantined` |
