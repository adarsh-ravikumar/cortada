use crate::{
    diagnostic::{Diagnostic, DiagnosticKind},
    lexer::{Token, TokenKind},
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub(crate) fn peek(&self, by: usize) -> &'a Token {
        self.tokens
            .get(self.position + by)
            .unwrap_or(self.tokens.last().unwrap())
    }

    pub(crate) fn advance_by(&mut self, by: usize) -> &'a Token {
        let next = self.peek(0);

        if next.kind != TokenKind::EOF {
            self.position += by;
        }

        next
    }

    pub(crate) fn advance(&mut self) -> &'a Token {
        self.advance_by(1)
    }

    pub(crate) fn skip_newlines(&mut self) {
        while self.peek(0).kind == TokenKind::Newline {
            self.advance();
        }
    }

    pub(crate) fn matches_any(&self, pattern: &[TokenKind]) -> Option<&Token> {
        let cur = self.peek(0);
        if pattern.contains(&cur.kind) {
            Some(cur)
        } else {
            None
        }
    }

    pub(crate) fn matches(&self, kind: TokenKind) -> Option<&Token> {
        let cur = self.peek(0);
        if cur.kind == kind { Some(cur) } else { None }
    }

    pub(crate) fn expect(&self, kind: TokenKind) -> Result<(), Diagnostic> {
        let cur = self.peek(0);
        if cur.kind == kind {
            return Ok(());
        }

        Err(Diagnostic::new(
            DiagnosticKind::Error,
            format!(
                "[{}] Expected '{:?}', got {:?}",
                self.position,
                kind,
                self.peek(0).kind
            ),
            cur.span,
        ))
    }
}
