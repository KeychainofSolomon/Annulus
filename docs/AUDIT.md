# Audit of the supplied v0.2 source

Date: 2026-11-10
Subject: `ars-magna-native-v0.2-source.zip` (6 files, 247 lines of Rust)
Toolchain used: Rust 1.97.0, Linux x86-64

The original v0.2 tree is preserved as the first commit of this repository, so every finding below can be checked against it.

## 1. Inventory

| File | Contents | Assessment |
|---|---|---|
| `src/main.rs` | Lexer, parser, AST, compiler, instruction set, stack VM and CLI in one file | Correct core, but no module boundaries, no positions in errors, several defects (section 3) |
| `Cargo.toml` | Binary crate, no dependencies, release profile with LTO, `panic = "abort"`, `strip` | Sound. No `Cargo.lock`, no pinned toolchain, no library target for testing |
| `.github/workflows/build.yml` | Builds release binaries on Windows and Linux runners | Builds only: no tests, lint or format check; unpinned `stable` toolchain; uploads with a glob that also catches build leftovers; runs on every push to every branch |
| `README.md` | Build, run, a one-paragraph language summary | Accurate for what it covers; does not state limitations |
| `examples/first_light.ars` | `7 * 5`, prints `35` | Works |
| `examples/logic.ars` | `42 == 42`, prints `true` | Works |

Safety review: no dependencies, no build scripts, no procedural macros, no `unsafe`, no network or file writes. The program reads exactly one file. It is safe to build and run.

## 2. What works

Verified by building the original and running 18 probe programs:

- Lexing of integers, strings, identifiers, the keywords `LET`/`DECLARE`, `MANIFEST`/`PRINT`, `TRUE`/`FALSE`, and the operators `+ - * / = == ( ) ;`.
- Precedence climbing with correct precedence (`==` < `+ -` < `* /`) and left associativity: `2 + 3 * 4` = 14, `10 - 3 - 2` = 5.
- Unary minus binding tighter than any binary operator: `-2 * 3` = -6.
- Checked 64-bit integer arithmetic: overflow and division by zero produce errors rather than wrong numbers or crashes, including `i64::MIN / -1`.
- Immutable bindings: a second declaration of the same name is refused.
- Both examples produce the documented output.

## 3. Defects found in v0.2

Severity: **High** = crash or corrupt behaviour on reachable input. **Medium** = wrong or misleading behaviour. **Low** = rough edge.

| # | Severity | Defect | Evidence | Status in v0.2.1 |
|---|---|---|---|---|
| D1 | High | Unbounded recursion: deeply nested parentheses or unary minus overflow the native stack and abort the process | 200 000 nested `(` → `fatal runtime error: stack overflow`, exit 134 | Fixed: expression depth limit (256) enforced during parsing; interpreter runs on a dedicated 16 MiB stack so the limit holds on every platform and build mode |
| D2 | High | Long operator chains build arbitrarily deep left-nested trees; compiling or dropping them recurses without bound | Derived from the structure of `compile_expr`; same crash class as D1 | Fixed: tree depth is tracked per node and bounded by the same limit |
| D3 | High | Writing to a closed output (for example piping into `head`) panics, and with `panic = "abort"` aborts with exit 134 | `ars-magna many.ars \| head -1` → panic message, exit 134 | Fixed: output errors are values; a closed pipe ends execution quietly with exit 0 |
| D4 | Medium | Undeclared names and redeclarations are detected only while running, after earlier statements have already produced output | `PRINT "before"; PRINT nope;` prints `before` then fails | Fixed: names are resolved at compile time; nothing runs if any name is wrong |
| D5 | Medium | Error messages carry no file, line or column, and show internal debug names | `let x = 5;` → `expected Semicolon, found Ident("x")` | Fixed: every diagnostic has `file:line:column`, a phase (`syntax`, `name`, `runtime`, `internal`), readable token names and, where useful, a hint |
| D6 | Medium | The NUL character is used as the end-of-input sentinel, so a literal NUL byte silently ends the program | `PRINT 1;\0PRINT 2;` prints only `1`, exit 0 | Fixed: end of input is represented explicitly; NUL is reported as `unexpected character U+0000` |
| D7 | Low | The most negative integer cannot be written, because the literal is lexed before the minus sign is applied | `PRINT -9223372036854775808;` → `integer out of range` | Fixed: a minus sign directly before an integer literal forms one negative literal |
| D8 | Low | An unterminated string is reported without a position | `PRINT "open;` → `unterminated string` | Fixed: reported at the opening quote |
| D9 | Low | The AST reuses lexer tokens as operators, which forces an `unreachable!()` in the compiler | `compile_expr` | Fixed: dedicated `BinaryOperator` and `UnaryOperator` types |
| D10 | Low | One compiler warning (`field 0 is never read` on `Expr::Unary`) | `cargo clippy` | Fixed: lint-clean under `clippy -D warnings` |
| D11 | Low | Global variable storage is a `HashMap` keyed by string, looked up on every access | `execute` | Fixed: names compile to numbered slots; the VM is a plain indexed array |

## 4. Behaviour kept as is, pending a language decision

These are not bugs, but they are undocumented choices that the formal specification must settle (milestone 2):

- `==` between values of different types yields `false` instead of an error: `1 == "1"` is `false`.
- Keywords are uppercase only; `let` is an ordinary identifier.
- Each statement keyword has two spellings (`LET`/`DECLARE`, `MANIFEST`/`PRINT`).
- Booleans are written `TRUE`/`FALSE` but printed `true`/`false`.
- Integer division truncates toward zero: `-7 / 2` = -3.
- Strings have no escape sequences and may span lines.
- There is no comment syntax.
- Type errors such as `"a" + "b"` are found at run time, not compile time.

## 5. Project and build issues

| Issue | Status in v0.2.1 |
|---|---|
| No automated tests | 50 tests: 39 unit, 11 integration (22 data-driven programs, the examples, CLI behaviour, stack-limit checks) |
| No `Cargo.lock` | Committed; all builds use `--locked` |
| Toolchain not pinned | `rust-toolchain.toml` pins 1.97.0; CI pins the same |
| Builds not reproducible or verified | Linux: two clean builds in different folders are byte-identical (checked locally and in CI). Windows: linker set to `/Brepro` for deterministic output; not verified, because no Windows toolchain is available here |
| CI uploads stray files | CI copies exactly one binary per platform under its final name, smoke-tests it and publishes a SHA-256 checksum |
| CI only builds | CI now runs format check, lint, debug and release tests on both platforms before packaging |
| Text-mode line-ending conversion on Windows checkouts would corrupt test data | `.gitattributes` pins test data to its exact bytes |
| No licence | Not addressed: the owner must choose a licence |

## 6. What must be redesigned before the handoff priorities can be built

The prototype is a sound starting point for the front end, but these parts cannot carry the target architecture and must be replaced rather than extended:

1. **Instruction set as a Rust enum only.** There is no serialised bytecode, no version, no constant pool, no jumps. Milestone 4 needs a defined binary format with a header, version, constant pool and verifier.
2. **Single flat namespace.** All names live in one global table. Functions, blocks, modules and circles all require lexical scopes and frames.
3. **No control flow in the VM.** The VM runs straight-line code; it has no jumps, calls, frames or return. It needs a call-frame design before functions or loops are possible.
4. **Effects are hard-wired.** Output is the only effect and goes straight to the output stream. The containment, examination and commit model (laws CONTAINMENT, EXAMINATION, RELEASE) requires every effect to pass through a journal that can be held, examined, committed or discarded. This must be designed into the VM early, not bolted on.
5. **No type system.** Types exist only at run time. A static checking pass belongs between parsing and code generation.
6. **No authority model.** Nothing in the runtime knows about identity, origin or permission. The capability design (milestone 6) must be part of the VM's core data model.
7. **Grammar is implicit.** The grammar exists only as parser code. Milestone 2 replaces it with a written grammar that the parser is tested against.

## 7. Summary

v0.2 is an honest, well-written expression calculator: about 250 lines covering a lexer, a Pratt-style parser, a small compiler and a stack VM. It is roughly 2 % of the language the handoff describes. Its front end is worth keeping and has been kept (restructured into modules, with all defects above fixed). Its VM, instruction set and effect handling must be redesigned before any of the Solomonic architecture can be implemented.
