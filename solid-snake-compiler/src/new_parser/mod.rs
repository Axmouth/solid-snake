pub mod error;
pub mod expression;
pub mod lexer;
pub mod statement;
pub mod typing;

use std::fmt;

use crate::ast::Span;
use crate::error_reporting::{CompileError, CompileErrorList};
use crate::new_parser::error::ParseError;
use crate::new_parser::lexer::token::{Spanned, Token};

pub type ParseResult<T> = Result<T, ParseError>;

pub trait Parse<'a>: Sized {
    fn parse(stream: &mut TokenStream<'a>) -> ParseResult<Self>;
}

pub struct ParseContext {
    pub errors: Vec<ParseError>,
    pub statement_id: usize,
}

impl ParseContext {
    pub fn next_id(&mut self) -> usize {
        let id = self.statement_id;
        self.statement_id += 1;
        id
    }

    pub fn to_compile_errors(self) -> CompileErrorList {
        self.errors
            .into_iter()
            .map(|e| CompileError::syntax_error(e.message, e.span))
            .collect()
    }

    pub fn record(&mut self, err: ParseError) {
        self.errors.push(err);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

pub struct TokenStream<'a> {
    tokens: Vec<Spanned<Token<'a>>>,
    pos: usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: Vec<Spanned<Token<'a>>>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn reset(&mut self) {
        self.pos = 0;
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StreamError {
    pub kind: StreamErrorKind,
    pub span: Span,
}

impl StreamError {
    pub fn new(kind: StreamErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum StreamErrorKind {
    UnexpectedToken {
        expected: String,
        found: String,
    },
    #[default]
    Eof,
}

impl fmt::Display for StreamErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StreamErrorKind::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "Unexpected token: expected {}, found {}",
                    expected, found
                )
            }
            StreamErrorKind::Eof => write!(f, "Unexpected end of file"),
        }
    }
}

impl TokenStream<'_> {
    #[inline(always)]
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&Spanned<Token>> {
        self.skip_comment();
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    #[inline(always)]
    pub fn skip_comment(&mut self) -> bool {
        if let Some(Spanned {
            node: Token::Comment(..),
            ..
        }) = self.tokens.get(self.pos)
        {
            self.pos += 1;
            return true;
        }

        false
    }

    pub fn skip_ws(&mut self) {
        while let Some(tok) = self.peek() {
            match tok.node {
                Token::Newline | Token::Comment(_) => {
                    self.next();
                }
                _ => break,
            }
        }
    }

    #[inline(always)]
    pub fn peek(&self) -> Option<&Spanned<Token>> {
        let mut offset = 0;
        if let Some(Spanned {
            node: Token::Comment(..),
            ..
        }) = self.tokens.get(self.pos)
        {
            offset += 1;
        }
        self.tokens.get(self.pos + offset)
    }

    #[inline(always)]
    pub fn expect(&mut self, expected: &Token) -> Result<Span, StreamError> {
        self.skip_comment();
        if let Some(tok) = self.tokens.get(self.pos) {
            if tok.node == *expected {
                self.pos += 1;
                Ok(tok.span)
            } else {
                Err(StreamError::new(
                    StreamErrorKind::UnexpectedToken {
                        expected: expected.err_display().to_string(),
                        found: tok.node.err_display().to_string(),
                    },
                    tok.span,
                ))
            }
        } else {
            let span = self.tokens.last().map(|t| t.span).unwrap_or_default();
            Err(StreamError::new(StreamErrorKind::Eof, span))
        }
    }

    pub fn save(&self) -> usize {
        self.pos
    }

    pub fn restore(&mut self, pos: usize) {
        self.pos = pos;
    }

    #[inline(always)]
    pub fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_text_eq;
    use crate::new_parser::{lexer::error::LexingError, *};
    use crate::{
        new_parser::{lexer::lex, statement::parse_program},
        preprocessor::preprocess_indentation,
    };

    fn parse_and_pretty(input: &str) -> (String, CompileErrorList) {
        let preprocessed = preprocess_indentation(input).unwrap();
        eprintln!("{}", preprocessed.transformed);
        let lexed: Vec<Result<Spanned<Token>, LexingError>> = lex(&preprocessed.transformed);
        let (tokens, lex_errors): (Vec<_>, Vec<_>) = lexed.into_iter().partition(Result::is_ok);

        let tokens: Vec<Spanned<Token>> = tokens.into_iter().map(Result::unwrap).collect();
        eprintln!(
            "{:?}",
            tokens.iter().map(|t| (&t.node, t.span)).collect::<Vec<_>>()
        );
        let mut errors = CompileErrorList::new();
        for err in lex_errors.into_iter().map(Result::unwrap_err) {
            errors.push_error(err.to_compile_error());
        }

        let mut stream = TokenStream::new(tokens);
        let (ast, parse_errors) = parse_program(&mut stream);
        errors.extend(parse_errors);

        let output = ast
            .iter()
            .map(|n| n.pretty(0))
            .collect::<Vec<_>>()
            .join("\n");

        (output, errors)
    }

    #[test]
    fn test_simple_var_decl() {
        let input = "let x = 42\n";
        let (actual, errors) = parse_and_pretty(input);

        assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);

        let expected = "[0:0] let x = 42";

        assert_text_eq!(actual.trim(), expected.trim());
    }

    #[test]
    fn test_if_stmt() {
        let input = r#"
if true:
    let x = 1
else:
    let x = 2
"#;
        let (pretty, errors) = parse_and_pretty(input);
        eprintln!("{:?}", errors);
        assert!(errors.is_empty());

        let expected = r#"
[1:0] if true:
[2:2]     let x = 1
[3:3] else:
[4:4]     let x = 2
"#;

        assert_text_eq!(pretty.trim(), expected.trim());
    }
}
