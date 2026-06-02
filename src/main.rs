use crate::{diagnostic::Logger, lexer::Lexer};

mod common;
mod diagnostic;
mod lexer;
mod utils;

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

    println!(
        "[{}]",
        lex.tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
}
