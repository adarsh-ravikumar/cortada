use crate::semantic::{IdentifierAnnotation, SemanticAnalyzer};

use crate::symbol_table::ScopeTable;
use crate::{
    common::Span,
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label},
    parser::{AstNode, AstNodeKind, AtomKind, BinaryExpr, UnaryExpr},
    semantic::{
        BoolAnnotation, CastAnnotation, NullAnnotation,
        annotated_node::{
            AtomAnnotation, BinaryAnnotation, ExpressionAnnotation, FloatAnnotation,
            IntegerAnnotation, UnaryAnnotation,
        },
        operator::{BinaryOpAnnotation, UnaryOpAnnotation},
    },
    symbol_table::{BuiltinType, TypeKind},
};

impl<'a> SemanticAnalyzer<'a> {
    pub fn annotate_expression(
        &mut self,
        expression: Box<AstNode>,
        scope: &ScopeTable,
    ) -> Result<Box<ExpressionAnnotation>, Diagnostic> {
        let span = expression.span;

        let expr = match expression.kind {
            AstNodeKind::Binary(expr) => {
                ExpressionAnnotation::Binary(self.annotate_binary_expression(expr, span, scope)?)
            }

            AstNodeKind::Unary(expr) => {
                ExpressionAnnotation::Unary(self.annotate_unary_expression(expr, span, scope)?)
            }

            AstNodeKind::Atom(atom) => {
                let atom_annotated = self.annotate_atom(atom, span, scope)?;
                ExpressionAnnotation::Atom(atom_annotated)
            }

            _ => panic!("invalid expression"),
        };

        // do some type checking perhaps

        Ok(Box::new(expr))
    }

    pub fn annotate_cast(
        &self,
        from: TypeKind,
        to: TypeKind,
        expr: Box<ExpressionAnnotation>,
    ) -> Box<ExpressionAnnotation> {
        if from == to {
            return expr;
        }

        Box::new(ExpressionAnnotation::Cast(CastAnnotation {
            from: from.clone(),
            to: to.clone(),
            expr,
        }))
    }

    pub fn annotate_binary_expression(
        &mut self,
        expr: BinaryExpr,
        span: Span,
        scope: &ScopeTable,
    ) -> Result<BinaryAnnotation, Diagnostic> {
        let lhs_span = expr.lhs.span;
        let rhs_span = expr.rhs.span;

        let mut lhs = self.annotate_expression(expr.lhs, scope)?;
        let mut rhs = self.annotate_expression(expr.rhs, scope)?;

        let op = BinaryOpAnnotation {
            operator: expr.op,
            span: expr.op_span,
        };

        let lhs_type = lhs.get_type();
        let rhs_type = rhs.get_type();

        let expr_type;

        if let Some(res) = op.get_result_type(lhs_type, rhs_type) {
            expr_type = res.clone();

            if lhs_type != &res {
                lhs = self.annotate_cast(lhs_type.clone(), res, lhs);
            } else if rhs_type != &res {
                rhs = self.annotate_cast(rhs_type.clone(), res, rhs);
            }
        }
        // bad operation
        else {
            return Err(Diagnostic {
                severity: DiagnosticSeverity::Error,
                class: DiagnosticClass::UnsupportedOperator,

                msg: format!(
                    "cannot apply operator '{}' to values of type '{}' and '{}'",
                    op.operator,
                    lhs_type.display(),
                    rhs_type.display()
                ),

                primary: Label {
                    span: op.span,
                    msg: format!(
                        "'{}' is not defined for operands of type '{}' and '{}'",
                        op.operator,
                        lhs_type.display(),
                        rhs_type.display()
                    ),
                    paranthesise: false,
                },

                secondary: vec![
                    Label {
                        span: lhs_span,
                        msg: format!("left-hand side has type '{}'", lhs_type.display(),),
                        paranthesise: true,
                    },
                    Label {
                        span: rhs_span,
                        msg: format!("right-hand side has type '{}'", rhs_type.display(),),
                        paranthesise: true,
                    },
                ],

                notes: vec![],
            });
        }

        Ok(BinaryAnnotation {
            lhs,
            rhs,
            op,
            span,
            expr_type,
        })
    }

    pub fn annotate_unary_expression(
        &mut self,
        expr: UnaryExpr,
        span: Span,
        scope: &ScopeTable,
    ) -> Result<UnaryAnnotation, Diagnostic> {
        let operand_span = expr.operand.span;

        let operand = self.annotate_expression(expr.operand, scope)?;
        let op = UnaryOpAnnotation {
            operator: expr.op,
            span: expr.op_span,
        };

        let expr_type;

        let operand_type = operand.get_type();

        if let Some(t) = op.get_result_type(operand_type) {
            expr_type = t;
        }
        // try cast
        else if let Some(t) = op.try_implicit_cast(operand_type) {
            // operand = self.annotate_cast(operand_type.clone(), target_type.clone(), operand);
            expr_type = t
        }
        // bad operation
        else {
            return Err(Diagnostic {
                severity: DiagnosticSeverity::Error,
                class: DiagnosticClass::UnsupportedOperator,

                msg: format!(
                    "cannot apply operator '{}' to value of type '{}'",
                    op.operator,
                    operand_type.display(),
                ),

                primary: Label {
                    span: op.span,
                    msg: format!(
                        "'{}' is not defined for '{}'",
                        op.operator,
                        operand_type.display(),
                    ),
                    paranthesise: false,
                },

                secondary: vec![Label {
                    span: operand_span,
                    msg: format!("operand has type '{}'", operand_type.display(),),
                    paranthesise: true,
                }],

                notes: vec![],
            });
        }
        Ok(UnaryAnnotation {
            op,
            operand,
            span,
            expr_type,
        })
    }

    pub fn annotate_identifier(
        &mut self,
        name: Span,
        scope: &ScopeTable,
    ) -> Result<AtomAnnotation, Diagnostic> {
        let symbol = self.symbol_table.get_symbol(name);

        let binding = match scope.get_id(symbol) {
            Some(id) => self.symbol_table.get_binding(id).unwrap(),
            None => {
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::UndefinedIdentifier,

                    msg: format!("use of undefined identifier `{}`", symbol),

                    primary: Label {
                        span: name,
                        msg: "identifier is not defined".into(),
                        paranthesise: false,
                    },

                    secondary: vec![],

                    notes: vec!["identifiers must be declared before they can be used".into()],
                });
            }
        };

        Ok(AtomAnnotation::Identifier(IdentifierAnnotation {
            entry: binding.id,
            atom_type: binding.binding_type.clone(),
            span: name,
        }))
    }

    pub fn annotate_atom(
        &mut self,
        atom: AtomKind,
        span: Span,
        scope: &ScopeTable,
    ) -> Result<AtomAnnotation, Diagnostic> {
        let atom = match atom {
            AtomKind::Integer(value) => AtomAnnotation::Integer(IntegerAnnotation {
                value,
                span,
                atom_type: TypeKind::Builtin(BuiltinType::Integer),
            }),

            AtomKind::Float(value) => AtomAnnotation::Float(FloatAnnotation {
                value,
                span,
                atom_type: TypeKind::Builtin(BuiltinType::Float),
            }),

            AtomKind::Bool(value) => AtomAnnotation::Bool(BoolAnnotation {
                value,
                span,
                atom_type: TypeKind::Builtin(BuiltinType::Bool),
            }),

            AtomKind::Null => AtomAnnotation::Null(NullAnnotation {
                span,
                atom_type: TypeKind::Builtin(BuiltinType::Null),
            }),

            AtomKind::Identifier => self.annotate_identifier(span, scope)?,
        };

        Ok(atom)
    }
}
