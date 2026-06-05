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
    KwrdReturn,
    KwrdBreak,
    KwrdContinue,
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
            "return" => Self::KwrdReturn,
            "break" => Self::KwrdBreak,
            "continue" => Self::KwrdContinue,
            _ => Self::Identifier,
        }
    }

    pub fn display(&self) -> &'static str {
        match self {
            TokenKind::Newline => "newline",
            TokenKind::Indent => "indentation",
            TokenKind::Dedent => "dedentation",

            TokenKind::Identifier => "identifier",
            TokenKind::Integer => "integer",
            TokenKind::Float => "float",

            TokenKind::Colon => ":",
            TokenKind::Comma => ",",

            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",

            TokenKind::LeftBracket => "[",
            TokenKind::RightBracket => "]",

            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",

            TokenKind::DoubleStar => "**",
            TokenKind::Star => "*",
            TokenKind::FwdSlash => "/",

            TokenKind::Plus => "+",
            TokenKind::Hyphen => "-",

            TokenKind::ThinArrow => "->",
            TokenKind::FatArrow => "=>",

            TokenKind::Dot => ".",
            TokenKind::DoubleDot => "..",

            TokenKind::RightAngle => ">",
            TokenKind::GreaterEqual => ">=",

            TokenKind::LeftAngle => "<",
            TokenKind::LesserEqual => "<=",

            TokenKind::Equal => "=",
            TokenKind::DoubleEqual => "==",
            TokenKind::NotEqual => "!=",

            TokenKind::KwrdIf => "if",
            TokenKind::KwrdElif => "elif",
            TokenKind::KwrdElse => "else",

            TokenKind::KwrdFor => "for",
            TokenKind::KwrdWhile => "while",

            TokenKind::KwrdFn => "fn",

            TokenKind::KwrdNot => "not",
            TokenKind::KwrdAnd => "and",
            TokenKind::KwrdOr => "or",

            TokenKind::KwrdReturn => "return",
            TokenKind::KwrdBreak => "break",
            TokenKind::KwrdContinue => "continue",

            TokenKind::EOF => "end of file",
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
