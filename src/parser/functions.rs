use crate::{
    common::Span,
    diagnostic::Diagnostic,
    lexer::TokenKind,
    parser::{
        Parser,
        node::{AstNode, Param},
        parser::ParserRes,
    },
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_fn_statement(&mut self) -> ParserRes {
        self.expect(TokenKind::KwrdFn)?;
        self.advance();

        self.expect(TokenKind::Identifier)?;
        let name = self.advance().span;

        self.expect(TokenKind::LeftParen)?;
        self.advance();

        let mut params: Vec<Param> = Vec::new();

        if self.peek(0).kind != TokenKind::RightParen {
            params.push(self.parse_param()?);

            while self.peek(0).kind == TokenKind::Comma {
                self.advance();

                if self.peek(0).kind == TokenKind::RightParen {
                    break;
                }

                params.push(self.parse_param()?);
            }
        }

        self.expect(TokenKind::RightParen)?;
        self.advance();

        let mut return_type: Option<Span> = None;

        if self.peek(0).kind == TokenKind::ThinArrow {
            self.advance();

            self.expect(TokenKind::Identifier)?;
            return_type = Some(self.advance().span);
        }

        let body = self.parse_suite()?;

        let start = name.start;
        let end = body.span.end;

        Ok(AstNode::fn_stmt(
            name,
            return_type,
            params,
            body,
            start,
            end,
        ))
    }

    pub(crate) fn parse_param(&mut self) -> Result<Param, Diagnostic> {
        self.expect(TokenKind::Identifier)?;
        let name = self.advance().span;

        let mut param_type: Option<Span> = None;

        let mut default_value: Option<Box<AstNode>> = None;

        if let Some(_) = self.matches(TokenKind::Colon) {
            self.advance();

            self.expect(TokenKind::Identifier)?;
            param_type = Some(self.advance().span);
        }

        if let Some(_) = self.matches(TokenKind::Equal) {
            self.advance();
            default_value = Some(self.parse_expression()?);
        }

        Ok(Param {
            name,
            param_type,
            default_value,
        })
    }
}
