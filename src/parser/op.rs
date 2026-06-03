use core::fmt;

use crate::lexer::{Token, TokenKind};

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
        }
    }
}

pub enum UnaryOp {
    Plus,
    Minus,
}

impl From<TokenKind> for UnaryOp {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Plus => Self::Plus,
            TokenKind::Hyphen => Self::Minus,
            _ => panic!("Invalid Unary Operator!"),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
        }
    }
}
