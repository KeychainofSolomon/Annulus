use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Text(String),
    List(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

impl Json {
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(map) => map.get(key),
            _ => None,
        }
    }

    pub fn text(&self) -> Option<&str> {
        match self {
            Json::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn list(&self) -> &[Json] {
        match self {
            Json::List(items) => items,
            _ => &[],
        }
    }

    pub fn number(&self) -> Option<f64> {
        match self {
            Json::Number(n) => Some(*n),
            _ => None,
        }
    }
}

pub fn parse(source: &str) -> Result<Json, String> {
    let chars: Vec<char> = source.chars().collect();
    let mut reader = Reader { chars, at: 0 };
    let value = reader.value()?;
    reader.space();
    if reader.at != reader.chars.len() {
        return Err(format!("trailing text at {}", reader.at));
    }
    Ok(value)
}

struct Reader {
    chars: Vec<char>,
    at: usize,
}

impl Reader {
    fn space(&mut self) {
        while self.chars.get(self.at).is_some_and(|c| c.is_whitespace()) {
            self.at += 1;
        }
    }

    fn eat(&mut self, c: char) -> Result<(), String> {
        self.space();
        if self.chars.get(self.at) == Some(&c) {
            self.at += 1;
            Ok(())
        } else {
            Err(format!("expected {c:?} at {}", self.at))
        }
    }

    fn word(&mut self, word: &str, value: Json) -> Result<Json, String> {
        for c in word.chars() {
            if self.chars.get(self.at) != Some(&c) {
                return Err(format!("expected {word} at {}", self.at));
            }
            self.at += 1;
        }
        Ok(value)
    }

    fn value(&mut self) -> Result<Json, String> {
        self.space();
        match self.chars.get(self.at) {
            Some('{') => {
                self.at += 1;
                let mut map = BTreeMap::new();
                self.space();
                if self.chars.get(self.at) == Some(&'}') {
                    self.at += 1;
                    return Ok(Json::Object(map));
                }
                loop {
                    self.space();
                    let key = self.string()?;
                    self.eat(':')?;
                    let value = self.value()?;
                    map.insert(key, value);
                    self.space();
                    match self.chars.get(self.at) {
                        Some(',') => self.at += 1,
                        Some('}') => {
                            self.at += 1;
                            return Ok(Json::Object(map));
                        }
                        _ => return Err(format!("bad object at {}", self.at)),
                    }
                }
            }
            Some('[') => {
                self.at += 1;
                let mut items = Vec::new();
                self.space();
                if self.chars.get(self.at) == Some(&']') {
                    self.at += 1;
                    return Ok(Json::List(items));
                }
                loop {
                    items.push(self.value()?);
                    self.space();
                    match self.chars.get(self.at) {
                        Some(',') => self.at += 1,
                        Some(']') => {
                            self.at += 1;
                            return Ok(Json::List(items));
                        }
                        _ => return Err(format!("bad list at {}", self.at)),
                    }
                }
            }
            Some('"') => Ok(Json::Text(self.string()?)),
            Some('t') => self.word("true", Json::Bool(true)),
            Some('f') => self.word("false", Json::Bool(false)),
            Some('n') => self.word("null", Json::Null),
            Some(c) if *c == '-' || c.is_ascii_digit() => {
                let start = self.at;
                while self
                    .chars
                    .get(self.at)
                    .is_some_and(|c| c.is_ascii_digit() || "+-.eE".contains(*c))
                {
                    self.at += 1;
                }
                let text: String = self.chars[start..self.at].iter().collect();
                text.parse()
                    .map(Json::Number)
                    .map_err(|_| format!("bad number at {start}"))
            }
            _ => Err(format!("unexpected text at {}", self.at)),
        }
    }

    fn hex4(&mut self) -> Result<u32, String> {
        let digits: String = self.chars.iter().skip(self.at).take(4).collect();
        self.at += 4;
        u32::from_str_radix(&digits, 16).map_err(|_| format!("bad escape at {}", self.at))
    }

    fn string(&mut self) -> Result<String, String> {
        if self.chars.get(self.at) != Some(&'"') {
            return Err(format!("expected a string at {}", self.at));
        }
        self.at += 1;
        let mut out = String::new();
        loop {
            let Some(&c) = self.chars.get(self.at) else {
                return Err("unterminated string".to_string());
            };
            self.at += 1;
            match c {
                '"' => return Ok(out),
                '\\' => {
                    let Some(&e) = self.chars.get(self.at) else {
                        return Err("unterminated escape".to_string());
                    };
                    self.at += 1;
                    match e {
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        '/' => out.push('/'),
                        'b' => out.push('\u{8}'),
                        'f' => out.push('\u{c}'),
                        'n' => out.push('\n'),
                        'r' => out.push('\r'),
                        't' => out.push('\t'),
                        'u' => {
                            let high = self.hex4()?;
                            let code = if (0xD800..0xDC00).contains(&high) {
                                if self.chars.get(self.at) != Some(&'\\')
                                    || self.chars.get(self.at + 1) != Some(&'u')
                                {
                                    return Err("lone surrogate".to_string());
                                }
                                self.at += 2;
                                let low = self.hex4()?;
                                0x10000 + ((high - 0xD800) << 10) + (low.wrapping_sub(0xDC00))
                            } else {
                                high
                            };
                            out.push(char::from_u32(code).ok_or("bad code point")?);
                        }
                        _ => return Err(format!("bad escape at {}", self.at)),
                    }
                }
                c => out.push(c),
            }
        }
    }
}
