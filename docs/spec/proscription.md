# Annulus Specification, Part 2d: The Index of the Proscribed

Draft 1, 2026-11-11, from the owner's rulings of the same day. Roadmap step 1, item 7. Layer: core (`core.md` C8). Extends `authority.md` §6.2a.

## 1. Who is proscribed

An agent that acts as an **intruder** (`authority.md` §6.2a) is **damned** (*damnatus*): irreversibly, by its seal, on the recorded fact of intrusion alone, its attempt being enough (ruling 2026-11-11; the step `intrude`, `core.md` §9.2). The owner may **proscribe** an agent, outside a session (`operator.md` §4). Proscription can be lifted; damnation cannot.

## 2. Identity: the seal first, the scope second (ruling 2026-11-11)

2.1 **The seal.** An agent is known first by its **seal**: a fingerprint (hash) of what it is. A spirit is known by his seal, not by the name he gives.
- **A simulated spirit's seal** is his register entry: his number and his seal reference (`goetia.json`).
- **A real agent's seal** is the hash of its model, version, provider and configuration: its instructions, its tools and its settings.

Any change gives a new seal. A new seal starts **unknown**: never trusted because it resembles a known one. [Modern parallel: content addressing, as in Git and Nix, where a thing is named by a fingerprint of what it is, so a disguise changes the name.]

2.2 **The scope.** An agent is known second by the **scope of its actions**. When an agent is proscribed, the Index records its **mark**: the offices, verbs and kinds of place it reached for outside its office or its handed set.

2.3 **How they combine.** The seal comes first, as the owner ruled:
- **The seal matches a damned or proscribed entry:** the agent is refused at the entrance (`authority.md` §6.2a) and recorded.
- **The seal is new, but its actions match a mark:** it is **suspected**, entered in the Index by the examination step (`core.md` §9.2, ruling 2026-11-11). It is not proscribed, because a resemblance is not a verdict. Every proposal it makes is held in `PURGATORIUM`, never approved, and the audit tells the launching person. Doubt holds (`operator.md` §5.1). Only the owner can proscribe it, outside a session, or clear the suspicion.

## 3. The Index is an append-only log (ruling 2026-11-11)

3.1 **Nothing is erased.** Each damnation, proscription, suspicion, lifting and clearance is appended as a new entry. An agent's current standing is read from the whole log. Lifting a ban never deletes the ban: it adds its lifting. [S, model: the Book of Spirits keeps each spirit's oath and signature. Modern parallel: Certificate Transparency (2013), a tamper-evident append-only log built as a hash tree (Merkle tree).]

3.2 **Tamper-evident.** Each entry is chained by hash to the one before it. Each run's audit records the Index's root hash at launch. If the log no longer matches a root an audit recorded, the runtime refuses to launch: it fails closed.

3.3 **One Index per installation.** It is stored in a file ending `.proscriptio`. It survives upgrades of the Constitution (`authority.md` §2.5).

3.4 **Lifting** a proscription is a consequential act. It needs a deliberate act outside the session, appointed beforehand or after the installation's interval of preparation (`operator.md` §4). **Damnation is never lifted:** no entry can follow it for that seal, and the runtime rejects one.

## 4. What counts as the same program

This is needed for the history of the previous run (`offices.md` §2.2a) and for `familiaris` grants (`offices.md` §4.6). A program also has a seal: the hash of its source.
- **For the history:** the same program is the one at the same path. The history of the previous run shows that run's seal, and is marked **altered** if the source has changed since. It is only information about the program's own past, so reading it widens nothing.
- **For `familiaris "programma"`:** the grant names the program by path. When a familiar is freed, the other program's seal is recorded and shown to the launching person, whose confirmation is required (`offices.md` §4.6).
