use crate::{
    ast::Span,
    new_parser::lexer::error::{LexingError, LexingErrorKind},
};
use logos::{Lexer, Logos};

fn newline_check<'a>(lex: &mut Lexer<'a, LogosToken<'a>>) {
    if lex.extras.pending_line_inc {
        lex.extras.line += 1;
        lex.extras.pending_line_inc = false;
    }
}

/// Update the line count and the char index.
fn newline_callback<'a>(lex: &mut Lexer<'a, LogosToken<'a>>) {
    newline_check(lex);

    // If we update it now, this newline will appear to be on next line
    lex.extras.pending_line_inc = true;

    // Compute span here (before incrementing line)
    lex.extras.column = lex.span().start - lex.extras.line_start;

    // Then update state for next line
    lex.extras.line_start = lex.span().end;
}

/// Compute the line and column position for the current word.
fn word_callback<'a>(lex: &mut Lexer<'a, LogosToken<'a>>) {
    newline_check(lex);

    let start = lex.span().start;
    lex.extras.column = start - lex.extras.line_start;
}

/// Compute the line and column position for the current word.
fn word_callback_str<'a>(lex: &mut Lexer<'a, LogosToken<'a>>) -> &'a str {
    word_callback(lex);

    lex.slice()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Extras {
    pub line: usize,
    pub column: usize,
    pub line_start: usize, // byte offset of start of current line
    pub pending_line_inc: bool,
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(error = LexingError)]
#[logos(extras = Extras)]
#[logos(skip r"[ \t]+")] // skip insignificant whitespace
pub enum LogosToken<'source> {
    // Comments
    #[regex(r"#.*", word_callback_str)]
    Comment(&'source str),

    // Literals
    #[regex(r"[0-9]+\.[0-9]+", word_callback_str)]
    Float(&'source str),

    #[regex(r"[0-9]+", word_callback_str)]
    Int(&'source str),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        word_callback(lex);
        let s = lex.slice();
        if s.len() >= 2 {
            Ok(&s[1..s.len()-1])
        } else {
            Err(LexingError::new(LexingErrorKind::InvalidString, Span { line: lex.extras.line, column: lex.extras.column, start: lex.span().start, end: lex.span().end }))
        }
    })]
    String(&'source str),
    #[regex(r#""[^"]*"#, |lex| {
        word_callback(lex);
        Result::<(), _>::Err(LexingError::new(LexingErrorKind::InvalidString, Span { line: lex.extras.line, column: lex.extras.column, start: lex.span().start, end: lex.span().end }))
    })]
    #[token("true")]
    True,
    #[token("false")]
    False,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", word_callback_str)]
    Identifier(&'source str),

    // Keywords
    #[token("let", word_callback)]
    Let,
    #[token("if", word_callback)]
    If,
    #[token("elif", word_callback)]
    Elif,
    #[token("else", word_callback)]
    Else,
    #[token("while", word_callback)]
    While,
    #[token("break", word_callback)]
    Break,
    #[token("continue", word_callback)]
    Continue,
    #[token("Int", word_callback)]
    IntType,
    #[token("Bool", word_callback)]
    BoolType,
    #[token("Float", word_callback)]
    FloatType,
    #[token("List", word_callback)]
    List,
    #[token("Array", word_callback)]
    Array,

    // Operators
    #[token("==", word_callback)]
    EqEq,
    #[token("!=", word_callback)]
    NotEq,
    #[token(">=", word_callback)]
    Gte,
    #[token("<=", word_callback)]
    Lte,
    #[token(">", word_callback)]
    Gt,
    #[token("<", word_callback)]
    Lt,
    #[token("=", word_callback)]
    Assign,
    #[token("+", word_callback)]
    Plus,
    #[token("-", word_callback)]
    Minus,
    #[token("*", word_callback)]
    Star,
    #[token("/", word_callback)]
    Slash,
    #[token("%", word_callback)]
    Percent,
    #[token("not", word_callback)]
    Not,
    #[token("and", word_callback)]
    And,
    #[token("or", word_callback)]
    Or,
    #[token("<<", word_callback)]
    LShift,
    #[token(">>", word_callback)]
    RShift,
    #[token("&", word_callback)]
    BitAnd,
    #[token("|", word_callback)]
    BitOr,

    // Symbols
    #[token("(", word_callback)]
    LParen,
    #[token(")", word_callback)]
    RParen,
    #[token("[", word_callback)]
    LBracket,
    #[token("]", word_callback)]
    RBracket,
    #[token("{", word_callback)]
    LBrace,
    #[token("}", word_callback)]
    RBrace,
    #[token(":", word_callback)]
    Colon,
    #[token(",", word_callback)]
    Comma,
    #[token(".", word_callback)]
    Dot,

    // Newline and indentation
    #[token("<<INDENT>>", word_callback)]
    Indent,
    #[token("<<DEDENT>>", word_callback)]
    Dedent,
    #[token("\n", newline_callback)]
    Newline,
}
