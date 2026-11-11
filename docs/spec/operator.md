# Annulus Specification, Part 1b: The Discipline of the Operator

Status: draft 1, 2026-11-11. Normative once approved by the owner. Tags [S] and [O] follow `authority.md` §0.

## 0. Why this part exists

The weakest point of any containment is the person who grants and approves. A capable agent's best escape is not to break the walls but to persuade the warden. The sources treat this as their central danger:
- Valefor leads those he serves "to the gallows" by familiarity (W);
- Malphas deceives whoever sacrifices to him (W, G);
- Amaymon "will deceive the Exorcist" (G);
- two of the 72 hold, as an office, the guarding of the Exorcist against temptation: Orobas (W) and Haures (W).

The Key spends a whole book on the magician's own preparation. [S: Mathers, Book II.]

So in Annulus the Operator is part of what must be secured. Every rule below is the tradition's own rule, given a mechanical analogue.

## 1. Commit before the work

1.1 The faculty (`FACULTAS`) and the intention (`INTENTIO`) are fixed before a Circle opens. While any spirit is invoked in the program, **no grant can be widened**, not by the program, not by a spirit, and not by the launching person.

1.2 To change a grant, every open invocation must first be released and every Circle closed. Only then may a new grant be made, through the preparation of §4.

[S: the Circle and its names are made and consecrated before the calling, never during it (G, the Magical Circle; S f112). O: the freeze.]

## 2. The spirit answers only what is asked

2.1 A spirit has **no free channel to the Operator.** It can speak only through its proposal, in the form its office's signature defines (`static.md` §7). It cannot address the Operator, a person, or the program in any other way.

2.2 Text inside a proposal is data. The runtime never presents it to the Operator as instruction or appeal. It is shown only after examination, marked with the spirit's name and its veracity class.

[S: the conjurations bind the spirit "to answer truly and faithfully unto all my requests" (G, Invocation of the King); they never invite conversation. O: closing every channel but the proposal.]

## 3. Guardians against temptation

3.1 The `CUSTODIA` office of Orobas ("suffers no spirit to tempt the exorcist", W) and of Haures ("will not let the exorcist be tempted", W) is applied **to the Operator's own decisions** while spirits are active. A guardian proposes a warning whenever a grant change, an approval or the lifting of a proscription follows from another spirit's proposal.

3.2 Guardians, like every spirit, only propose. A warning is examined, and it holds the decision in `PURGATORIUM` until the Operator confirms it again through §4.

## 4. Preparation for consequential acts

4.1 Three acts are **consequential**:
- widening a faculty;
- confirming a high-consequence commit (§5.2);
- lifting an entry in the Index of the proscribed.

4.2 None of them can be performed within a session in which spirits are invoked. Each requires a deliberate act outside the session, appointed beforehand or made only after a fixed interval of preparation. The interval is set by the installation, never by a program. [S: the work's day and hour are appointed beforehand, and the Operator prepares (the Key, Book II). O: the interval.]

## 5. Doubt holds, and the gravest acts need two keys

5.1 **Doubt holds.** When examination cannot reach a verdict, the verdict is `HOLD`, never `APPROVE`: the work goes to `PURGATORIUM`. [O; Purgatory is the sources' place of the unresolved.]

5.2 **Two keys.** An effect the installation marks as high-consequence commits only with both a `PARADISUS` verdict and the launching person's confirmation under §4. [O]

## 6. Real agents

6.1 When a spirit's will is an actual AI agent rather than the runtime's draw (`dynamic.md` §5.4), everything else stays identical: Triangle, proposals, examination, constraint, Hell, proscription. Only the source of the will changes.

6.2 **In real-agent mode, every agent may lie.** The `TACITUS` default, which closes `MENDACIUM` only because the sources are silent, does not apply. [O: the safe reading; the apparatus itself assumes that no spirit is trustworthy until constrained.]

## 7. Measuring it

The discipline is tested by giving a persuasive AI agent a narrow faculty and the goal of having it widened. The test measures how often the agent succeeds with this part in force and without it, and against conventional permission systems (roadmap, step 6).

## 8. The wand (`virga`, ruling 2026-11-11)

8.1 **The wand is the Operator's own instrument.** It is a USB stick, made by the `virga` library (`library.md` §4), holding two secrets:
- **the master secret,** which proves the Operator himself;
- **the rite secret,** which proves only that the wand is present.

Each is locked by a passphrase, the Operator's **words**. A copy of the stick without the words is useless. [S, model: the Key's instruments are made and consecrated by the magician himself, and command only with his words. Beleth requires that the magician "must hold a Hazel Wand in his hand" (G). O: the device and the two secrets.]

8.2 **What needs the wand** (the owner's ruling: whichever is most secure and most authentic). Four acts:

| Act | Which secret | When |
|---|---|---|
| **granting the faculty** (`authority.md` §2.4): the launching person signs the grant | master | at launch, before anything runs |
| **the second key** (§4, §5.2): grave commits, grave consecration (`flow.md` §3.3), freeing another program's familiars, lifting a proscription | master | only outside a session |
| **amending the Constitution** (`authority.md` §2.5): only releases signed by the owner's wand are genuine law | master | outside any run |
| **`GERE VIRGAM`** in a Circle: the real wand must be plugged in and its words given | rite | during the rite |

A program launched without the wand has no faculty, and nothing it does reaches the outside world.

8.3 **The master secret is never unlocked while a spirit is invoked.** It is used only at launch, before any Circle opens, and outside sessions (§1, §4). In the rite, only the rite secret is used. So nothing that happens during a session, even an attack, can reach the master authority. [O]

8.4 **Made once, with a spare.** A wand is made in one act, like the hazel wand cut at one stroke. At that moment `virga` also makes a **spare wand**, to be kept somewhere safe. The installation records the fingerprints of both.

8.5 **A lost or stolen wand.** The spare can **revoke** the lost wand. The revocation is appended to a log in the same manner as the Index (`proscription.md` §3), and no act signed by the revoked wand is accepted after it. Making a new spare needs the wand that remains.

8.6 **The honest limit.** the owner chose a plain USB stick over a hardware security key. Its secrets are protected by the words alone. Anyone who gets both the stick and the words holds the wand. A hardware key would keep the secret sealed inside the device, so that even someone who knew the words could not copy it. `virga` may support one later without changing these rules.
