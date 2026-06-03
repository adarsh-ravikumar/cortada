use core::fmt;

use crate::common::Span;

pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Power,
    Divide,
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
