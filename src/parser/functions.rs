use crate::{
    lexer::TokenKind,
    parser::{Parser, node::AstNode, parser::ParserRes},
};

impl<'a> Parser<'a> {
    pub fn parse_fn_statement(&mut self) -> ParserRes {
        let start = self.peek(0).span.start;

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

        let mut params: Vec<Box<AstNode>> = Vec::new();

        if self.peek(0).kind != TokenKind::RightParen {
            params.push(self.parse_var_decl());

            while self.peek(0).kind == TokenKind::Comma {
                self.advance();

                if self.peek(0).kind == TokenKind::RightParen {
                    break;
                }

                params.push(self.parse_var_decl());
            }
        }

        self.expect(TokenKind::RightParen);
        self.advance();

        let mut return_type: Option<Box<AstNode>> = None;

        if self.peek(0).kind == TokenKind::ThinArrow {
            self.advance();

            return_type = Some(self.parse_type_expression());
        }

        let end = self.peek(0).span.end;

        let body = self.parse_suite();

        AstNode::fn_stmt(name, return_type, params, body, start, end)
    }
}
