use crate::common::Span;

pub enum DiagnosticSeverity {
    Warn,
    Error,
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

    // semantic
    UndefinedIdentifier,
    TypeMismatch,
    UnknownType,
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

            Self::UndefinedIdentifier => "E201",
            Self::TypeMismatch => "E202",
            Self::UnknownType => "E203",
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
