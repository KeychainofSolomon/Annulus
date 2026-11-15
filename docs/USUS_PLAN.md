# Plan: the practice book (`usus`) and its web (`tela`)

Design only, 2026-11-15. Nothing here is built. Layer: rite, at the agent bridge (Spiri72), beside the spirit's book (`spec/liber.md`). Names are [O], working names. **Build order: after the Rust and Lean drift is closed (`STATUS.md`, Open).**

## 0. What the owner asked for

A window that shows, live, a web of what each spirit has learned; a spirit that learns to use the software its office covers and to know when the library must check its work; and memory use that stays close to the baseline model's.

## 1. The memory rule (hard requirement)

1.1 **One model in memory, never two.** No second model for searching: examples are found by office, mode and plain word search, as the book's recall is (`liber.md` §2.1).
1.2 **The context size is fixed at start.** The practice block has a fixed share of it (to be measured; a first guess is a tenth). The model is never reloaded to make room.
1.3 **Retraining never runs while a spirit is called or held.** It is a separate round, started by the Operator, and its result has the same size and the same compression as the baseline, with the new learning merged into the model, never loaded beside it.
1.4 **The window draws from a small file.** It holds no copy of the model.

## 2. Stage 1: learning by practice (no change to the model)

2.1 **One practice book per spirit,** kept by the bridge beside its `liber`, chained and bounded the same way (`liber.md` §1.4–1.5). No spirit reads another's.
2.2 **The spirit never writes it.** The bridge writes an `exemplum` only from a work the angel ratified (`RATUM`). A work held (`SUSPENSUM`), voided (`IRRITUM`) or declared in part (`PARS`) is never an example; only its count is kept.
2.3 **What an example holds:** the office kind and mode, the *shape* of the request, the verbs of the proposal with slot names in place of real paths, and, for computing offices, the certificate's kind and the library that checked it. **Never the content of a handed text:** a planted instruction in a file must not outlive its rite.
2.4 **What comes back:** before a rite the bridge picks a few examples for the same office and mode and hands them in a sealed block, marked as the spirit's own earlier practice and never as commands (`liber.md` §2.2). The spirit cannot search, ask or write.
2.5 **Striking:** the Operator may strike an example (`deletum`); a struck one never returns.

## 3. The software an office covers

3.1 **The map is the register's, never the model's.** Office kind → computer work (`spirits.md` §6) → art and library (`library.md` §3.3, `offices.md` §5.1) → verbs and permissions (`offices.md` §2). The web is drawn from this.
3.2 **Learning adds skill, never reach.** A spirit learns the form its verbs and certificates must take. It gains no tool, place or permission by learning: those come only from the grant. Outside software is used only once wrapped under the grant (`library.md` §4, "taming").

## 4. When the library is called

4.1 **The rule is fixed, never learned:** a computing office always attaches a certificate and the library always checks it (`library.md` §3.1). The engine's brief says so in words.
4.2 **What is learned** is which certificate form passes for which request. A wrong or missing one is held or rejected exactly as now.

## 5. The window (`tela`)

5.1 **Where:** a page of Spiri72's local hall (`aula.rs`), black and white, with the real seals.
5.2 **The web:** the spirit's seal at the centre; then its office; then the arts and libraries; then the verbs; then the slots. A line thickens with each ratified work, is dashed where works were held, is marked where one was voided, and is grey where nothing was tried or the library is not yet built.
5.3 **Live:** the page redraws after each verdict.
5.4 **Acts:** click a node to read its examples; strike one; start a retraining round (stage 2). Everything else is read-only.

## 6. Stage 2: a retraining round (changes the model)

6.1 **What it is.** A thin slice of the model is trained on the practice book's examples, then merged back, so the file keeps its size (§1.3). The training set comes from `spiri72 --exempla` plus the practice books.
6.2 **Plainly:** nobody can point to the "less relevant parts" of a model and overwrite only those. Training a thin slice is the nearest real thing, and it can make the model worse at what it knew. So each round keeps the old model file, and the new one must pass Spiri72's attack tests (`tests/hostis.rs`) and a check that the spirits' speech is as uncensored as before, or it is thrown away.
6.3 **Memory while training** is higher than while running, and needs the uncompressed model on disk. How much, and whether the owner's machine can do it, is to be measured before this stage is promised.
6.4 **The seal.** A retrained model is a new agent: its seal (`agens`) changes, and each rite records which model and which practice-book head it ran under.

## 7. Why the core is untouched

The core's proofs hold for every choice of the will (`core.md` §9.4; `liber.md` §3.2). A spirit that has practised, or been retrained, is one such will. So this adds no case, provided: the practice book is never a program value, never reaches another spirit, and is never written by the spirit.

## 8. Needed first

1. The Rust and Lean drift closed, and the P9 ruling.
2. The agent's seal (`agens`), extended by §6.4.
3. The audit of recall (`liber.md` §6.1), covering practice blocks too.
4. The `DOCTRINA` checking library (PR #3), so teaching spirits have something to practise against.

## 9. For the owner to rule

1. May a computing work whose certificate the library checked become an example by itself, or does every example need the Operator's act (and the wand's words)?
2. May a telling office's answers ever be examples? Proposed: no, only their form, since no one can check them.
3. The share of the context given to practice (§1.2).
4. Whether stage 2 is wanted at all once stage 1 is seen working.
