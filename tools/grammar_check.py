import pathlib
import sys

KEYWORDS = {
    "SIT", "MUTABILIS", "MANIFESTA", "SI", "ALITER", "DUM", "PER", "IN", "DESINE", "PERGE",
    "OPUS", "REDDE", "FORMA", "LIBER", "ADHIBE", "TENTA", "CAPE", "ET", "VEL", "NON",
    "VERUM", "FALSUM", "NIHIL", "NUMERUS", "VERITAS", "VERBUM", "SERIES",
    "FACULTAS", "CIRCULUS", "RITU", "INTENTIO", "INVOCA", "AD", "OFFICIUM", "PETE",
    "SIGILLUM", "INSCRIBE", "ATRAMENTO", "ATRAMENTUM", "CONSECRA", "EXAMINA",
    "PARADISUS", "PURGATORIUM", "INFERNUS", "VADE", "PACE", "DIMITTE", "RELEGA",
    "NOMEN", "SPIRITUS", "ANGELUS",
    "GERE", "ANULUM", "VIRGAM", "HEXAGRAMMUM", "PENTAGRAMMUM", "OFFER", "CONSTRINGE", "APPELLA", "VETA", "REGEM",
    "HORA", "DIES", "ADVOCA", "TRIANGULUM", "VAS",
}
RESERVED = {"HORA", "DIES", "ADVOCA", "TRIANGULUM", "VAS"}
OPERATORS = ["==", "!=", "<=", ">=", "->", "+", "-", "*", "/", "%", "=", "<", ">", ":"]
PUNCTUATION = set("(){}[],;.")
COMPARISONS = {"==", "!=", "<", "<=", ">", ">="}
LITERALS = {"VERUM", "FALSUM", "NIHIL", "PARADISUS", "PURGATORIUM", "INFERNUS"}
BASE_TYPES = {"NUMERUS", "VERITAS", "VERBUM", "NIHIL", "ATRAMENTUM", "SIGILLUM", "SPIRITUS", "ANGELUS"}
MAX_INTEGER = 2**63 - 1


class GrammarError(Exception):
    pass


def is_word_char(c):
    return c.isascii() and (c.isalnum() or c == "_")


def lex(text):
    tokens = []
    i = 0
    line, column = 1, 1

    def advance(n):
        nonlocal i, line, column
        for _ in range(n):
            if text[i] == "\n":
                line, column = line + 1, 1
            else:
                column += 1
            i += 1

    while i < len(text):
        c = text[i]
        if c.isspace():
            advance(1)
            continue
        at = (line, column)
        if is_word_char(c):
            j = i
            while j < len(text) and is_word_char(text[j]):
                j += 1
            word = text[i:j]
            if word.isdigit():
                if int(word) > MAX_INTEGER:
                    raise GrammarError(f"{at}: integer too large")
                kind = "integer"
            elif all(ch.isupper() or ch == "_" for ch in word) and word[0].isupper():
                if word not in KEYWORDS:
                    raise GrammarError(f"{at}: unknown keyword {word}")
                if word in RESERVED:
                    raise GrammarError(f"{at}: reserved keyword {word}")
                kind = "keyword"
            elif word[0].islower() and all(ch.islower() or ch.isdigit() or ch == "_" for ch in word):
                kind = "identifier"
            else:
                raise GrammarError(f"{at}: malformed word {word}")
            tokens.append((kind, word, at))
            advance(j - i)
            continue
        if c == '"':
            j = i + 1
            while True:
                if j >= len(text) or text[j] == "\n":
                    raise GrammarError(f"{at}: unterminated string")
                if text[j] == '"':
                    break
                if text[j] == "\\":
                    nxt = text[j + 1] if j + 1 < len(text) else ""
                    if nxt in ('"', "\\", "n", "t"):
                        j += 2
                        continue
                    if nxt == "u" and text[j + 2:j + 3] == "{":
                        k = j + 3
                        while k < len(text) and text[k] in "0123456789abcdefABCDEF":
                            k += 1
                        if k == j + 3 or k > j + 9 or k >= len(text) or text[k] != "}":
                            raise GrammarError(f"{at}: bad unicode escape")
                        value = int(text[j + 3:k], 16)
                        if value > 0x10FFFF or 0xD800 <= value <= 0xDFFF:
                            raise GrammarError(f"{at}: escape is not a Unicode scalar value")
                        j = k + 1
                        continue
                    raise GrammarError(f"{at}: bad escape")
                j += 1
            tokens.append(("string", text[i:j + 1], at))
            advance(j + 1 - i)
            continue
        for op in OPERATORS:
            if text.startswith(op, i):
                tokens.append(("operator", op, at))
                advance(len(op))
                break
        else:
            if c in PUNCTUATION:
                tokens.append(("punctuation", c, at))
                advance(1)
            else:
                raise GrammarError(f"{at}: unexpected character {c!r}")
    tokens.append(("end", "", (line, column)))
    return tokens


class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos = 0

    def peek(self, offset=0):
        return self.tokens[min(self.pos + offset, len(self.tokens) - 1)]

    def at(self, value):
        kind, text, _ = self.peek()
        return text == value and kind in ("keyword", "operator", "punctuation")

    def at_kind(self, kind):
        return self.peek()[0] == kind

    def expect(self, value):
        if not self.at(value):
            self.fail(f"expected {value}")
        self.pos += 1

    def expect_kind(self, kind):
        if not self.at_kind(kind):
            self.fail(f"expected {kind}")
        self.pos += 1

    def accept(self, value):
        if self.at(value):
            self.pos += 1
            return True
        return False

    def fail(self, message):
        kind, text, where = self.peek()
        raise GrammarError(f"{where}: {message}, found {text or kind}")

    def program(self):
        if self.at("FACULTAS"):
            self.faculty()
        while not self.at_kind("end"):
            self.item()

    def faculty(self):
        self.expect("FACULTAS")
        self.expect("{")
        while not self.at("}"):
            self.expect_kind("identifier")
            self.expect_kind("string")
            while self.at_kind("identifier"):
                self.pos += 1
            if self.at_kind("integer"):
                self.pos += 1
            self.expect(";")
        self.expect("}")

    def item(self):
        if self.at("OPUS"):
            self.function()
        elif self.at("FORMA"):
            self.form()
        elif self.at("LIBER") or self.at("ADHIBE"):
            self.pos += 1
            self.path()
            self.expect(";")
        else:
            self.statement()

    def path(self):
        self.expect_kind("identifier")
        while self.accept("."):
            self.expect_kind("identifier")

    def form(self):
        self.expect("FORMA")
        self.expect_kind("identifier")
        self.expect("{")
        if not self.at("}"):
            self.typed_name()
            while self.accept(","):
                self.typed_name()
        self.expect("}")

    def typed_name(self):
        self.expect_kind("identifier")
        self.expect(":")
        self.type()

    def function(self):
        self.expect("OPUS")
        self.expect_kind("identifier")
        self.expect("(")
        if not self.at(")"):
            self.typed_name()
            while self.accept(","):
                self.typed_name()
        self.expect(")")
        if self.accept("->"):
            self.type()
        self.block()

    def type(self):
        kind, text, _ = self.peek()
        if kind == "keyword" and text in BASE_TYPES:
            self.pos += 1
        elif self.accept("SERIES"):
            self.expect("[")
            self.type()
            self.expect("]")
        elif kind == "identifier":
            self.pos += 1
        else:
            self.fail("expected a type")

    def block(self):
        self.expect("{")
        while not self.at("}"):
            if self.at_kind("end"):
                self.fail("unclosed block")
            self.statement()
        self.expect("}")

    def statement(self):
        if self.accept("SIT"):
            self.accept("MUTABILIS")
            self.expect_kind("identifier")
            if self.accept(":"):
                self.type()
            self.expect("=")
            self.expression()
            self.expect(";")
        elif self.accept("MANIFESTA"):
            self.expression()
            self.expect(";")
        elif self.at("SI"):
            self.conditional()
        elif self.accept("DUM"):
            self.expression()
            self.block()
        elif self.accept("PER"):
            self.expect_kind("identifier")
            self.expect("IN")
            self.expression()
            self.block()
        elif self.accept("DESINE") or self.accept("PERGE"):
            self.expect(";")
        elif self.accept("REDDE"):
            if not self.at(";"):
                self.expression()
            self.expect(";")
        elif self.accept("TENTA"):
            self.block()
            self.expect("CAPE")
            self.expect_kind("identifier")
            self.block()
        elif self.at("CIRCULUS"):
            self.circle()
        elif self.accept("VADE"):
            self.expect("IN")
            self.expect("PACE")
            self.expression()
            self.expect(";")
        elif self.accept("GERE"):
            if not (self.accept("ANULUM") or self.accept("VIRGAM") or self.accept("HEXAGRAMMUM") or self.accept("PENTAGRAMMUM")):
                self.fail("expected an instrument")
            self.expect(";")
        elif self.accept("OFFER") or self.accept("APPELLA") or self.accept("VETA"):
            self.expression()
            self.expect(";")
        elif self.accept("CONSTRINGE"):
            self.expression()
            self.expect("PER")
            self.expect("NOMEN")
            self.expect_kind("string")
            self.expect(";")
        elif self.accept("DIMITTE") or self.accept("RELEGA"):
            self.expression()
            self.expect(";")
        else:
            is_place = self.expression()
            if self.accept("="):
                if not is_place:
                    self.fail("left side of = is not a place")
                self.expression()
            self.expect(";")

    def conditional(self):
        self.expect("SI")
        self.expression()
        self.block()
        if self.accept("ALITER"):
            if self.at("SI"):
                self.conditional()
            else:
                self.block()

    def circle(self):
        self.expect("CIRCULUS")
        self.expect("RITU")
        self.expect_kind("identifier")
        self.expect("{")
        self.expect("INTENTIO")
        self.expect_kind("string")
        self.expect(";")
        while not self.at("}"):
            if self.at_kind("end"):
                self.fail("unclosed circle")
            self.statement()
        self.expect("}")

    def expression(self):
        place = self.conjunction()
        while self.accept("VEL"):
            self.conjunction()
            place = False
        return place

    def conjunction(self):
        place = self.comparison()
        while self.accept("ET"):
            self.comparison()
            place = False
        return place

    def comparison(self):
        place = self.sum()
        kind, text, _ = self.peek()
        if kind == "operator" and text in COMPARISONS:
            self.pos += 1
            self.sum()
            place = False
            kind, text, _ = self.peek()
            if kind == "operator" and text in COMPARISONS:
                self.fail("comparisons do not chain")
        return place

    def sum(self):
        place = self.product()
        while self.at("+") or self.at("-"):
            self.pos += 1
            self.product()
            place = False
        return place

    def product(self):
        place = self.unary()
        while self.at("*") or self.at("/") or self.at("%"):
            self.pos += 1
            self.unary()
            place = False
        return place

    def unary(self):
        if self.at("-") or self.at("NON") or self.at("CONSECRA") or self.at("EXAMINA"):
            self.pos += 1
            self.unary()
            return False
        if self.accept("INSCRIBE"):
            self.unary()
            self.expect("ATRAMENTO")
            self.unary()
            return False
        return self.postfix()

    def postfix(self):
        place = self.primary()
        while True:
            if self.accept("("):
                self.arguments_until(")")
                place = False
            elif self.accept("."):
                self.expect_kind("identifier")
            elif self.accept("["):
                self.expression()
                self.expect("]")
            else:
                return place

    def arguments_until(self, closer):
        if not self.at(closer):
            self.expression()
            while self.accept(","):
                self.expression()
        self.expect(closer)

    def primary(self):
        kind, text, _ = self.peek()
        if kind in ("integer", "string"):
            self.pos += 1
            return False
        if kind == "identifier":
            self.pos += 1
            return True
        if kind == "keyword" and text in LITERALS:
            self.pos += 1
            return False
        if self.accept("("):
            self.expression()
            self.expect(")")
            return False
        if self.accept("["):
            self.arguments_until("]")
            return False
        if self.accept("INVOCA"):
            if self.accept("REGEM"):
                self.expect_kind("identifier")
                return False
            self.entity()
            self.expect("AD")
            self.expect("OFFICIUM")
            self.expect_kind("identifier")
            return False
        if self.accept("PETE"):
            self.expect_kind("identifier")
            self.expect("(")
            self.arguments_until(")")
            return False
        if self.accept("SIGILLUM"):
            self.entity()
            return False
        if self.accept("ATRAMENTUM"):
            self.expect_kind("string")
            return False
        if self.accept("NOMEN"):
            self.expect_kind("string")
            return False
        if self.at("SPIRITUS") or self.at("ANGELUS"):
            self.entity()
            return False
        self.fail("expected an expression")

    def entity(self):
        if self.at("SPIRITUS") or self.at("ANGELUS"):
            self.pos += 1
            self.expect_kind("integer")
        else:
            self.expect_kind("identifier")


def check(text):
    parser = Parser(lex(text))
    parser.program()


def main(directory):
    root = pathlib.Path(directory)
    failures = 0
    total = 0
    for expected in ("accept", "reject"):
        for path in sorted((root / expected).glob("*.ann")):
            total += 1
            try:
                check(path.read_text(encoding="utf-8"))
                outcome, detail = "accept", ""
            except GrammarError as error:
                outcome, detail = "reject", str(error)
            if outcome != expected:
                failures += 1
                print(f"FAIL {expected}/{path.name}: got {outcome} {detail}")
            elif expected == "reject":
                print(f"ok   reject/{path.name}: {detail}")
            else:
                print(f"ok   accept/{path.name}")
    print(f"{total - failures} of {total} as expected")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else "tests/grammar"))
