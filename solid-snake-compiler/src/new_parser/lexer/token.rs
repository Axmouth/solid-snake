// lexer/token.rs

use crate::ast::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'source> {
    // Grouped logically
    Int(&'source str),
    Float(&'source str),
    String(&'source str),
    Bool(bool),
    Identifier(&'source str),

    // Keywords
    Let,
    If,
    Elif,
    Else,
    While,
    Break,
    Continue,

    // Types
    IntType,
    BoolType,
    FloatType,
    List,
    Array,

    // Operators
    EqEq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    Not,
    And,
    Or,
    LShift,
    RShift,
    BitAnd,
    BitOr,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Dot,

    // Indentation
    Indent,
    Dedent,
    Newline,

    // Comments
    Comment(&'source str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}
