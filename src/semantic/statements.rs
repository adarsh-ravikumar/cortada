use crate::{
    parser::{AstNode, AstNodeKind, Program, Statements},
    semantic::{
        AnnotatedStatements, SemanticAnalyzer,
        annotated_node::{AnnotatedTree, ExpressionAnnotation, StatementAnnotation},
    },
    symbol_table::ScopeTable,
};

impl<'a, 'scope> SemanticAnalyzer<'a>
where
    'a: 'scope,
{
    pub(crate) fn annotate_program(&mut self, program: Program) -> AnnotatedTree {
        let mut scope: Box<ScopeTable<'a, 'scope>> = ScopeTable::new(None, false);

        // stmts
        let statements = match program.statements.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts, &mut scope),
            _ => unreachable!(),
        };

        AnnotatedTree { statements }
    }

    pub(crate) fn annotate_statements(
        &mut self,
        statements: Statements,
        scope: &mut Box<ScopeTable<'a, 'scope>>,
    ) -> AnnotatedStatements {
        let stmts = statements.stmts;
        let mut annotated: Vec<StatementAnnotation> = Vec::new();

        for stmt in stmts {
            annotated.push(self.annotate_statement(stmt, scope));
        }

        AnnotatedStatements {
            statements: annotated,
        }
    }

    pub(crate) fn annotate_statement(
        &mut self,
        statement: Box<AstNode>,
        scope: &mut Box<ScopeTable<'a, 'scope>>,
    ) -> StatementAnnotation {
        match statement.kind {
            AstNodeKind::Binary(_) | AstNodeKind::Unary(_) => {
                let expr = *self.annotate_expression(statement, scope);
                StatementAnnotation::Expression(expr)
            }

            AstNodeKind::VarDecl(decl) => {
                let decl = self.annotate_var_decl(decl, statement.span, scope);
                StatementAnnotation::VarDecl(decl)
            }

            AstNodeKind::VarAssign(assign) => {
                let assign = self.annotate_var_assign(assign, scope);
                StatementAnnotation::VarAssign(assign)
            }

            AstNodeKind::If(stmt) => {
                let stmt = self.annotate_if_statement(stmt, scope);
                StatementAnnotation::If(stmt)
            }

            AstNodeKind::While(stmt) => {
                let stmt = self.annotate_while_statement(stmt, scope);
                StatementAnnotation::While(stmt)
            }

            AstNodeKind::Atom(atom) => {
                let atom_annotated = self.annotate_atom(atom, statement.span, scope);
                StatementAnnotation::Expression(ExpressionAnnotation::Atom(atom_annotated))
            }

            _ => unreachable!(),
        }
    }
}
