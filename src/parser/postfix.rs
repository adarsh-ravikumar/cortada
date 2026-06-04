use crate::{
    lexer::TokenKind,
    parser::{Parser, node::AstNode, parser::ParserRes},
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_call_op(&mut self, callee: Box<AstNode>) -> ParserRes {
        self.expect(TokenKind::LeftParen)?;
        self.advance();

        let start = callee.span.start;

        let mut args: Vec<Box<AstNode>> = Vec::new();

        if self.peek(0).kind == TokenKind::RightParen {
            return Ok(AstNode::call(callee, args, start, self.advance().span.end));
        }

        args.push(self.parse_expression()?);

        while self.peek(0).kind == TokenKind::Comma {
            self.advance();

            if self.peek(0).kind == TokenKind::RightParen {
                break;
            }

            args.push(self.parse_expression()?);
        }

        self.expect(TokenKind::RightParen)?;

        let end = self.advance().span.end;

        return Ok(AstNode::call(callee, args, start, end));
    }
}
