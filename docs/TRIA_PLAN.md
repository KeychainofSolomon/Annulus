# Draft: the kings, the escalation, and a refused consecration

Draft, 2026-11-15. Nothing here is built. It covers three open items of `STATUS.md`: `INVOCA REGEM`, `RECUSATIO` and its escalation, and the form by which a program receives a refused `CONSECRA`.

## 0. The finding that shapes the plan

**None of the three needs a new rule in the core.** The core already has the steps they use: the rite's act (`act`), banishment (`relegate`), a spirit's turn (`turn`) and consecration (`consecrate`). The kings and the escalation are rite content and interpreter work; the consecration form is grammar, checker and interpreter. So the Lean model keeps its rules, and proving P9 does not have to wait for them.

Two small points do touch the core's edge, and are listed in §4.

## 1. `INVOCA REGEM`

**What the spec already fixes.** The syntax (`grammar.md`), the type `NIHIL` and rule A13 (`static.md` §4.5b), the hours and the three effects (`dynamic.md` §4), the kings and who is under each (`registers/reges.md`), and that a President obeys the `goetia` king of his stated quarter (`authority.md` §8).

**How it lands.**
1. A new act of the rite, `Rex(king)`: no spirit named, no ink. The core's `act` step records it, as it records `GERE`.
2. The `goetia` rite admits it only in the kings' hours (Weyer ch. III: the third hour to noon, the ninth hour to evening) and only for a king of its own register. Outside them the run ends in a denial, as any refused act does.
3. The rite reads back from the audit which kings were invoked in the open Circle, as it reads which instruments are held. From that:
   - a President under that king may be admitted (never at nightfall);
   - escalation step 4 counts as done for every spirit under him (§2);
   - nothing else: it closes no outcome of the will.
4. The checker stops refusing `INVOCA REGEM` as outside the subset; A13 is already checked.

**Where:** `nucleus/types.rs` (the act), `ritus.rs` (admission, the reading back, Presidents), `lingua/check.rs` and `lingua/exsequi.rs` (the expression), `horae.rs` (the kings' hours).

**Tests:** each of the four kings in and out of hours; a President with and without his king, and at nightfall; a king of another rite; the king's invocation not carrying into a second Circle; Zagan, Gaap, Marbas, Foras, Malphas and Osé each coming under the right king.

**Open:** the ink for two-rank seals (`STATUS.md`) decides what a President who is also another rank pays; it does not block the kings. `rudd` stays provisional (`static.md` §4.5b).

## 2. `RECUSATIO` and the escalation

**What the spec already fixes.** The seven steps and their sources (`spirits.md` §5); that the runtime performs the next step, records it and draws again, each step past the first spending one unit of the seal's ink, the seventh being `RELEGA` (`dynamic.md` §5.2); that a refused request resumes at step 3 (§5.3); that the hexagram closes `RECUSATIO` once the spirit has appeared (`spirits.md` §4.4); and Paimon unoffered (`dynamic.md` §4).

**How it lands.**
1. A new act of the rite, `Gradus(invocation, step, cause)`: it names the spirit, spends the seal's ink for steps 2 to 6, and says what it answers (a refusal to appear, or a refusal of a request). The core's `act` already checks that the spirit is here and that the ink is left, and spends it.
2. **Refusing to appear.** `INVOCA` admits the spirit as now. The rite then draws its will. On `RECUSATIO` the runtime performs the next `Gradus`, and draws again. Step 4 is skipped, and recorded as skipped, where the king is not stated; it is recorded as already done where `INVOCA REGEM` came first. Step 7 is the core's `relegate`: the invocation stays, relegated.
3. **Refusing a request.** The spirit's turn declares `RECUSATIO`, contributes nothing, and leaves the proposal pending. The runtime performs `Gradus` from step 3 and schedules the spirit again.
4. **When the ink runs out** before the spirit yields, the act is refused and the run ends in a denial: the Operator brought too little.
5. **The rite opens `RECUSATIO`** where the register leaves it open: every spirit until constrained, closed by the hexagram after appearance, and Paimon's case as written.
6. **The draw.** In simulated mode the draw comes from the run's seed, so a replay draws the same (P9). With a real model the spirit declares its own refusal.
7. **The step number** is read from the audit's `Gradus` records. See §4.1.

**Where:** `nucleus/types.rs` (the act; `RECUSATIO` among the outcomes a real model may declare), `ritus.rs` (opening the outcome, the steps, the skip), `lingua/exsequi.rs` (the loop at `INVOCA` and at `PETE`), Spiri72 (`forma.rs`: `recusatio` in the grammar the model answers in; a later change in that repository).

**Tests:** a spirit that yields at each step; one that never yields and is banished at the seventh; ink spent step by step and the denial when it runs out; step 4 skipped, and already done; a request refused and resumed at step 3; the hexagram closing refusal; Paimon with and without his offering; the same seed giving the same steps; under the rite that says yes to everything, no step gaining a spirit anything.

**Not in this draft:** spirit-specific malice (`MALITIA`), which `STATUS.md` lists beside the escalation. It is a separate piece.

## 3. A refused `CONSECRA`

**The gap.** `core.md` §9.3a says a refused consecration is "an answer the program receives". The language has no form for it, so today the interpreter ends the run. The difficulty is the type: `CONSECRA v` is typed as consecrated, and after a refusal there is no consecrated value to give.

**Proposed form [O].** The consecrated value exists only inside the branch where consecration succeeded:

```
SI CONSECRA RESPONSUM propositum IN verba {
    SI verba == "ita" { MANIFESTA "verum dixit"; }
} ALITER {
    MANIFESTA "non consecratur";
}
```

- `verba` is bound, consecrated, in the first block only. In the `ALITER` block it does not exist, and the original stays quarantined.
- The bare expression `CONSECRA v` stays as it is: a refusal ends the run. Existing programs do not change.
- No new keyword: `SI`, `IN` and `ALITER` are already in the language.
- The core records the refusal either way, and nothing else changes (§9.3a).

**Why this form.** The checker's promise stays simple: a name typed consecrated always holds a consecrated value. That is the property P5 will have to prove about the checker, and this form adds no case to it.

**Where:** `grammar.md` and `GRAMMAR.ebnf` (the conditional), `static.md` (the binding's scope and mark), `dynamic.md` §4.2 (replacing the subset note), `lingua/parse.rs`, `check.rs`, `exsequi.rs`.

**Tests:** both branches; the name unusable in `ALITER` and after the block; the original still quarantined after a refusal; the refusal recorded; the bare form unchanged; a grave consecration in the same form once the grave store is built.

## 4. The two points at the core's edge

4.1 **The escalation step in the state.** `core.md` §9.1 lists each invocation's escalation step as part of the state, and the Lean model carries an `escalation` field that no rule uses. The Rust core has no such field. Proposed: the step is the rite's, read from the audit, and `core.md` and the Lean model say so (the field removed). The core's own bound on escalation is the ink, which it already spends. This is a wording change to a normative section, so it needs the owner's word.

4.2 **A real model may declare `RECUSATIO`.** The core keeps a short list of outcomes a real model may declare for itself (`OBSEQUIUM`, `PARS`, `MORA`). `RECUSATIO` joins it. The Lean model leaves that list abstract, so no proof changes.

## 5. Order of building

1. The consecration form (§3): smallest, touches no rite.
2. The kings (§1): the escalation's step 4 leans on them.
3. The escalation (§2).
4. Spiri72: `recusatio` in the model's grammar.

Each is its own change, with `STATUS.md` updated as it lands.

## 6. For the owner to rule

1. The consecration form of §3: accept it, or keep a refusal ending the run.
2. §4.1: the escalation step read from the audit, and `core.md` §9.1 reworded.
3. When the ink runs out mid-escalation (§2.4): a denial that ends the run, as proposed, or an immediate banishment of the spirit.
4. Whether a real model's refusal to appear is asked of the model itself at `INVOCA` (one more question to it per calling), or drawn by the engine from the spirit's temper.
