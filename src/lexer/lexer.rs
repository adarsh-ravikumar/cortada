use crate::{
    common::{IOFile, Span},
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label},
    lexer::{Token, TokenKind},
};

struct Delimiter {
    ch: u8,
    start: usize,
}

impl Delimiter {
    fn pair(ch: u8) -> u8 {
        match ch {
            b'(' => b')',
            b'[' => b']',
            b'{' => b'}',
            _ => unreachable!(),
        }
    }
}

pub struct Lexer<'a> {
    src: &'a IOFile,
    position: usize,
    indentation: Vec<usize>,
    delimiter: Vec<Delimiter>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a IOFile) -> Lexer<'a> {
        Self {
            src,
            position: 0,
            indentation: vec![0],
            delimiter: Vec::new(),
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
            let start: Span = Span::new(start, self.position + 1);

            self.advance_by(2);

            loop {
                if self.is_at_end() {
                    return Err(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        class: DiagnosticClass::UnmatchedDelimiter,
                        msg: "unclosed multi-line comment".into(),
                        primary: Label {
                            span: Span::new(self.position - 1, self.position - 1),
                            msg: "expected closing '~~~' before end of file".into(),
                            paranthesise: false,
                        },
                        secondary: vec![Label {
                            span: start,
                            msg: "comment started here".into(),
                            paranthesise: false,
                        }],
                        notes: vec!["multi-line comments must be terminated with '~~~'".into()],
                    });
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
            b'|' => TokenKind::Pipe,

            b'(' => {
                self.delimiter.push(Delimiter { ch, start });
                TokenKind::LeftParen
            }

            b')' => {
                let pair = self.delimiter.pop();

                if pair.is_none() {
                    return Err(Self::unmatched_delimiter(Delimiter { ch, start }));
                }

                let pair = pair.unwrap();

                if pair.ch != b'(' {
                    return Err(Self::mismatched_delimiter(pair, Delimiter { ch, start }));
                }

                TokenKind::RightParen
            }

            b'[' => {
                self.delimiter.push(Delimiter { ch, start });
                TokenKind::LeftBracket
            }

            b']' => {
                let pair = self.delimiter.pop();

                if pair.is_none() {
                    return Err(Self::unmatched_delimiter(Delimiter { ch, start }));
                }

                let pair = pair.unwrap();

                if pair.ch != b'[' {
                    return Err(Self::mismatched_delimiter(pair, Delimiter { ch, start }));
                }

                TokenKind::RightBracket
            }

            b'{' => {
                self.delimiter.push(Delimiter { ch, start });
                TokenKind::LeftBrace
            }

            b'}' => {
                let pair = self.delimiter.pop();

                if pair.is_none() {
                    return Err(Self::unmatched_delimiter(Delimiter { ch, start }));
                }

                let pair = pair.unwrap();

                if pair.ch != b'{' {
                    return Err(Self::mismatched_delimiter(pair, Delimiter { ch, start }));
                }

                TokenKind::RightBrace
            }

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
                    return Err(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        class: DiagnosticClass::UnexpectedChar,
                        msg: "unexpected character '!'".into(),
                        primary: Label {
                            span: Span::new(start, start),
                            msg: "'!' is not a valid token".into(),
                            paranthesise: false,
                        },
                        secondary: vec![],
                        notes: vec!["if you meant not-equal, use '!='".into()],
                    });
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
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::UnexpectedChar,
                    msg: format!("unexpected character '{}'", char::from(ch)),
                    primary: Label {
                        span: Span::new(start, start),
                        msg: format!("'{}' is not a valid token", char::from(ch)),
                        paranthesise: false,
                    },
                    secondary: vec![],
                    notes: vec![],
                });
            }
        };

        Ok(Some(Token::new(tok_type, start, self.position)))
    }

    fn track_indent(&mut self) -> Result<Vec<Token>, Diagnostic> {
        let start = self.position;

        let mut tokens: Vec<Token> = Vec::new();

        let mut indent: usize = 0;

        let starting_indent = self.peek(0);

        if starting_indent != b' '
            && starting_indent != b'\t'
            && starting_indent != b'\n'
            && self.indentation.is_empty()
        {
            return Ok(tokens);
        }

        loop {
            match self.peek(0) {
                ch if ch == b' ' || ch == b'\t' => {
                    if ch != starting_indent {
                        return Err(Diagnostic {
                            severity: DiagnosticSeverity::Error,
                            class: DiagnosticClass::InvalidLayout,
                            msg: format!("inconsistent use of tabs and spaces in indentation"),
                            primary: Label {
                                span: Span::new(start, self.position),
                                msg: format!("indentation contains both tabs and spaces"),
                                paranthesise: false,
                            },
                            secondary: vec![],
                            notes: vec!["use only tabs or only spaces for indentation".into()],
                        });
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
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::InvalidLayout,

                    msg: "invalid indentation level".into(),

                    primary: Label {
                        span: Span::new(start, self.position),
                        msg: format!("indentation level is {indent}"),
                        paranthesise: false,
                    },

                    secondary: vec![],

                    notes: vec![format!(
                        "expected one of: {}",
                        self.indentation
                            .iter()
                            .map(|i| i.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )],
                });
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

            match &token.kind {
                TokenKind::EOF => {
                    while let Some(v) = self.indentation.pop() {
                        if v == 0 {
                            continue;
                        }

                        tokens.push(Token::new(TokenKind::Dedent, self.position, self.position));
                    }

                    tokens.push(token);

                    break;
                }

                TokenKind::Newline => {
                    if !self.delimiter.is_empty() {
                        continue;
                    }

                    tokens.push(token);
                    tokens.extend(self.track_indent()?)
                }

                _ => tokens.push(token),
            }
        }

        if let Some(open) = self.delimiter.pop() {
            let eof_pos = tokens.last().unwrap().span.start;

            return Err(Diagnostic {
                severity: DiagnosticSeverity::Error,
                class: DiagnosticClass::UnmatchedDelimiter,

                msg: "unclosed delimiter".into(),

                primary: Label {
                    span: Span::new(eof_pos, eof_pos),
                    msg: format!(
                        "expected '{}' before end of file",
                        char::from(Delimiter::pair(open.ch))
                    ),
                    paranthesise: false,
                },

                secondary: vec![Label {
                    span: Span::new(open.start, open.start + 1),
                    msg: format!("'{}' opened here", char::from(open.ch)),
                    paranthesise: false,
                }],

                notes: vec![],
            });
        };

        Ok(tokens)
    }

    // Errors
    fn unmatched_delimiter(got: Delimiter) -> Diagnostic {
        Diagnostic {
            severity: DiagnosticSeverity::Error,
            class: DiagnosticClass::UnmatchedDelimiter,
            msg: "Closing delimiter without matching opening delimiter".into(),
            primary: Label {
                span: Span::new(got.start, got.start),
                msg: format!("Found '{}'", char::from(got.ch)),
                paranthesise: false,
            },
            secondary: vec![],
            notes: vec![],
        }
    }

    fn mismatched_delimiter(expected: Delimiter, got: Delimiter) -> Diagnostic {
        Diagnostic {
            severity: DiagnosticSeverity::Error,
            class: DiagnosticClass::UnmatchedDelimiter,
            msg: format!(
                "mismatched delimiter: expected '{}', found '{}'",
                char::from(Delimiter::pair(expected.ch)),
                char::from(got.ch)
            ),
            primary: Label {
                span: Span::new(got.start, got.start),
                msg: format!("expected '{}'", char::from(Delimiter::pair(expected.ch))),
                paranthesise: false,
            },
            secondary: vec![Label {
                span: Span::new(expected.start, expected.start),
                msg: format!("'{}' opened here", char::from(expected.ch)),
                paranthesise: false,
            }],
            notes: vec![],
        }
    }
}
