# ARS MAGNA

Native Rust implementation of the ARS MAGNA language.

## Build

Requires Rust stable.

```sh
cargo build --release
```

## Run

```sh
cargo run --release -- examples/first_light.ars
```

## Language

```ars
LET x = 7;
LET y = 5;
LET result = x * y;
MANIFEST result;
```

`LET` and `DECLARE` introduce immutable bindings. `MANIFEST` and `PRINT` write values to standard output. Expressions support integer arithmetic, equality, booleans, strings, and parentheses.

## Binary targets

- `x86_64-pc-windows-msvc`
- `x86_64-unknown-linux-gnu`

The build workflow produces both native binaries on their respective runners.
