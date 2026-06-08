use crate::{
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label},
    parser::{AstNode, AstNodeKind, IfStatement, WhileStatement},
    semantic::{
        ElifAnnotation, ExpressionAnnotation, IfAnnotation, SemanticAnalyzer, WhileAnnotation,
    },
    symbol_table::{BuiltinType, ScopeTable, TypeKind},
};

impl<'a, 'scope> SemanticAnalyzer<'a> {
    fn annotate_condition(
        &mut self,
        condition_node: Box<AstNode>,
        scope: &ScopeTable<'a, 'scope>,
    ) -> Result<ExpressionAnnotation, Diagnostic> {
        let condition_span = condition_node.span;
        let condition = *self.annotate_expression(condition_node, scope)?;

        match condition.get_type() {
            TypeKind::Builtin(BuiltinType::Bool) => Ok(condition),

            condition_type => {
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::TypeMismatch,

                    msg: format!(
                        "condition must evaluate to a truthy value, found type `{}`",
                        condition_type.display()
                    ),

                    primary: Label {
                        span: condition_span,
                        msg: format!("this expression has type `{}`", condition_type.display()),
                        paranthesise: true,
                    },

                    secondary: vec![],

                    notes: vec![
        "conditions may only evaluate to values that can be interpreted as true or false".into(),
                    ],
                });
            }
        }
    }

    pub fn annotate_if_statement(
        &mut self,
        statement: IfStatement,
        scope: &mut Box<ScopeTable<'a, 'scope>>,
    ) -> Result<IfAnnotation, Diagnostic> {
        let condition = self.annotate_condition(statement.condition, scope)?;

        let mut if_scope = ScopeTable::new(Some(scope), true);

        let if_body = match statement.body.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts, &mut if_scope)?,
            _ => unreachable!(),
        };

        let mut elifs: Vec<ElifAnnotation> = Vec::new();

        for elif in statement.elif_stmts {
            let condition = self.annotate_condition(elif.condition, scope)?;

            let mut elif_scope = ScopeTable::new(Some(scope), true);

            let elif_body = match elif.body.kind {
                AstNodeKind::Statements(stmts) => {
                    self.annotate_statements(stmts, &mut elif_scope)?
                }
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
                        self.annotate_statements(stmts, &mut else_scope)?
                    }
                    _ => unreachable!(),
                };

                Ok(IfAnnotation {
                    condition,
                    body: if_body,
                    elif_stmts: elifs,
                    else_stmt: Some(else_body),
                })
            }

            None => Ok(IfAnnotation {
                condition,
                body: if_body,
                elif_stmts: elifs,
                else_stmt: None,
            }),
        }
    }

    pub fn annotate_while_statement(
        &mut self,
        statement: WhileStatement,
        scope: &mut Box<ScopeTable<'a, 'scope>>,
    ) -> Result<WhileAnnotation, Diagnostic> {
        let condition = self.annotate_condition(statement.condition, scope)?;

        let mut if_scope = ScopeTable::new(Some(scope), true);

        let if_body = match statement.body.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts, &mut if_scope)?,
            _ => unreachable!(),
        };

        match statement.else_stmt {
            Some(stmt) => {
                let mut else_scope = ScopeTable::new(Some(scope), true);

                let else_body = match stmt.kind {
                    AstNodeKind::Statements(stmts) => {
                        self.annotate_statements(stmts, &mut else_scope)?
                    }
                    _ => unreachable!(),
                };

                Ok(WhileAnnotation {
                    condition,
                    body: if_body,
                    else_stmt: Some(else_body),
                })
            }

            None => Ok(WhileAnnotation {
                condition,
                body: if_body,
                else_stmt: None,
            }),
        }
    }
}
