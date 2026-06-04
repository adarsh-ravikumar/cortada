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

pub struct Statements {
    pub stmts: Vec<Box<AstNode>>,
}

pub struct IfStatement {
    pub condition: Box<AstNode>,
    pub body: Box<AstNode>,
    pub elif_stmts: Vec<ElifBranch>,
    pub else_stmt: Option<Box<AstNode>>,
}

pub struct ElifBranch {
    pub condition: Box<AstNode>,
    pub body: Box<AstNode>,
}

pub struct WhileStatement {
    pub condition: Box<AstNode>,
    pub body: Box<AstNode>,
    pub else_stmt: Option<Box<AstNode>>,
}

pub struct VarDeclStatement {
    pub name: IdentifierExpr,
    pub var_type: Option<IdentifierExpr>,
    pub value: Option<Box<AstNode>>,
}

pub struct VarAssignStatement {
    pub name: IdentifierExpr,
    pub value: Box<AstNode>,
}

// Node Kinds
pub enum AstNodeKind {
    Integer(IntegerExpr),
    Float(FloatExpr),
    Identifier(IdentifierExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),

    // Statements
    If(IfStatement),
    While(WhileStatement),
    VarDecl(VarDeclStatement),
    VarAssign(VarAssignStatement),
    Statements(Statements),
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
