use crate::{
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label, LabelKind},
    lexer::TokenKind,
    parser::{Parser, node::AstNode, parser::ParserRes},
};

impl<'a> Parser<'a> {
    pub fn parse_statements(&mut self) -> ParserRes {
        self.skip_newlines();

        let mut stmts: Vec<Box<AstNode>> = Vec::new();

        let start = self.peek(0).span.start;

        loop {
            if let Some(_) = self.matches(TokenKind::EOF) {
                return AstNode::statements(stmts, start, self.peek(0).span.end);
            } else if let Some(_) = self.matches(TokenKind::Dedent) {
                self.advance();
                return AstNode::statements(stmts, start, self.peek(0).span.end);
            }

            stmts.push(self.parse_statement());

            self.skip_newlines();
        }
    }

    pub fn parse_statement(&mut self) -> ParserRes {
        self.skip_newlines();

        match self.peek(0).kind {
            TokenKind::KwrdFn => self.parse_fn_statement(),
            TokenKind::KwrdWhile => self.parse_while_statement(),
            TokenKind::KwrdIf => self.parse_if_statement(),
            TokenKind::KwrdReturn => self.parse_return_statement(),
            TokenKind::KwrdBreak => self.parse_break_statement(),
            TokenKind::KwrdContinue => self.parse_continue_statement(),
            TokenKind::Identifier => self.parse_ident_leading_statement(),

            TokenKind::Indent => {
                self.err_and_recover(
                    Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        class: DiagnosticClass::InvalidLayout,

                        msg: "unexpected indentation".into(),

                        location: self.peek(0).span,

                        labels: vec![Label {
                            span: self.peek(0).span,
                            msg: "no block was started here".into(),
                            paranthesise: false,
                            kind: LabelKind::Primary,
                        }],

                        notes: vec![],
                    },
                    |kind| {
                        matches!(
                            kind,
                            TokenKind::Newline | TokenKind::Dedent | TokenKind::EOF
                        )
                    },
                );

                AstNode::error()
            }

            _ => self.parse_expression(),
        }
    }

    pub fn parse_return_statement(&mut self) -> ParserRes {
        if !self.expect(TokenKind::KwrdReturn) {
            return AstNode::error();
        }

        let current = self.advance();

        let start = current.span.start;
        let end = current.span.end;

        if self.peek(0).kind == TokenKind::Newline {
            return AstNode::return_stmt(None, start, end);
        }

        let expr = self.parse_expression();

        AstNode::return_stmt(Some(expr), start, end)
    }

    pub fn parse_break_statement(&mut self) -> ParserRes {
        if !self.expect(TokenKind::KwrdBreak) {
            return AstNode::error();
        }

        let tok = self.advance();

        match self.peek(0).kind {
            TokenKind::Newline | TokenKind::Dedent | TokenKind::EOF => {}

            t => {
                self.err_and_recover(
                    Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        class: DiagnosticClass::UnexpectedToken,

                        msg: "unexpected token after 'break'".into(),

                        location: self.peek(0).span,
                        labels: vec![Label {
                            span: self.peek(0).span,
                            msg: format!("found {}", t.display()),
                            paranthesise: false,
                            kind: LabelKind::Primary,
                        }],

                        notes: vec!["'break' does not accept an expression or value".into()],
                    },
                    |kind| {
                        matches!(
                            kind,
                            TokenKind::Newline | TokenKind::EOF | TokenKind::Dedent
                        )
                    },
                );
            }
        }

        AstNode::break_stmt(tok.span.start, tok.span.end)
    }

    pub fn parse_continue_statement(&mut self) -> ParserRes {
        if !self.expect(TokenKind::KwrdContinue) {
            return AstNode::error();
        }

        let tok = self.advance();

        match self.peek(0).kind {
            TokenKind::Newline | TokenKind::Dedent | TokenKind::EOF => {}

            t => {
                self.err_and_recover(
                    Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        class: DiagnosticClass::UnexpectedToken,

                        msg: "unexpected token after 'continue'".into(),

                        location: self.peek(0).span,
                        labels: vec![Label {
                            span: self.peek(0).span,
                            msg: format!("found {}", t.display()),
                            paranthesise: false,
                            kind: LabelKind::Primary,
                        }],

                        notes: vec!["'continue' does not accept an expression or value".into()],
                    },
                    |kind| {
                        matches!(
                            kind,
                            TokenKind::Newline | TokenKind::EOF | TokenKind::Dedent
                        )
                    },
                );
            }
        }

        AstNode::continue_stmt(tok.span.start, tok.span.end)
    }
}
