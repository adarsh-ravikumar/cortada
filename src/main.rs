mod common;
mod pretty;
mod utils;

mod lexer;
mod parser;

mod diagnostic;

use crate::{
    diagnostic::Logger,
    lexer::Lexer,
    parser::Parser,
    pretty::{AstPrinter, TokenPrinter},
};

fn main() {
    let file = match utils::IOFile::from_path("./examples/test.ctd") {
        Ok(f) => f,
        Err(msg) => return println!("{msg}"),
    };

    let mut lexer = Lexer::new(&file);

    let toks = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => return println!("{}", Logger::generate_log(&file, e)),
    };

    TokenPrinter::print(&toks, &file);

    let mut parser = Parser::new(&file, &toks);

    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => return println!("{}", Logger::generate_log(&file, e)),
    };

    AstPrinter::print(&ast, &file);
}
