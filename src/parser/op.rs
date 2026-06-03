use core::fmt;

use crate::lexer::TokenKind;

pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Power,
    Divide,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    IsEqual,
    NotEqual,
    And,
    Or,
}

impl From<TokenKind> for BinaryOp {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Plus => Self::Add,
            TokenKind::Hyphen => Self::Subtract,
            TokenKind::Star => Self::Multiply,
            TokenKind::DoubleStar => Self::Power,
            TokenKind::FwdSlash => Self::Divide,
            TokenKind::LeftAngle => Self::LessThan,
            TokenKind::LesserEqual => Self::LessThanEqual,
            TokenKind::RightAngle => Self::GreaterThan,
            TokenKind::GreaterEqual => Self::GreaterThanEqual,
            TokenKind::DoubleEqual => Self::IsEqual,
            TokenKind::NotEqual => Self::NotEqual,
            TokenKind::KwrdAnd => Self::And,
            TokenKind::KwrdOr => Self::Or,
            _ => panic!("Invalid Binary Operator!"),
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Power => write!(f, "**"),
            Self::Divide => write!(f, "/"),
            Self::LessThan => write!(f, "<"),
            Self::GreaterThan => write!(f, ">"),
            Self::LessThanEqual => write!(f, "<="),
            Self::GreaterThanEqual => write!(f, ">="),
            Self::IsEqual => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
        }
    }
}

pub enum UnaryOp {
    Plus,
    Minus,
    Not,
}

impl From<TokenKind> for UnaryOp {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Plus => Self::Plus,
            TokenKind::Hyphen => Self::Minus,
            TokenKind::KwrdNot => Self::Not,
            _ => panic!("Invalid Unary Operator!"),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Not => write!(f, "not"),
        }
    }
}
