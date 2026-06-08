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
    pub fn common_numeric_type(&self, lhs: &TypeKind, rhs: &TypeKind) -> Option<TypeKind> {
        let lhs_rank = lhs.rank()?;
        let rhs_rank = rhs.rank()?;

        let (from, to) = if lhs_rank > rhs_rank {
            (rhs, lhs)
        } else if lhs_rank < rhs_rank {
            (lhs, rhs)
        } else {
            return Some(lhs.clone());
        };

        match from.try_implicit_cast(to) {
            true => Some(to.clone()),
            false => None,
        }
    }

    pub fn get_result_type(&self, lhs: &TypeKind, rhs: &TypeKind) -> Option<TypeKind> {
        match self.operator {
            BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply => {
                self.common_numeric_type(lhs, rhs)
            }

            BinaryOp::LessThan
            | BinaryOp::LessThanEqual
            | BinaryOp::GreaterThan
            | BinaryOp::GreaterThanEqual => {
                self.common_numeric_type(lhs, rhs)?;

                Some(TypeKind::Builtin(BuiltinType::Bool))
            }

            BinaryOp::Divide => {
                let float_type = TypeKind::Builtin(BuiltinType::Float);
                if !lhs.try_implicit_cast(&float_type) {
                    return None;
                }

                if !rhs.try_implicit_cast(&float_type) {
                    return None;
                }

                Some(float_type)
            }

            BinaryOp::IsEqual | BinaryOp::NotEqual => {
                let bool_type = TypeKind::Builtin(BuiltinType::Bool);

                if lhs == rhs
                    || self.common_numeric_type(lhs, rhs).is_some()
                    || lhs.accepts(rhs)
                    || rhs.accepts(lhs)
                {
                    return Some(bool_type);
                }

                None
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

    pub fn try_implicit_cast(&self, _: &TypeKind) -> Option<TypeKind> {
        match self.operator {
            UnaryOp::Plus | UnaryOp::Minus => {
                return None;
            }

            _ => panic!("unimplemented"),
        }
    }
}
