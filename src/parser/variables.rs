use crate::{
    lexer::TokenKind,
    parser::{Parser, node::AstNode, parser::ParserRes},
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_ident_leading_statement(&mut self) -> ParserRes {
        self.expect(TokenKind::Identifier)?;

        let next_tok = self.peek(1);

        match next_tok.kind {
            TokenKind::Colon => self.parse_var_decl(),
            TokenKind::Equal => self.parse_var_assign(),
            _ => self.parse_expression(),
        }
    }

    pub(crate) fn parse_var_decl(&mut self) -> ParserRes {
        self.expect(TokenKind::Identifier)?;
        let name = self.advance().span;

        self.expect(TokenKind::Colon)?;
        self.advance();

        let mut var_type: Option<Box<AstNode>> = None;

        if self.peek(0).kind != TokenKind::Equal {
            var_type = Some(self.parse_type_expression()?);
        }

        self.expect(TokenKind::Equal)?;
        self.advance();

        let value = self.parse_expression()?;

        let start = name.start;

        Ok(AstNode::var_decl(
            name,
            var_type,
            value,
            start,
            self.peek(0).span.start,
        ))
    }

    pub(crate) fn parse_var_assign(&mut self) -> ParserRes {
        self.expect(TokenKind::Identifier)?;
        let name = self.advance().span;

        self.expect(TokenKind::Equal)?;
        self.advance();

        let value = self.parse_expression()?;

        let start = name.start;

        Ok(AstNode::var_assign(
            name,
            value,
            start,
            self.peek(0).span.end,
        ))
    }
}
