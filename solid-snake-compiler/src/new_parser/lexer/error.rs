// lexer/error.rs

use std::fmt;

use crate::ast::Span;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LexingError {
    pub kind: LexingErrorKind,
    pub span: Span,
}

impl LexingError {
    pub fn new(kind: LexingErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn span(&self) -> Span {
        self.span // expose it
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum LexingErrorKind {
    InvalidNumber(String),
    InvalidString,
    InvalidCharacter(char),
    #[default]
    Unknown,
}

impl From<std::num::ParseIntError> for LexingErrorKind {
    fn from(_: std::num::ParseIntError) -> Self {
        LexingErrorKind::InvalidNumber("invalid integer".into())
    }
}

impl fmt::Display for LexingErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LexingErrorKind::*;
        match self {
            InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            InvalidString => write!(f, "Invalid string"),
            InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
            Unknown => write!(f, "Unknown lexing error"),
        }
    }
}
