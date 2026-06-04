use crate::common::Span;
use crate::parser::{BinaryOp, UnaryOp};

pub struct Statements {
    pub stmts: Vec<Box<AstNode>>,
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

pub struct FnStatement {
    pub name: IdentifierExpr,
    pub return_type: Option<IdentifierExpr>,
    pub params: Vec<Param>,
    pub body: Box<AstNode>,
}

pub struct Param {
    pub name: IdentifierExpr,
    pub param_type: Option<IdentifierExpr>,
    pub default_value: Option<Box<AstNode>>,
}

pub struct WhileStatement {
    pub condition: Box<AstNode>,
    pub body: Box<AstNode>,
    pub else_stmt: Option<Box<AstNode>>,
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

type ReturnStatement = Option<Box<AstNode>>;

pub struct BinaryExpr {
    pub lhs: Box<AstNode>,
    pub rhs: Box<AstNode>,
    pub op: BinaryOp,
}

pub struct UnaryExpr {
    pub op: UnaryOp,
    pub operand: Box<AstNode>,
}

pub struct CallExpr {
    pub callee: Box<AstNode>,
    pub args: Vec<Box<AstNode>>,
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

// Node Kinds
pub enum AstNodeKind {
    Statements(Statements),

    VarDecl(VarDeclStatement),
    VarAssign(VarAssignStatement),

    Fn(FnStatement),

    While(WhileStatement),
    If(IfStatement),

    Return(ReturnStatement),
    Break,
    Continue,

    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Call(CallExpr),

    Integer(IntegerExpr),
    Float(FloatExpr),
    Identifier(IdentifierExpr),
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
