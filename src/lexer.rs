use std::iter::Peekable;
use std::str::Chars;

use crate::diagnostic::{Diagnostic, Phase};
use crate::span::Span;
use crate::token::{keyword, Token, TokenKind};

pub fn tokenize(source: &str) -> Result<Vec<Token>, Diagnostic> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token()?;
        let finished = token.kind == TokenKind::EndOfInput;
        tokens.push(token);
        if finished {
            return Ok(tokens);
        }
    }
}

struct Lexer<'source> {
    characters: Peekable<Chars<'source>>,
    line: usize,
    column: usize,
}

impl<'source> Lexer<'source> {
    fn new(source: &'source str) -> Self {
        Self {
            characters: source.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    fn position(&self) -> Span {
        Span::new(self.line, self.column)
    }

    fn peek(&mut self) -> Option<char> {
        self.characters.peek().copied()
    }

    fn bump(&mut self) -> Option<char> {
        let character = self.characters.next()?;
        if character == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(character)
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.bump();
        }
    }

    fn next_token(&mut self) -> Result<Token, Diagnostic> {
        self.skip_whitespace();
        let span = self.position();
        let Some(character) = self.bump() else {
            return Ok(Token {
                kind: TokenKind::EndOfInput,
                span,
            });
        };
        let kind = match character {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            ';' => TokenKind::Semicolon,
            '=' if self.peek() == Some('=') => {
                self.bump();
                TokenKind::EqualEqual
            }
            '=' => TokenKind::Assign,
            '"' => self.text(span)?,
            '0'..='9' => self.integer(character, span)?,
            first if first.is_ascii_alphabetic() || first == '_' => self.word(first),
            other => {
                return Err(Diagnostic::new(
                    Phase::Syntax,
                    format!("unexpected character {}", describe_character(other)),
                    span,
                ))
            }
        };
        Ok(Token { kind, span })
    }

    fn text(&mut self, opening: Span) -> Result<TokenKind, Diagnostic> {
        let mut contents = String::new();
        loop {
            match self.bump() {
                Some('"') => return Ok(TokenKind::Text(contents)),
                Some(character) => contents.push(character),
                None => {
                    return Err(Diagnostic::new(
                        Phase::Syntax,
                        "unterminated string literal",
                        opening,
                    ))
                }
            }
        }
    }

    fn integer(&mut self, first: char, span: Span) -> Result<TokenKind, Diagnostic> {
        let mut digits = String::from(first);
        while let Some(digit) = self.peek().filter(char::is_ascii_digit) {
            digits.push(digit);
            self.bump();
        }
        digits.parse::<u64>().map(TokenKind::Integer).map_err(|_| {
            Diagnostic::new(
                Phase::Syntax,
                format!("integer literal `{digits}` is out of range"),
                span,
            )
        })
    }

    fn word(&mut self, first: char) -> TokenKind {
        let mut word = String::from(first);
        while let Some(character) = self
            .peek()
            .filter(|character| character.is_ascii_alphanumeric() || *character == '_')
        {
            word.push(character);
            self.bump();
        }
        keyword(&word).unwrap_or(TokenKind::Identifier(word))
    }
}

fn describe_character(character: char) -> String {
    if character.is_control() {
        format!("U+{:04X}", u32::from(character))
    } else {
        format!("`{character}`")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(source: &str) -> Vec<TokenKind> {
        tokenize(source)
            .expect("source should tokenize")
            .into_iter()
            .map(|token| token.kind)
            .collect()
    }

    fn error(source: &str) -> Diagnostic {
        tokenize(source).expect_err("source should be rejected")
    }

    #[test]
    fn recognises_every_token_kind() {
        assert_eq!(
            kinds("LET DECLARE MANIFEST PRINT TRUE FALSE name_1 42 \"text\" + - * / = == ( ) ;"),
            vec![
                TokenKind::Declare,
                TokenKind::Declare,
                TokenKind::Manifest,
                TokenKind::Manifest,
                TokenKind::True,
                TokenKind::False,
                TokenKind::Identifier("name_1".into()),
                TokenKind::Integer(42),
                TokenKind::Text("text".into()),
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::Assign,
                TokenKind::EqualEqual,
                TokenKind::LeftParen,
                TokenKind::RightParen,
                TokenKind::Semicolon,
                TokenKind::EndOfInput,
            ]
        );
    }

    #[test]
    fn keywords_are_case_sensitive() {
        assert_eq!(
            kinds("let"),
            vec![TokenKind::Identifier("let".into()), TokenKind::EndOfInput]
        );
    }

    #[test]
    fn tracks_line_and_column() {
        let tokens = tokenize("LET\n  x").expect("source should tokenize");
        assert_eq!(tokens[0].span, Span::new(1, 1));
        assert_eq!(tokens[1].span, Span::new(2, 3));
        assert_eq!(tokens[2].span, Span::new(2, 4));
    }

    #[test]
    fn strings_may_span_lines() {
        assert_eq!(
            kinds("\"a\nb\""),
            vec![TokenKind::Text("a\nb".into()), TokenKind::EndOfInput]
        );
    }

    #[test]
    fn unterminated_string_points_at_opening_quote() {
        let diagnostic = error("PRINT 1;\nPRINT \"open;");
        assert_eq!(diagnostic.message, "unterminated string literal");
        assert_eq!(diagnostic.span, Span::new(2, 7));
    }

    #[test]
    fn null_character_is_rejected_rather_than_ending_input() {
        let diagnostic = error("PRINT 1;\0PRINT 2;");
        assert_eq!(diagnostic.message, "unexpected character U+0000");
        assert_eq!(diagnostic.span, Span::new(1, 9));
    }

    #[test]
    fn accepts_the_largest_unsigned_magnitude() {
        assert_eq!(
            kinds("18446744073709551615"),
            vec![TokenKind::Integer(u64::MAX), TokenKind::EndOfInput]
        );
    }

    #[test]
    fn rejects_integer_beyond_unsigned_range() {
        assert_eq!(
            error("18446744073709551616").message,
            "integer literal `18446744073709551616` is out of range"
        );
    }

    #[test]
    fn rejects_unknown_character() {
        let diagnostic = error("PRINT 1 # comment");
        assert_eq!(diagnostic.message, "unexpected character `#`");
        assert_eq!(diagnostic.span, Span::new(1, 9));
    }

    #[test]
    fn empty_source_yields_only_end_of_input() {
        assert_eq!(kinds("  \r\n\t"), vec![TokenKind::EndOfInput]);
    }
}
