# Annulus Specification, Part 2c: Concurrency

Draft 1, 2026-11-11, from the owner's rulings of the same day. Roadmap step 1, item 5. Layer: core.

## 1. Each Triangle is an actor (ruling 2026-11-11)

1.1 **Separate workers.** Every invocation runs in its own Triangle, as a separate worker (a **vat**) with its own queue. It shares no memory with any other worker. It does one piece of work (a **turn**) at a time, so no locks are ever needed inside it. The program itself is the Operator's worker. [Older parallel: the E language's "vats" (1990s): isolated workers that talk only by messages and promises. A powerful design, rarely adopted.]

1.2 **Only the Operator sends.** In E, any worker may message any other it holds a reference to. In Annulus, no spirit holds a reference to anything outside its own Triangle. The Operator sends requests, and the spirit can only reply. So the actor model keeps the rule that a spirit answers only what is asked and has no channel of his own (`operator.md` §2). [S]

1.3 **Requests do not wait.** `PETE` sends a request and returns the proposal at once, as a **promise**: a proposal still being worked on. The Operator goes on working. `EXAMINA p` waits until `p` is complete, then examines it. No syntax changes.

1.4 **Pipelining.** The Operator may queue further requests to the same spirit before an earlier one is complete. He may never build a request from a proposal's content before it is judged and consecrated (`flow.md` §2.2).

## 2. Lifetimes stay bound to the rite

2.1 **Actors that cannot linger.** Ordinary actors live on their own. Annulus binds them:
- a spirit's worker ends when he is released (`dynamic.md` §7);
- his helpers end with him, as a king's attendants do (`offices.md` §4.4);
- every worker ends when its Circle closes.

Nothing outlives the block that created it.

2.2 **Helpers** (`FAMILIARES "voca"`) are further workers inside the spirit's Triangle. They are capped by his legions (`dynamic.md` §8), and each holds a subset of his handed set (`offices.md` §4.2).

## 3. Commits are one at a time

3.1 **The core performs commits strictly one at a time,** in the order proposals are judged. Each is all or nothing. Before each commit, the core re-checks it against the state left by the commits before it (`core.md` §3.3). Two proposals that touch the same place therefore never interfere. The second sees the first's result, and is checked again.

3.2 **Races are an office, not an accident.** `DISCORDIA`'s races and load (`offices.md` §2.3) become one more verb, `perturba`. It reorders the turns of named invocations in the same Circle for the rest of that Circle: disorder in timing only, never in authority.

## 4. Deterministic simulation (ruling 2026-11-11)

4.1 **In simulated mode** (`--voluntas simulata`, `dynamic.md` §5.6), every worker runs on a single simulated clock. One scheduler orders all turns from the launch seed, so any interleaving, and any bug it causes, replays exactly. [Modern parallel: deterministic simulation testing in FoundationDB, TigerBeetle and Antithesis.]

4.2 **In real-agent mode,** workers run truly in parallel. The audit records the actual order of every turn and commit, so the run's decisions can be replayed, though not the agents' own thinking.

## 5. Interactions

- **Earls** (`dynamic.md` §5.1): while an Earl is invoked, no other worker is opened in the program.
- **Andras's killing** (`dynamic.md` §5.5) ends every worker at once.
