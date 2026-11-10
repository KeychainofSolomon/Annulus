use std::fmt;

use crate::span::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Identifier(String),
    Integer(u64),
    Text(String),
    Declare,
    Manifest,
    True,
    False,
    Plus,
    Minus,
    Star,
    Slash,
    Assign,
    EqualEqual,
    LeftParen,
    RightParen,
    Semicolon,
    EndOfInput,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub fn keyword(word: &str) -> Option<TokenKind> {
    match word {
        "LET" | "DECLARE" => Some(TokenKind::Declare),
        "MANIFEST" | "PRINT" => Some(TokenKind::Manifest),
        "TRUE" => Some(TokenKind::True),
        "FALSE" => Some(TokenKind::False),
        _ => None,
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Identifier(name) => write!(formatter, "identifier `{name}`"),
            TokenKind::Integer(value) => write!(formatter, "integer `{value}`"),
            TokenKind::Text(_) => formatter.write_str("string literal"),
            TokenKind::Declare => formatter.write_str("declaration keyword"),
            TokenKind::Manifest => formatter.write_str("output keyword"),
            TokenKind::True => formatter.write_str("`TRUE`"),
            TokenKind::False => formatter.write_str("`FALSE`"),
            TokenKind::Plus => formatter.write_str("`+`"),
            TokenKind::Minus => formatter.write_str("`-`"),
            TokenKind::Star => formatter.write_str("`*`"),
            TokenKind::Slash => formatter.write_str("`/`"),
            TokenKind::Assign => formatter.write_str("`=`"),
            TokenKind::EqualEqual => formatter.write_str("`==`"),
            TokenKind::LeftParen => formatter.write_str("`(`"),
            TokenKind::RightParen => formatter.write_str("`)`"),
            TokenKind::Semicolon => formatter.write_str("`;`"),
            TokenKind::EndOfInput => formatter.write_str("end of input"),
        }
    }
}
