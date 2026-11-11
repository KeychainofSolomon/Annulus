# Changelog

## Unreleased

### Amended (approved by the owner, 2026-11-11)
- **A spirit's name belongs to the spirit** (`static.md` §2.1, error N5). No program name may be spelled as any label of a register entry, in any recorded spelling, even if the program never refers to that entry. Reason: §2.1 allowed `SIT furfur = INVOCA furfur …` while N5 forbade declaring a register label; the owner chose N5, which keeps `authority.md` §2.3 (no program shadows a Constitution entry). The grammar example and five grammar test programs now name the invocation `vocatus`.

### Added
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
