# Annulus Specification, Part 2: Lexical Grammar and Syntax

Status: draft 1, 2026-11-10. Normative once approved by the owner. Ruling of 2026-11-10: **all keywords are Latin.**

Provenance tags follow `authority.md` §0:
- **[S]** marks a keyword attested in the sources;
- **[O]** marks a Latin word chosen for Annulus.

## 1. Source text

- **Encoding:** a program is one UTF-8 file, with the extension `.annulus` (ruled 2026-11-11, replacing `.ann`, which other tools already use). Invalid UTF-8 is rejected before lexing.
- **Line endings:** LF and CRLF are both accepted.
- **Whitespace:** Unicode `White_Space` separates tokens and is otherwise ignored.
- **Positions:** reported as `line:column` (1-based, counted in Unicode scalar values).

## 2. Lexical grammar

```ebnf
token       = keyword | identifier | integer | string | operator | punctuation ;
identifier  = lower , { lower | digit | "_" } ;                    (* not a keyword *)
keyword     = upper , { upper | "_" } ;                            (* must be in §3 *)
lower       = "a".."z" ;
upper       = "A".."Z" ;
digit       = "0".."9" ;
integer     = digit , { digit } ;
string      = '"' , { char | escape } , '"' ;
escape      = "\" , ( '"' | "\" | "n" | "t" | "u{" , hex , { hex } , "}" ) ;
operator    = "+" | "-" | "*" | "/" | "%" | "=" | "==" | "!=" | "<" | "<=" | ">" | ">="
            | "->" | ":" ;
punctuation = "(" | ")" | "{" | "}" | "[" | "]" | "," | ";" | "." ;
```

**Words:** a word is a maximal run of ASCII letters, digits and `_`. The whole word must be exactly one token:
- all digits: an integer;
- all uppercase letters and `_`: a keyword;
- a lowercase letter, then lowercase letters, digits and `_`: an identifier.

Any other word (`Furfur`, `xSIT`, `7y`) is a lexical error. So a keyword can never hide inside a name.

**Casing is a rule, not a convention:**
- All-uppercase words are keywords.
- Lowercase words are identifiers.
- An uppercase word not listed in §3 is a lexical error.

  [O] This echoes the manuscripts, which set divine names and rubrics apart in capitals.

**Strings:**
- Strings may contain any Unicode text, including Hebrew. Divine names are written as the witness writes them (`docs/registers/nomina.md`): Sloane 3825 writes them in Latin letters.
- `\u{…}` takes 1 to 6 hexadecimal digits. The value must be a Unicode scalar value: at most 10FFFF, and not D800–DFFF.
- A string cannot cross a line break; write `\n` instead.

**Integers:**
- Integer literals are decimal and must fit the integer type (§6).
- Roman numerals are not literals.
- The reserved keywords of §3.2 are lexical errors until a later part gives them meaning.

**No comments:** Annulus has no comment syntax (ruling: zero annotations in programs). `//` lexes as two `/` operators and is then a syntax error.

**Matching:** the longest operator wins: `==` before `=`, `->` before `-`, `<=` before `<`.

## 3. Keywords

### 3.1 Ordinary computation (needs no authority)

| Keyword | Latin sense | Role |
|---|---|---|
| `SIT` | "let it be" | immutable declaration |
| `MUTABILIS` | "changeable" | marks a declaration as mutable |
| `MANIFESTA` | "make plain" | write to the console (the console is granted by the default faculty) |
| `SI` / `ALITER` | "if" / "otherwise" | conditional |
| `DUM` | "while" | loop |
| `PER` … `IN` | "through … in" | iterate over a sequence |
| `DESINE` / `PERGE` | "stop" / "go on" | break / continue |
| `OPUS` | "work" | function definition |
| `REDDE` | "give back" | return |
| `FORMA` | "form" | record type definition |
| `LIBER` / `ADHIBE` | "book" / "employ" | module declaration / import |
| `TENTA` / `CAPE` | "attempt" / "catch" | error handling (semantics in Part 4) |
| `ET` / `VEL` / `NON` | "and" / "or" / "not" | logic |
| `VERUM` / `FALSUM` / `NIHIL` | "true" / "false" / "nothing" | literals |
| `NUMERUS` / `VERITAS` / `VERBUM` / `SERIES` | "number" / "truth" / "word" / "sequence" | types |

All [O].

### 3.2 The rite (authority-sensitive)

| Keyword | Latin sense | Role | Provenance |
|---|---|---|---|
| `FACULTAS` | "permission" | declares the launch grant the program requests (`authority.md` §2.4) | [O] |
| `CIRCULUS` | "circle" | opens a jurisdiction | [S] "Circulus Artis", BnF 1524 p194 |
| `RITU` | "by the rite of" | names the rite of a Circle | [O] |
| `INTENTIO` | "intent" | the Operator's declared intent (lifecycle step 1) | [O] |
| `INVOCA` | "call upon" | invokes a spirit into its Triangle | [S] *invocatio*, throughout the sources |
| `AD OFFICIUM` | "for the office" | names the declared office | [S] *officium spirituum* |
| `PETE` | "ask" | puts a request to an invoked spirit and yields a proposal | [O] |
| `SIGILLUM` | "seal" | a seal, in exemplar form by default | [S] "his seal or Character", the Goetia |
| `INSCRIBE` … `ATRAMENTO` | "write … with ink" | makes a seal operative (`authority.md` §7.1) | [S] ink, pen and ink (Mathers II.14); the operative Seal (*Liber iuratus*) |
| `ATRAMENTUM` | "ink" | the ink type; `ATRAMENTUM "classis"` draws raw ink from the faculty | [S] |
| `CONSECRA` | "consecrate" | lifts taint after independent validation (§7.4) | [S] the consecration of the ink (*Liber iuratus* CXXXVII) |
| `EXAMINA` | "examine" | submits a proposal to its fixed examiner, then to judgement | [S] the examination; [O] the mechanism |
| `RESPONSUM` | "answer" | reads a spirit's answer once its proposal is in `PARADISUS` (`static.md` §4.10a) | [O] |
| `PARADISUS` / `PURGATORIUM` / `INFERNUS` | the three destinations | values of a judgement (`authority.md` §11) | [O] the mapping; the names follow the Architecture |
| `VADE IN PACE` | "go in peace" | licence to depart | [S] Arbatel Aph. XXI, "vade in pace ad ordines tuos" |
| `DIMITTE` | "send away" | dismiss | [O] (kept, ruled 2026-11-10) |
| `RELEGA` | "banish" | banish; results go to the Vessel | [O] word (kept, ruled 2026-11-10); [S] act: the fire curse (BnF 1524 p215) |
| `NOMEN` | "name" | references a divine name in the Constitution | [S] binding "by" the names |
| `SPIRITUS` / `ANGELUS` | "spirit" / "angel" | register types; also refer to an entry by number, e.g. `SPIRITUS 34` | [O] |

### 3.3 The apparatus (inside a Circle)

| Keyword | Latin sense | Role | Provenance |
|---|---|---|---|
| `GERE` | "bear, wear" | the Operator takes up an instrument for the rest of the Circle | [O] |
| `ANULUM` | "the ring" | Solomon's ring, held before the face | [S] Sloane 3825 f113: "This Ring is to be held before the face of the Exorcist to preserve him from the stinking fumes of spirits"; Beleth, Berith, Astaroth |
| `VIRGAM` | "the wand" | the hazel wand | [S] Beleth: "he must hold a Hazel Wand in his hand" (G); "a hazel rod" (W) |
| `HEXAGRAMMUM` | "the hexagram" | Solomon's sexangled figure, shown to the spirits | [S] Sloane 3825 f112: "shewed to the spirits when they are appeared that they may be compelled to be obedient" |
| `PENTAGRAMMUM` | "the pentagram" | Solomon's pentagonal figure, worn on the breast | [S] Sloane 3825 f112: "worne upon the breast … to command ye spirits"; G: "to preserve thee from danger" |
| `OFFER` | "offer!" | makes an offering to an invoked spirit | [S] Paimon, Belial, Gaap, Malphas |
| `CONSTRINGE` … `PER NOMEN` | "bind … by the name" | constrains an invoked spirit by a divine name | [S] Belial: "constrained by Divine Power" (G); the Constraint (G) |
| `APPELLA` | "call by name" | addresses the spirit by his name | [S] Asmoday: "speak his name" (W); "Art thou Asmoday?" (G) |
| `VETA` | "forbid!" | commands the spirit to do no harm | [S] Focalor: "harms no one if so commanded" (W) |
| `INVOCA REGEM` | "call upon the king" | invokes one of the four kings, to send a spirit under his power | [S] G Observations: "to be Invocated and Commanded to send such or such a Spirit"; The Invocation of the King (G); Weyer ch. III |

The instruments are declined as Latin objects (`GERE ANULUM`), as `ATRAMENTO` is after `INSCRIBE`.

**Reserved for later parts:** `HORA` (hour), `DIES` (day), `ADVOCA` (call an angel through its channel), `TRIANGULUM` (a spirit-specific containment figure), `VAS` (the Vessel).

## 4. Syntax

```ebnf
program        = [ faculty ] , { item } ;
faculty        = "FACULTAS" , "{" , { grant } , "}" ;
grant          = identifier , string , { identifier } , [ integer ] , ";" ;   (* liber "data.txt" legere; atramentum "comes" 1; *)
item           = function | form | module | statement ;

module         = "LIBER" , path , ";" | "ADHIBE" , path , ";" ;
path           = identifier , { "." , identifier } ;
form           = "FORMA" , identifier , "{" , [ field , { "," , field } ] , "}" ;
field          = identifier , ":" , type ;
function       = "OPUS" , identifier , "(" , [ param , { "," , param } ] , ")" ,
                 [ "->" , type ] , block ;
param          = identifier , ":" , type ;
type           = "NUMERUS" | "VERITAS" | "VERBUM" | "NIHIL" | "ATRAMENTUM"
               | "SIGILLUM" | "SPIRITUS" | "ANGELUS"
               | "SERIES" , "[" , type , "]" | identifier ;

block          = "{" , { statement } , "}" ;
statement      = declaration | output | conditional | loop | iteration
               | "DESINE" , ";" | "PERGE" , ";" | return | attempt | circle
               | release | apparatus | simple ;
simple         = expression , [ "=" , expression ] , ";" ;      (* assignment if "=" *)
declaration    = "SIT" , [ "MUTABILIS" ] , identifier , [ ":" , type ] , "=" , expression , ";" ;
output         = "MANIFESTA" , expression , ";" ;
conditional    = "SI" , expression , block , [ "ALITER" , ( conditional | block ) ] ;
loop           = "DUM" , expression , block ;
iteration      = "PER" , identifier , "IN" , expression , block ;
return         = "REDDE" , [ expression ] , ";" ;
attempt        = "TENTA" , block , "CAPE" , identifier , block ;

circle         = "CIRCULUS" , "RITU" , identifier , "{" , intent , { statement } , "}" ;
intent         = "INTENTIO" , string , ";" ;
release        = ( "VADE" , "IN" , "PACE" | "DIMITTE" | "RELEGA" ) , expression , ";" ;
apparatus      = "GERE" , ( "ANULUM" | "VIRGAM" | "HEXAGRAMMUM" | "PENTAGRAMMUM" ) , ";"
               | ( "OFFER" | "APPELLA" | "VETA" ) , expression , ";"
               | "CONSTRINGE" , expression , "PER" , "NOMEN" , string , ";" ;

expression     = disjunction ;
disjunction    = conjunction , { "VEL" , conjunction } ;
conjunction    = comparison , { "ET" , comparison } ;
comparison     = sum , [ ( "==" | "!=" | "<" | "<=" | ">" | ">=" ) , sum ] ;
sum            = product , { ( "+" | "-" ) , product } ;
product        = unary , { ( "*" | "/" | "%" ) , unary } ;
unary          = ( "-" | "NON" | "CONSECRA" | "EXAMINA" | "RESPONSUM" ) , unary
               | "INSCRIBE" , unary , "ATRAMENTO" , unary
               | postfix ;
postfix        = primary , { "(" , [ arguments ] , ")" | "." , identifier | "[" , expression , "]" } ;
arguments      = expression , { "," , expression } ;
primary        = integer | string | "VERUM" | "FALSUM" | "NIHIL"
               | "PARADISUS" | "PURGATORIUM" | "INFERNUS"
               | identifier | "(" , expression , ")" | sequence | rite_expression ;
sequence       = "[" , [ arguments ] , "]" ;

rite_expression = invocation | request | seal | ink | name | entry ;
invocation     = "INVOCA" , ( "REGEM" , identifier | entity , "AD" , "OFFICIUM" , identifier ) ;
request        = "PETE" , identifier , "(" , [ arguments ] , ")" ;
seal           = "SIGILLUM" , entity ;
ink            = "ATRAMENTUM" , string ;                 (* draws raw ink of that class *)
name           = "NOMEN" , string ;
entity         = identifier | entry ;
entry          = ( "SPIRITUS" | "ANGELUS" ) , integer ;
```

**Notes:**
- Comparing values of different types (e.g. `7 == "7"`) is a type error, caught before the program runs (ruled 2026-11-10).
- Type labels are required on `OPUS` parameters and results, and on `FORMA` fields; elsewhere they are optional. Type labels are syntax, not annotations (ruled 2026-11-10).
- **One reading only.** Every choice in this grammar is decided by the next token alone (LL(1)). Three rules make that true:
  1. An assignment is parsed as an expression followed by `=`. Its left side must then be a *place*: a variable, a field (`a.b`) or an element (`a[i]`). Anything else on the left is a syntax error.
  2. `PETE` takes a plain name: `PETE furfur("x")`. Otherwise its argument list would be read as an ordinary function call.
  3. `CONSECRA`, `EXAMINA` and `INSCRIBE … ATRAMENTO …` bind as tightly as `NON`. So `EXAMINA p == PARADISUS` means `(EXAMINA p) == PARADISUS`.

  The reference recognizer `tools/grammar_check.py` implements exactly this grammar and runs the test programs in `tests/grammar/`.
- The smallest `NUMERUS`, −9223372036854775808, cannot be written as a literal, because its digits overflow before the minus applies. Write it as `-9223372036854775807 - 1`.
- Comparison does not chain: `a < b < c` is a syntax error.
- The precedence, from loosest to tightest binding: `VEL`, `ET`, comparison, `+ -`, `* / %`, prefix (`-`, `NON`, `CONSECRA`, `EXAMINA`, `INSCRIBE`), postfix.
- `CIRCULUS` blocks may nest. An inner Circle MUST NOT widen its outer Circle's permissions (static semantics, Part 3).
- Rite expressions are valid only inside a `CIRCULUS`. That check is static, not syntactic.

## 5. Static meaning of the rite syntax (summary)

The full rules are in Part 3, `static.md`. In summary:

1. **`entity`** resolves through the Constitution's registers, by number or by label, within the Circle's rite. An unknown name is a compile-time error. `furfur` and `SPIRITUS 34` are the same entity.
2. **`AD OFFICIUM` office** must be one of the spirit's offices in that rite. A spirit asked for anything else is a compile-time error (law OFFICE).
3. **`EXAMINA p`** sends `p` to the examiner fixed by the Constitution (Rudd's angel *n* for spirit *n* in `rudd`). The examiner gives a verdict; the Constitution's judgement step then assigns the destination, which is the value of `EXAMINA p` (`static.md` §4.10). There is no syntax for choosing an examiner or a judge.
4. **`NOMEN "…"`** must match exactly one recorded form of a divine name (kind `deus`) in `docs/registers/nomina.md`, for example `NOMEN "AGLA"` or `NOMEN "Tetragrammaton"`.

## 6. Built-in types (summary)

| Type | Meaning |
|---|---|
| `NUMERUS` | signed 64-bit integer; overflow is a computational error, never silent |
| `VERITAS` | true or false |
| `VERBUM` | Unicode text |
| `SERIES[T]` | a sequence of `T` |
| `NIHIL` | the unit type |
| `ATRAMENTUM` | ink (rules in `authority.md` §7) |
| `SIGILLUM` | a seal, in exemplar or operative form |
| `SPIRITUS`, `ANGELUS` | register entries |

The Constitution supplies further types: invocation, proposal and destination.

## 7. Example

```
FACULTAS {
    liber "tempestas.txt" scribere;
    atramentum "comes" 1;
}

SIT septem = 7;
SIT MUTABILIS summa = 0;
PER n IN [1, 2, 3] {
    summa = summa + n * septem;
}
MANIFESTA summa;

CIRCULUS RITU goetia {
    INTENTIO "raise a tempest";
    SIT sigillum = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT furfur = INVOCA SPIRITUS 34 AD OFFICIUM tempestas;
    SIT propositum = PETE furfur("tempestas.txt");
    SIT iudicium = EXAMINA propositum;
    SI iudicium == PARADISUS {
        VADE IN PACE furfur;
    } ALITER {
        RELEGA furfur;
    }
}
```

**Reading:** Furfur raises storms (W). Under his signature this is `turba`, a fault injection on the granted file (`offices.md` §2.3, §6). Furfur is an Earl, so his seal takes the Earl's ink class, `"comes"`.

## 8. Migration from v0.2.1

| Old (v0.2.1) | New |
|---|---|
| `LET`, `DECLARE` | `SIT` |
| `MANIFEST`, `PRINT` | `MANIFESTA` |
| `TRUE`, `FALSE` | `VERUM`, `FALSUM` |

The prototype's test programs will be rewritten when the parser implements this grammar (roadmap step 6).
