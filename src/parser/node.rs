use crate::common::Span;
use crate::parser::{BinaryOp, UnaryOp};

#[derive(Debug)]
pub struct Statements {
    pub stmts: Vec<Box<AstNode>>,
}

#[derive(Debug)]
pub struct VarDeclStatement {
    pub name: Span,
    pub var_type: Option<Span>,
    pub value: Option<Box<AstNode>>,
    pub value_span: Span,
}

#[derive(Debug)]
pub struct VarAssignStatement {
    pub name: Span,
    pub value: Option<Box<AstNode>>,
    pub value_span: Span,
}

#[derive(Debug)]
pub struct FnStatement {
    pub name: Span,
    pub return_type: Option<Span>,
    pub params: Vec<Param>,
    pub body: Box<AstNode>,
}

#[derive(Debug)]
pub struct Param {
    pub name: Span,
    pub param_type: Option<Span>,
    pub default_value: Option<Box<AstNode>>,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Box<AstNode>,
    pub body: Box<AstNode>,
    pub else_stmt: Option<Box<AstNode>>,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Box<AstNode>,
    pub body: Box<AstNode>,
    pub elif_stmts: Vec<ElifBranch>,
    pub else_stmt: Option<Box<AstNode>>,
}

#[derive(Debug)]
pub struct ElifBranch {
    pub condition: Box<AstNode>,
    pub body: Box<AstNode>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub expr: Option<Box<AstNode>>,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Box<AstNode>,
    pub rhs: Box<AstNode>,
    pub op: BinaryOp,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub operand: Box<AstNode>,
}

#[derive(Debug)]
pub struct CallExpr {
    pub callee: Box<AstNode>,
    pub args: Vec<Box<AstNode>>,
}

#[derive(Debug)]
pub struct IntegerExpr {
    pub value: i64,
}

#[derive(Debug)]
pub struct FloatExpr {
    pub value: f64,
}

#[derive(Debug)]
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
    Identifier,
}

// Node
#[derive(Debug)]
pub struct AstNode {
    pub kind: AstNodeKind,
    pub span: Span,
}

impl AstNode {
    pub fn statements(stmts: Vec<Box<AstNode>>, start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Statements(Statements { stmts }),
            span: Span::new(start, end),
        })
    }

    pub fn var_decl(
        name: Span,
        var_type: Option<Span>,
        value: Option<Box<AstNode>>,
        value_span: Span,
        start: usize,
        end: usize,
    ) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::VarDecl(VarDeclStatement {
                name,
                var_type,
                value_span,
                value,
            }),
            span: Span::new(start, end),
        })
    }

    pub fn var_assign(
        name: Span,
        value: Option<Box<AstNode>>,
        value_span: Span,
        start: usize,
        end: usize,
    ) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::VarAssign(VarAssignStatement {
                name,
                value,
                value_span,
            }),
            span: Span::new(start, end),
        })
    }

    pub fn fn_stmt(
        name: Span,
        return_type: Option<Span>,
        params: Vec<Param>,
        body: Box<AstNode>,
        start: usize,
        end: usize,
    ) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Fn(FnStatement {
                name,
                return_type,
                params,
                body,
            }),
            span: Span::new(start, end),
        })
    }

    pub fn while_stmt(
        condition: Box<AstNode>,
        body: Box<AstNode>,
        else_stmt: Option<Box<AstNode>>,
        start: usize,
        end: usize,
    ) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::While(WhileStatement {
                condition,
                body,
                else_stmt,
            }),
            span: Span::new(start, end),
        })
    }

    pub fn if_stmt(
        condition: Box<AstNode>,
        body: Box<AstNode>,
        elif_stmts: Vec<ElifBranch>,
        else_stmt: Option<Box<AstNode>>,
        start: usize,
        end: usize,
    ) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::If(IfStatement {
                condition,
                body,
                elif_stmts,
                else_stmt,
            }),
            span: Span::new(start, end),
        })
    }

    pub fn return_stmt(expr: Option<Box<AstNode>>, start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Return(ReturnStatement { expr }),
            span: Span::new(start, end),
        })
    }

    pub fn break_stmt(start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Break,
            span: Span::new(start, end),
        })
    }

    pub fn continue_stmt(start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Continue,
            span: Span::new(start, end),
        })
    }

    pub fn binary(
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
        op: BinaryOp,
        start: usize,
        end: usize,
    ) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Binary(BinaryExpr { lhs, rhs, op }),
            span: Span::new(start, end),
        })
    }

    pub fn unary(op: UnaryOp, operand: Box<AstNode>, start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Unary(UnaryExpr { op, operand }),
            span: Span::new(start, end),
        })
    }

    pub fn call(
        callee: Box<AstNode>,
        args: Vec<Box<AstNode>>,
        start: usize,
        end: usize,
    ) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Call(CallExpr { callee, args }),
            span: Span::new(start, end),
        })
    }

    pub fn integer(value: i64, start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Integer(IntegerExpr { value }),
            span: Span::new(start, end),
        })
    }

    pub fn float(value: f64, start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Float(FloatExpr { value }),
            span: Span::new(start, end),
        })
    }

    pub fn identifier(start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Identifier,
            span: Span::new(start, end),
        })
    }
}
