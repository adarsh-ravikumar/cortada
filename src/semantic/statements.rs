use crate::{
    diagnostic::Diagnostic,
    parser::{AstNode, AstNodeKind, Program, Statements},
    semantic::{
        AnnotatedStatements, SemanticAnalyzer,
        annotated_node::{AnnotatedTree, ExpressionAnnotation, StatementAnnotation},
    },
    symbol_table::ScopeTable,
};

impl<'a> SemanticAnalyzer<'a> {
    pub(crate) fn annotate_program(
        &mut self,
        program: Program,
    ) -> Result<AnnotatedTree, Diagnostic> {
        let mut scope = ScopeTable::new(None, false);

        // stmts
        let statements = match program.statements.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts, &mut scope)?,
            _ => unreachable!(),
        };

        Ok(AnnotatedTree { statements })
    }

    pub(crate) fn annotate_statements(
        &mut self,
        statements: Statements,
        scope: &mut ScopeTable<'a>,
    ) -> Result<AnnotatedStatements, Diagnostic> {
        let stmts = statements.stmts;
        let mut annotated: Vec<StatementAnnotation> = Vec::new();

        for stmt in stmts {
            annotated.push(self.annotate_statement(stmt, scope)?);
        }

        Ok(AnnotatedStatements {
            statements: annotated,
        })
    }

    pub(crate) fn annotate_statement(
        &mut self,
        statement: Box<AstNode>,
        scope: &mut ScopeTable<'a>,
    ) -> Result<StatementAnnotation, Diagnostic> {
        match statement.kind {
            AstNodeKind::Binary(_) | AstNodeKind::Unary(_) => {
                let expr = *self.annotate_expression(statement, scope)?;
                Ok(StatementAnnotation::Expression(expr))
            }

            AstNodeKind::VarDecl(decl) => {
                let decl = self.annotate_var_decl(decl, statement.span, scope)?;
                Ok(StatementAnnotation::VarDecl(decl))
            }

            AstNodeKind::VarAssign(assign) => {
                let assign = self.annotate_var_assign(assign, scope)?;
                Ok(StatementAnnotation::VarAssign(assign))
            }

            AstNodeKind::Atom(atom) => {
                let atom_annotated = self.annotate_atom(atom, statement.span, scope)?;
                Ok(StatementAnnotation::Expression(ExpressionAnnotation::Atom(
                    atom_annotated,
                )))
            }

            _ => panic!("not implemented"),
        }
    }
}
