use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.start, self.end)
    }
}

pub static ERRONEOUS_SPAN: Span = Span { start: 0, end: 0 };

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
