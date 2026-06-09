use crate::{
    lexer::TokenKind,
    parser::{
        Parser,
        node::{AstNode, Param},
        parser::ParserRes,
    },
};

impl<'a> Parser<'a> {
    pub fn parse_fn_statement(&mut self) -> ParserRes {
        if !self.expect(TokenKind::KwrdFn) {
            return AstNode::error();
        }

        self.advance();

        if !self.expect_identifier("function name") {
            return AstNode::error();
        }

        let name = self.advance().span;

        if !self.expect(TokenKind::LeftParen) {
            return AstNode::error();
        }

        self.advance();

        let mut params: Vec<Param> = Vec::new();

        if self.peek(0).kind != TokenKind::RightParen {
            if let Some(param) = self.parse_param() {
                params.push(param);
            }

            while self.peek(0).kind == TokenKind::Comma {
                self.advance();

                if self.peek(0).kind == TokenKind::RightParen {
                    break;
                }

                if let Some(param) = self.parse_param() {
                    params.push(param);
                }
            }
        }

        self.expect(TokenKind::RightParen);
        self.advance();

        let mut return_type: Option<Box<AstNode>> = None;

        if self.peek(0).kind == TokenKind::ThinArrow {
            self.advance();

            return_type = Some(self.parse_type_expression());
        }

        let body = self.parse_suite();

        let start = name.start;
        let end = body.span.end;

        AstNode::fn_stmt(name, return_type, params, body, start, end)
    }

    pub fn parse_param(&mut self) -> Option<Param> {
        let name = if !self.expect_identifier("parameter name") {
            self.recover(|kind| {
                matches!(
                    kind,
                    TokenKind::Comma | TokenKind::RightParen | TokenKind::EOF | TokenKind::Colon
                )
            });
            self.peek(0).span
        } else {
            self.advance().span
        };

        let mut param_type: Option<Box<AstNode>> = None;

        let mut default_value: Option<Box<AstNode>> = None;

        if let Some(_) = self.matches(TokenKind::Colon) {
            self.advance();

            param_type = Some(self.parse_type_expression());
        }

        if let Some(_) = self.matches(TokenKind::Equal) {
            self.advance();
            default_value = Some(self.parse_expression());
        }

        Some(Param {
            name,
            param_type,
            default_value,
        })
    }
}
