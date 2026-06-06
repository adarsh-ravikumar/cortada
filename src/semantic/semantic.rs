use crate::{
    common::{IOFile, Span, SymbolPool, Type},
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label},
    parser::{AstNode, AstNodeKind, BinaryExpr, FloatExpr, IntegerExpr, Statements, UnaryExpr},
    semantic::{
        CastAnnotation,
        annotated_node::{
            AnnotatedTree, AtomAnnotation, BinaryAnnotation, ExpressionAnnotation, FloatAnnotation,
            IntegerAnnotation, StatementAnnotation, UnaryAnnotation,
        },
        operator::{BinaryOpAnnotation, UnaryOpAnnotation},
    },
};

pub struct SemanticAnalyzer<'a> {
    file: &'a IOFile,
    pub pool: SymbolPool<'a>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(file: &'a IOFile) -> Self {
        Self {
            file,
            pool: SymbolPool::new(file),
        }
    }

    pub fn annotate_statements(
        &mut self,
        statements: Statements,
    ) -> Result<AnnotatedTree, Diagnostic> {
        let stmts = statements.stmts;
        let mut annotated: Vec<StatementAnnotation> = Vec::new();

        for stmt in stmts {
            annotated.push(self.annotate_statement(stmt)?);
        }

        Ok(AnnotatedTree {
            statements: annotated,
        })
    }

    pub fn annotate_statement(
        &mut self,
        statement: Box<AstNode>,
    ) -> Result<StatementAnnotation, Diagnostic> {
        match statement.kind {
            AstNodeKind::Binary(_) | AstNodeKind::Unary(_) => {
                let expr = *self.annotate_expression(statement)?;
                Ok(StatementAnnotation::Expression(expr))
            }

            AstNodeKind::Integer(_) | AstNodeKind::Float(_) | AstNodeKind::Identifier => {
                let atom_annotated = self.annotate_atom(statement)?;
                Ok(StatementAnnotation::Expression(ExpressionAnnotation::Atom(
                    atom_annotated,
                )))
            }

            _ => panic!("not implemented"),
        }
    }

    pub fn annotate_expression(
        &mut self,
        expression: Box<AstNode>,
    ) -> Result<Box<ExpressionAnnotation>, Diagnostic> {
        let span = expression.span;

        let expr = match expression.kind {
            AstNodeKind::Binary(expr) => {
                ExpressionAnnotation::Binary(self.annotate_binary_expression(expr, span)?)
            }

            AstNodeKind::Unary(expr) => {
                ExpressionAnnotation::Unary(self.annotate_unary_expression(expr, span)?)
            }

            AstNodeKind::Integer(_) | AstNodeKind::Float(_) | AstNodeKind::Identifier => {
                let atom_annotated = self.annotate_atom(expression)?;
                ExpressionAnnotation::Atom(atom_annotated)
            }

            _ => panic!("invalid expression"),
        };

        // do some type checking perhaps

        Ok(Box::new(expr))
    }

    pub fn annotate_cast(
        &mut self,
        from: Type,
        to: Type,
        expr: Box<ExpressionAnnotation>,
    ) -> Box<ExpressionAnnotation> {
        Box::new(ExpressionAnnotation::Cast(CastAnnotation {
            from,
            to,
            expr,
        }))
    }

    pub fn annotate_binary_expression(
        &mut self,
        expr: BinaryExpr,
        span: Span,
    ) -> Result<BinaryAnnotation, Diagnostic> {
        let lhs_span = expr.lhs.span;
        let rhs_span = expr.rhs.span;

        let lhs = self.annotate_expression(expr.lhs)?;
        let mut rhs = self.annotate_expression(expr.rhs)?;

        let op = BinaryOpAnnotation {
            operator: expr.op,
            span: expr.op_span,
        };

        let lhs_type = lhs.get_type();
        let rhs_type = rhs.get_type();

        let expr_type;

        if let Some(t) = op.get_result_type(lhs_type, rhs_type) {
            expr_type = t;
        }
        // try cast
        else if let Some(t) = op.try_cast(lhs_type, rhs_type) {
            let (target_type, evaled_type) = t;
            rhs = self.annotate_cast(rhs_type, target_type, rhs);
            expr_type = evaled_type
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
                        "'{}' is not defined for '{}' and '{}'",
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
    ) -> Result<UnaryAnnotation, Diagnostic> {
        let operand_span = expr.operand.span;

        let mut operand = self.annotate_expression(expr.operand)?;
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
        else if let Some(t) = op.try_cast(operand_type) {
            let (target_type, evaled_type) = t;
            operand = self.annotate_cast(operand_type, target_type, operand);
            expr_type = evaled_type
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

    pub fn annotate_atom(&mut self, atom: Box<AstNode>) -> Result<AtomAnnotation, Diagnostic> {
        let span = atom.span;

        let atom = match atom.kind {
            AstNodeKind::Integer(expr) => {
                AtomAnnotation::Integer(self.annotate_integer(expr, span)?)
            }
            AstNodeKind::Float(expr) => AtomAnnotation::Float(self.annotate_float(expr, span)?),
            _ => panic!("atom visit not implemented"),
        };

        Ok(atom)
    }

    pub fn annotate_integer(
        &mut self,
        expr: IntegerExpr,
        span: Span,
    ) -> Result<IntegerAnnotation, Diagnostic> {
        Ok(IntegerAnnotation {
            value: expr.value,
            span,
            atom_type: Type::Integer,
        })
    }

    pub fn annotate_float(
        &mut self,
        expr: FloatExpr,
        span: Span,
    ) -> Result<FloatAnnotation, Diagnostic> {
        Ok(FloatAnnotation {
            value: expr.value,
            span,
            atom_type: Type::Float,
        })
    }

    // pub fn annotate_identifier(&mut self) -> Result<IdentifierAnnotation, Diagnostic> {}

    pub fn create_annotated_tree(
        &mut self,
        ast: Box<AstNode>,
    ) -> Result<AnnotatedTree, Diagnostic> {
        let kind = ast.kind;

        match kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
            _ => panic!("Grammar enforces that the topmost node must be a statements node"),
        }
    }
}
