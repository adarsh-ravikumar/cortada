use core::fmt;

use crate::{common::Span, utils::Style};

pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub msg: String,
    pub span: Span,
}

#[derive(Debug)]
pub enum DiagnosticKind {
    Warn,
    Error,
}

impl fmt::Display for DiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Warn => write!(
                f,
                "{}{}warn{}{}",
                Style::BOLD,
                Style::BRIGHT_YELLOW,
                Style::RESET,
                Style::RESET_BOLD
            ),
            Self::Error => write!(
                f,
                "{}{}Error{}{}",
                Style::BOLD,
                Style::BRIGHT_RED,
                Style::RESET,
                Style::RESET_BOLD
            ),
        }
    }
}

impl Diagnostic {
    pub fn new(kind: DiagnosticKind, msg: String, span: Span) -> Self {
        Self { kind, msg, span }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.kind, self.msg)
    }
}
