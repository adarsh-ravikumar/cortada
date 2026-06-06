use crate::{
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label},
    lexer::TokenKind,
    parser::{Parser, node::AstNode, parser::ParserRes},
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_statements(&mut self) -> ParserRes {
        self.skip_newlines();

        let mut stmts: Vec<Box<AstNode>> = Vec::new();

        let start = self.peek(0).span.start;

        loop {
            if let Some(_) = self.matches_any(&[TokenKind::EOF, TokenKind::Dedent]) {
                return Ok(AstNode::statements(stmts, start, self.peek(0).span.end));
            }

            stmts.push(self.parse_statement()?);

            self.skip_newlines();
        }
    }

    pub(crate) fn parse_statement(&mut self) -> ParserRes {
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
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::InvalidLayout,

                    msg: "unexpected indentation".into(),

                    primary: Label {
                        span: self.peek(0).span,
                        msg: "no block was started here".into(),
                        paranthesise: false,
                    },

                    secondary: vec![],

                    notes: vec![],
                });
            }
            _ => self.parse_expression(),
        }
    }

    pub(crate) fn parse_return_statement(&mut self) -> ParserRes {
        self.expect(TokenKind::KwrdReturn)?;
        let start = self.advance().span.start;

        if self.peek(0).kind == TokenKind::Newline {
            return Ok(AstNode::return_stmt(None, start, self.peek(0).span.end));
        }

        let expr = self.parse_expression()?;

        let end = expr.span.end;

        Ok(AstNode::return_stmt(Some(expr), start, end))
    }

    pub(crate) fn parse_break_statement(&mut self) -> ParserRes {
        self.expect(TokenKind::KwrdBreak)?;
        let tok = self.advance();

        match self.peek(0).kind {
            TokenKind::Newline | TokenKind::Dedent | TokenKind::EOF => {}

            t => {
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::UnexpectedToken,

                    msg: "unexpected token after 'break'".into(),

                    primary: Label {
                        span: self.peek(0).span,
                        msg: format!("found {}", t.display()),
                        paranthesise: false,
                    },

                    secondary: vec![],

                    notes: vec!["'break' does not accept an expression or value".into()],
                });
            }
        }

        Ok(AstNode::break_stmt(tok.span.start, tok.span.end))
    }

    pub(crate) fn parse_continue_statement(&mut self) -> ParserRes {
        self.expect(TokenKind::KwrdContinue)?;
        let tok = self.advance();

        match self.peek(0).kind {
            TokenKind::Newline | TokenKind::Dedent | TokenKind::EOF => {}

            t => {
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::UnexpectedToken,

                    msg: "unexpected token after 'continue'".into(),

                    primary: Label {
                        span: self.peek(0).span,
                        msg: format!("found {}", t.display()),
                        paranthesise: false,
                    },

                    secondary: vec![],

                    notes: vec!["'continue' does not accept an expression or value".into()],
                });
            }
        }

        Ok(AstNode::continue_stmt(tok.span.start, tok.span.end))
    }
}
