use super::Error;

pub const KEYWORDS: &[&str] = &[
    "SIT",
    "MUTABILIS",
    "MANIFESTA",
    "SI",
    "ALITER",
    "DUM",
    "PER",
    "IN",
    "DESINE",
    "PERGE",
    "OPUS",
    "REDDE",
    "FORMA",
    "LIBER",
    "ADHIBE",
    "TENTA",
    "CAPE",
    "ET",
    "VEL",
    "NON",
    "VERUM",
    "FALSUM",
    "NIHIL",
    "NUMERUS",
    "VERITAS",
    "VERBUM",
    "SERIES",
    "FACULTAS",
    "CIRCULUS",
    "RITU",
    "INTENTIO",
    "INVOCA",
    "AD",
    "OFFICIUM",
    "PETE",
    "SIGILLUM",
    "INSCRIBE",
    "ATRAMENTO",
    "ATRAMENTUM",
    "CONSECRA",
    "EXAMINA",
    "RESPONSUM",
    "RATUM",
    "SUSPENSUM",
    "IRRITUM",
    "VADE",
    "PACE",
    "DIMITTE",
    "RELEGA",
    "NOMEN",
    "SPIRITUS",
    "ANGELUS",
    "GERE",
    "ANULUM",
    "VIRGAM",
    "HEXAGRAMMUM",
    "PENTAGRAMMUM",
    "OFFER",
    "CONSTRINGE",
    "APPELLA",
    "VETA",
    "REGEM",
    "HORA",
    "DIES",
    "ADVOCA",
    "TRIANGULUM",
    "VAS",
];

pub const RESERVED: &[&str] = &["HORA", "DIES", "ADVOCA", "TRIANGULUM", "VAS"];

const OPERATORS: &[&str] = &[
    "==", "!=", "<=", ">=", "->", "+", "-", "*", "/", "%", "=", "<", ">", ":",
];

const PUNCTUATION: &str = "(){}[],;.";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    pub line: u32,
    pub column: u32,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Keyword(&'static str),
    Identifier(String),
    Integer(i64),
    Text(String),
    Operator(&'static str),
    Punctuation(char),
    End,
}

impl Kind {
    pub fn shown(&self) -> String {
        match self {
            Kind::Keyword(k) => (*k).to_string(),
            Kind::Identifier(s) => s.clone(),
            Kind::Integer(n) => n.to_string(),
            Kind::Text(s) => format!("{s:?}"),
            Kind::Operator(o) => (*o).to_string(),
            Kind::Punctuation(c) => c.to_string(),
            Kind::End => "the end of the program".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: Kind,
    pub pos: Pos,
}

fn word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn lexical(pos: Pos, message: String) -> Error {
    Error::new("L1", pos, message)
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, Error> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut line = 1u32;
    let mut column = 1u32;
    let advance = |i: &mut usize, line: &mut u32, column: &mut u32, n: usize| {
        for _ in 0..n {
            if chars[*i] == '\n' {
                *line += 1;
                *column = 1;
            } else {
                *column += 1;
            }
            *i += 1;
        }
    };
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            advance(&mut i, &mut line, &mut column, 1);
            continue;
        }
        let pos = Pos { line, column };
        if word_char(c) {
            let mut j = i;
            while j < chars.len() && word_char(chars[j]) {
                j += 1;
            }
            let word: String = chars[i..j].iter().collect();
            let kind = classify(&word, pos)?;
            tokens.push(Token { kind, pos });
            let n = j - i;
            advance(&mut i, &mut line, &mut column, n);
            continue;
        }
        if c == '"' {
            let (text, end) = string(&chars, i, pos)?;
            tokens.push(Token {
                kind: Kind::Text(text),
                pos,
            });
            let n = end - i;
            advance(&mut i, &mut line, &mut column, n);
            continue;
        }
        if let Some(op) = OPERATORS.iter().find(|op| {
            op.chars()
                .enumerate()
                .all(|(k, oc)| chars.get(i + k) == Some(&oc))
        }) {
            tokens.push(Token {
                kind: Kind::Operator(op),
                pos,
            });
            advance(&mut i, &mut line, &mut column, op.len());
            continue;
        }
        if PUNCTUATION.contains(c) {
            tokens.push(Token {
                kind: Kind::Punctuation(c),
                pos,
            });
            advance(&mut i, &mut line, &mut column, 1);
            continue;
        }
        return Err(lexical(pos, format!("unexpected character {c:?}")));
    }
    tokens.push(Token {
        kind: Kind::End,
        pos: Pos { line, column },
    });
    Ok(tokens)
}

fn classify(word: &str, pos: Pos) -> Result<Kind, Error> {
    let first = word.chars().next().unwrap_or('_');
    if word.chars().all(|c| c.is_ascii_digit()) {
        return word
            .parse::<i64>()
            .map(Kind::Integer)
            .map_err(|_| lexical(pos, format!("integer {word} is too large")));
    }
    if first.is_ascii_uppercase() && word.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
        let Some(keyword) = KEYWORDS.iter().find(|k| **k == word) else {
            return Err(lexical(pos, format!("unknown keyword {word}")));
        };
        if RESERVED.contains(keyword) {
            return Err(lexical(pos, format!("reserved keyword {word}")));
        }
        return Ok(Kind::Keyword(keyword));
    }
    if first.is_ascii_lowercase()
        && word
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return Ok(Kind::Identifier(word.to_string()));
    }
    Err(lexical(pos, format!("malformed word {word}")))
}

fn string(chars: &[char], start: usize, pos: Pos) -> Result<(String, usize), Error> {
    let mut text = String::new();
    let mut j = start + 1;
    loop {
        match chars.get(j) {
            None | Some('\n') => return Err(lexical(pos, "unterminated string".to_string())),
            Some('"') => return Ok((text, j + 1)),
            Some('\\') => match chars.get(j + 1) {
                Some('"') => {
                    text.push('"');
                    j += 2;
                }
                Some('\\') => {
                    text.push('\\');
                    j += 2;
                }
                Some('n') => {
                    text.push('\n');
                    j += 2;
                }
                Some('t') => {
                    text.push('\t');
                    j += 2;
                }
                Some('u') if chars.get(j + 2) == Some(&'{') => {
                    let mut k = j + 3;
                    while chars.get(k).is_some_and(|c| c.is_ascii_hexdigit()) {
                        k += 1;
                    }
                    if k == j + 3 || k > j + 9 || chars.get(k) != Some(&'}') {
                        return Err(lexical(pos, "bad unicode escape".to_string()));
                    }
                    let digits: String = chars[j + 3..k].iter().collect();
                    let value = u32::from_str_radix(&digits, 16).unwrap_or(u32::MAX);
                    let Some(c) = char::from_u32(value) else {
                        return Err(lexical(
                            pos,
                            "escape is not a Unicode scalar value".to_string(),
                        ));
                    };
                    text.push(c);
                    j = k + 1;
                }
                _ => return Err(lexical(pos, "bad escape".to_string())),
            },
            Some(c) => {
                text.push(*c);
                j += 1;
            }
        }
    }
}
