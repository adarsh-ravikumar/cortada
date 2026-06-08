use crate::{
    diagnostic::Diagnostic,
    parser::{AstNode, AstNodeKind, Program, Statements},
    semantic::{
        AnnotatedStatements, SemanticAnalyzer,
        annotated_node::{AnnotatedTree, ExpressionAnnotation, StatementAnnotation},
    },
};

impl<'a> SemanticAnalyzer<'a> {
    pub(crate) fn annotate_program(
        &mut self,
        program: Program,
    ) -> Result<AnnotatedTree, Diagnostic> {
        // stmts
        let statements = match program.statements.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts)?,
            _ => unreachable!(),
        };

        Ok(AnnotatedTree { statements })
    }

    pub(crate) fn annotate_statements(
        &mut self,
        statements: Statements,
    ) -> Result<AnnotatedStatements, Diagnostic> {
        let stmts = statements.stmts;
        let mut annotated: Vec<StatementAnnotation> = Vec::new();

        for stmt in stmts {
            annotated.push(self.annotate_statement(stmt)?);
        }

        Ok(AnnotatedStatements {
            statements: annotated,
        })
    }

    pub(crate) fn annotate_statement(
        &mut self,
        statement: Box<AstNode>,
    ) -> Result<StatementAnnotation, Diagnostic> {
        match statement.kind {
            AstNodeKind::Binary(_) | AstNodeKind::Unary(_) => {
                let expr = *self.annotate_expression(statement)?;
                Ok(StatementAnnotation::Expression(expr))
            }

            AstNodeKind::VarDecl(decl) => {
                let decl = self.annotate_var_decl(decl, statement.span)?;
                Ok(StatementAnnotation::VarDecl(decl))
            }

            AstNodeKind::VarAssign(assign) => {
                let assign = self.annotate_var_assign(assign)?;
                Ok(StatementAnnotation::VarAssign(assign))
            }

            AstNodeKind::Atom(atom) => {
                let atom_annotated = self.annotate_atom(atom, statement.span)?;
                Ok(StatementAnnotation::Expression(ExpressionAnnotation::Atom(
                    atom_annotated,
                )))
            }

            _ => panic!("not implemented"),
        }
    }
}
