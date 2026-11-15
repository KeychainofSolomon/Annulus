# Draft: the kings, the escalation, and a refused consecration

Draft, 2026-11-15. **The consecration form (§3) was accepted and built the same day** (`flow.md` §3.5); §3 is kept as the record of its design and research. The kings (§1) and the escalation (§2) are not built. It covers three open items of `STATUS.md`: `INVOCA REGEM`, `RECUSATIO` and its escalation, and the form by which a program receives a refused `CONSECRA`.

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

The bare expression `CONSECRA v` stays as it is: a refusal ends the run. No new keyword: `SI`, `IN` and `ALITER` are already in the language.

### 3.1 What the research found (2026-11-15)

**The one real risk: the refusal is itself a signal.** `flow.md` §2.2 rests on this sentence: "Because no quarantined value can be a condition, no decision can depend on one even indirectly." This form is the first place where that stops being true. Whether the `ALITER` block runs depends on the spirit's words, and nothing was consecrated. A spirit that wants the `ALITER` block to run only has to answer badly.

**How it is closed.** A refusal is treated as **one more member of the validator's closed list**. A validator that admits *ita* and *non* (`flow.md` §3.2) gives the spirit two choices; this form gives it three: *ita*, *non*, or "unfit". The Operator, by writing `ALITER`, consents to that third choice exactly as he consents to the other two. So the `ALITER` block gets the rules of a block steered by an ordinary-grade consecrated value, no more and no less. With that, the form opens no kind of influence the language does not already allow.

**The model already agrees.** In the Lean model the consecrate instruction continues with the answer, yes or no (`Basic.lean`: `consecrate (v) (k : Bool → Exec)`), and the released record of P5 holds every consecration with its answer, refusals included (`Theorems.lean`: `declassOf`). So P5 as stated already says: a spirit's words reach the program only through the recorded answers of consecration, accepted or refused. No statement changes.

**The literature has this construct.** In language-based security it is a *checked endorsement*: untrusted data is tested, and the program branches on the test. The accepted way to judge it is by how much influence it hands the attacker (Askarov and Myers, 2011; Myers, Sabelfeld and Zdancewic, 2006). Here that influence is one choice among the closed list plus "unfit".

**Solomonic reading.** "Consecrate everything" is the Key's own summary of the art (BnF Ital. 1524, 1446, last chapter): nothing unconsecrated enters the work. Consecration is done by the names of God, never by the Operator's will or the spirit's; and "the key of our Art is to fear God" (1446, I.1). So: an unfit thing is simply not used; the Operator cannot overrule the refusal; and the spirit's unfit words stay in the Triangle (`operator.md` §2). The sources describe no consecration that fails and is then handled, so the branch itself is [O]. Its nearest model is the spirit whose speech cannot be used until he is compelled (Paimon, `spirits.md` §4.1): the answer to unfit words is the constraint and a second asking, or the licence to depart.

### 3.2 The rules that make it hold

1. **The name exists only where consecration succeeded.** `verba` is bound, consecrated and immutable, in the first block only. It does not exist in `ALITER` or after the form. It follows the ordinary rules for names (`static.md` §2), so it cannot be spelled as a spirit's label.
2. **`ALITER` is required.** The program must say what it does with unfit words, even if the block is empty. A refusal is never passed over in silence.
3. **`ALITER` is steered at ordinary grade.** Whatever a consecrated value may not steer, a refusal may not steer: no grave act (`flow.md` §3.3, A16).
4. **One answer only.** The program learns "refused" and nothing else: not whether the words were outside the list or the validator ran out of its work limit. The reason is in the audit, for the Operator.
5. **A fault is not a refusal.** If the office has no validator, or the value is not a spirit's text, that is the Constitution failing to provide: a denial (class 2), which ends the run and cannot be caught. Only a validator that ran and did not say yes sends the program to `ALITER`.
6. **Every doubt is a refusal.** Out of work limit, an answer that is neither yes nor no, a validator that cannot be found by its fingerprint: all refuse. Nothing in `ALITER` can consecrate.
7. **Each consecration stands alone.** A yes for one value says nothing about another, and the validator is the one the rite assigns to that office: the program does not choose it.
8. **The refusal is recorded before either block runs**, as now.
9. **Ordinary grade only.** The grave grade keeps its own path (the one-use grant, `core.md` §9.3c); this form does not offer it.

### 3.3 What the research turned up in today's code

1. **The mark does not follow control** (`lingua/check.rs`). A flag set inside a block that a consecrated value steers stays clean, and a clean value may steer a grave act. Rule A16 is also not yet in the checker. Today this harms nothing: a program in the subset cannot perform a grave act itself, and a grave effect asked of a spirit waits for the second key at commit. But before any grave act can be written in a program, the checker must carry the steering grade into what is assigned under it, for `SI` on a consecrated value as much as for `ALITER`. This is owed to P5 in any case.
2. **A second consecration of the same words is refused.** `CONSECRA RESPONSUM p` written twice ends the run at the second, with "the validator does not accept", though the words were accepted a moment before. It fails closed, so it is safe, but under the new form it would send the program to `ALITER` with a wrong "no". To fix when the form is built: consecrating what is already consecrated is done, not refused.
3. **A value computed from a spirit's words can be consecrated** (`CONSECRA (RESPONSUM p + "")` is accepted). This is sound under the closed-list rule, since the validator judges the final text. To check when building: which validator applies when the words of two offices are combined; it should be refused.
4. **The validator is the single gate, and it is still known by name.** Everything above trusts the validator. The Constitution fingerprints it by its name, not its code (`STATUS.md`, the open drift point). The form is no weaker than `CONSECRA` is today, but neither can be called watertight until real validators are fingerprinted by their code.
5. **A spirit can keep a program asking.** If a program loops, asking again after each refusal, a spirit that always answers badly keeps it there. Nothing is gained by it but time. The Solomonic bound is the ink: an unfit answer may be met with the constraint, and the escalation ends (§2).

### 3.4 Verdict

The form holds, on the nine rules of §3.2. It adds one choice to what a spirit can already make, and the Operator writes down what follows from it. Two things outside the form limit how strong the whole is: item 1 of §3.3 before grave acts enter programs, and item 4 before any real model does harmful offices.

**Where:** `grammar.md` and `GRAMMAR.ebnf` (the conditional), `static.md` (the binding's scope and mark, rules 1 to 3), `flow.md` §2.2 (the sentence quoted above, amended to name this one case), `dynamic.md` §4.2 (replacing the subset note), `lingua/parse.rs`, `check.rs`, `exsequi.rs`.

**Tests:** both branches; the name unusable in `ALITER` and after the form; `ALITER` missing is an error; the original still quarantined after a refusal; the refusal recorded first; a validator out of its work limit goes to `ALITER`; no validator for the office ends the run; the program cannot tell the two kinds of refusal apart; a second consecration of the same words succeeds; the words of two offices combined are refused; the bare form unchanged.

**Sources for §3.1:** Askarov and Myers, "Attacker Control and Impact for Confidentiality and Integrity", *Logical Methods in Computer Science* 7(3), 2011; Myers, Sabelfeld and Zdancewic, "Enforcing Robust Declassification and Qualified Robustness", *Journal of Computer Security* 14(2), 2006; NIST SP 800-207, *Zero Trust Architecture*, 2020; MITRE CWE-636, "Not Failing Securely".

## 4. The two points at the core's edge

4.1 **The escalation step in the state. Ruled 2026-11-15** (by Claude, as the owner delegated: "what's most intuitive to the principles"). The step is the rite's, read from the audit's `Gradus` records. The seven steps are the Operator's own conjurations, said in order, so they are rite content; the core's part is the record. And one record is better than two: a counter in the core could disagree with the audit, which is signed and cannot be cut. The core's own bound on the escalation is the ink, which it already spends. When the escalation is built, `core.md` §9.1 is reworded and the unused `escalation` field leaves the Lean model.

4.2 **A real model may declare `RECUSATIO`.** The core keeps a short list of outcomes a real model may declare for itself (`OBSEQUIUM`, `PARS`, `MORA`). `RECUSATIO` joins it. The Lean model leaves that list abstract, so no proof changes.

## 5. Order of building

1. ~~The consecration form (§3).~~ Built, 2026-11-15.
2. The kings (§1): the escalation's step 4 leans on them.
3. The escalation (§2).
4. Spiri72: `recusatio` in the model's grammar.

Each is its own change, with `STATUS.md` updated as it lands.

## 6. For the owner to rule

1. ~~The consecration form of §3.~~ **Accepted by the owner, 2026-11-15, with the nine rules of §3.2.**
2. ~~The escalation step read from the audit.~~ Ruled, §4.1.
3. When the ink runs out mid-escalation (§2.4): a denial that ends the run, as proposed, or an immediate banishment of the spirit.
4. **Ruled by the owner, 2026-11-15: a spirit can refuse to appear when it is called**, "for authenticity's sake". Still to confirm: that the refusal is the model's own answer at `INVOCA`, and not the engine's draw. A model that always refuses could make a spirit unusable unless refusal closes at some step (see 5).
5. **Where refusal ends.** The spec says two things. `dynamic.md` §4: `RECUSATIO` "stays open until the Spirits' Chain" (step 5), so from there the spirit must come. `dynamic.md` §5.2: after the seventh step the spirit is banished. In the Goetia as Claude recalls the 1904 text, the fire and the curse end with the spirit coming, and no spirit is left that never comes; the banishment is Annulus's own. This is from recollection and must be checked against Sloane 3825 (frames 119 to 125) before it is relied on. To rule: at which step refusal closes, and whether banishment at the seventh is kept.
6. **Ruled by the owner, 2026-11-15: the consecration of ink needs no condition of its own.** It is presumed done when Spiri72 starts. `CONSECRA ATRAMENTUM` stays the formal step it is; no hour, name or validator is added to it. `authority.md` §7.4, which asks for validation by another authority, is to be reworded for ink when this draft is built.
