# Annulus Specification, Part 2: Lexical Grammar and Syntax

Status: draft 1, 2026-11-10. Normative once approved by the owner. Ruling of 2026-11-10: **all keywords are Latin.**

Provenance tags follow `authority.md` §0:
- **[S]** marks a keyword attested in the sources;
- **[O]** marks a Latin word chosen for Annulus.

## 1. Source text

- **Encoding:** a program is one UTF-8 file, with the extension `.ars`. Invalid UTF-8 is rejected before lexing.
- **Line endings:** LF and CRLF are both accepted.
- **Whitespace:** Unicode `White_Space` separates tokens and is otherwise ignored.
- **Positions:** reported as `line:column` (1-based, counted in Unicode scalar values).

## 2. Lexical grammar

```ebnf
token       = keyword | identifier | integer | string | operator | punctuation ;
comment     = "//" , { any character except line break } ;      (* ignored *)
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

**Casing is a rule, not a convention:**
- All-uppercase words are keywords.
- Lowercase words are identifiers.
- An uppercase word not listed in §3 is a lexical error.

  [O] This echoes the manuscripts, which set divine names and rubrics apart in capitals.

**Strings:**
- Strings may contain any Unicode text, including Hebrew. Divine names and angel names are written in their own script (§5.4).
- `\u{…}` escapes take a hexadecimal code point.

**Integers:**
- Integer literals are decimal and must fit the integer type (§6).
- Roman numerals are not literals.

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
| `ATRAMENTUM` | "ink" | the ink type | [S] |
| `CONSECRA` | "consecrate" | lifts taint after independent validation (§7.4) | [S] the consecration of the ink (*Liber iuratus* CXXXVII) |
| `EXAMINA` | "examine" | submits a proposal to its fixed examiner, then to judgement | [S] the examination; [O] the mechanism |
| `PARADISUS` / `PURGATORIUM` / `INFERNUS` | the three destinations | values of a judgement (`authority.md` §11) | [O] the mapping; the names follow the Architecture |
| `VADE IN PACE` | "go in peace" | licence to depart | [S] Arbatel Aph. XXI, "vade in pace ad ordines tuos" |
| `DIMITTE` | "send away" | dismiss | [O] |
| `RELEGA` | "banish" | banish; results go to the Vessel | [O] word; [S] act: the fire curse (BnF 1524 p215) |
| `NOMEN` | "name" | references a divine name in the Constitution | [S] binding "by" the names |
| `SPIRITUS` / `ANGELUS` | "spirit" / "angel" | register types; also refer to an entry by number, e.g. `SPIRITUS 34` | [O] |

**Reserved for later parts:** `HORA` (hour), `DIES` (day), `ADVOCA` (call an angel through its channel), `TRIANGULUM` (a spirit-specific containment figure), `VAS` (the Vessel).

## 4. Syntax

```ebnf
program        = [ faculty ] , { item } ;
faculty        = "FACULTAS" , "{" , { grant } , "}" ;
grant          = identifier , string , { identifier } , ";" ;   (* e.g. liber "data.txt" legere; *)
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
statement      = declaration | assignment | output | conditional | loop | iteration
               | "DESINE" , ";" | "PERGE" , ";" | return | attempt | circle
               | release | expression , ";" ;
declaration    = "SIT" , [ "MUTABILIS" ] , identifier , [ ":" , type ] , "=" , expression , ";" ;
assignment     = place , "=" , expression , ";" ;
place          = identifier , { "." , identifier | "[" , expression , "]" } ;
output         = "MANIFESTA" , expression , ";" ;
conditional    = "SI" , expression , block , [ "ALITER" , ( conditional | block ) ] ;
loop           = "DUM" , expression , block ;
iteration      = "PER" , identifier , "IN" , expression , block ;
return         = "REDDE" , [ expression ] , ";" ;
attempt        = "TENTA" , block , "CAPE" , identifier , block ;

circle         = "CIRCULUS" , "RITU" , identifier , "{" , intent , { statement } , "}" ;
intent         = "INTENTIO" , string , ";" ;
release        = ( "VADE" , "IN" , "PACE" | "DIMITTE" | "RELEGA" ) , expression , ";" ;

expression     = disjunction ;
disjunction    = conjunction , { "VEL" , conjunction } ;
conjunction    = comparison , { "ET" , comparison } ;
comparison     = sum , [ ( "==" | "!=" | "<" | "<=" | ">" | ">=" ) , sum ] ;
sum            = product , { ( "+" | "-" ) , product } ;
product        = unary , { ( "*" | "/" | "%" ) , unary } ;
unary          = ( "-" | "NON" ) , unary | postfix ;
postfix        = primary , { "(" , [ arguments ] , ")" | "." , identifier | "[" , expression , "]" } ;
arguments      = expression , { "," , expression } ;
primary        = integer | string | "VERUM" | "FALSUM" | "NIHIL"
               | "PARADISUS" | "PURGATORIUM" | "INFERNUS"
               | identifier | "(" , expression , ")" | sequence | rite_expression ;
sequence       = "[" , [ arguments ] , "]" ;

rite_expression = invocation | request | inscription | consecration | examination
                | seal | name | entry ;
invocation     = "INVOCA" , entity , "AD" , "OFFICIUM" , identifier ;
request        = "PETE" , expression , "(" , [ arguments ] , ")" ;
inscription    = "INSCRIBE" , expression , "ATRAMENTO" , expression ;
consecration   = "CONSECRA" , expression ;
examination    = "EXAMINA" , expression ;
seal           = "SIGILLUM" , entity ;
name           = "NOMEN" , string ;
entity         = identifier | entry ;
entry          = ( "SPIRITUS" | "ANGELUS" ) , integer ;
```

**Notes:**
- Comparison does not chain: `a < b < c` is a syntax error.
- The precedence, from loosest to tightest binding: `VEL`, `ET`, comparison, `+ -`, `* / %`, unary `- NON`, postfix.
- `CIRCULUS` blocks may nest. An inner Circle MUST NOT widen its outer Circle's permissions (static semantics, Part 3).
- Rite expressions are valid only inside a `CIRCULUS`. That check is static, not syntactic.

## 5. Static meaning of the rite syntax (summary)

Full detail comes in Part 3. In summary:

1. **`entity`** resolves through the Constitution's registers, by number or by label, within the Circle's rite. An unknown name is a compile-time error. `furfur` and `SPIRITUS 34` are the same entity.
2. **`AD OFFICIUM` office** must be one of the spirit's offices in that rite. A spirit asked for anything else is a compile-time error (law OFFICE).
3. **`EXAMINA p`** sends `p` to the examiner fixed by the Constitution (Rudd's angel *n* for spirit *n* in `rudd`). It yields a destination. There is no syntax for choosing an examiner.
4. **`NOMEN "…"`** must match a divine name entry exactly, in its own script, for example `NOMEN "אגלא"`.

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
}

SIT septem = 7;
SIT MUTABILIS summa = 0;
PER n IN [1, 2, 3] {
    summa = summa + n * septem;
}
MANIFESTA summa;

CIRCULUS RITU goetia {
    INTENTIO "record the coming storm";
    SIT furfur = INVOCA SPIRITUS 34 AD OFFICIUM tempestas;
    SIT sigillum = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA atramentum_licitum;
    SIT propositum = PETE furfur("tempestas.txt");
    SIT iudicium = EXAMINA propositum;
    SI iudicium == PARADISUS {
        VADE IN PACE furfur;
    } ALITER {
        RELEGA furfur;
    }
}
```

**Open in this example:** the actual offices of each spirit, written as computational jobs (Furfur's office here is a placeholder), and the way ink is drawn from the faculty (`atramentum_licitum`). Both belong to Part 3.

## 8. Migration from v0.2.1

| Old (v0.2.1) | New |
|---|---|
| `LET`, `DECLARE` | `SIT` |
| `MANIFEST`, `PRINT` | `MANIFESTA` |
| `TRUE`, `FALSE` | `VERUM`, `FALSUM` |

The prototype's test programs will be rewritten when the parser implements this grammar (roadmap step 6).
