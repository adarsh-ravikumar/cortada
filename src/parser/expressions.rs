use crate::{
    common::Span,
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label},
    lexer::TokenKind,
    parser::{BinaryOp, Parser, UnaryOp, node::AstNode, parser::ParserRes},
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self) -> ParserRes {
        self.parse_or_expression()
    }

    pub(crate) fn parse_binary_expr(
        &mut self,
        lhs_fn: fn(&mut Self) -> ParserRes,
        rhs_fn: fn(&mut Self) -> ParserRes,
        pattern: &[TokenKind],
    ) -> ParserRes {
        let start = self.peek(0).span.start;
        let mut lhs = lhs_fn(self)?;

        while let Some(tok) = self.matches_any(pattern) {
            let op_span = tok.span;
            let op = BinaryOp::from(tok.kind);
            self.advance();

            let rhs = rhs_fn(self)?;

            let end = rhs.span.end;

            lhs = AstNode::binary(lhs, rhs, op, op_span, start, end)
        }

        Ok(lhs)
    }

    pub(crate) fn parse_or_expression(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_and_expression,
            Self::parse_and_expression,
            &[TokenKind::KwrdOr],
        )
    }

    pub(crate) fn parse_and_expression(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_not_expression,
            Self::parse_not_expression,
            &[TokenKind::KwrdAnd],
        )
    }

    pub(crate) fn parse_not_expression(&mut self) -> ParserRes {
        let start = self.peek(0).span.start;

        if let Some(tok) = self.matches(TokenKind::KwrdNot) {
            let op_span = tok.span;
            let op = UnaryOp::from(tok.kind);

            self.advance();

            let operand = self.parse_not_expression()?;

            let end = operand.span.end;

            return Ok(AstNode::unary(op, op_span, operand, start, end));
        }

        self.parse_boolean_expression()
    }

    pub(crate) fn parse_boolean_expression(&mut self) -> ParserRes {
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

    pub(crate) fn parse_arithmetic_expression(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_term,
            Self::parse_term,
            &[TokenKind::Plus, TokenKind::Hyphen],
        )
    }

    pub(crate) fn parse_term(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_factor,
            Self::parse_factor,
            &[TokenKind::Star, TokenKind::FwdSlash],
        )
    }

    pub(crate) fn parse_factor(&mut self) -> ParserRes {
        let start = self.peek(0).span.start;

        if let Some(tok) = self.matches_any(&[TokenKind::Plus, TokenKind::Hyphen]) {
            let op_span = tok.span;
            let op = UnaryOp::from(tok.kind);

            self.advance();

            let operand = self.parse_factor()?;

            let end = operand.span.end;

            return Ok(AstNode::unary(op, op_span, operand, start, end));
        }

        self.parse_postfix()
    }

    pub(crate) fn parse_postfix(&mut self) -> ParserRes {
        let mut operand = self.parse_atom()?;

        loop {
            match self.peek(0).kind {
                TokenKind::LeftParen => operand = self.parse_call_op(operand)?,
                _ => break,
            }
        }

        Ok(operand)
    }

    pub(crate) fn parse_atom(&mut self) -> ParserRes {
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

            TokenKind::LeftParen => {
                let open_paren_span = self.advance().span;

                let node = self.parse_or_expression()?;

                if self.matches(TokenKind::RightParen).is_none() {
                    return Err(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        class: DiagnosticClass::UnmatchedDelimiter,

                        msg: "unclosed parenthesized expression".into(),

                        primary: Label {
                            span: Span::new(start, self.peek(0).span.end),
                            msg: "expected ')'".into(),
                            paranthesise: false,
                        },

                        secondary: vec![Label {
                            span: open_paren_span,
                            msg: "'(' opened here".into(),
                            paranthesise: false,
                        }],

                        notes: vec![],
                    });
                }

                self.advance();

                return Ok(node);
            }

            kind => {
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::ExpectedExpression,

                    msg: "expected expression".into(),

                    primary: Label {
                        span: Span::new(start, self.peek(0).span.end),
                        msg: format!("found {}", kind.display()),
                            paranthesise: false,
                    },

                    secondary: vec![],
                    notes: vec!["an expression can be a literal, identifier, function call, or parenthesized expression".into(),],
                });
            }
        };

        self.advance();
        Ok(node)
    }
}
