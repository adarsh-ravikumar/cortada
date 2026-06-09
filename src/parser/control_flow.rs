use crate::diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label, LabelKind};
use crate::parser::Parser;
use crate::parser::parser::ParserRes;
use crate::{
    common::Span,
    lexer::TokenKind,
    parser::node::{AstNode, ElifBranch},
};

impl<'a> Parser<'a> {
    pub fn parse_while_statement(&mut self) -> ParserRes {
        let cur = self.peek(0);

        let start = cur.span.start;

        if !self.expect(TokenKind::KwrdWhile) {
            return AstNode::error();
        }

        self.advance();

        let condition = self.parse_expression();

        let body = self.parse_suite();

        self.skip_newlines();

        if let Some(tok) = self.matches(TokenKind::KwrdElse) {
            let start = tok.span.start;

            self.advance();

            let else_stmt = self.parse_suite();

            let end = else_stmt.span.end;

            self.skip_newlines();

            return AstNode::while_stmt(condition, body, Some(else_stmt), start, end);
        }

        AstNode::while_stmt(condition, body, None, start, self.peek(0).span.end)
    }

    pub fn parse_if_statement(&mut self) -> ParserRes {
        let mut elif_stmts: Vec<ElifBranch> = Vec::new();

        let cur = self.peek(0);

        let start = cur.span.start;

        if !self.expect(TokenKind::KwrdIf) {
            return AstNode::error();
        }

        self.advance();

        let condition = self.parse_expression();

        let body = self.parse_suite();

        self.skip_newlines();

        while let Some(_) = self.matches(TokenKind::KwrdElif) {
            self.advance();

            let condition = self.parse_expression();

            let body = self.parse_suite();

            self.skip_newlines();

            elif_stmts.push(ElifBranch { condition, body });
        }

        if let Some(tok) = self.matches(TokenKind::KwrdElse) {
            let start = tok.span.start;

            self.advance();

            let else_stmt = self.parse_suite();

            let end = else_stmt.span.end;

            self.skip_newlines();
            return AstNode::if_stmt(condition, body, elif_stmts, Some(else_stmt), start, end);
        }

        AstNode::if_stmt(
            condition,
            body,
            elif_stmts,
            None,
            start,
            self.peek(0).span.end,
        )
    }

    pub fn parse_suite(&mut self) -> ParserRes {
        let mut stmts: Vec<Box<AstNode>> = Vec::new();

        self.expect(TokenKind::Colon);
        let col_span = self.advance().span;

        self.skip_newlines();

        if self.peek(0).kind == TokenKind::EOF {
            self.err_and_recover(
                Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::InvalidLayout,

                    msg: "missing body".into(),

                    location: Span::new(col_span.start, col_span.end),
                    labels: vec![Label {
                        span: Span::new(col_span.start, col_span.end),
                        msg: "expected indented body before end of file".into(),
                        paranthesise: false,
                        kind: LabelKind::Primary,
                    }],

                    notes: vec![],
                },
                |tok| matches!(tok, TokenKind::Dedent | TokenKind::EOF),
            );

            return AstNode::error();
        }

        if self.peek(0).kind != TokenKind::Indent {
            let cur_span = self.peek(0).span;
            self.err_and_recover(
                Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::InvalidLayout,

                    msg: "expected indented block".into(),

                    location: Span::new(col_span.start, col_span.end),
                    labels: vec![
                        Label {
                            span: Span::new(col_span.start, col_span.end),
                            msg: "a block must be indented after ':'".into(),
                            paranthesise: false,
                            kind: LabelKind::Primary,
                        },
                        Label {
                            span: Span::new(cur_span.start, cur_span.end),
                            msg: "expected indentation before this statement".into(),
                            paranthesise: false,
                            kind: LabelKind::Secondary,
                        },
                    ],

                    notes: vec![],
                },
                |tok| matches!(tok, TokenKind::Dedent | TokenKind::EOF),
            );

            return AstNode::error();
        }

        self.advance();

        self.skip_newlines();

        let start = self.peek(0).span.start;

        loop {
            if let Some(_) = self.matches_any(&[TokenKind::Dedent, TokenKind::EOF]) {
                self.advance();
                return AstNode::statements(stmts, start, self.peek(0).span.end);
            }

            if self.peek(0).kind == TokenKind::Indent {
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

                while matches!(self.peek(0).kind, TokenKind::Newline | TokenKind::Dedent) {
                    self.advance();
                }
            }

            stmts.push(self.parse_statement());

            self.skip_newlines();
        }
    }
}
