# Changelog

## Unreleased

### Amended (approved by the owner, 2026-11-11)
- **Entries are declined by case** (`grammar.md` §3.2, §4): `INVOCA SPIRITUM 34` (accusative), `SIGILLUM SPIRITUS 34` / `SIGILLUM ANGELI 34` (genitive), `SPIRITUS 34` alone (nominative), as `GERE ANULUM` and `INVOCA REGEM` already were. Reason: an outside review noted `INVOCA SPIRITUS` is not Latin.
- **The axiom** reads *nulla potestas a se ipsa oritur*: *sibi ipsi* meant "for itself", not "from itself".
- **Proof, until the proofs exist** (`authority.md` §2.5): the Lean model updated and building, every theorem check and attack test passing, and an independent audit.
- **`TEMPESTAS` is outside the step-2 subset** until the storm (fault injection) exists; it used to commit an unchanged write, a false record.
- **A spirit's name belongs to the spirit** (`static.md` §2.1, error N5). No program name may be spelled as any label of a register entry, in any recorded spelling, even if the program never refers to that entry. Reason: §2.1 allowed `SIT furfur = INVOCA furfur …` while N5 forbade declaring a register label; the owner chose N5, which keeps `authority.md` §2.3 (no program shadows a Constitution entry). The grammar example and five grammar test programs now name the invocation `vocatus`.

### Fixed
- **A hole in the quarantine check**, found by an outside review: loop bodies were checked twice, so a chain of three or more assignments could carry a spirit's words into a loop condition. Loops are now checked until no mark changes.

### Added
- A second guard at run time: a condition still marked as a spirit's words is refused (class 2), behind the checker.
- CI compares the Rust parser with `tools/grammar_check.py` program by program, to the line and column (`tests/grammar/positions.json`).
- Step 2b front end, `src/lingua/`: Latin lexer, parser for the whole grammar, the goetia register read from its JSON, and the static checker for the step-2 subset.

## 0.2.1 — 2026-11-10

Milestone 1: audit and reproducible build. No language features added; see `docs/AUDIT.md` for every finding.

### Fixed
- Deeply nested expressions crashed the process with a stack overflow; they are now rejected with a syntax error (limit 256 levels).
- Writing to a closed output (for example `| head`) aborted with a panic; it now ends quietly.
- Undeclared names and redeclarations are now found before the program runs, not partway through.
- A NUL byte in the source silently ended the program; it is now an error.
- `-9223372036854775808` can now be written.
- Error messages now include file, line and column, readable token names and hints.

### Changed
- Source split into modules: `lexer`, `parser`, `ast`, `compiler`, `program`, `vm`, `value`, `diagnostic`, `span`, `token`, plus a library target used by the tests.
- Names compile to numbered storage slots instead of a string-keyed table.
- The interpreter runs on its own 16 MiB stack, so limits are identical on Windows and Linux.

### Added
- `--version` and `--help`; defined exit codes (0, 1, 2).
- 50 automated tests.
- `Cargo.lock`, pinned toolchain (`rust-toolchain.toml`, Rust 1.97.0), deterministic Windows linking.
- CI: format check, lint, debug and release tests on both platforms, a reproducibility check, and packaging with checksums.
- Documentation: `docs/AUDIT.md`, `docs/SPEC.md`, `docs/STATUS.md`, `docs/ROADMAP.md`.

## 0.2.0

Supplied prototype.
