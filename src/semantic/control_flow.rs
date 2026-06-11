use crate::{
    parser::{AstNode, AstNodeKind, IfStatement, WhileStatement},
    semantic::{
        ElifAnnotation, ExpressionAnnotation, IfAnnotation, SemanticAnalyzer, WhileAnnotation,
    },
    symbol_table::{ContextKind, TypeKind},
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

        self.symbol_table
            .context_stack
            .enter_context(ContextKind::Conditional);

        let if_body = match statement.body.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
            _ => panic!("Body must be a statements node"),
        };
        self.symbol_table.context_stack.exit_context();

        let mut elifs: Vec<ElifAnnotation> = Vec::new();

        for elif in statement.elif_stmts {
            let condition = self.annotate_condition(elif.condition);

            self.symbol_table
                .context_stack
                .enter_context(ContextKind::Conditional);
            let elif_body = match elif.body.kind {
                AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
                _ => unreachable!(),
            };
            self.symbol_table.context_stack.exit_context();

            elifs.push(ElifAnnotation {
                condition,
                body: elif_body,
            });
        }

        match statement.else_stmt {
            Some(stmt) => {
                self.symbol_table
                    .context_stack
                    .enter_context(ContextKind::Conditional);
                let else_body = match stmt.kind {
                    AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
                    _ => unreachable!(),
                };
                self.symbol_table.context_stack.exit_context();

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

        self.symbol_table
            .context_stack
            .enter_context(ContextKind::Loop);
        let while_body = match statement.body.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
            _ => panic!("Body must be a statements node"),
        };
        self.symbol_table.context_stack.exit_context();

        match statement.else_stmt {
            Some(stmt) => {
                self.symbol_table
                    .context_stack
                    .enter_context(ContextKind::Conditional);
                let else_body = match stmt.kind {
                    AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
                    _ => unreachable!(),
                };
                self.symbol_table.context_stack.exit_context();

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
}
