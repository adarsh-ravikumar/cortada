use crate::{
    diagnostic::Diagnostic,
    lexer::{Token, TokenKind},
    parser::node::AstNode,
    utils::IOFile,
};

pub struct Parser<'a> {
    pub src: &'a IOFile,
    pub tokens: &'a Vec<Token>,
    pub position: usize,
}

pub type ParserRes = Result<Box<AstNode>, Diagnostic>;

impl<'a> Parser<'a> {
    pub fn new(file: &'a IOFile, tokens: &'a Vec<Token>) -> Self {
        Self {
            src: file,
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> ParserRes {
        self.skip_newlines();

        let res = self.parse_statements()?;

        self.skip_newlines();

        self.expect(TokenKind::EOF)?;

        Ok(res)
    }
}
