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
    Comma,
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
    RightAngle,
    LesserEqual,
    LeftAngle,
    DoubleEqual,
    FatArrow,
    Equal,
    NotEqual,
    KwrdIf,
    KwrdElif,
    KwrdElse,
    KwrdFor,
    KwrdWhile,
    KwrdFn,
    KwrdNot,
    KwrdAnd,
    KwrdOr,
    EOF,
}

impl TokenKind {
    pub fn keyword(lexeme: &str) -> TokenKind {
        match lexeme {
            "if" => Self::KwrdIf,
            "elif" => Self::KwrdElif,
            "else" => Self::KwrdElse,
            "for" => Self::KwrdFor,
            "while" => Self::KwrdWhile,
            "fn" => Self::KwrdFn,
            "not" => Self::KwrdNot,
            "and" => Self::KwrdAnd,
            "or" => Self::KwrdOr,
            _ => Self::Identifier,
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
