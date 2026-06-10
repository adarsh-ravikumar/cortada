use crate::{
    common::Span,
    semantic::operator::{BinaryOpAnnotation, UnaryOpAnnotation},
    symbol_table::{BindingId, BuiltinType, FunctionId, TypeKind},
};

pub struct AnnotatedTree {
    pub statements: AnnotatedStatements,
}

pub struct AnnotatedStatements {
    pub statements: Vec<StatementAnnotation>,
}

pub enum StatementAnnotation {
    VarDecl(VarDeclAnnotation),
    VarAssign(VarAssignAnnotation),
    If(IfAnnotation),
    While(WhileAnnotation),
    Fn(FunctionAnnotation),
    Expression(ExpressionAnnotation),
}

pub struct VarDeclAnnotation {
    pub entry: BindingId,
    pub value: ExpressionAnnotation,
}

pub struct VarAssignAnnotation {
    pub entry_reference: BindingId,
    pub value: ExpressionAnnotation,
}

pub struct FunctionAnnotation {
    pub entry: FunctionId,
    pub params: Vec<VarDeclAnnotation>,
    pub body: AnnotatedStatements,
}

pub struct IfAnnotation {
    pub condition: ExpressionAnnotation,
    pub body: AnnotatedStatements,
    pub elif_stmts: Vec<ElifAnnotation>,
    pub else_stmt: Option<AnnotatedStatements>,
}

pub struct ElifAnnotation {
    pub condition: ExpressionAnnotation,
    pub body: AnnotatedStatements,
}

pub struct WhileAnnotation {
    pub condition: ExpressionAnnotation,
    pub body: AnnotatedStatements,
    pub else_stmt: Option<AnnotatedStatements>,
}

pub enum ExpressionAnnotation {
    Atom(AtomAnnotation),
    Binary(BinaryAnnotation),
    Unary(UnaryAnnotation),
    Cast(CastAnnotation),
    Null,
    Error,
}

impl ExpressionAnnotation {
    pub fn get_type(&self) -> &TypeKind {
        match self {
            Self::Binary(expr) => &expr.expr_type,
            Self::Unary(expr) => &expr.expr_type,
            Self::Atom(atom) => atom.get_type(),
            Self::Cast(cast) => &cast.to,
            Self::Null => &TypeKind::Builtin(BuiltinType::Null),
            Self::Error => &TypeKind::Error,
        }
    }
}

pub struct CastAnnotation {
    pub from: TypeKind,
    pub to: TypeKind,
    pub expr: Box<ExpressionAnnotation>,
}

pub struct BinaryAnnotation {
    pub lhs: Box<ExpressionAnnotation>,
    pub rhs: Box<ExpressionAnnotation>,
    pub op: BinaryOpAnnotation,

    pub span: Span,
    pub expr_type: TypeKind,
}

pub struct UnaryAnnotation {
    pub operand: Box<ExpressionAnnotation>,
    pub op: UnaryOpAnnotation,

    pub span: Span,
    pub expr_type: TypeKind,
}

pub enum AtomAnnotation {
    Integer(IntegerAnnotation),
    Float(FloatAnnotation),
    Bool(BoolAnnotation),
    Null(NullAnnotation),
    Identifier(IdentifierAnnotation),
}

impl AtomAnnotation {
    pub fn get_type(&self) -> &TypeKind {
        match self {
            Self::Integer(atom) => &atom.atom_type,
            Self::Float(atom) => &atom.atom_type,
            Self::Bool(atom) => &atom.atom_type,
            Self::Null(atom) => &atom.atom_type,
            Self::Identifier(atom) => &atom.atom_type,
        }
    }
}

pub struct IntegerAnnotation {
    pub value: i64,
    pub span: Span,
    pub atom_type: TypeKind,
}

pub struct FloatAnnotation {
    pub value: f64,
    pub span: Span,
    pub atom_type: TypeKind,
}

pub struct BoolAnnotation {
    pub value: bool,
    pub span: Span,
    pub atom_type: TypeKind,
}

pub struct NullAnnotation {
    pub span: Span,
    pub atom_type: TypeKind,
}

pub struct IdentifierAnnotation {
    pub entry: usize,
    pub atom_type: TypeKind,
    pub span: Span,
}
