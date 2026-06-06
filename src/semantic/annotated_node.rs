use core::panic;

use crate::{
    common::{Span, Type},
    semantic::operator::{BinaryOpAnnotation, UnaryOpAnnotation},
};

pub struct AnnotatedTree {
    pub statements: Vec<StatementAnnotation>,
}

pub enum StatementAnnotation {
    VarDecl(VarDeclAnnotation),
    VarAssign(VarAssignAnnotation),
    Expression(ExpressionAnnotation),
}

pub struct VarDeclAnnotation {
    pub symbol_entry: usize,
    pub value: ExpressionAnnotation,
}

pub struct VarAssignAnnotation {
    pub symbol_entry: usize,
    pub value: ExpressionAnnotation,
}

pub enum ExpressionAnnotation {
    Atom(AtomAnnotation),
    Binary(BinaryAnnotation),
    Unary(UnaryAnnotation),
    Cast(CastAnnotation),
}

impl ExpressionAnnotation {
    pub fn get_type(&self) -> Type {
        match self {
            Self::Binary(expr) => expr.expr_type,
            Self::Unary(expr) => expr.expr_type,
            Self::Atom(atom) => atom.get_type(),
            Self::Cast(cast) => cast.to,
        }
    }
}

pub struct CastAnnotation {
    pub from: Type,
    pub to: Type,
    pub expr: Box<ExpressionAnnotation>,
}

pub struct BinaryAnnotation {
    pub lhs: Box<ExpressionAnnotation>,
    pub rhs: Box<ExpressionAnnotation>,
    pub op: BinaryOpAnnotation,

    pub span: Span,
    pub expr_type: Type,
}

pub struct UnaryAnnotation {
    pub operand: Box<ExpressionAnnotation>,
    pub op: UnaryOpAnnotation,

    pub span: Span,
    pub expr_type: Type,
}

pub enum AtomAnnotation {
    Integer(IntegerAnnotation),
    Float(FloatAnnotation),
    Identifier(IdentifierAnnotation),
}

impl AtomAnnotation {
    pub fn get_type(&self) -> Type {
        match self {
            Self::Integer(atom) => atom.atom_type,
            Self::Float(atom) => atom.atom_type,
            _ => panic!("not implemented"),
        }
    }
}

pub struct IntegerAnnotation {
    pub value: i64,
    pub span: Span,
    pub atom_type: Type,
}

pub struct FloatAnnotation {
    pub value: f64,
    pub span: Span,
    pub atom_type: Type,
}

pub struct IdentifierAnnotation {
    pub entry: usize,
    pub span: Span,
}
