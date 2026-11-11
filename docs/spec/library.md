# Annulus Specification, Part 2b: The Standard Library and its Grants

Draft 1, 2026-11-11, from the owner's rulings of the same day. Roadmap step 1, item 6. Layer: core for grants and the powerbox; rite for which art each library serves.

## 1. No ambient authority

Every library operation that touches the outside world takes its authority from a named grant in the faculty. Nothing is global: no "current directory", no open network, no clock that a spirit can reach. A library function with no grant behind it can only compute. [O. Older parallel: object-capability systems (Capsicum on FreeBSD, 2010; WASI's capability handles, 2019 onwards), where a program holds only the handles it was given.]

## 2. The powerbox: you choose the files (ruling 2026-11-11)

2.1 **Slots.** A program may name a **slot** instead of a file:

    FACULTAS {
        locus "chronicon" legere scribere;
    }

`"chronicon"` is the slot's name. Everywhere a file name could be written (in a grant or as a place in `PETE`), the slot's name can be written instead. Slot names are constants, so the type checker treats a slot like a file (`offices.md` §1.2, §3.2).

2.2 **The choice is the grant.** At launch, the runtime opens its own trusted file picker, one for each slot. The launching person chooses the real file. That choice **is** the grant, with exactly the permissions the slot lists. The binding is recorded in the audit and cannot change during the run. A slot left unfilled is refused, like any part of the faculty that was not granted (`authority.md` §2.4). The program sees the slot's name, never the real path.

2.3 **Why this keeps the guarantee.** The type checker still knows every place before the program runs, because it knows the slots. Only the launching person, never the program and never a spirit, decides which real file each slot means. [S, model: only the Operator's own hand prepares the work. Older parallel: the "powerbox" of the E language and of Sandstorm (2000s): the user's pick in a trusted dialog is the permission. It never caught on widely.] It replaces the "groups of files" idea in `offices.md` §1.2.

## 3. The domain libraries and their certificates

3.1 **Proof-carrying answers** (ruling 2026-11-11). A computing office (`offices.md` §2.2) MUST attach a **certificate** (`testimonium`) to its result. The domain library's checker verifies it, which is cheap however hard the work was. With no certificate, or one the library cannot check, the result is `HOLD`. With a certificate that fails, it is `REJECT`. With a valid one, it is `APPROVE`. The checker's run is the core's step `consecrate(v, k)`; that step, not the verdict, consecrates the result (`flow.md` §3.4). [S, model: a spirit is compelled "to answer truly". The angel tests the answer, not the spirit's word. Older parallel: proof-carrying code (Necula, 1996), which never took off, revived in 2024–25 by AI theorem provers whose proofs a small checker verifies.]

3.2 **What the certificates are.** Each library defines the certificate it can check, for example:
- **arithmetic:** the result with a check that can be redone quickly (a factorisation, say);
- **parsing:** the parse tree, which the checker turns back into the input;
- **translation between lossless formats:** a round trip;
- **dates and ephemerides:** the computed positions, which the checker recomputes from tables;
- **proofs:** a proof term for a small proof checker.

Where no cheap certificate exists, the library recomputes instead. Where it can do neither, the office's results are always held.

3.3 **Which art each office serves** is a rite field in the register (`offices.md` §5.1).

## 4. The libraries to build (approved by the owner, 2026-11-11)

In the order they are needed:
1. **`horae`, the hours:** sunrise, sunset, temporal and planetary hours, and the Moon's age at the declared place. Every time window needs it.
2. **`registra`, the registers:** loading the registers, applying the witness precedence (`provenance.md`), and citing the deciding source.
3. **`calendarium`, the calendar:** Julian and Gregorian conversion, and the *computus* for Easter, feasts and fasts.
4. **`probatio`, the proof checker:** a tiny trusted checker in the manner of Metamath (1990s), small enough to trust completely, for `DOCTRINA` certificates. Then a checker for each art: parsing, formats, arithmetic.
5. **`nomina`, names and numbers:** gematria, the variant spellings of divine names, Hebrew, Greek and Latin transliteration.
6. **`figurae`, squares and seals:** the planetary magic squares, sigils built from them, the Sigillum Dei and the pentacles, and checking an operative seal against its exemplar.
7. **`acta`, the Operator's tools:** reading an audit in plain language, replaying a run, and checking the Index.
8. **`virga`, the wand:** making the wand and its spare on a USB stick, signing with it, and revoking a lost one (`operator.md` §8).
9. **`agens`, the agent bridge** (roadmap step 4): a real AI as a spirit, with its seal fingerprinted (`proscription.md` §2.1).

**Outside code.** Libraries from elsewhere may be used for pure computation. Anything that touches the outside world MUST be wrapped so that it gets its authority only through the grant (§1), or rewritten. [Object-capability practice calls this "taming".] The names above are [O]: working names.

## 5. The network

Deferred. When added, each host is a grant of its own (`rete "host" …`), and nothing reaches the network without one.
