use std::fmt;

use crate::ast::Span;
use crate::error_reporting::CompileError;
use crate::new_parser::lexer::token::{Spanned, Token};

use super::StreamError;
use super::lexer::error::LexingError;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub span: Span,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<LexingError> for ParseError {
    fn from(error: LexingError) -> ParseError {
        ParseError {
            span: error.span,
            message: error.kind.to_string(),
        }
    }
}

impl From<StreamError> for ParseError {
    fn from(error: StreamError) -> ParseError {
        ParseError {
            span: error.span,
            message: error.kind.to_string(),
        }
    }
}

impl ParseError {
    pub fn new(span: Span, msg: impl Into<String>) -> Self {
        Self {
            span,
            message: msg.into(),
        }
    }

    pub fn unexpected(tok: &Spanned<Token>, expected: &str) -> Self {
        Self::new(
            tok.span,
            format!("Expected {}, found {}", expected, tok.node.err_display()),
        )
    }

    pub fn eof(expected: &str) -> Self {
        Self::new(
            Span {
                start: 0,
                end: 0,
                line: 0,
                column: 0,
            },
            format!("Unexpected end of input (expected {})", expected),
        )
    }

    pub fn to_compile_error(self) -> CompileError {
        CompileError::syntax_error(self.message, self.span)
    }
}
