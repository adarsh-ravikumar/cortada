use crate::{
    parser::{AstNode, AstNodeKind, IfStatement, WhileStatement},
    semantic::{
        ElifAnnotation, ExpressionAnnotation, IfAnnotation, SemanticAnalyzer, WhileAnnotation,
    },
    symbol_table::{ScopeTable, TypeKind},
};

impl<'a, 'scope> SemanticAnalyzer<'a> {
    fn annotate_condition(
        &mut self,
        condition_node: Box<AstNode>,
        scope: &ScopeTable<'a, 'scope>,
    ) -> ExpressionAnnotation {
        let condition = *self.annotate_expression(condition_node, scope);

        match condition.get_type() {
            TypeKind::Error => ExpressionAnnotation::Error,
            _ => condition,
        }
    }

    pub fn annotate_if_statement(
        &mut self,
        statement: IfStatement,
        scope: &mut Box<ScopeTable<'a, 'scope>>,
    ) -> IfAnnotation {
        let condition = self.annotate_condition(statement.condition, scope);

        let mut if_scope = ScopeTable::new(Some(scope), true);

        let if_body = match statement.body.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts, &mut if_scope),
            _ => panic!("Body must be a statements node"),
        };

        let mut elifs: Vec<ElifAnnotation> = Vec::new();

        for elif in statement.elif_stmts {
            let condition = self.annotate_condition(elif.condition, scope);

            let mut elif_scope = ScopeTable::new(Some(scope), true);

            let elif_body = match elif.body.kind {
                AstNodeKind::Statements(stmts) => self.annotate_statements(stmts, &mut elif_scope),
                _ => unreachable!(),
            };

            elifs.push(ElifAnnotation {
                condition,
                body: elif_body,
            })
        }

        match statement.else_stmt {
            Some(stmt) => {
                let mut else_scope = ScopeTable::new(Some(scope), true);

                let else_body = match stmt.kind {
                    AstNodeKind::Statements(stmts) => {
                        self.annotate_statements(stmts, &mut else_scope)
                    }
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

    pub fn annotate_while_statement(
        &mut self,
        statement: WhileStatement,
        scope: &mut Box<ScopeTable<'a, 'scope>>,
    ) -> WhileAnnotation {
        let condition = self.annotate_condition(statement.condition, scope);

        let mut if_scope = ScopeTable::new(Some(scope), true);

        let if_body = match statement.body.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts, &mut if_scope),
            _ => panic!("Body must be a statements node"),
        };

        match statement.else_stmt {
            Some(stmt) => {
                let mut else_scope = ScopeTable::new(Some(scope), true);

                let else_body = match stmt.kind {
                    AstNodeKind::Statements(stmts) => {
                        self.annotate_statements(stmts, &mut else_scope)
                    }
                    _ => unreachable!(),
                };

                WhileAnnotation {
                    condition,
                    body: if_body,
                    else_stmt: Some(else_body),
                }
            }

            None => WhileAnnotation {
                condition,
                body: if_body,
                else_stmt: None,
            },
        }
    }
}
