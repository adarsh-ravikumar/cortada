use crate::{
    common::{Span, Type},
    parser::{BinaryOp, UnaryOp},
};

pub struct BinaryOpAnnotation {
    pub operator: BinaryOp,
    pub span: Span,
}

impl BinaryOpAnnotation {
    pub fn get_result_type(&self, lhs: Type, rhs: Type) -> Option<Type> {
        match self.operator {
            _ => {
                if lhs == rhs {
                    let result = lhs;
                    return Some(result);
                }
                None
            }
        }
    }

    pub fn try_cast(&self, lhs: Type, rhs: Type) -> Option<(Type, Type)> {
        match self.operator {
            _ => {
                let result = lhs;

                if !rhs.try_cast(lhs) {
                    return None;
                }

                Some((lhs, result))
            }
        }
    }
}

pub struct UnaryOpAnnotation {
    pub operator: UnaryOp,
    pub span: Span,
}

impl UnaryOpAnnotation {
    pub fn get_result_type(&self, operand: Type) -> Option<Type> {
        match self.operator {
            UnaryOp::Plus | UnaryOp::Minus => {
                if !matches!(operand, Type::Integer | Type::Float) {
                    return None;
                }

                Some(operand)
            }

            _ => panic!("unimplemented"),
        }
    }

    pub fn try_cast(&self, _: Type) -> Option<(Type, Type)> {
        match self.operator {
            UnaryOp::Plus | UnaryOp::Minus => {
                return None;
            }

            _ => panic!("unimplemented"),
        }
    }
}
