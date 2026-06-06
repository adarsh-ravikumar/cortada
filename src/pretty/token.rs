use crate::{
    common::IOFile,
    lexer::{Token, TokenKind},
    utils::Style,
};

pub struct TokenPrinter;

impl TokenPrinter {
    pub fn print(tokens: &Vec<Token>, src: &IOFile) {
        for (idx, tok) in tokens.iter().enumerate() {
            match tok.kind {
                TokenKind::Integer | TokenKind::Float => print!(
                    "{}[{:03}] ({:4}:{:4})\t{}{:?}{}({}\"{}\"{})\n",
                    Style::BRIGHT_BLACK,
                    idx,
                    tok.span.start,
                    tok.span.end,
                    Style::CYAN,
                    tok.kind,
                    Style::RESET,
                    Style::BRIGHT_GREEN,
                    src.view_span(tok.span),
                    Style::RESET,
                ),
                TokenKind::Identifier => print!(
                    "{}[{:03}] ({:4}:{:4})\t{}{:?}({}\"{}\"{})\n",
                    Style::BRIGHT_BLACK,
                    idx,
                    tok.span.start,
                    tok.span.end,
                    Style::RESET,
                    tok.kind,
                    Style::BRIGHT_GREEN,
                    src.view_span(tok.span),
                    Style::RESET,
                ),
                TokenKind::KwrdIf
                | TokenKind::KwrdElif
                | TokenKind::KwrdElse
                | TokenKind::KwrdWhile
                | TokenKind::KwrdFor
                | TokenKind::KwrdFn => print!(
                    "{}[{:03}] ({:4}:{:4})\t{}{}{:?}{}{}\n",
                    Style::BRIGHT_BLACK,
                    idx,
                    tok.span.start,
                    tok.span.end,
                    Style::BOLD,
                    Style::BRIGHT_BLUE,
                    tok.kind,
                    Style::RESET,
                    Style::RESET_BOLD,
                ),
                TokenKind::Newline => print!(
                    "{}[{:03}] ({:4}:{:4})\t{}{}{:?}{}{}\n",
                    Style::BRIGHT_BLACK,
                    idx,
                    tok.span.start,
                    tok.span.end,
                    Style::DIM,
                    Style::MAGENTA,
                    tok.kind,
                    Style::RESET,
                    Style::RESET_DIM,
                ),
                TokenKind::EOF => print!(
                    "{}[{:03}] ({:4}:{:4})\t{}{}{:?}{}{}\n",
                    Style::BRIGHT_BLACK,
                    idx,
                    tok.span.start,
                    tok.span.end,
                    Style::BOLD,
                    Style::MAGENTA,
                    tok.kind,
                    Style::RESET,
                    Style::RESET_BOLD,
                ),
                _ => print!(
                    "{}[{:03}] ({:4}:{:4})\t{}{:?}{}\n",
                    Style::BRIGHT_BLACK,
                    idx,
                    tok.span.start,
                    tok.span.end,
                    Style::YELLOW,
                    tok.kind,
                    Style::RESET,
                ),
            }
        }

        println!(
            "\n{}{} tokens emitted{}\n",
            Style::BRIGHT_GREEN,
            tokens.len(),
            Style::RESET
        )
    }
}
