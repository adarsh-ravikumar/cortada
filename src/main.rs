mod common;
mod pretty;
mod utils;

mod lexer;
mod parser;
mod semantic;

mod diagnostic;

use crate::{
    diagnostic::DiagnosticRenderer,
    lexer::Lexer,
    parser::Parser,
    pretty::{AstPrinter, SymbolTablePrinter, TokenPrinter},
    semantic::SemanticAnalyzer,
};

fn main() {
    let file = match utils::IOFile::from_path("./examples/test.ctd") {
        Ok(f) => f,
        Err(msg) => return println!("{msg}"),
    };

    let mut lexer = Lexer::new(&file);

    let toks = match lexer.tokenize() {
        Ok(t) => t,
        Err(diag) => {
            let report = DiagnosticRenderer::render(diag, &file);
            println!("\n\n{report}");
            return;
        }
    };

    TokenPrinter::print(&toks, &file);

    let mut parser = Parser::new(&file, &toks);

    let ast = match parser.parse() {
        Ok(a) => a,
        Err(diag) => {
            let report = DiagnosticRenderer::render(diag, &file);
            println!("\n\n{report}");
            return;
        }
    };

    AstPrinter::print(&ast, &file);

    let mut analyzer = SemanticAnalyzer::new(&file, &ast);

    if let Err(diag) = analyzer.build_table() {
        let report = DiagnosticRenderer::render(diag, &file);
        println!("\n\n{report}");
        return;
    }

    SymbolTablePrinter::print(&analyzer);
}
