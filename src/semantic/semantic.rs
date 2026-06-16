use crate::{
    common::IOFile,
    context::SymbolTable,
    diagnostic::Diagnostic,
    parser::{AstNode, AstNodeKind},
    semantic::annotated_node::AnnotatedTree,
};

pub struct SemanticAnalyzer<'a> {
    file: &'a IOFile,
    pub symbol_table: SymbolTable<'a>,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(file: &'a IOFile) -> Self {
        Self {
            file,
            symbol_table: SymbolTable::new(file),
            diagnostics: Vec::new(),
        }
    }

    pub fn create_annotated_tree(&mut self, ast: Box<AstNode>) -> AnnotatedTree {
        let kind = ast.kind;

        match kind {
            AstNodeKind::Program(program) => self.annotate_program(program),
            _ => panic!("Grammar enforces that the topmost node must be a program node"),
        }
    }
}
