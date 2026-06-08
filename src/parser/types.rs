use crate::{
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label},
    lexer::TokenKind,
    parser::{
        Parser,
        node::{AstNode, TypePrimaryKind},
        parser::ParserRes,
    },
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_type_expression(&mut self) -> ParserRes {
        self.parse_type_union()
    }

    pub(crate) fn parse_type_union(&mut self) -> ParserRes {
        let primary = self.parse_type_primary()?;

        let start = primary.span.start;

        if self.peek(0).kind != TokenKind::Pipe {
            return Ok(primary);
        }

        let mut variants = vec![primary];

        while self.peek(0).kind == TokenKind::Pipe {
            self.advance();
            variants.push(self.parse_type_primary()?);
        }

        let end = variants.last().unwrap().span.end;

        Ok(AstNode::type_union(variants, start, end))
    }

    pub(crate) fn parse_type_primary(&mut self) -> ParserRes {
        let cur_tok = self.peek(0);

        let start;
        let end;

        let kind = match cur_tok.kind {
            TokenKind::KwrdInt => {
                start = cur_tok.span.start;
                end = cur_tok.span.end;
                TypePrimaryKind::Integer
            }

            TokenKind::KwrdFloat => {
                start = cur_tok.span.start;
                end = cur_tok.span.end;
                TypePrimaryKind::Float
            }

            TokenKind::KwrdBool => {
                start = cur_tok.span.start;
                end = cur_tok.span.end;
                TypePrimaryKind::Bool
            }

            _ => {
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::InvalidTypeExpression,

                    msg: "expected a type expression".into(),

                    primary: Label {
                        span: cur_tok.span,
                        msg: "expected a type expression here".into(),
                        paranthesise: false,
                    },

                    secondary: vec![],

                    notes: vec!["a type expression may be a built-in type or an identifier".into()],
                });
            }
        };

        self.advance();

        Ok(AstNode::type_primary(kind, start, end))
    }
}
