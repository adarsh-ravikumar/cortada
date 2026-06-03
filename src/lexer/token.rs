use core::fmt;

use crate::common::Span;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenKind {
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

impl TokenKind {
    pub fn keyword(lexeme: &str) -> TokenKind {
        match lexeme {
            "if" => TokenKind::KwrdIf,
            "elif" => TokenKind::KwrdElif,
            "else" => TokenKind::KwrdElse,
            "for" => TokenKind::KwrdFor,
            "while" => TokenKind::KwrdWhile,
            "fn" => TokenKind::KwrdFn,
            _ => TokenKind::Identifier,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, start_pos: usize, end_pos: usize) -> Self {
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
