use crate::{
    lexer::TokenKind,
    parser::{Parser, node::AstNode, parser::ParserRes},
};

impl<'a> Parser<'a> {
    pub fn parse_ident_leading_statement(&mut self) -> ParserRes {
        self.expect(TokenKind::Identifier);

        let next_tok = self.peek(1);

        match next_tok.kind {
            TokenKind::Colon => self.parse_var_decl(),
            TokenKind::Equal => self.parse_var_assign(),
            _ => self.parse_expression(),
        }
    }

    pub fn parse_var_decl(&mut self) -> ParserRes {
        if !self.expect(TokenKind::Identifier) {
            return AstNode::error();
        }
        let name = self.advance().span;

        if !self.expect(TokenKind::Colon) {
            return AstNode::error();
        }
        self.advance();

        let mut var_type: Option<Box<AstNode>> = None;

        if self.peek(0).kind != TokenKind::Equal {
            var_type = Some(self.parse_type_expression());
        }

        self.expect(TokenKind::Equal);
        self.advance();

        let value = self.parse_expression();

        let start = name.start;

        AstNode::var_decl(name, var_type, value, start, self.peek(0).span.start)
    }

    pub fn parse_var_assign(&mut self) -> ParserRes {
        if !self.expect(TokenKind::Identifier) {
            return AstNode::error();
        }
        let name = self.advance().span;

        self.expect(TokenKind::Equal);
        self.advance();

        let value = self.parse_expression();

        let start = name.start;

        AstNode::var_assign(name, value, start, self.peek(0).span.end)
    }
}
