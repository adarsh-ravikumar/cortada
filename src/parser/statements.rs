use crate::{
    diagnostic::{Diagnostic, DiagnosticKind},
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
                return Err(Diagnostic::new(
                    DiagnosticKind::Error,
                    format!(
                        "[{}] Expected terminator after continue, got {:?}",
                        self.position, t
                    ),
                    self.peek(0).span,
                ));
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
                return Err(Diagnostic::new(
                    DiagnosticKind::Error,
                    format!(
                        "[{}] Expected terminator after continue, got {:?}",
                        self.position, t
                    ),
                    self.peek(0).span,
                ));
            }
        }

        Ok(AstNode::continue_stmt(tok.span.start, tok.span.end))
    }
}
