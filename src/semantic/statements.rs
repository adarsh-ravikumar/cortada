use crate::{
    parser::{AstNode, AstNodeKind, Program, Statements},
    semantic::{
        AnnotatedStatements, SemanticAnalyzer,
        annotated_node::{AnnotatedTree, ExpressionAnnotation, StatementAnnotation},
    },
};

impl<'a> SemanticAnalyzer<'a> {
    pub fn annotate_program(&mut self, program: Program) -> AnnotatedTree {
        self.symbol_table.enter_scope();

        let statements = match program.statements.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
            _ => unreachable!(),
        };

        self.symbol_table.exit_scope();

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
                StatementAnnotation::Return(self.annotate_return_statement(stmt))
            }

            AstNodeKind::Atom(atom) => {
                let atom_annotated = self.annotate_atom(atom, statement.span);
                StatementAnnotation::Expression(ExpressionAnnotation::Atom(atom_annotated))
            }

            _ => panic!("Unexpected statement"),
        }
    }
}
