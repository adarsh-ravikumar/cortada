use core::fmt;

use crate::{common::Span, lexer::TokenKind};

pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Power,
    Divide,
}

impl From<TokenKind> for BinaryOp {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Plus => Self::Add,
            TokenKind::Hyphen => Self::Subtract,
            TokenKind::Star => Self::Multiply,
            TokenKind::DoubleStar => Self::Power,
            TokenKind::FwdSlash => Self::Divide,
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

pub struct UnaryExpr {
    pub op: UnaryOp,
    pub rhs: Box<AstNode>,
}

pub struct BinaryExpr {
    pub lhs: Box<AstNode>,
    pub rhs: Box<AstNode>,
    pub op: BinaryOp,
}

pub struct IntegerExpr {
    pub value: i64,
}

pub struct FloatExpr {
    pub value: f64,
}

pub struct IdentifierExpr {
    pub span: Span,
}

pub enum AstNodeKind {
    Integer(IntegerExpr),
    Float(FloatExpr),
    Identifier(IdentifierExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
}

pub struct AstNode {
    pub kind: AstNodeKind,
    pub span: Span,
}

impl AstNode {
    pub fn new(kind: AstNodeKind, start: usize, end: usize) -> Self {
        Self {
            kind,
            span: Span::new(start, end),
        }
    }
}
