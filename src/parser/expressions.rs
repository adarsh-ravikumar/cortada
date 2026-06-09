use crate::{
    common::Span,
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label, LabelKind},
    lexer::TokenKind,
    parser::{BinaryOp, Parser, UnaryOp, node::AstNode, parser::ParserRes},
};

impl<'a> Parser<'a> {
    pub fn parse_expression(&mut self) -> ParserRes {
        self.parse_or_expression()
    }

    fn is_in_atom_synchronize_set(&self, kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::RightParen
                | TokenKind::RightBracket
                | TokenKind::Comma
                | TokenKind::Colon
                | TokenKind::Dedent
                | TokenKind::Newline
                | TokenKind::EOF
        )
    }

    pub fn parse_binary_expr(
        &mut self,
        lhs_fn: fn(&mut Self) -> ParserRes,
        rhs_fn: fn(&mut Self) -> ParserRes,
        pattern: &[TokenKind],
    ) -> ParserRes {
        let start = self.peek(0).span.start;
        let mut lhs = lhs_fn(self);

        while let Some(tok) = self.matches_any(pattern) {
            let op_span = tok.span;

            let op = BinaryOp::from(tok.kind);

            self.advance();

            // if the atom synchronizes using these tokens
            // none of them must be consumed by any members as the first token
            // which means we can guarantee that the rhs is bound to fail
            if self.is_in_atom_synchronize_set(self.peek(0).kind) {
                // this is guaranteed to fail
                // so we emit failure here.
                self.diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::ExpectedExpression,

                    msg: format!("expected operand after '{}'", op.to_string()),

                    location: Span::new(start, self.peek(0).span.end),

                    labels: vec![
                        Label {
                            span: Span::new(self.peek(0).span.start, self.peek(0).span.end),
                            msg: format!("found {}", self.peek(0).kind.display()),
                            paranthesise: false,
                            kind: LabelKind::Primary,
                        },
                        Label {
                            span: op_span,
                            msg: format!("'{}' here", op.to_string()),
                            paranthesise: false,
                            kind: LabelKind::Secondary,
                        },
                    ],

                    notes: vec![],
                });

                return AstNode::error();
            }

            let rhs = rhs_fn(self);

            let end = rhs.span.end;

            lhs = AstNode::binary(lhs, rhs, op, op_span, start, end)
        }

        lhs
    }

    pub fn parse_or_expression(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_and_expression,
            Self::parse_and_expression,
            &[TokenKind::KwrdOr],
        )
    }

    pub fn parse_and_expression(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_not_expression,
            Self::parse_not_expression,
            &[TokenKind::KwrdAnd],
        )
    }

    pub fn parse_not_expression(&mut self) -> ParserRes {
        let start = self.peek(0).span.start;

        if let Some(tok) = self.matches(TokenKind::KwrdNot) {
            let op_span = tok.span;
            let op = UnaryOp::from(tok.kind);

            self.advance();

            // if the atom synchronizes using these tokens
            // none of them must be consumed by any members as the first token
            // which means we can guarantee that the rhs is bound to fail
            if self.is_in_atom_synchronize_set(self.peek(0).kind) {
                // this is guaranteed to fail
                // so we emit failure here.
                self.diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::ExpectedExpression,

                    msg: "expected operand after 'not'".into(),

                    location: Span::new(start, self.peek(0).span.end),

                    labels: vec![
                        Label {
                            span: Span::new(self.peek(0).span.start, self.peek(0).span.end),
                            msg: format!("found {}", self.peek(0).kind.display()),
                            paranthesise: false,
                            kind: LabelKind::Primary,
                        },
                        Label {
                            span: op_span,
                            msg: "'not' here".into(),
                            paranthesise: false,
                            kind: LabelKind::Secondary,
                        },
                    ],

                    notes: vec![],
                });

                return AstNode::error();
            }

            let operand = self.parse_not_expression();

            let end = operand.span.end;

            AstNode::unary(op, op_span, operand, start, end);
        }

        self.parse_boolean_expression()
    }

    pub fn parse_boolean_expression(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_arithmetic_expression,
            Self::parse_arithmetic_expression,
            &[
                TokenKind::LeftAngle,
                TokenKind::LesserEqual,
                TokenKind::RightAngle,
                TokenKind::GreaterEqual,
                TokenKind::DoubleEqual,
                TokenKind::NotEqual,
            ],
        )
    }

    pub fn parse_arithmetic_expression(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_term,
            Self::parse_term,
            &[TokenKind::Plus, TokenKind::Hyphen],
        )
    }

    pub fn parse_term(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_factor,
            Self::parse_factor,
            &[TokenKind::Star, TokenKind::FwdSlash],
        )
    }

    pub fn parse_factor(&mut self) -> ParserRes {
        let start = self.peek(0).span.start;

        if let Some(tok) = self.matches_any(&[TokenKind::Plus, TokenKind::Hyphen]) {
            let op_span = tok.span;
            let op = UnaryOp::from(tok.kind);

            self.advance();

            // if the atom synchronizes using these tokens
            // none of them must be consumed by any members as the first token
            // which means we can guarantee that the rhs is bound to fail
            if self.is_in_atom_synchronize_set(self.peek(0).kind) {
                // this is guaranteed to fail
                // so we emit failure here.
                self.diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::ExpectedExpression,

                    msg: format!("expected operand after '{}'", op.to_string()),

                    location: Span::new(start, self.peek(0).span.end),

                    labels: vec![
                        Label {
                            span: Span::new(self.peek(0).span.start, self.peek(0).span.end),
                            msg: format!("found {}", self.peek(0).kind.display()),
                            paranthesise: false,
                            kind: LabelKind::Primary,
                        },
                        Label {
                            span: op_span,
                            msg: format!("'{}' here", op.to_string()),
                            paranthesise: false,
                            kind: LabelKind::Secondary,
                        },
                    ],

                    notes: vec![],
                });

                return AstNode::error();
            }

            let operand = self.parse_factor();

            let end = operand.span.end;

            return AstNode::unary(op, op_span, operand, start, end);
        }

        self.parse_postfix()
    }

    pub fn parse_postfix(&mut self) -> ParserRes {
        let mut operand = self.parse_atom();

        loop {
            match self.peek(0).kind {
                TokenKind::LeftParen => operand = self.parse_call_op(operand),
                _ => break,
            }
        }

        operand
    }

    pub fn parse_atom(&mut self) -> ParserRes {
        let next_tok = self.peek(0);

        let start = next_tok.span.start;

        let node = match next_tok.kind {
            TokenKind::Integer => {
                let value = self.src.view_span(next_tok.span);
                AstNode::integer(
                    value.parse().unwrap(),
                    next_tok.span.start,
                    next_tok.span.end,
                )
            }

            TokenKind::Float => {
                let value = self.src.view_span(next_tok.span);
                AstNode::float(
                    value.parse().unwrap(),
                    next_tok.span.start,
                    next_tok.span.end,
                )
            }

            TokenKind::Identifier => AstNode::identifier(next_tok.span.start, next_tok.span.end),

            TokenKind::KwrdNull => AstNode::null(next_tok.span.start, next_tok.span.end),

            TokenKind::KwrdTrue => AstNode::bool(true, next_tok.span.start, next_tok.span.end),

            TokenKind::KwrdFalse => AstNode::bool(false, next_tok.span.start, next_tok.span.end),

            TokenKind::LeftParen => {
                let open_paren_span = self.advance().span;

                // empty!
                if self.peek(0).kind == TokenKind::RightParen {
                    self.diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        class: DiagnosticClass::UnmatchedDelimiter,

                        msg: "empty paranthesised expression".into(),

                        location: Span::new(start, next_tok.span.end),
                        labels: vec![
                            Label {
                                span: Span::new(start, next_tok.span.end),
                                msg: "expected expression, found ')'".into(),
                                paranthesise: false,
                                kind: LabelKind::Primary,
                            },
                            Label {
                                span: open_paren_span,
                                msg: "'(' opened here".into(),
                                paranthesise: false,
                                kind: LabelKind::Secondary,
                            },
                        ],

                        notes: vec![],
                    });

                    self.advance();

                    return AstNode::error();
                }

                let node = self.parse_or_expression();

                if self.matches(TokenKind::RightParen).is_none() {
                    self.diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        class: DiagnosticClass::UnmatchedDelimiter,

                        msg: "unclosed parenthesized expression".into(),

                        location: Span::new(start, next_tok.span.end),
                        labels: vec![
                            Label {
                                span: Span::new(start, next_tok.span.end),
                                msg: "expected ')'".into(),
                                paranthesise: false,
                                kind: LabelKind::Primary,
                            },
                            Label {
                                span: open_paren_span,
                                msg: "'(' opened here".into(),
                                paranthesise: false,
                                kind: LabelKind::Secondary,
                            },
                        ],

                        notes: vec![],
                    });
                }

                self.advance();

                return node;
            }

            kind => {
                self.err_and_recover(
                    Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        class: DiagnosticClass::ExpectedExpression,

                        msg: "expected expression".into(),

                        location: Span::new(start, next_tok.span.end),

                        labels: vec![Label {
                            span: Span::new(start, next_tok.span.end),
                            msg: format!("found {}", kind.display()),
                            paranthesise: false,
                            kind: LabelKind::Primary,
                        }],

                        notes: vec![],
                    },
                    |tok| {
                        matches!(
                            tok,
                            TokenKind::RightParen
                                | TokenKind::RightBracket
                                | TokenKind::Comma
                                | TokenKind::Colon
                                | TokenKind::Dedent
                                | TokenKind::Newline
                                | TokenKind::EOF
                        )
                    },
                );

                return AstNode::error();
            }
        };

        self.advance();
        node
    }
}
