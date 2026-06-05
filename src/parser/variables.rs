use crate::{
    common::Span,
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

        let mut var_type: Option<Span> = None;

        if self.peek(0).kind == TokenKind::Identifier {
            var_type = Some(self.advance().span);
        }

        let value: Option<Box<AstNode>>;

        let value_span: Span;

        self.expect(TokenKind::Equal)?;
        self.advance();

        if let Some(tok) = self.matches(TokenKind::KwrdNull) {
            value = None;
            value_span = tok.span;
            self.advance();
        } else {
            let expr = self.parse_expression()?;
            value_span = expr.span;
            value = Some(expr);
        }

        let start = name.start;

        Ok(AstNode::var_decl(
            name,
            var_type,
            value,
            value_span,
            start,
            self.peek(0).span.end,
        ))
    }

    pub(crate) fn parse_var_assign(&mut self) -> ParserRes {
        self.expect(TokenKind::Identifier)?;
        let name = self.advance().span;

        self.expect(TokenKind::Equal)?;
        self.advance();

        let value_span: Span;

        let value = if let Some(tok) = self.matches(TokenKind::KwrdNull) {
            value_span = tok.span;
            self.advance();

            None
        } else {
            let expr = self.parse_expression()?;
            value_span = expr.span;
            Some(expr)
        };

        let start = name.start;

        Ok(AstNode::var_assign(
            name,
            value,
            value_span,
            start,
            self.peek(0).span.end,
        ))
    }
}
