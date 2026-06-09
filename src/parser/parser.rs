use crate::{
    common::IOFile,
    diagnostic::Diagnostic,
    lexer::{Token, TokenKind},
    parser::node::AstNode,
};

pub struct Parser<'a> {
    pub src: &'a IOFile,
    pub tokens: &'a Vec<Token>,
    pub position: usize,
    pub diagnostics: Vec<Diagnostic>,
}

pub type ParserRes = Box<AstNode>;

impl<'a> Parser<'a> {
    pub fn new(file: &'a IOFile, tokens: &'a Vec<Token>) -> Self {
        Self {
            src: file,
            tokens,
            position: 0,
            diagnostics: Vec::new(),
        }
    }

    pub fn recover(&mut self, sync: fn(TokenKind) -> bool) {
        while !sync(self.peek(0).kind) {
            if self.peek(0).kind != TokenKind::EOF {
                self.advance();
            }
        }
    }

    pub fn err_and_recover(&mut self, diag: Diagnostic, synchronize: fn(TokenKind) -> bool) {
        self.diagnostics.push(diag);

        self.recover(synchronize);
    }

    pub fn parse(&mut self) -> ParserRes {
        self.skip_newlines();

        let statements = self.parse_statements();

        self.skip_newlines();

        if !self.expect(TokenKind::EOF) {
            return AstNode::error();
        }

        let start = statements.span.start;
        let end = statements.span.end;

        AstNode::program(statements, start, end)
    }
}
