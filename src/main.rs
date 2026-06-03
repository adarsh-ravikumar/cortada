mod common;
mod diagnostic;
mod lexer;
mod pretty;
mod utils;

use crate::{diagnostic::Logger, lexer::Lexer, pretty::TokenPrinter};

fn main() {
    let file = match utils::IOFile::from_path("./examples/test.ctd") {
        Ok(f) => f,
        Err(msg) => return println!("{msg}"),
    };

    let mut lex = Lexer::new(&file);

    let toks = match lex.tokenize() {
        Ok(t) => t,
        Err(e) => return println!("{}", Logger::generate_log(&file, e)),
    };

    TokenPrinter::print(&toks, &file);
}
