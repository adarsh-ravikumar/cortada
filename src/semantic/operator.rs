use crate::{
    common::Span,
    parser::{BinaryOp, UnaryOp},
    symbol_table::{BuiltinType, TypeKind},
};

pub struct BinaryOpAnnotation {
    pub operator: BinaryOp,
    pub span: Span,
}

impl BinaryOpAnnotation {
    pub fn get_result_type(&self, lhs: &TypeKind, rhs: &TypeKind) -> Option<TypeKind> {
        match self.operator {
            _ => {
                if lhs == rhs {
                    let result = lhs.clone();
                    return Some(result);
                }
                None
            }
        }
    }

    pub fn try_cast(&self, lhs: &TypeKind, rhs: &TypeKind) -> Option<TypeKind> {
        match self.operator {
            BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply => {
                let lhs_rank = lhs.rank();
                let rhs_rank = rhs.rank();

                let (from, to) = if lhs_rank > rhs_rank {
                    (rhs, lhs)
                } else {
                    (lhs, rhs)
                };

                match from.try_cast(to) {
                    true => Some(to.clone()),
                    false => None,
                }
            }

            BinaryOp::Divide => {
                let float_type = TypeKind::Builtin(BuiltinType::Float);
                if !lhs.try_cast(&float_type) {
                    return None;
                }

                if !rhs.try_cast(&float_type) {
                    return None;
                }

                Some(float_type)
            }

            _ => panic!("unimplemented"),
        }
    }
}

pub struct UnaryOpAnnotation {
    pub operator: UnaryOp,
    pub span: Span,
}

impl UnaryOpAnnotation {
    pub fn get_result_type(&self, operand: &TypeKind) -> Option<TypeKind> {
        match self.operator {
            UnaryOp::Plus | UnaryOp::Minus => {
                if !matches!(
                    operand,
                    &TypeKind::Builtin(BuiltinType::Integer)
                        | &TypeKind::Builtin(BuiltinType::Float)
                ) {
                    return None;
                }

                Some(operand.clone())
            }

            _ => panic!("unimplemented"),
        }
    }

    pub fn try_cast(&self, _: &TypeKind) -> Option<TypeKind> {
        match self.operator {
            UnaryOp::Plus | UnaryOp::Minus => {
                return None;
            }

            _ => panic!("unimplemented"),
        }
    }
}
