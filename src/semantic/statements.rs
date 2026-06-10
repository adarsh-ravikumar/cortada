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
                StatementAnnotation::Expression(*self.annotate_expression(statement, scope))
            }

            AstNodeKind::VarDecl(decl) => {
                StatementAnnotation::VarDecl(self.annotate_var_decl(decl, statement.span, scope))
            }

            AstNodeKind::VarAssign(assign) => {
                StatementAnnotation::VarAssign(self.annotate_var_assign(assign, scope))
            }

            AstNodeKind::If(stmt) => {
                StatementAnnotation::If(self.annotate_if_statement(stmt, scope))
            }

            AstNodeKind::While(stmt) => {
                StatementAnnotation::While(self.annotate_while_statement(stmt, scope))
            }

            AstNodeKind::Fn(stmt) => {
                StatementAnnotation::Fn(self.annotate_function(stmt, statement.span, scope))
            }

            AstNodeKind::Atom(atom) => {
                let atom_annotated = self.annotate_atom(atom, statement.span, scope);
                StatementAnnotation::Expression(ExpressionAnnotation::Atom(atom_annotated))
            }

            _ => panic!("Unexpected statement"),
        }
    }
}
