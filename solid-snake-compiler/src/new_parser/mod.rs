pub mod lexer;

use crate::ast::Span;
use crate::new_parser::lexer::token::{Spanned, Token};

pub struct TokenStream<'a> {
    tokens: Vec<Spanned<Token<'a>>>,
    pos: usize,
}

#[derive(Debug, Clone)]
pub enum StreamError {
    UnexpectedToken {
        expected: String,
        found: String,
        span: Span,
    },
    Eof,
}

impl TokenStream<'_> {
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&Spanned<Token>> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    pub fn peek(&self, offset: usize) -> Option<&Spanned<Token>> {
        self.tokens.get(self.pos + offset)
    }

    pub fn expect(&mut self, expected: &Token) -> Result<&Spanned<Token>, StreamError> {
        if let Some(tok) = self.tokens.get(self.pos) {
            if tok.node == *expected {
                self.pos += 1;
                Ok(tok)
            } else {
                Err(StreamError::UnexpectedToken {
                    expected: format!("{:?}", expected),
                    found: format!("{:?}", tok.node),
                    span: tok.span,
                })
            }
        } else {
            Err(StreamError::Eof)
        }
    }

    pub fn save(&self) -> usize {
        self.pos
    }

    pub fn restore(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}
