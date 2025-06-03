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
    TypeDef,

    // Types
    IntType,
    UIntType,
    BoolType,
    FloatType,
    StringType,
    List,
    Array,
    Enum,
    ByteType,

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
    Ampersand,
    Pipe,

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

impl Token<'_> {
    pub fn err_display(&self) -> String {
        match self {
            Token::Int(s) => format!("integer literal '{}'", s),
            Token::Float(s) => format!("float literal '{}'", s),
            Token::String(s) => format!("string literal {:?}", s),
            Token::Bool(v) => format!("boolean literal '{}'", v),
            Token::Identifier(s) => format!("identifier '{}'", s),
            Token::Let => "keyword 'let'".into(),
            Token::If => "keyword 'if'".into(),
            Token::Elif => "keyword 'elif'".into(),
            Token::Else => "keyword 'else'".into(),
            Token::While => "keyword 'while'".into(),
            Token::Break => "keyword 'break'".into(),
            Token::Continue => "keyword 'continue'".into(),
            Token::IntType => "type 'Int'".into(),
            Token::UIntType => "type 'UInt'".into(),
            Token::BoolType => "type 'Bool'".into(),
            Token::FloatType => "type 'Float'".into(),
            Token::StringType => "type 'String'".into(),
            Token::ByteType => "type 'Byte'".into(),
            Token::List => "type constructor 'List'".into(),
            Token::Array => "type constructor 'Array'".into(),
            Token::Enum => "keyword 'enum'".into(),
            Token::EqEq => "'=='".into(),
            Token::NotEq => "'!='".into(),
            Token::Gt => "'>'".into(),
            Token::Gte => "'>='".into(),
            Token::Lt => "'<'".into(),
            Token::Lte => "'<='".into(),
            Token::Plus => "'+'".into(),
            Token::Minus => "'-'".into(),
            Token::Star => "'*'".into(),
            Token::Slash => "'/'".into(),
            Token::Percent => "'%'".into(),
            Token::Assign => "'='".into(),
            Token::Not => "'not'".into(),
            Token::And => "'and'".into(),
            Token::Or => "'or'".into(),
            Token::LShift => "'<<'".into(),
            Token::RShift => "'>>'".into(),
            Token::Ampersand => "'&'".into(),
            Token::Pipe => "'|'".into(),
            Token::LParen => "'('".into(),
            Token::RParen => "')'".into(),
            Token::LBrace => "'{'".into(),
            Token::RBrace => "'}'".into(),
            Token::LBracket => "'['".into(),
            Token::RBracket => "']'".into(),
            Token::Colon => "':'".into(),
            Token::Comma => "','".into(),
            Token::Dot => "'.'".into(),
            Token::Indent => "<indent>".into(),
            Token::Dedent => "<dedent>".into(),
            Token::Newline => "<newline>".into(),
            Token::Comment(s) => format!("comment {:?}", s),
            Token::TypeDef => "'type'".into(),
                    }
    }
}
