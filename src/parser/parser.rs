use std::env::join_paths;

use crate::{
    common::Span,
    diagnostic::{Diagnostic, DiagnosticKind},
    lexer::{Token, TokenKind},
    parser::{
        BinaryOp, UnaryOp,
        node::{
            AstNode, AstNodeKind, BinaryExpr, FloatExpr, IdentifierExpr, IntegerExpr, StmtsExpr,
            UnaryExpr,
        },
    },
    utils::IOFile,
};

pub struct Parser<'a> {
    src: &'a IOFile,
    tokens: &'a Vec<Token>,
    position: usize,
}

type ParserRes = Result<Box<AstNode>, Diagnostic>;

impl<'a> Parser<'a> {
    pub fn new(file: &'a IOFile, tokens: &'a Vec<Token>) -> Self {
        Self {
            src: file,
            tokens,
            position: 0,
        }
    }

    fn peek(&self, by: usize) -> &'a Token {
        self.tokens
            .get(self.position + by)
            .unwrap_or(self.tokens.last().unwrap())
    }

    fn advance_by(&mut self, by: usize) -> &'a Token {
        let next = self.peek(0);

        if next.kind != TokenKind::EOF {
            self.position += by;
        }

        next
    }

    fn advance(&mut self) -> &'a Token {
        self.advance_by(1)
    }

    fn skip_newlines(&mut self) {
        while self.peek(0).kind == TokenKind::Newline {
            self.advance();
        }
    }

    fn matches_any(&self, pattern: &[TokenKind]) -> Option<&Token> {
        let cur = self.peek(0);
        if pattern.contains(&cur.kind) {
            Some(cur)
        } else {
            None
        }
    }

    fn matches(&self, kind: TokenKind) -> Option<&Token> {
        let cur = self.peek(0);
        if cur.kind == kind { Some(cur) } else { None }
    }

    fn parse_binary_expr(
        &mut self,
        lhs_fn: fn(&mut Self) -> ParserRes,
        rhs_fn: fn(&mut Self) -> ParserRes,
        pattern: &[TokenKind],
    ) -> ParserRes {
        let start = self.peek(0).span.start;
        let mut lhs = lhs_fn(self)?;

        while let Some(tok) = self.matches_any(pattern) {
            let op = BinaryOp::from(tok.kind);
            self.advance();

            let rhs = rhs_fn(self)?;

            let end = rhs.span.end;

            lhs = Box::new(AstNode::new(
                AstNodeKind::Binary(BinaryExpr { lhs, op, rhs }),
                start,
                end,
            ))
        }

        Ok(lhs)
    }

    fn parse_statements(&mut self) -> ParserRes {
        self.skip_newlines();

        let mut stmts: Vec<Box<AstNode>> = Vec::new();

        let start = self.peek(0).span.start;

        loop {
            self.skip_newlines();

            if let Some(_) = self.matches(TokenKind::EOF) {
                return Ok(Box::new(AstNode::new(
                    AstNodeKind::Statements(StmtsExpr { stmts }),
                    start,
                    self.peek(0).span.end,
                )));
            }

            stmts.push(self.parse_statement()?);

            if self.matches(TokenKind::Newline).is_none() {
                return Err(Diagnostic::new(
                    DiagnosticKind::Error,
                    format!("Expected newline, got {:?}", self.peek(0).kind),
                    Span::new(start, self.peek(0).span.end),
                ));
            }
        }
    }

    fn parse_statement(&mut self) -> ParserRes {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_term,
            Self::parse_term,
            &[TokenKind::Plus, TokenKind::Hyphen],
        )
    }

    fn parse_term(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_factor,
            Self::parse_factor,
            &[TokenKind::Star, TokenKind::FwdSlash],
        )
    }

    fn parse_factor(&mut self) -> ParserRes {
        let start = self.peek(0).span.start;

        if let Some(tok) = self.matches_any(&[TokenKind::Plus, TokenKind::Hyphen]) {
            let op = UnaryOp::from(tok.kind);

            self.advance();

            let rhs = self.parse_factor()?;

            let end = rhs.span.end;

            return Ok(Box::new(AstNode::new(
                AstNodeKind::Unary(UnaryExpr { op, rhs }),
                start,
                end,
            )));
        }

        self.parse_atom()
    }

    fn parse_atom(&mut self) -> ParserRes {
        let next_tok = self.peek(0);

        let start = next_tok.span.start;

        let kind: AstNodeKind = match next_tok.kind {
            TokenKind::Integer => {
                let num = self.src.view_span(next_tok.span);
                AstNodeKind::Integer(IntegerExpr {
                    value: num.parse().unwrap(),
                })
            }

            TokenKind::Float => {
                let num = self.src.view_span(next_tok.span);
                AstNodeKind::Float(FloatExpr {
                    value: num.parse().unwrap(),
                })
            }

            TokenKind::Identifier => AstNodeKind::Identifier(IdentifierExpr {
                span: next_tok.span,
            }),

            TokenKind::LeftParen => {
                self.advance();
                let node = self.parse_expression()?;

                if self.matches(TokenKind::RightParen).is_none() {
                    return Err(Diagnostic::new(
                        DiagnosticKind::Error,
                        format!("Expected ')', got {:?}", self.peek(0).kind),
                        Span::new(start, self.peek(0).span.end),
                    ));
                }

                self.advance();

                return Ok(node);
            }

            kind => {
                return Err(Diagnostic::new(
                    DiagnosticKind::Error,
                    format!("Expected int, float or identifier, got {:?}", kind),
                    Span::new(start, self.peek(0).span.end),
                ));
            }
        };

        self.advance();

        Ok(Box::new(AstNode::new(kind, start, self.peek(0).span.end)))
    }

    pub fn parse(&mut self) -> ParserRes {
        self.skip_newlines();

        let res = self.parse_statements()?;

        self.skip_newlines();

        let cur = self.peek(0);

        if cur.kind != TokenKind::EOF {
            return Err(Diagnostic::new(
                DiagnosticKind::Error,
                format!("Expected EOF, Got {:?}", cur.kind),
                Span::new(self.position, self.position),
            ));
        }

        Ok(res)
    }
}
