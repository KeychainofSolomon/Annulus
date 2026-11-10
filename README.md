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
$ annulus examples/first_light.ars
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
annulus examples/first_light.ars
annulus --version
```

## Release binaries

CI builds and smoke-tests both platform binaries, each with a SHA-256 checksum:

- `annulus-windows-x86_64.exe`
- `annulus-linux-x86_64` (requires glibc 2.34 or newer)

## Documentation

| Document | Contents |
|---|---|
| [`docs/SPEC.md`](docs/SPEC.md) | The language exactly as implemented |
| [`docs/spec/authority.md`](docs/spec/authority.md) | Normative draft: the authority model (constitution, laws, rites, seals and ink, examination, judgement) |
| [`docs/spec/provenance.md`](docs/spec/provenance.md) | The source-precedence rule and the dated register of witnesses |
| [`docs/AUDIT.md`](docs/AUDIT.md) | Audit of the v0.2 prototype: what worked, what was broken, what must be redesigned |
| [`docs/STATUS.md`](docs/STATUS.md) | Implemented and unimplemented features |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | Milestones and open design decisions |
| [`CHANGELOG.md`](CHANGELOG.md) | Release history |
