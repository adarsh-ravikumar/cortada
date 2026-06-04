use crate::common::Span;

pub enum DiagnosticSeverity {
    Warn,
    Error,
}

pub enum DiagnosticClass {
    UnexpectedChar,
    UnmatchedDelimiter,

    ExpectedToken,
    ExpectedExpression,
    UnexpectedToken,
    InvalidLayout,
}

impl DiagnosticClass {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnexpectedChar => "E001",
            Self::UnmatchedDelimiter => "E002",

            Self::ExpectedToken => "E003",
            Self::ExpectedExpression => "E004",
            Self::UnexpectedToken => "E005",
            Self::InvalidLayout => "E006",
        }
    }
}

pub struct Label {
    pub span: Span,
    pub msg: String,
}

pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub class: DiagnosticClass,

    pub msg: String,

    pub primary: Label,
    pub secondary: Vec<Label>,

    pub notes: Vec<String>,
}
