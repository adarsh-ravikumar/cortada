use std::fmt::Binary;

use crate::{
    common::Span,
    diagnostic::{Diagnostic, DiagnosticKind},
    lexer::{Token, TokenKind},
    parser::node::{
        AstNode, AstNodeKind, BinaryExpr, BinaryOp, FloatExpr, IdentifierExpr, IntegerExpr,
    },
    utils::IOFile,
};

pub struct Parser<'a> {
    src: &'a IOFile,
    tokens: &'a Vec<Token>,
    position: usize,
}

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

    fn matches_any(&self, pattern: &[TokenKind]) -> bool {
        pattern.contains(&self.peek(0).kind)
    }

    fn matches(&self, kind: TokenKind) -> bool {
        self.peek(0).kind == kind
    }

    fn parse_expression(&mut self) -> Result<Box<AstNode>, Diagnostic> {
        let start = self.position;
        let mut lhs = self.parse_term()?;

        while self.matches_any(&[TokenKind::Plus, TokenKind::Hyphen]) {
            let op = match self.advance().kind {
                TokenKind::Plus => Some(BinaryOp::Add),
                TokenKind::Hyphen => Some(BinaryOp::Subtract),
                _ => None,
            }
            .unwrap();

            let rhs = self.parse_term()?;

            lhs = Box::new(AstNode::new(
                AstNodeKind::Binary(BinaryExpr { lhs, op, rhs }),
                start,
                self.position,
            ))
        }

        Ok(lhs)
    }

    fn parse_term(&mut self) -> Result<Box<AstNode>, Diagnostic> {
        let start = self.position;
        let mut lhs = self.parse_factor()?;

        while self.matches_any(&[TokenKind::Star, TokenKind::FwdSlash]) {
            let op = match self.advance().kind {
                TokenKind::Star => Some(BinaryOp::Multiply),
                TokenKind::FwdSlash => Some(BinaryOp::Divide),
                _ => None,
            }
            .unwrap();

            let rhs = self.parse_factor()?;

            lhs = Box::new(AstNode::new(
                AstNodeKind::Binary(BinaryExpr { lhs, op, rhs }),
                start,
                self.position,
            ))
        }

        Ok(lhs)
    }

    fn parse_factor(&mut self) -> Result<Box<AstNode>, Diagnostic> {
        self.skip_newlines();

        let next_tok = self.peek(0);

        let start = self.position;

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

                if !self.matches(TokenKind::RightParen) {
                    return Err(Diagnostic::new(
                        DiagnosticKind::Error,
                        format!("Expected ')', got {:?}", self.peek(0).kind),
                        Span::new(start, self.position),
                    ));
                }

                self.advance();

                return Ok(node);
            }

            kind => {
                return Err(Diagnostic::new(
                    DiagnosticKind::Error,
                    format!("Expected int, float or identifier, got {:?}", kind),
                    Span::new(start, self.position),
                ));
            }
        };

        self.advance();

        Ok(Box::new(AstNode::new(kind, start, self.position)))
    }

    pub fn parse(&mut self) -> Result<Box<AstNode>, Diagnostic> {
        self.skip_newlines();

        let res = self.parse_expression()?;

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
