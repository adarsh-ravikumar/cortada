use crate::common::Span;
use crate::parser::{BinaryOp, UnaryOp};

// Expressions
pub struct IntegerExpr {
    pub value: i64,
}

pub struct FloatExpr {
    pub value: f64,
}

pub struct IdentifierExpr {
    pub span: Span,
}

pub struct BinaryExpr {
    pub lhs: Box<AstNode>,
    pub rhs: Box<AstNode>,
    pub op: BinaryOp,
}

pub struct UnaryExpr {
    pub op: UnaryOp,
    pub rhs: Box<AstNode>,
}

pub struct StmtsExpr {
    pub stmts: Vec<Box<AstNode>>,
}

// Node Kinds
pub enum AstNodeKind {
    Integer(IntegerExpr),
    Float(FloatExpr),
    Identifier(IdentifierExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Statements(StmtsExpr),
}

// Node
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
