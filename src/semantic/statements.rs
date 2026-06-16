use crate::{
    common::{ERRONEOUS_SPAN, Span},
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label, LabelKind},
    parser::{AstNode, AstNodeKind, Program, ReturnStatement, Statements},
    semantic::{
        AnnotatedStatements, AnnotatedTree, ExpressionAnnotation, ReturnAnnotation,
        SemanticAnalyzer, StatementAnnotation,
    },
};

impl<'a> SemanticAnalyzer<'a> {
    pub fn annotate_program(&mut self, program: Program) -> AnnotatedTree {
        let statements = match program.statements.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
            _ => unreachable!(),
        };

        AnnotatedTree { statements }
    }

    pub fn annotate_statements(&mut self, statements: Statements) -> AnnotatedStatements {
        let stmts = statements.stmts;
        let mut annotated: Vec<StatementAnnotation> = Vec::new();

        for stmt in stmts {
            annotated.push(self.annotate_statement(stmt));
        }

        AnnotatedStatements {
            statements: annotated,
        }
    }

    pub fn annotate_statement(&mut self, statement: Box<AstNode>) -> StatementAnnotation {
        match statement.kind {
            AstNodeKind::Binary(_) | AstNodeKind::Unary(_) => {
                StatementAnnotation::Expression(*self.annotate_expression(statement))
            }

            AstNodeKind::VarDecl(decl) => {
                StatementAnnotation::VarDecl(self.annotate_var_decl(decl, statement.span))
            }

            AstNodeKind::VarAssign(assign) => {
                StatementAnnotation::VarAssign(self.annotate_var_assign(assign))
            }

            AstNodeKind::If(stmt) => StatementAnnotation::If(self.annotate_if_statement(stmt)),

            AstNodeKind::While(stmt) => {
                StatementAnnotation::While(self.annotate_while_statement(stmt))
            }

            AstNodeKind::Fn(stmt) => {
                StatementAnnotation::Fn(self.annotate_function(stmt, statement.span))
            }

            AstNodeKind::Return(stmt) => {
                StatementAnnotation::Return(self.annotate_return_statement(stmt, statement.span))
            }

            AstNodeKind::Break => {
                self.annotate_break_statement(statement.span);
                StatementAnnotation::Break
            }

            AstNodeKind::Continue => {
                self.annotate_continue_statement(statement.span);
                StatementAnnotation::Break
            }

            AstNodeKind::Atom(atom) => {
                let atom_annotated = self.annotate_atom(atom, statement.span);
                StatementAnnotation::Expression(ExpressionAnnotation::Atom(atom_annotated))
            }

            _ => panic!("Unexpected statement"),
        }
    }

    pub fn annotate_return_statement(
        &mut self,
        stmt: ReturnStatement,
        return_span: Span,
    ) -> ReturnAnnotation {
        let expr_span;

        let return_expr = if let Some(expr) = stmt.expr {
            expr_span = expr.span;
            *self.annotate_expression(expr)
        } else {
            expr_span = ERRONEOUS_SPAN;
            ExpressionAnnotation::Null
        };

        let return_type = return_expr.get_type();

        if let Some(diag) = self.symbol_table.context_stack.try_set_context_return(
            return_type.clone(),
            return_span,
            expr_span,
            true,
        ) {
            self.diagnostics.push(diag);
        }

        ReturnAnnotation {
            return_type: return_type.clone(),
            expr: return_expr,
        }
    }

    pub fn annotate_break_statement(&mut self, span: Span) {
        if let Some(diag) = self.symbol_table.context_stack.try_set_context_break(span) {
            self.diagnostics.push(diag)
        }
    }

    pub fn annotate_continue_statement(&mut self, span: Span) {
        if let Some(diag) = self
            .symbol_table
            .context_stack
            .try_set_context_continue(span)
        {
            self.diagnostics.push(diag)
        }
    }
}
