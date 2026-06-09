use crate::{
    common::Span,
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label, LabelKind},
    lexer::{Token, TokenKind},
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub fn peek(&self, by: usize) -> &'a Token {
        self.tokens
            .get(self.position + by)
            .unwrap_or(self.tokens.last().unwrap())
    }

    pub fn advance_by(&mut self, by: usize) -> &'a Token {
        let next = self.peek(0);

        if next.kind != TokenKind::EOF {
            self.position += by;
        }

        next
    }

    pub fn advance(&mut self) -> &'a Token {
        self.advance_by(1)
    }

    pub fn skip_newlines(&mut self) {
        while self.peek(0).kind == TokenKind::Newline {
            self.advance();
        }
    }

    pub fn matches_any(&self, pattern: &[TokenKind]) -> Option<&Token> {
        let cur = self.peek(0);
        if pattern.contains(&cur.kind) {
            Some(cur)
        } else {
            None
        }
    }

    pub fn matches(&self, kind: TokenKind) -> Option<&Token> {
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

    pub fn expect_identifier(&mut self, name: &'static str) -> bool {
        let cur = self.peek(0);

        if cur.kind == TokenKind::Identifier {
            return true;
        }

        self.err_and_recover(
            Diagnostic {
                severity: DiagnosticSeverity::Error,
                class: DiagnosticClass::ExpectedToken,

                msg: format!("expected {name}"),

                location: self.diagnostic_span(cur),
                labels: vec![Label {
                    span: self.diagnostic_span(cur),
                    msg: format!("found '{}'", cur.kind.display()),
                    paranthesise: false,
                    kind: LabelKind::Primary,
                }],

                notes: vec![],
            },
            |_| true,
        );

        false
    }

    pub fn expect(&mut self, kind: TokenKind) -> bool {
        let cur = self.peek(0);

        if cur.kind == kind {
            return true;
        }

        self.diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Error,
            class: DiagnosticClass::ExpectedToken,

            msg: format!("expected '{}'", kind.display()),

            location: self.diagnostic_span(cur),
            labels: vec![Label {
                span: self.diagnostic_span(cur),
                msg: format!("found '{}'", cur.kind.display()),
                paranthesise: false,
                kind: LabelKind::Primary,
            }],

            notes: vec![],
        });

        false
    }
}
