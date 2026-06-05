use crate::diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label};
use crate::parser::Parser;
use crate::parser::parser::ParserRes;
use crate::{
    common::Span,
    lexer::TokenKind,
    parser::node::{AstNode, ElifBranch},
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_while_statement(&mut self) -> ParserRes {
        let cur = self.peek(0);

        let start = cur.span.start;

        self.expect(TokenKind::KwrdWhile)?;

        self.advance();

        let condition = self.parse_expression()?;

        let body = self.parse_suite()?;

        self.skip_newlines();

        if let Some(tok) = self.matches(TokenKind::KwrdElse) {
            let start = tok.span.start;

            self.advance();

            let else_stmt = self.parse_suite()?;

            let end = else_stmt.span.end;

            self.skip_newlines();

            return Ok(AstNode::while_stmt(
                condition,
                body,
                Some(else_stmt),
                start,
                end,
            ));
        }

        return Ok(AstNode::while_stmt(
            condition,
            body,
            None,
            start,
            self.peek(0).span.end,
        ));
    }

    pub(crate) fn parse_if_statement(&mut self) -> ParserRes {
        let mut elif_stmts: Vec<ElifBranch> = Vec::new();

        let cur = self.peek(0);

        let start = cur.span.start;

        self.expect(TokenKind::KwrdIf)?;

        self.advance();

        let condition = self.parse_expression()?;

        let body = self.parse_suite()?;

        self.skip_newlines();

        while let Some(_) = self.matches(TokenKind::KwrdElif) {
            self.advance();

            let condition = self.parse_expression()?;

            let body = self.parse_suite()?;

            self.skip_newlines();

            elif_stmts.push(ElifBranch { condition, body });
        }

        if let Some(tok) = self.matches(TokenKind::KwrdElse) {
            let start = tok.span.start;

            self.advance();

            let else_stmt = self.parse_suite()?;

            let end = else_stmt.span.end;

            self.skip_newlines();

            return Ok(AstNode::if_stmt(
                condition,
                body,
                elif_stmts,
                Some(else_stmt),
                start,
                end,
            ));
        }

        return Ok(AstNode::if_stmt(
            condition,
            body,
            elif_stmts,
            None,
            start,
            self.peek(0).span.end,
        ));
    }

    pub(crate) fn parse_suite(&mut self) -> ParserRes {
        let mut stmts: Vec<Box<AstNode>> = Vec::new();

        self.expect(TokenKind::Colon)?;

        let col_span = self.advance().span;

        self.skip_newlines();

        if self.peek(0).kind == TokenKind::EOF {
            return Err(Diagnostic {
                severity: DiagnosticSeverity::Error,
                class: DiagnosticClass::InvalidLayout,

                msg: "missing body".into(),

                primary: Label {
                    span: Span::new(col_span.start, col_span.end),
                    msg: "expected indented body before end of file".into(),
                },

                secondary: vec![],

                notes: vec![],
            });
        }

        if self.peek(0).kind != TokenKind::Indent {
            let cur_span = self.peek(0).span;
            return Err(Diagnostic {
                severity: DiagnosticSeverity::Error,
                class: DiagnosticClass::InvalidLayout,

                msg: "expected indented block".into(),

                primary: Label {
                    span: Span::new(col_span.start, col_span.end),
                    msg: "a block must be indented after ':'".into(),
                },

                secondary: vec![Label {
                    span: Span::new(cur_span.start, cur_span.end),
                    msg: "expected indentation before this statement".into(),
                }],

                notes: vec![],
            });
        }

        self.advance();

        self.skip_newlines();

        let start = self.peek(0).span.start;

        loop {
            if let Some(_) = self.matches(TokenKind::Dedent) {
                self.advance();

                return Ok(AstNode::statements(stmts, start, self.peek(0).span.end));
            }

            stmts.push(self.parse_statement()?);

            self.skip_newlines();

            if let Some(tok) = self.matches(TokenKind::Indent) {
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::InvalidLayout,

                    msg: "unexpected indentation".into(),

                    primary: Label {
                        span: tok.span,
                        msg: "this line is indented but no new block was started".into(),
                    },

                    secondary: vec![],

                    notes: vec![],
                });
            }
        }
    }
}
