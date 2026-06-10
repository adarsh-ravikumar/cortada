use crate::common::Span;
use crate::parser::{BinaryOp, UnaryOp};

#[derive(Debug)]
pub struct Program {
    pub statements: Box<AstNode>,
}

#[derive(Debug)]
pub struct Statements {
    pub stmts: Vec<Box<AstNode>>,
}

#[derive(Debug)]
pub struct VarDeclStatement {
    pub name: Span,
    pub var_type: Option<Box<AstNode>>,
    pub value: Option<Box<AstNode>>,
    pub value_span: Option<Span>,
}

#[derive(Debug)]
pub struct VarAssignStatement {
    pub name: Span,
    pub value: Box<AstNode>,
    pub value_span: Span,
}

#[derive(Debug)]
pub struct FnStatement {
    pub name: Span,
    pub return_type: Option<Box<AstNode>>,
    pub params: Vec<Box<AstNode>>,
    pub body: Box<AstNode>,
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
pub enum TypePrimaryKind {
    Integer,
    Float,
    Bool,
}

#[derive(Debug)]
pub struct TypePrimary {
    pub kind: TypePrimaryKind,
}

#[derive(Debug)]
pub struct TypeUnion {
    pub variants: Vec<Box<AstNode>>,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Box<AstNode>,
    pub rhs: Box<AstNode>,
    pub op: BinaryOp,
    pub op_span: Span,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub op_span: Span,
    pub operand: Box<AstNode>,
}

#[derive(Debug)]
pub struct CallExpr {
    pub callee: Box<AstNode>,
    pub args: Vec<Box<AstNode>>,
}

#[derive(Debug)]
pub enum AtomKind {
    Integer(i64),
    Float(f64),
    Bool(bool),
    Identifier,
    Null,
}

#[derive(Debug)]
pub enum AstNodeKind {
    Program(Program),

    Statements(Statements),

    VarDecl(VarDeclStatement),
    VarAssign(VarAssignStatement),

    Fn(FnStatement),

    While(WhileStatement),
    If(IfStatement),

    Return(ReturnStatement),
    Break,
    Continue,

    TypeUnion(TypeUnion),
    TypePrimary(TypePrimary),

    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Call(CallExpr),

    Atom(AtomKind),

    Error,
}

// Node
#[derive(Debug)]
pub struct AstNode {
    pub kind: AstNodeKind,
    pub span: Span,
}

impl AstNode {
    pub fn error() -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Error,
            span: Span::new(0, 0),
        })
    }

    pub fn program(statements: Box<AstNode>, start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Program(Program { statements }),
            span: Span::new(start, end),
        })
    }

    pub fn statements(stmts: Vec<Box<AstNode>>, start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Statements(Statements { stmts }),
            span: Span::new(start, end),
        })
    }

    pub fn var_decl(
        name: Span,
        var_type: Option<Box<AstNode>>,
        value_span: Option<Span>,
        value: Option<Box<AstNode>>,
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

    pub fn var_assign(name: Span, value: Box<AstNode>, start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::VarAssign(VarAssignStatement {
                name,
                value_span: value.span,
                value,
            }),
            span: Span::new(start, end),
        })
    }

    pub fn fn_stmt(
        name: Span,
        return_type: Option<Box<AstNode>>,
        params: Vec<Box<AstNode>>,
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

    pub fn type_primary(kind: TypePrimaryKind, start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::TypePrimary(TypePrimary { kind }),
            span: Span::new(start, end),
        })
    }

    pub fn type_union(ty: Vec<Box<AstNode>>, start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::TypeUnion(TypeUnion { variants: ty }),
            span: Span::new(start, end),
        })
    }

    pub fn binary(
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
        op: BinaryOp,
        op_span: Span,
        start: usize,
        end: usize,
    ) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Binary(BinaryExpr {
                lhs,
                rhs,
                op,
                op_span,
            }),
            span: Span::new(start, end),
        })
    }

    pub fn unary(
        op: UnaryOp,
        op_span: Span,
        operand: Box<AstNode>,
        start: usize,
        end: usize,
    ) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Unary(UnaryExpr {
                op,
                operand,
                op_span,
            }),
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
            kind: AstNodeKind::Atom(AtomKind::Integer(value)),
            span: Span::new(start, end),
        })
    }

    pub fn float(value: f64, start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Atom(AtomKind::Float(value)),
            span: Span::new(start, end),
        })
    }

    pub fn bool(value: bool, start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Atom(AtomKind::Bool(value)),
            span: Span::new(start, end),
        })
    }

    pub fn identifier(start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Atom(AtomKind::Identifier),
            span: Span::new(start, end),
        })
    }

    pub fn null(start: usize, end: usize) -> Box<Self> {
        Box::new(Self {
            kind: AstNodeKind::Atom(AtomKind::Null),
            span: Span::new(start, end),
        })
    }
}
