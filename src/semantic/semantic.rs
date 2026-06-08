use crate::{
    common::IOFile,
    diagnostic::Diagnostic,
    parser::{AstNode, AstNodeKind},
    semantic::annotated_node::AnnotatedTree,
    symbol_table::SymbolTable,
};

pub struct SemanticAnalyzer<'a> {
    file: &'a IOFile,
    pub symbol_table: SymbolTable<'a>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(file: &'a IOFile) -> Self {
        Self {
            file,
            symbol_table: SymbolTable::new(file),
        }
    }

    pub fn create_annotated_tree(
        &mut self,
        ast: Box<AstNode>,
    ) -> Result<AnnotatedTree, Diagnostic> {
        let kind = ast.kind;

        match kind {
            AstNodeKind::Program(program) => self.annotate_program(program),
            _ => panic!("Grammar enforces that the topmost node must be a program node"),
        }
    }
}
