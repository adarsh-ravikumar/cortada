use crate::{
    common::Span,
    diagnostic::{Diagnostic, DiagnosticKind},
    lexer::{Token, TokenKind},
    utils::IOFile,
};

pub struct Lexer<'a> {
    src: &'a IOFile,
    position: usize,
    indentation: Vec<usize>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a IOFile) -> Lexer<'a> {
        Self {
            src,
            position: 0,
            indentation: vec![0],
        }
    }

    fn advance(&mut self) -> u8 {
        self.advance_by(1)
    }

    fn advance_by(&mut self, by: usize) -> u8 {
        let next = self.peek(0);

        if next != b'\0' {
            self.position += by;
        }

        next
    }

    fn match_char(&mut self, expected: u8) -> bool {
        if self.peek(0) == expected {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self, by: usize) -> u8 {
        self.src.get(self.position + by).copied().unwrap_or(b'\0')
    }

    fn is_at_end(&self) -> bool {
        self.peek(0) == b'\0'
    }

    fn tokenize_number(&mut self, start: usize) -> Token {
        let mut seen_dot = false;

        loop {
            match self.peek(0) {
                b'0'..=b'9' => {
                    self.advance();
                }

                b'.' => {
                    if !seen_dot && self.peek(1).is_ascii_digit() {
                        self.advance();
                        seen_dot = true;
                    } else {
                        break;
                    }
                }

                _ => break,
            }
        }

        let kind = if seen_dot {
            TokenKind::Float
        } else {
            TokenKind::Integer
        };

        Token::new(kind, start, self.position)
    }

    fn tokenize_identifier(&mut self, start: usize) -> Token {
        loop {
            if matches!(self.peek(0), b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9') {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = self.src.view(start, self.position);

        let kind = TokenKind::keyword(lexeme);

        Token::new(kind, start, self.position)
    }

    fn handle_comment(&mut self, start: usize) -> Result<(), Diagnostic> {
        // multi-line
        if self.peek(0) == b'~' && self.peek(1) == b'~' {
            self.advance_by(2);

            loop {
                if self.is_at_end() {
                    return Err(Diagnostic::new(
                        DiagnosticKind::Error,
                        "Unterminated multi-line comment".to_string(),
                        Span::new(start, self.position),
                    ));
                }

                if self.peek(0) == b'~' && self.peek(1) == b'~' && self.peek(2) == b'~' {
                    self.advance_by(3);
                    break;
                }

                self.advance();
            }

            return Ok(());
        }

        // single line
        while self.peek(0) != b'\n' && self.peek(0) != b'\0' {
            self.advance();
        }

        self.advance();

        Ok(())
    }

    fn next_token(&mut self) -> Result<Option<Token>, Diagnostic> {
        let start = self.position;

        if self.is_at_end() {
            return Ok(Some(Token::new(TokenKind::EOF, start, start)));
        }

        let ch = self.advance();

        if ch == b' ' {
            return Ok(None);
        }

        let tok_type = match ch {
            b':' => TokenKind::Colon,
            b',' => TokenKind::Comma,
            b'(' => TokenKind::LeftParen,
            b')' => TokenKind::RightParen,
            b'[' => TokenKind::LeftBracket,
            b']' => TokenKind::RightBracket,
            b'{' => TokenKind::LeftBrace,
            b'}' => TokenKind::RightBrace,
            b'*' => {
                if self.match_char(b'*') {
                    TokenKind::DoubleStar
                } else {
                    TokenKind::Star
                }
            }
            b'/' => TokenKind::FwdSlash,
            b'+' => TokenKind::Plus,
            b'-' => {
                if self.match_char(b'>') {
                    TokenKind::ThinArrow
                } else {
                    TokenKind::Hyphen
                }
            }
            b'.' => {
                if self.match_char(b'.') {
                    TokenKind::DoubleDot
                } else {
                    TokenKind::Dot
                }
            }
            b'<' => {
                if self.match_char(b'=') {
                    TokenKind::LesserEqual
                } else {
                    TokenKind::LeftAngle
                }
            }
            b'>' => {
                if self.match_char(b'=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::RightAngle
                }
            }
            b'=' => {
                if self.match_char(b'=') {
                    TokenKind::DoubleEqual
                } else if self.match_char(b'>') {
                    TokenKind::FatArrow
                } else {
                    TokenKind::Equal
                }
            }

            b'!' => {
                if self.match_char(b'=') {
                    TokenKind::NotEqual
                } else {
                    return Err(Diagnostic::new(
                        DiagnosticKind::Error,
                        "Unkown symbol '!'".to_string(),
                        Span::new(start, self.position),
                    ));
                }
            }

            b'\n' => TokenKind::Newline,

            b'0'..=b'9' => return Ok(Some(self.tokenize_number(start))),

            b'a'..=b'z' | b'A'..=b'Z' | b'_' => return Ok(Some(self.tokenize_identifier(start))),

            b'~' => {
                self.handle_comment(start)?;
                return Ok(None);
            }

            ch => {
                return Err(Diagnostic::new(
                    DiagnosticKind::Error,
                    format!("Unknown symbol '{}'", char::from(ch)),
                    Span::new(start, self.position),
                ));
            }
        };

        Ok(Some(Token::new(tok_type, start, self.position)))
    }

    fn track_indent(&mut self) -> Result<Vec<Token>, Diagnostic> {
        let start = self.position;

        let mut tokens: Vec<Token> = Vec::new();

        let mut indent: usize = 0;

        let starting_char = self.peek(0);

        if starting_char != b' '
            && starting_char != b'\t'
            && starting_char != b'\n'
            && self.indentation.is_empty()
        {
            return Ok(tokens);
        }

        let char_to_diagnostics = |ch: u8| if ch == b' ' { "<space>" } else { "<tab>" };

        loop {
            match self.peek(0) {
                ch if ch == b' ' || ch == b'\t' => {
                    if ch != starting_char {
                        return Err(Diagnostic::new(
                            DiagnosticKind::Error,
                            format!(
                                "Indentation cannot be mixed. Expected {}, got {}",
                                char_to_diagnostics(starting_char),
                                char_to_diagnostics(ch)
                            ),
                            Span::new(start, self.position),
                        ));
                    }

                    indent += if ch == b' ' { 1 } else { 4 };
                }

                b'\n' => return Ok(tokens),
                _ => break,
            }

            self.advance();
        }

        let current = *self.indentation.last().unwrap();

        if indent == current {
            return Ok(tokens);
        }

        if indent > current {
            self.indentation.push(indent);
            tokens.push(Token::new(TokenKind::Indent, start, self.position))
        } else {
            while let Some(&top) = self.indentation.last() {
                if indent >= top {
                    break;
                }

                self.indentation.pop();
                tokens.push(Token::new(TokenKind::Dedent, start, self.position))
            }

            let last = *self.indentation.last().unwrap();

            if indent != last {
                return Err(Diagnostic::new(
                    DiagnosticKind::Error,
                    format!(
                        "Invalid indentation level. Expected one of {}, got {indent}",
                        self.indentation
                            .iter()
                            .map(|i| i.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    ),
                    Span::new(start, self.position),
                ));
            }
        }

        Ok(tokens)
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Diagnostic> {
        let mut tokens: Vec<Token> = self.track_indent()?;

        loop {
            let token = match self.next_token()? {
                None => continue,
                Some(t) => t,
            };

            let kind = token.kind.clone();

            tokens.push(token);

            match kind {
                TokenKind::EOF => {
                    let eof = tokens.pop().unwrap();

                    while let Some(v) = self.indentation.pop() {
                        if v == 0 {
                            continue;
                        }

                        tokens.push(Token::new(TokenKind::Dedent, self.position, self.position));
                    }

                    tokens.push(eof);

                    return Ok(tokens);
                }

                TokenKind::Newline => tokens.extend(self.track_indent()?),

                _ => continue,
            }
        }
    }
}
