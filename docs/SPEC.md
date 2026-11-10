# ARS MAGNA language specification — v0.2.1 (as implemented)

Status: **descriptive**. This document records exactly what the v0.2.1 implementation does. It is not the design of the full language; milestone 2 replaces it with a normative specification. Every rule here is covered by a test.

## 1. Source text

- A program is one UTF-8 text file. Invalid UTF-8 is rejected before lexing.
- Positions are reported as `line:column`, both starting at 1. Columns count Unicode scalar values; a tab counts as one column.
- Whitespace is any character for which Unicode's `White_Space` property holds. It separates tokens and is otherwise ignored. CRLF and LF line endings are both accepted.

## 2. Lexical grammar

```ebnf
token        = keyword | identifier | integer | string | operator | punctuation ;
keyword      = "LET" | "DECLARE" | "MANIFEST" | "PRINT" | "TRUE" | "FALSE" ;
identifier   = ( letter | "_" ) , { letter | digit | "_" } ;   (* not a keyword *)
letter       = "A".."Z" | "a".."z" ;
digit        = "0".."9" ;
integer      = digit , { digit } ;
string       = '"' , { any character except '"' } , '"' ;
operator     = "+" | "-" | "*" | "/" | "==" | "=" ;
punctuation  = "(" | ")" | ";" ;
```

- Keywords are case-sensitive. `let` is an identifier.
- `LET` and `DECLARE` are the same token; `MANIFEST` and `PRINT` are the same token.
- Identifiers are ASCII only.
- An integer literal's magnitude must fit in 64 unsigned bits at lexing time; its final range is checked by the parser (section 3).
- Strings have no escape sequences and may contain line breaks.
- There are no comments. Any other character, including NUL, is an error.
- `==` is matched before `=`.

## 3. Syntax

```ebnf
program      = { statement } ;
statement    = declaration | manifestation | evaluation ;
declaration  = ( "LET" | "DECLARE" ) , identifier , "=" , expression , ";" ;
manifestation = ( "MANIFEST" | "PRINT" ) , expression , ";" ;
evaluation   = expression , ";" ;

expression   = equality ;
equality     = additive , { "==" , additive } ;
additive     = multiplicative , { ( "+" | "-" ) , multiplicative } ;
multiplicative = unary , { ( "*" | "/" ) , unary } ;
unary        = "-" , integer | "-" , unary | primary ;
primary      = integer | string | "TRUE" | "FALSE" | identifier | "(" , expression , ")" ;
```

- All binary operators are left-associative.
- `"-" , integer` (a minus sign immediately followed by an integer literal) forms a single negative literal, so `-9223372036854775808` is valid. Any other integer literal above 9 223 372 036 854 775 807 is a syntax error.
- An expression may nest at most 256 levels deep, counting both parentheses and operator nesting. Deeper input is a syntax error, never a crash.

## 4. Values

| Type | Range | Printed as |
|---|---|---|
| integer | signed 64-bit, −2⁶³ to 2⁶³−1 | decimal, with `-` for negatives |
| boolean | `TRUE`, `FALSE` | `true`, `false` |
| string | any sequence of Unicode characters | the characters themselves, unquoted |

## 5. Semantics

Statements run top to bottom, once each.

- **Declaration** evaluates the expression and binds the name to the value. A binding is immutable and lives until the program ends. A name may be declared only once, and only names declared by earlier statements may be used, so a declaration cannot refer to itself.
- **Manifestation** evaluates the expression and writes its printed form followed by a line feed to standard output.
- **Evaluation** evaluates the expression and discards the value.

Operators:

| Operator | Operands | Result | Errors |
|---|---|---|---|
| `a + b`, `a - b`, `a * b` | integer, integer | integer | overflow |
| `a / b` | integer, integer | integer, truncated toward zero | division by zero; overflow (`-2⁶³ / -1`) |
| `-a` | integer | integer | overflow (`-(-2⁶³)`) |
| `a == b` | any, any | boolean; `false` when the types differ | none |

Operands are evaluated left to right.

## 6. Phases and errors

A program goes through four phases. Errors in the first three phases stop the program before any statement runs.

1. **Lexing**: characters to tokens.
2. **Parsing**: tokens to a syntax tree.
3. **Compilation**: names resolved to storage slots; the tree becomes instructions.
4. **Execution**: a stack machine runs the instructions.

Every error is reported on standard error as:

```text
<file>:<line>:<column>: <phase> error: <message>
  hint: <hint>            (only for some errors)
```

| Phase label | Raised by | Examples |
|---|---|---|
| `syntax error` | lexing, parsing | unexpected character, missing `;`, literal out of range, nesting too deep |
| `name error` | compilation | name not declared, name already declared |
| `runtime error` | execution | overflow, division by zero, wrong operand type |
| `internal error` | execution | a broken invariant in the implementation itself; always a bug |

Output already written before a runtime error is kept.

## 7. Command line

```text
ars-magna <source.ars>     run a program
ars-magna --version        print the version
ars-magna --help           print usage
```

| Exit status | Meaning |
|---|---|
| 0 | success, or output closed by the reader (for example `\| head`) |
| 1 | the program failed with a diagnostic, or the file could not be read |
| 2 | wrong command-line usage |

The interpreter runs on its own thread with a 16 MiB stack, so its limits are the same on every platform.

## 8. Platform notes

- Linux binary: requires glibc 2.34 or newer.
- Windows binary: statically linked C runtime; no redistributable needed.
