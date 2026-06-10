use crate::{
    parser::{AstNode, AstNodeKind, IfStatement, ReturnStatement, WhileStatement},
    semantic::{
        ElifAnnotation, ExpressionAnnotation, IfAnnotation, ReturnAnnotation, SemanticAnalyzer,
        WhileAnnotation,
    },
    symbol_table::TypeKind,
};

impl<'a> SemanticAnalyzer<'a> {
    fn annotate_condition(&mut self, condition_node: Box<AstNode>) -> ExpressionAnnotation {
        let condition = *self.annotate_expression(condition_node);

        match condition.get_type() {
            TypeKind::Error => ExpressionAnnotation::Error,
            _ => condition,
        }
    }

    pub fn annotate_if_statement(&mut self, statement: IfStatement) -> IfAnnotation {
        let condition = self.annotate_condition(statement.condition);

        let if_body = match statement.body.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
            _ => panic!("Body must be a statements node"),
        };

        let mut elifs: Vec<ElifAnnotation> = Vec::new();

        for elif in statement.elif_stmts {
            let condition = self.annotate_condition(elif.condition);

            let elif_body = match elif.body.kind {
                AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
                _ => unreachable!(),
            };

            elifs.push(ElifAnnotation {
                condition,
                body: elif_body,
            });
        }

        match statement.else_stmt {
            Some(stmt) => {
                let else_body = match stmt.kind {
                    AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
                    _ => unreachable!(),
                };

                IfAnnotation {
                    condition,
                    body: if_body,
                    elif_stmts: elifs,
                    else_stmt: Some(else_body),
                }
            }

            None => IfAnnotation {
                condition,
                body: if_body,
                elif_stmts: elifs,
                else_stmt: None,
            },
        }
    }

    pub fn annotate_while_statement(&mut self, statement: WhileStatement) -> WhileAnnotation {
        let condition = self.annotate_condition(statement.condition);

        let while_body = match statement.body.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
            _ => panic!("Body must be a statements node"),
        };

        match statement.else_stmt {
            Some(stmt) => {
                let else_body = match stmt.kind {
                    AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
                    _ => unreachable!(),
                };

                WhileAnnotation {
                    condition,
                    body: while_body,
                    else_stmt: Some(else_body),
                }
            }

            None => WhileAnnotation {
                condition,
                body: while_body,
                else_stmt: None,
            },
        }
    }

    pub fn annotate_return_statement(&mut self, stmt: ReturnStatement) -> ReturnAnnotation {
        let return_expr = if let Some(expr) = stmt.expr {
            *self.annotate_expression(expr)
        } else {
            ExpressionAnnotation::Null
        };

        let return_type = return_expr.get_type();

        // TODO :update current context

        ReturnAnnotation {
            return_type: return_type.clone(),
            expr: return_expr,
        }
    }
}
