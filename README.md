# ARS MAGNA

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
$ ars-magna examples/first_light.ars
35
```

## Build

Requires Rust 1.97.0. With `rustup` installed, the pinned version is selected automatically from `rust-toolchain.toml`.

```sh
cargo build --release --locked
```

The binary is `target/release/ars-magna` (Linux) or `target\release\ars-magna.exe` (Windows).

## Test

```sh
cargo test --locked
```

## Run

```sh
ars-magna examples/first_light.ars
ars-magna --version
```

## Release binaries

CI builds and smoke-tests both platform binaries, each with a SHA-256 checksum:

- `ars-magna-windows-x86_64.exe`
- `ars-magna-linux-x86_64` (requires glibc 2.34 or newer)

## Documentation

| Document | Contents |
|---|---|
| [`docs/SPEC.md`](docs/SPEC.md) | The language exactly as implemented |
| [`docs/AUDIT.md`](docs/AUDIT.md) | Audit of the v0.2 prototype: what worked, what was broken, what must be redesigned |
| [`docs/STATUS.md`](docs/STATUS.md) | Implemented and unimplemented features |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | Milestones and open design decisions |
| [`CHANGELOG.md`](CHANGELOG.md) | Release history |
