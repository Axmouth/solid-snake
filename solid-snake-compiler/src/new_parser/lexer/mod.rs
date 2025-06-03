pub mod error;
mod logos_token;
pub mod token;

use logos::Logos;

use crate::ast::Span;
use crate::new_parser::lexer::error::LexingError;
use crate::new_parser::lexer::logos_token::LogosToken;
use crate::new_parser::lexer::token::{Spanned, Token};

/// Convert source into `Vec<Spanned<Token>>`, preserving span info.
#[inline(always)]
pub fn lex(source: &str) -> Vec<Result<Spanned<Token>, LexingError>> {
    let mut out = vec![];
    let mut lex = LogosToken::lexer(source);

    while let Some(res) = lex.next() {
        let span = Span {
            start: lex.span().start,
            end: lex.span().end,
            line: lex.extras.line,
            column: lex.extras.column,
        };

        match res {
            Ok(tok) => {
                let token = match tok {
                    LogosToken::Int(i) => Token::Int(i),
                    LogosToken::Float(f) => Token::Float(f),
                    LogosToken::String(s) => Token::String(s),
                    LogosToken::True => Token::Bool(true),
                    LogosToken::False => Token::Bool(false),
                    LogosToken::Identifier(id) => Token::Identifier(id),
                    LogosToken::Comment(c) => Token::Comment(c),
                    LogosToken::Let => Token::Let,
                    LogosToken::If => Token::If,
                    LogosToken::Elif => Token::Elif,
                    LogosToken::Else => Token::Else,
                    LogosToken::While => Token::While,
                    LogosToken::Break => Token::Break,
                    LogosToken::Continue => Token::Continue,
                    LogosToken::TypeDef => Token::TypeDef,
                    LogosToken::IntType => Token::IntType,
                    LogosToken::UIntType => Token::UIntType,
                    LogosToken::BoolType => Token::BoolType,
                    LogosToken::FloatType => Token::FloatType,
                    LogosToken::StringType => Token::StringType,
                    LogosToken::Enum => Token::Enum,
                    LogosToken::List => Token::List,
                    LogosToken::Array => Token::Array,
                    LogosToken::ByteType => Token::ByteType,
                    LogosToken::EqEq => Token::EqEq,
                    LogosToken::NotEq => Token::NotEq,
                    LogosToken::Gte => Token::Gte,
                    LogosToken::Lte => Token::Lte,
                    LogosToken::Gt => Token::Gt,
                    LogosToken::Lt => Token::Lt,
                    LogosToken::Assign => Token::Assign,
                    LogosToken::Plus => Token::Plus,
                    LogosToken::Minus => Token::Minus,
                    LogosToken::Star => Token::Star,
                    LogosToken::Slash => Token::Slash,
                    LogosToken::Percent => Token::Percent,
                    LogosToken::Not => Token::Not,
                    LogosToken::And => Token::And,
                    LogosToken::Or => Token::Or,
                    LogosToken::LShift => Token::LShift,
                    LogosToken::RShift => Token::RShift,
                    LogosToken::Ampersand => Token::Ampersand,
                    LogosToken::Pipe => Token::Pipe,
                    LogosToken::LParen => Token::LParen,
                    LogosToken::RParen => Token::RParen,
                    LogosToken::LBracket => Token::LBracket,
                    LogosToken::RBracket => Token::RBracket,
                    LogosToken::LBrace => Token::LBrace,
                    LogosToken::RBrace => Token::RBrace,
                    LogosToken::Colon => Token::Colon,
                    LogosToken::Comma => Token::Comma,
                    LogosToken::Dot => Token::Dot,
                    LogosToken::Indent => Token::Indent,
                    LogosToken::Dedent => Token::Dedent,
                    LogosToken::Newline => Token::Newline,
                };

                out.push(Ok(Spanned { node: token, span }));
            }

            Err(e) => out.push(Err(e)),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::logos_token::Extras;
    // TODO
    // Add tests for any newly added tokens.
    // Hex / binary numbers	❌ Not supported yet	Add later if needed
    // Identifiers with Unicode	❌ Not supported yet	Might want to reject explicitly
    use super::*;
    use crate::new_parser::lexer::error::{LexingError, LexingErrorKind};
    use crate::new_parser::lexer::lex;
    use crate::new_parser::lexer::token::{Spanned, Token};

    use test_case::test_case;

    fn extract_errors(source: &str) -> Vec<LexingError> {
        lex(source).into_iter().filter_map(Result::err).collect()
    }

    fn extract_kinds(tokens: Vec<Result<Spanned<Token>, LexingError>>) -> Vec<Token> {
        tokens
            .into_iter()
            .filter_map(Result::ok)
            .map(|s| s.node)
            .collect()
    }

    fn extract_tokens(source: &str) -> Vec<Spanned<Token>> {
        lex(source).into_iter().filter_map(Result::ok).collect()
    }

    #[test]
    fn test_basic_literals() {
        let input = r#"123 3.14 "hello" true false foo_bar"#;
        let tokens = extract_kinds(lex(input));
        assert_eq!(
            tokens,
            vec![
                Token::Int("123"),
                Token::Float("3.14"),
                Token::String("hello"),
                Token::Bool(true),
                Token::Bool(false),
                Token::Identifier("foo_bar"),
            ]
        );
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let input = "let letx if iff while whilee";
        let tokens = extract_kinds(lex(input));
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("letx"),
                Token::If,
                Token::Identifier("iff"),
                Token::While,
                Token::Identifier("whilee"),
            ]
        );
    }

    #[test]
    fn test_operators_and_symbols() {
        let input = r#"== != >= <= < > = + - * / % << >> & | ( ) [ ] { } : , ."#;
        let tokens = extract_kinds(lex(input));
        assert_eq!(
            tokens,
            vec![
                Token::EqEq,
                Token::NotEq,
                Token::Gte,
                Token::Lte,
                Token::Lt,
                Token::Gt,
                Token::Assign,
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Slash,
                Token::Percent,
                Token::LShift,
                Token::RShift,
                Token::Ampersand,
                Token::Pipe,
                Token::LParen,
                Token::RParen,
                Token::LBracket,
                Token::RBracket,
                Token::LBrace,
                Token::RBrace,
                Token::Colon,
                Token::Comma,
                Token::Dot,
            ]
        );
    }

    #[test]
    fn test_indent_dedent_newline() {
        let input = "x\n<<INDENT>>\ny\n<<DEDENT>>\nz";
        let tokens = extract_kinds(lex(input));
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("x"),
                Token::Newline,
                Token::Indent,
                Token::Newline,
                Token::Identifier("y"),
                Token::Newline,
                Token::Dedent,
                Token::Newline,
                Token::Identifier("z"),
            ]
        );
    }

    #[test]
    fn test_comments_preserved() {
        let input = r#"
x = 1 # assign x
# full line comment
"#;
        let tokens = extract_kinds(lex(input));
        assert_eq!(
            tokens,
            vec![
                Token::Newline,
                Token::Identifier("x"),
                Token::Assign,
                Token::Int("1"),
                Token::Comment("# assign x"),
                Token::Newline,
                Token::Comment("# full line comment"),
                Token::Newline,
            ]
        );
    }

    #[test]
    fn test_invalid_tokens() {
        let input = r#"@"#;
        let tokens = lex(input);
        assert!(matches!(
            tokens[0].as_ref().unwrap_err().kind,
            LexingErrorKind::Unknown
        ));
    }

    #[test]
    fn test_unterminated_string() {
        let input = r#""unterminated"#;
        let tokens = lex(input);
        assert!(matches!(
            tokens[0].as_ref().unwrap_err().kind,
            LexingErrorKind::InvalidString
        ));
    }

    #[test]
    fn test_full_line() {
        let input = r#"let x = 1 + 2"#;
        let tokens = extract_kinds(lex(input));
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x"),
                Token::Assign,
                Token::Int("1"),
                Token::Plus,
                Token::Int("2"),
            ]
        );
    }

    #[test]
    fn test_shadowing_keywords() {
        let input = "truex falsey notz";
        let tokens = extract_kinds(lex(input));
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("truex"),
                Token::Identifier("falsey"),
                Token::Identifier("notz"),
            ]
        );
    }

    #[test]
    fn test_multiline_string() {
        let input = r#""hello\nworld""#;
        let tokens = extract_kinds(lex(input));
        assert_eq!(tokens, vec![Token::String("hello\\nworld")]);
    }

    #[test]
    fn test_operator_series() {
        let input = "+-*/==!=<<>>";
        let tokens = extract_kinds(lex(input));
        assert_eq!(
            tokens,
            vec![
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Slash,
                Token::EqEq,
                Token::NotEq,
                Token::LShift,
                Token::RShift
            ]
        );
    }

    #[test]
    fn test_realistic_block() {
        let input = r#"
let x = 42
if x > 10:
<<INDENT>>
x = x + 1
<<DEDENT>>
"#;

        let tokens = extract_kinds(lex(input));
        let expected = vec![
            Token::Newline,
            Token::Let,
            Token::Identifier("x"),
            Token::Assign,
            Token::Int("42"),
            Token::Newline,
            Token::If,
            Token::Identifier("x"),
            Token::Gt,
            Token::Int("10"),
            Token::Colon,
            Token::Newline,
            Token::Indent,
            Token::Newline,
            Token::Identifier("x"),
            Token::Assign,
            Token::Identifier("x"),
            Token::Plus,
            Token::Int("1"),
            Token::Newline,
            Token::Dedent,
            Token::Newline,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_escaped_quotes_in_string() {
        let input = r#""a \"quoted\" string""#;
        let tokens = extract_kinds(lex(input));
        assert_eq!(tokens, vec![Token::String(r#"a \"quoted\" string"#)]);
    }

    #[test]
    fn test_blank_lines() {
        let input = "a\n\n\nb";
        let tokens = extract_kinds(lex(input));
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a"),
                Token::Newline,
                Token::Newline,
                Token::Newline,
                Token::Identifier("b"),
            ]
        );
    }

    #[test]
    fn test_indent_dedent_with_newlines() {
        let input = "<<INDENT>>\nfoo\n<<DEDENT>>\nbar";
        let tokens = extract_kinds(lex(input));
        assert_eq!(
            tokens,
            vec![
                Token::Indent,
                Token::Newline,
                Token::Identifier("foo"),
                Token::Newline,
                Token::Dedent,
                Token::Newline,
                Token::Identifier("bar"),
            ]
        );
    }

    #[test]
    fn test_leading_newline() {
        let input = "\nfoo";
        let tokens = extract_kinds(lex(input));
        assert_eq!(tokens, vec![Token::Newline, Token::Identifier("foo"),]);
    }

    #[test]
    fn test_line_column_tracking() {
        let src = "let x = 5\n  y = x + 1";
        let mut lex = LogosToken::lexer(src);
        lex.extras = Extras::default();

        while let Some(Ok(token)) = lex.next() {
            let span = lex.span();
            println!(
                "{:?} at line {}, column {} (bytes {}..{})",
                token, lex.extras.line, lex.extras.column, span.start, span.end
            );
        }
    }

    #[test]
    fn test_valid_string_literal() {
        let tokens = lex(r#""hello world""#);
        assert!(matches!(
            &tokens[0],
            Ok(t) if t.node == Token::String("hello world")
        ));
    }

    #[test]
    fn test_empty_string() {
        let tokens = lex(r#""""#);
        assert!(matches!(
            &tokens[0],
            Ok(t) if t.node == Token::String("")
        ));
    }

    #[test]
    fn test_unterminated_string_literal() {
        let errors = extract_errors(r#""hello"#);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].kind, LexingErrorKind::InvalidString);
    }

    #[test]
    fn test_single_quote_string() {
        let errors = extract_errors(r#"""#);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].kind, LexingErrorKind::InvalidString);
    }

    #[test]
    fn test_escaped_unterminated_string() {
        let errors = extract_errors(r#""abc\"#);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].kind, LexingErrorKind::InvalidString);
    }

    #[test]
    fn test_string_with_escaped_quote() {
        let tokens = lex(r#""abc\"def""#);
        assert!(matches!(
            &tokens[0],
            Ok(t) if t.node == Token::String(r#"abc\"def"#)
        ));
    }

    #[test]
    fn test_unterminated_string_error_span() {
        let src = r#""unterminated"#;
        let err = lex(src).into_iter().find_map(Result::err).unwrap();

        let span = err.span();
        assert_eq!(span.start, 0);
        assert_eq!(span.end, src.len());
        assert_eq!(span.line, 0);
    }

    #[test]
    fn test_newline_span_position() {
        let tokens = extract_tokens("x = 1\n");
        let newline = &tokens[3]; // 0: x, 1: =, 2: 1, 3: \n
        assert_eq!(newline.node, Token::Newline);
        assert_eq!(newline.span.line, 0);
        assert_eq!(newline.span.column, 5);
    }

    #[test]
    fn test_multiple_newlines_span_tracking() {
        let src = "\n\n\nx";
        let tokens: Vec<_> = lex(src).into_iter().collect::<Result<_, _>>().unwrap();

        assert_eq!(tokens.len(), 4);

        assert_eq!(tokens[0].node, Token::Newline);
        assert_eq!(tokens[0].span.line, 0);

        assert_eq!(tokens[1].node, Token::Newline);
        assert_eq!(tokens[1].span.line, 1);

        assert_eq!(tokens[2].node, Token::Newline);
        assert_eq!(tokens[2].span.line, 2);

        assert_eq!(tokens[3].node, Token::Identifier("x"));
        assert_eq!(tokens[3].span.line, 3);
        assert_eq!(tokens[3].span.column, 0);
    }

    #[test_case("let", Token::Let)]
    #[test_case("if", Token::If)]
    #[test_case("elif", Token::Elif)]
    #[test_case("else", Token::Else)]
    #[test_case("while", Token::While)]
    #[test_case("break", Token::Break)]
    #[test_case("continue", Token::Continue)]
    #[test_case("<<INDENT>>", Token::Indent)]
    #[test_case("<<DEDENT>>", Token::Dedent)]
    fn test_keywords(input: &str, expected: Token) {
        let tokens = extract_kinds(lex(input));
        assert_eq!(tokens, vec![expected]);
    }

    #[test_case("Int", Token::IntType)]
    #[test_case("Bool", Token::BoolType)]
    #[test_case("Float", Token::FloatType)]
    #[test_case("List", Token::List)]
    #[test_case("Array", Token::Array)]
    fn test_type_keywords(input: &str, expected: Token) {
        let tokens = extract_kinds(lex(input));
        assert_eq!(tokens, vec![expected]);
    }

    #[test_case("+", Token::Plus)]
    #[test_case("-", Token::Minus)]
    #[test_case("*", Token::Star)]
    #[test_case("/", Token::Slash)]
    #[test_case("%", Token::Percent)]
    #[test_case("==", Token::EqEq)]
    #[test_case("!=", Token::NotEq)]
    #[test_case("<", Token::Lt)]
    #[test_case("<=", Token::Lte)]
    #[test_case(">", Token::Gt)]
    #[test_case(">=", Token::Gte)]
    #[test_case("=", Token::Assign)]
    #[test_case("and", Token::And)]
    #[test_case("or", Token::Or)]
    #[test_case("not", Token::Not)]
    #[test_case("<<", Token::LShift)]
    #[test_case(">>", Token::RShift)]
    #[test_case("&", Token::Ampersand)]
    #[test_case("|", Token::Pipe)]
    fn test_operators(input: &str, expected: Token) {
        let tokens = extract_kinds(lex(input));
        assert_eq!(tokens, vec![expected]);
    }

    #[test_case("(", Token::LParen)]
    #[test_case(")", Token::RParen)]
    #[test_case("{", Token::LBrace)]
    #[test_case("}", Token::RBrace)]
    #[test_case("[", Token::LBracket)]
    #[test_case("]", Token::RBracket)]
    #[test_case(":", Token::Colon)]
    #[test_case(",", Token::Comma)]
    #[test_case(".", Token::Dot)]
    fn test_delimiters(input: &str, expected: Token) {
        let tokens = extract_kinds(lex(input));
        assert_eq!(tokens, vec![expected]);
    }

    #[test_case("let x = 5\n", 0, Token::Let, 0, 3, 0, 0)]
    #[test_case("let x = 5\n", 1, Token::Identifier("x"), 4, 5, 0, 4)]
    #[test_case("let x = 5\n", 2, Token::Assign, 6, 7, 0, 6)]
    #[test_case("let x = 5\n", 3, Token::Int("5"), 8, 9, 0, 8)]
    #[test_case("let x = 5\n", 4, Token::Newline, 9, 10, 0, 9)]
    fn test_token_spans_simple(
        src: &str,
        index: usize,
        expected_tok: Token,
        start: usize,
        end: usize,
        line: usize,
        column: usize,
    ) {
        let tokens = extract_tokens(src);
        assert_eq!(tokens[index].node, expected_tok);
        assert_eq!(
            tokens[index].span,
            Span {
                start,
                end,
                line,
                column
            }
        );
    }

    #[test_case("a = 1\nb = 2\n", 0, Token::Identifier("a"), 0, 1, 0, 0)]
    #[test_case("a = 1\nb = 2\n", 3, Token::Newline, 5, 6, 0, 5)]
    #[test_case("a = 1\nb = 2\n", 4, Token::Identifier("b"), 6, 7, 1, 0)]
    #[test_case("a = 1\nb = 2\n", 6, Token::Int("2"), 10, 11, 1, 4)]
    fn test_token_spans_multiline(
        src: &str,
        index: usize,
        expected_tok: Token,
        start: usize,
        end: usize,
        line: usize,
        column: usize,
    ) {
        let tokens = extract_tokens(src);
        assert_eq!(tokens[index].node, expected_tok);
        assert_eq!(
            tokens[index].span,
            Span {
                start,
                end,
                line,
                column
            }
        );
    }

    #[test_case(r#""hello""#, Token::String("hello"), 0, 7, 0, 0)]
    #[test_case(r#""""#, Token::String(""), 0, 2, 0, 0)]
    #[test_case(r#""hi\"there""#, Token::String(r#"hi\"there"#), 0, 11, 0, 0)]
    fn test_string_token_spans(
        src: &str,
        expected_tok: Token,
        start: usize,
        end: usize,
        line: usize,
        column: usize,
    ) {
        let tokens = extract_tokens(src);
        assert_eq!(tokens[0].node, expected_tok);
        assert_eq!(
            tokens[0].span,
            Span {
                start,
                end,
                line,
                column
            }
        );
    }
}
