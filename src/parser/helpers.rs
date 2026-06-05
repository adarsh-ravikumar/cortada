use crate::{
    common::Span,
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label},
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

    fn diagnostic_span(&self, tok: &Token) -> Span {
        match tok.kind {
            TokenKind::Newline | TokenKind::Dedent | TokenKind::EOF => {
                Span::new(tok.span.start, tok.span.start)
            }

            _ => tok.span,
        }
    }

    pub(crate) fn expect_identifier(&self, name: &'static str) -> Result<(), Diagnostic> {
        let cur = self.peek(0);

        if cur.kind == TokenKind::Identifier {
            return Ok(());
        }

        Err(Diagnostic {
            severity: DiagnosticSeverity::Error,
            class: DiagnosticClass::ExpectedToken,

            msg: format!("expected {name}"),

            primary: Label {
                span: self.diagnostic_span(cur),
                msg: format!("found '{}'", cur.kind.display()),
            },

            secondary: vec![],

            notes: vec![],
        })
    }
    pub(crate) fn expect(&self, kind: TokenKind) -> Result<(), Diagnostic> {
        let cur = self.peek(0);

        if cur.kind == kind {
            return Ok(());
        }

        Err(Diagnostic {
            severity: DiagnosticSeverity::Error,
            class: DiagnosticClass::ExpectedToken,

            msg: format!("expected '{}'", kind.display()),

            primary: Label {
                span: self.diagnostic_span(cur),
                msg: format!("found '{}'", cur.kind.display()),
            },

            secondary: vec![],

            notes: vec![],
        })
    }
}
