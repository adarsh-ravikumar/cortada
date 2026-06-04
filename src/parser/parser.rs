use crate::{
    common::Span,
    diagnostic::{Diagnostic, DiagnosticKind},
    lexer::{Token, TokenKind},
    parser::{
        BinaryOp, UnaryOp,
        node::{
            AstNode, AstNodeKind, BinaryExpr, CallExpr, ElifBranch, FloatExpr, FnStatement,
            IdentifierExpr, IfStatement, IntegerExpr, Param, Statements, UnaryExpr,
            VarAssignStatement, VarDeclStatement, WhileStatement,
        },
    },
    utils::IOFile,
};

pub struct Parser<'a> {
    src: &'a IOFile,
    tokens: &'a Vec<Token>,
    position: usize,
}

type ParserRes = Result<Box<AstNode>, Diagnostic>;

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

    fn matches_any(&self, pattern: &[TokenKind]) -> Option<&Token> {
        let cur = self.peek(0);
        if pattern.contains(&cur.kind) {
            Some(cur)
        } else {
            None
        }
    }

    fn matches(&self, kind: TokenKind) -> Option<&Token> {
        let cur = self.peek(0);
        if cur.kind == kind { Some(cur) } else { None }
    }

    fn expect(&self, kind: TokenKind) -> Result<(), Diagnostic> {
        let cur = self.peek(0);
        if cur.kind == kind {
            return Ok(());
        }

        Err(Diagnostic::new(
            DiagnosticKind::Error,
            format!(
                "[{}] Expected '{:?}', got {:?}",
                self.position,
                kind,
                self.peek(0).kind
            ),
            cur.span,
        ))
    }

    fn parse_binary_expr(
        &mut self,
        lhs_fn: fn(&mut Self) -> ParserRes,
        rhs_fn: fn(&mut Self) -> ParserRes,
        pattern: &[TokenKind],
    ) -> ParserRes {
        let start = self.peek(0).span.start;
        let mut lhs = lhs_fn(self)?;

        while let Some(tok) = self.matches_any(pattern) {
            let op = BinaryOp::from(tok.kind);
            self.advance();

            let rhs = rhs_fn(self)?;

            let end = rhs.span.end;

            lhs = Box::new(AstNode::new(
                AstNodeKind::Binary(BinaryExpr { lhs, op, rhs }),
                start,
                end,
            ))
        }

        Ok(lhs)
    }

    fn parse_statements(&mut self) -> ParserRes {
        self.skip_newlines();

        let mut stmts: Vec<Box<AstNode>> = Vec::new();

        let start = self.peek(0).span.start;

        loop {
            if let Some(_) = self.matches_any(&[TokenKind::EOF, TokenKind::Dedent]) {
                return Ok(Box::new(AstNode::new(
                    AstNodeKind::Statements(Statements { stmts }),
                    start,
                    self.peek(0).span.end,
                )));
            }

            stmts.push(self.parse_statement()?);

            self.skip_newlines();
        }
    }

    fn parse_statement(&mut self) -> ParserRes {
        self.skip_newlines();

        match self.peek(0).kind {
            TokenKind::KwrdFn => self.parse_fn_statement(),
            TokenKind::KwrdWhile => self.parse_while_statement(),
            TokenKind::KwrdIf => self.parse_if_statement(),
            TokenKind::KwrdReturn => self.parse_return_statement(),
            TokenKind::KwrdBreak => self.parse_break_statement(),
            TokenKind::KwrdContinue => self.parse_continue_statement(),
            TokenKind::Identifier => self.parse_ident_leading_statement(),
            _ => self.parse_expression(),
        }
    }

    fn parse_ident_leading_statement(&mut self) -> ParserRes {
        self.expect(TokenKind::Identifier)?;

        let next_tok = self.peek(1);

        match next_tok.kind {
            TokenKind::Colon => self.parse_var_decl(),
            TokenKind::Equal => self.parse_var_assign(),
            _ => self.parse_expression(),
        }
    }

    fn parse_var_decl(&mut self) -> ParserRes {
        self.expect(TokenKind::Identifier)?;
        let ident_tok = self.advance();
        let name = IdentifierExpr {
            span: ident_tok.span,
        };

        self.expect(TokenKind::Colon)?;
        self.advance();

        let mut var_type: Option<IdentifierExpr> = None;

        let mut value: Option<Box<AstNode>> = None;

        if self.peek(0).kind == TokenKind::Identifier {
            let tok = self.advance();
            var_type = Some(IdentifierExpr { span: tok.span });
        }

        if let Some(_) = self.matches(TokenKind::Equal) {
            self.advance();
            value = Some(self.parse_expression()?);
        }

        let start = name.span.start;

        Ok(Box::new(AstNode::new(
            AstNodeKind::VarDecl(VarDeclStatement {
                name,
                var_type,
                value,
            }),
            start,
            self.peek(0).span.start,
        )))
    }

    fn parse_var_assign(&mut self) -> ParserRes {
        self.expect(TokenKind::Identifier)?;
        let ident_tok = self.advance();
        let name = IdentifierExpr {
            span: ident_tok.span,
        };

        self.expect(TokenKind::Equal)?;
        self.advance();

        let value = self.parse_expression()?;

        let start = name.span.start;

        Ok(Box::new(AstNode::new(
            AstNodeKind::VarAssign(VarAssignStatement { name, value }),
            start,
            self.peek(0).span.start,
        )))
    }

    fn parse_fn_statement(&mut self) -> ParserRes {
        self.expect(TokenKind::KwrdFn)?;
        self.advance();

        self.expect(TokenKind::Identifier)?;
        let ident_tok = self.advance();
        let name = IdentifierExpr {
            span: ident_tok.span,
        };

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

        let mut return_type: Option<IdentifierExpr> = None;

        if self.peek(0).kind == TokenKind::ThinArrow {
            self.advance();

            self.expect(TokenKind::Identifier)?;
            let ident_tok = self.advance();

            return_type = Some(IdentifierExpr {
                span: ident_tok.span,
            });
        }

        let body = self.parse_suite()?;

        let start = name.span.start;
        let end = body.span.end;

        Ok(Box::new(AstNode::new(
            AstNodeKind::Fn(FnStatement {
                name,
                return_type,
                params,
                body,
            }),
            start,
            end,
        )))
    }

    fn parse_param(&mut self) -> Result<Param, Diagnostic> {
        self.expect(TokenKind::Identifier)?;
        let ident_tok = self.advance();
        let name = IdentifierExpr {
            span: ident_tok.span,
        };

        let mut param_type: Option<IdentifierExpr> = None;

        let mut default_value: Option<Box<AstNode>> = None;

        if let Some(_) = self.matches(TokenKind::Colon) {
            self.advance();

            self.expect(TokenKind::Identifier)?;
            param_type = Some(IdentifierExpr {
                span: self.advance().span,
            });
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

    fn parse_while_statement(&mut self) -> ParserRes {
        let cur = self.peek(0);

        let start = cur.span.start;

        self.expect(TokenKind::KwrdWhile)?;

        self.advance();

        let condition = self.parse_expression()?;

        let body = self.parse_suite()?;

        self.skip_newlines();

        if let Some(tok) = self.matches(TokenKind::KwrdElse) {
            let start = tok.span.start;

            self.advance();

            let else_stmt = self.parse_suite()?;

            let end = else_stmt.span.end;

            self.skip_newlines();

            return Ok(Box::new(AstNode::new(
                AstNodeKind::While(WhileStatement {
                    condition,
                    body,
                    else_stmt: Some(else_stmt),
                }),
                start,
                end,
            )));
        }

        Ok(Box::new(AstNode::new(
            AstNodeKind::While(WhileStatement {
                condition,
                body,
                else_stmt: None,
            }),
            start,
            self.peek(0).span.end,
        )))
    }

    fn parse_if_statement(&mut self) -> ParserRes {
        let mut elif_stmts: Vec<ElifBranch> = Vec::new();

        let cur = self.peek(0);

        let start = cur.span.start;

        self.expect(TokenKind::KwrdIf)?;

        self.advance();

        let condition = self.parse_expression()?;

        let body = self.parse_suite()?;

        self.skip_newlines();

        while let Some(_) = self.matches(TokenKind::KwrdElif) {
            self.advance();

            let condition = self.parse_expression()?;

            let body = self.parse_suite()?;

            self.skip_newlines();

            elif_stmts.push(ElifBranch { condition, body });
        }

        if let Some(tok) = self.matches(TokenKind::KwrdElse) {
            let start = tok.span.start;

            self.advance();

            let else_stmt = self.parse_suite()?;

            let end = else_stmt.span.end;

            self.skip_newlines();

            return Ok(Box::new(AstNode::new(
                AstNodeKind::If(IfStatement {
                    condition,
                    body,
                    elif_stmts,
                    else_stmt: Some(else_stmt),
                }),
                start,
                end,
            )));
        }

        Ok(Box::new(AstNode::new(
            AstNodeKind::If(IfStatement {
                condition,
                body,
                elif_stmts,
                else_stmt: None,
            }),
            start,
            self.peek(0).span.end,
        )))
    }

    fn parse_suite(&mut self) -> ParserRes {
        let mut stmts: Vec<Box<AstNode>> = Vec::new();

        let mut start = self.peek(0).span.start;

        self.expect(TokenKind::Colon)?;

        self.advance();

        self.skip_newlines();

        if self.peek(0).kind != TokenKind::Indent {
            return Err(Diagnostic::new(
                DiagnosticKind::Error,
                format!(
                    "[{}] Expected Indentation, got {:?}",
                    self.position,
                    self.peek(0).kind
                ),
                Span::new(start, self.peek(0).span.end),
            ));
        }

        self.advance();

        self.skip_newlines();

        start = self.peek(0).span.start;

        loop {
            if let Some(_) = self.matches(TokenKind::Dedent) {
                self.advance();

                return Ok(Box::new(AstNode::new(
                    AstNodeKind::Statements(Statements { stmts }),
                    start,
                    self.peek(0).span.end,
                )));
            }

            stmts.push(self.parse_statement()?);

            self.skip_newlines();

            if let Some(tok) = self.matches(TokenKind::Indent) {
                return Err(Diagnostic::new(
                    DiagnosticKind::Error,
                    format!("[{}] Unexpected Indent", self.position),
                    tok.span,
                ));
            }
        }
    }

    fn parse_return_statement(&mut self) -> ParserRes {
        self.expect(TokenKind::KwrdReturn)?;
        let start = self.advance().span.start;

        if self.peek(0).kind == TokenKind::Newline {
            return Ok(Box::new(AstNode::new(
                AstNodeKind::Return(None),
                start,
                self.peek(0).span.end,
            )));
        }

        let expr = self.parse_expression()?;

        let end = expr.span.end;

        Ok(Box::new(AstNode::new(
            AstNodeKind::Return(Some(expr)),
            start,
            end,
        )))
    }

    fn parse_break_statement(&mut self) -> ParserRes {
        self.expect(TokenKind::KwrdBreak)?;
        let tok = self.advance();

        match self.peek(0).kind {
            TokenKind::Newline | TokenKind::Dedent | TokenKind::EOF => {}

            t => {
                return Err(Diagnostic::new(
                    DiagnosticKind::Error,
                    format!(
                        "[{}] Expected terminator after continue, got {:?}",
                        self.position, t
                    ),
                    self.peek(0).span,
                ));
            }
        }

        Ok(Box::new(AstNode::new(
            AstNodeKind::Break,
            tok.span.start,
            tok.span.end,
        )))
    }

    fn parse_continue_statement(&mut self) -> ParserRes {
        self.expect(TokenKind::KwrdContinue)?;
        let tok = self.advance();

        match self.peek(0).kind {
            TokenKind::Newline | TokenKind::Dedent | TokenKind::EOF => {}

            t => {
                return Err(Diagnostic::new(
                    DiagnosticKind::Error,
                    format!(
                        "[{}] Expected terminator after continue, got {:?}",
                        self.position, t
                    ),
                    self.peek(0).span,
                ));
            }
        }

        Ok(Box::new(AstNode::new(
            AstNodeKind::Continue,
            tok.span.start,
            tok.span.end,
        )))
    }

    fn parse_expression(&mut self) -> ParserRes {
        self.parse_or_expression()
    }

    fn parse_or_expression(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_and_expression,
            Self::parse_and_expression,
            &[TokenKind::KwrdOr],
        )
    }

    fn parse_and_expression(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_not_expression,
            Self::parse_not_expression,
            &[TokenKind::KwrdAnd],
        )
    }

    fn parse_not_expression(&mut self) -> ParserRes {
        let start = self.peek(0).span.start;

        if let Some(tok) = self.matches(TokenKind::KwrdNot) {
            let op = UnaryOp::from(tok.kind);

            self.advance();

            let operand = self.parse_not_expression()?;

            let end = operand.span.end;

            return Ok(Box::new(AstNode::new(
                AstNodeKind::Unary(UnaryExpr { op, operand }),
                start,
                end,
            )));
        }

        self.parse_boolean_expression()
    }

    fn parse_boolean_expression(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_arithmetic_expression,
            Self::parse_arithmetic_expression,
            &[
                TokenKind::LeftAngle,
                TokenKind::LesserEqual,
                TokenKind::RightAngle,
                TokenKind::GreaterEqual,
                TokenKind::DoubleEqual,
                TokenKind::NotEqual,
            ],
        )
    }

    fn parse_arithmetic_expression(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_term,
            Self::parse_term,
            &[TokenKind::Plus, TokenKind::Hyphen],
        )
    }

    fn parse_term(&mut self) -> ParserRes {
        self.parse_binary_expr(
            Self::parse_factor,
            Self::parse_factor,
            &[TokenKind::Star, TokenKind::FwdSlash],
        )
    }

    fn parse_factor(&mut self) -> ParserRes {
        let start = self.peek(0).span.start;

        if let Some(tok) = self.matches_any(&[TokenKind::Plus, TokenKind::Hyphen]) {
            let op = UnaryOp::from(tok.kind);

            self.advance();

            let operand = self.parse_factor()?;

            let end = operand.span.end;

            return Ok(Box::new(AstNode::new(
                AstNodeKind::Unary(UnaryExpr { op, operand }),
                start,
                end,
            )));
        }

        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> ParserRes {
        let mut operand = self.parse_atom()?;

        loop {
            match self.peek(0).kind {
                TokenKind::LeftParen => operand = self.parse_call_expr(operand)?,
                _ => break,
            }
        }

        Ok(operand)
    }

    fn parse_call_expr(&mut self, callee: Box<AstNode>) -> ParserRes {
        self.expect(TokenKind::LeftParen)?;
        self.advance();

        let start = callee.span.start;

        let mut args: Vec<Box<AstNode>> = Vec::new();

        if self.peek(0).kind == TokenKind::RightParen {
            let t = self.advance();
            return Ok(Box::new(AstNode::new(
                AstNodeKind::Call(CallExpr { callee, args }),
                start,
                t.span.end,
            )));
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

        return Ok(Box::new(AstNode::new(
            AstNodeKind::Call(CallExpr { callee, args }),
            start,
            end,
        )));
    }

    fn parse_atom(&mut self) -> ParserRes {
        let next_tok = self.peek(0);

        let start = next_tok.span.start;

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
                let node = self.parse_or_expression()?;

                if self.matches(TokenKind::RightParen).is_none() {
                    return Err(Diagnostic::new(
                        DiagnosticKind::Error,
                        format!(
                            "[{}] Expected ')', got {:?}",
                            self.position,
                            self.peek(0).kind
                        ),
                        Span::new(start, self.peek(0).span.end),
                    ));
                }

                self.advance();

                return Ok(node);
            }

            kind => {
                return Err(Diagnostic::new(
                    DiagnosticKind::Error,
                    format!(
                        "[{}] Expected int, float or identifier, got {:?}",
                        self.position, kind
                    ),
                    Span::new(start, self.peek(0).span.end),
                ));
            }
        };

        self.advance();

        Ok(Box::new(AstNode::new(kind, start, self.peek(0).span.end)))
    }

    pub fn parse(&mut self) -> ParserRes {
        self.skip_newlines();

        let res = self.parse_statements()?;

        self.skip_newlines();

        let cur = self.peek(0);

        if cur.kind != TokenKind::EOF {
            return Err(Diagnostic::new(
                DiagnosticKind::Error,
                format!("[{}] Expected EOF, Got {:?}", self.position, cur.kind),
                Span::new(self.position, self.position),
            ));
        }

        Ok(res)
    }
}
