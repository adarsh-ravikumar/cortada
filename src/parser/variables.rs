use crate::{
    common::Span,
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

        let start = name.start;

        let var_type: Option<Box<AstNode>>;

        if !self.expect(TokenKind::Colon) {
            return AstNode::error();
        }

        let value: Option<Box<AstNode>>;
        let value_span: Option<Span>;

        // "=" expression
        if self.peek(0).kind == TokenKind::Equal {
            self.advance();
            var_type = None;

            let expr = self.parse_expression();
            value_span = Some(expr.span);
            value = Some(expr);
        } else {
            // type_expression ("=" expression)?
            var_type = Some(self.parse_type_expression());

            if self.peek(0).kind != TokenKind::Equal {
                value_span = None;
                value = None;
            } else {
                self.advance();

                let expr = self.parse_expression();
                value_span = Some(expr.span);
                value = Some(expr);
            }

            self.advance();
        }

        AstNode::var_decl(
            name,
            var_type,
            value_span,
            value,
            start,
            self.peek(0).span.start,
        )
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
