use core::fmt;

use crate::common::Span;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenType {
    Newline,
    Indent,
    Dedent,
    Identifier,
    Integer,
    Float,
    Colon,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    DoubleStar,
    Star,
    FwdSlash,
    Plus,
    ThinArrow,
    Hyphen,
    DoubleDot,
    Dot,
    GreaterEqual,
    Greater,
    LesserEqual,
    Lesser,
    DoubleEqual,
    FatArrow,
    Equal,
    KwrdIf,
    KwrdElif,
    KwrdElse,
    KwrdFor,
    KwrdWhile,
    KwrdFn,

    EOF,
}

impl TokenType {
    pub fn keyword(lexeme: &str) -> TokenType {
        match lexeme {
            "if" => TokenType::KwrdIf,
            "elif" => TokenType::KwrdElif,
            "else" => TokenType::KwrdElse,
            "for" => TokenType::KwrdFor,
            "while" => TokenType::KwrdWhile,
            "fn" => TokenType::KwrdFn,
            _ => TokenType::Identifier,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Token {
    pub kind: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenType, start_pos: usize, end_pos: usize) -> Self {
        Self {
            kind,
            span: Span::new(start_pos, end_pos),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.span)
    }
}
