mod common;
mod pretty;
mod utils;

mod lexer;
mod parser;
mod semantic;
mod symbol_table;

mod diagnostic;

use crate::{
    common::IOFile,
    diagnostic::DiagnosticRenderer,
    lexer::Lexer,
    parser::Parser,
    pretty::{AnnotatedTreePrinter, AstPrinter, TokenPrinter},
    semantic::SemanticAnalyzer,
};

fn main() {
    let file = match IOFile::from_path("./examples/test.ctd") {
        Ok(f) => f,
        Err(msg) => return println!("{msg}"),
    };

    let mut diag_renderer = DiagnosticRenderer::new(&file);

    let mut lexer = Lexer::new(&file);

    let toks = lexer.tokenize();
    if !lexer.diagnostics.is_empty() {
        let report = diag_renderer.render(&mut lexer.diagnostics);
        println!("{report}");
        return;
    }

    //    println!("TOKENS:");
    //   TokenPrinter::print(&toks, &file);

    let mut parser = Parser::new(&file, &toks);

    let ast = parser.parse();
    if !parser.diagnostics.is_empty() {
        let report = diag_renderer.render(&mut parser.diagnostics);
        println!("{report}");
        return;
    }

    // println!("ABSTRACT SYNTAX TREE:");
    // AstPrinter::print(&ast, &file);
    // println!("\n");

    let mut analyzer = SemanticAnalyzer::new(&file);

    let annotated_tree = analyzer.create_annotated_tree(ast);
    if !analyzer.diagnostics.is_empty() {
        let report = diag_renderer.render(&mut analyzer.diagnostics);
        println!("{report}");
        return;
    }

    println!("ANNOTATED ABSTRACT SYNTAX TREE:");
    let printer = AnnotatedTreePrinter {
        symbol_table: &analyzer.symbol_table,
    };

    printer.print(&annotated_tree);
}
