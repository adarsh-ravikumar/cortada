use crate::{common::Span, utils::Style};

pub enum DiagnosticSeverity {
    Warn,
    Error,
}

impl DiagnosticSeverity {
    pub fn color(&self) -> &str {
        match self {
            Self::Warn => Style::BRIGHT_YELLOW,
            Self::Error => Style::BRIGHT_RED,
        }
    }
}

pub enum DiagnosticClass {
    // lexer
    UnexpectedChar,
    UnmatchedDelimiter,

    // parser
    ExpectedToken,
    ExpectedExpression,
    UnexpectedToken,
    InvalidLayout,
    InvalidTypeExpression,

    // semantic
    UndefinedIdentifier,
    TypeMismatch,
    UnknownType,
    UnsupportedOperator,
}

impl DiagnosticClass {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnexpectedChar => "E001",
            Self::UnmatchedDelimiter => "E002",

            Self::ExpectedToken => "E101",
            Self::ExpectedExpression => "E102",
            Self::UnexpectedToken => "E103",
            Self::InvalidLayout => "E104",
            Self::InvalidTypeExpression => "E105",

            Self::UndefinedIdentifier => "E201",
            Self::TypeMismatch => "E202",
            Self::UnknownType => "E203",
            Self::UnsupportedOperator => "E204",
        }
    }
}

pub enum LabelKind {
    Primary,
    Secondary,
}

pub struct Label {
    pub span: Span,
    pub msg: String,
    pub kind: LabelKind,
    pub paranthesise: bool,
}

pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub class: DiagnosticClass,

    pub msg: String,

    pub location: Span,

    pub labels: Vec<Label>,
    pub notes: Vec<String>,
}
