# Annulus

A programming language with a Solomonic cosmological and computational foundation, implemented natively in Rust.

**Current state (v0.2.1):** a small, tested expression language with immutable bindings and output. The full language described in the engineering handoff is not built yet; see [`docs/STATUS.md`](docs/STATUS.md) for exactly what exists and [`docs/ROADMAP.md`](docs/ROADMAP.md) for the plan.

## Example

```ars
LET x = 7;
LET y = 5;
LET result = x * y;
MANIFEST result;
```

```text
$ annulus examples/first_light.annulus
35
```

## Build

Requires Rust 1.97.0. With `rustup` installed, the pinned version is selected automatically from `rust-toolchain.toml`.

```sh
cargo build --release --locked
```

The binary is `target/release/annulus` (Linux) or `target\release\annulus.exe` (Windows).

## Test

```sh
cargo test --locked
```

## Run

```sh
annulus examples/first_light.annulus
annulus --version
```

## Release binaries

CI builds and smoke-tests both platform binaries, each with a SHA-256 checksum:

- `annulus-windows-x86_64.exe`
- `annulus-linux-x86_64` (requires glibc 2.34 or newer)

## Documentation

| Document | Contents |
|---|---|
| [`docs/PROTOTYPE.md`](docs/PROTOTYPE.md) | The old v0.2.1 prototype, exactly as implemented. **Not** the specification |
| [`docs/spec/core.md`](docs/spec/core.md) | Part 0: the proven core, the rite layer, and the interface between them (draft 1) |
| [`formal/README.md`](formal/README.md) | The core's formal model in Lean 4: §9 and P1–P9 stated; P1, P2, P4, P6 (b), P7, P8 and the damnation law proved; checked in CI (draft 1) |
| [`docs/spec/authority.md`](docs/spec/authority.md) | Normative draft: the authority model (constitution, laws, rites, seals and ink, examination, judgement) |
| [`docs/spec/provenance.md`](docs/spec/provenance.md) | The source-precedence rule and the dated register of witnesses |
| [`docs/spec/grammar.md`](docs/spec/grammar.md) | Lexical grammar, the Latin keyword table and the EBNF syntax (draft 1) |
| [`docs/spec/spirits.md`](docs/spec/spirits.md) | The spirits: offices as computer work, will, malice, counterbalances and wards (draft 1) |
| [`docs/spec/operator.md`](docs/spec/operator.md) | The discipline of the Operator (Part 1b, draft 1) |
| [`docs/spec/offices.md`](docs/spec/offices.md) | Office signatures: what each office is handed and gives back (Part 1c, draft 1) |
| [`docs/registers/goetia.md`](docs/registers/goetia.md) | The register of the 72 spirits, generated from `goetia.json` |
| [`docs/registers/nomina.md`](docs/registers/nomina.md) | Divine and sacred names from the Circle, figures and conjuration of Sloane 3825 |
| [`docs/registers/reges.md`](docs/registers/reges.md) | The four kings of the quarters, per rite |
| [`docs/spec/operator.md`](docs/spec/operator.md) | The discipline of the Operator: securing the person who grants and approves (draft 1) |
| [`docs/spec/flow.md`](docs/spec/flow.md) | Information flow: the quarantine of spirits' words, consecration by gravity (draft 1) |
| [`docs/spec/concurrency.md`](docs/spec/concurrency.md) | Concurrency: each Triangle an actor; only the Operator sends; deterministic simulation (draft 1) |
| [`docs/spec/library.md`](docs/spec/library.md) | Standard library: no ambient authority, the powerbox, proof-carrying answers (draft 1) |
| [`docs/spec/proscription.md`](docs/spec/proscription.md) | The Index of the proscribed: identity by seal then scope, an append-only log (draft 1) |
| [`docs/spec/static.md`](docs/spec/static.md) | Static semantics: names, types and the compile-time authority checks (draft 1) |
| [`docs/spec/dynamic.md`](docs/spec/dynamic.md) | Dynamic semantics: evaluation, failures, ink, the will at runtime, examination, release, audit (draft 1) |
| [`docs/AUDIT.md`](docs/AUDIT.md) | Audit of the v0.2 prototype: what worked, what was broken, what must be redesigned |
| [`docs/STATUS.md`](docs/STATUS.md) | Implemented and unimplemented features |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | Milestones and open design decisions |
| [`CHANGELOG.md`](CHANGELOG.md) | Release history |

## Licence

Public domain (see `UNLICENSE`). Anyone may use, change or sell this code for any purpose; no credit is needed.
