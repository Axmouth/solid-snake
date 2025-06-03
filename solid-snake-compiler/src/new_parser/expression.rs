use crate::ast::{BinaryOp, Expr, Span, UnaryOp};
use crate::new_parser::lexer::token::Token;
use crate::new_parser::{ParseError, ParseResult, TokenStream};

use super::ParseContext;

// TODO use a stack to avoid recursion
#[inline(always)]
pub fn parse_expr(stream: &mut TokenStream<'_>, ctx: &mut ParseContext) -> ParseResult<Expr> {
    parse_logical_or(stream, ctx)
}

#[inline(always)]
fn parse_logical_or(stream: &mut TokenStream<'_>, ctx: &mut ParseContext) -> ParseResult<Expr> {
    parse_binary_chain(stream, parse_logical_and, &[Token::Or], ctx)
}

#[inline(always)]
fn parse_logical_and(stream: &mut TokenStream<'_>, ctx: &mut ParseContext) -> ParseResult<Expr> {
    parse_binary_chain(stream, parse_equality, &[Token::And], ctx)
}

#[inline(always)]
fn parse_equality(stream: &mut TokenStream<'_>, ctx: &mut ParseContext) -> ParseResult<Expr> {
    parse_binary_chain(stream, parse_comparison, &[Token::EqEq, Token::NotEq], ctx)
}

#[inline(always)]
fn parse_comparison(stream: &mut TokenStream<'_>, ctx: &mut ParseContext) -> ParseResult<Expr> {
    parse_binary_chain(
        stream,
        parse_add_sub,
        &[Token::Lt, Token::Lte, Token::Gt, Token::Gte],
        ctx,
    )
}

#[inline(always)]
fn parse_add_sub(stream: &mut TokenStream<'_>, ctx: &mut ParseContext) -> ParseResult<Expr> {
    parse_binary_chain(stream, parse_mul_div, &[Token::Plus, Token::Minus], ctx)
}

#[inline(always)]
fn parse_mul_div(stream: &mut TokenStream<'_>, ctx: &mut ParseContext) -> ParseResult<Expr> {
    parse_binary_chain(
        stream,
        parse_unary,
        &[Token::Star, Token::Slash, Token::Percent],
        ctx,
    )
}

#[inline(always)]
fn parse_binary_chain<'a>(
    stream: &mut TokenStream<'a>,
    sub_parser: fn(&mut TokenStream<'a>, ctx: &mut ParseContext) -> ParseResult<Expr>,
    ops: &[Token],
    ctx: &mut ParseContext,
) -> ParseResult<Expr> {
    let mut expr = sub_parser(stream, ctx)?;

    while let Some(op_tok) = stream.peek() {
        if let Some(op) = match_binary_op(&op_tok.node, ops) {
            let op_span = op_tok.span;
            stream.next(); // consume operator
            let right = match sub_parser(stream, ctx) {
                Ok(expr) => expr,
                Err(err) => {
                    ctx.record(err);
                    Expr::invalid(op_span, ctx.statement_id)
                }
            };

            let span = Span {
                start: expr.span.start,
                end: right.span.end,
                ..expr.span
            };

            expr = Expr::bin_op(expr, right, op, span, ctx.statement_id);
        } else {
            break;
        }
    }

    Ok(expr)
}

#[inline(always)]
fn match_binary_op(token: &Token, ops: &[Token]) -> Option<BinaryOp> {
    if ops.contains(token) {
        BinaryOp::from_token(token)
    } else {
        None
    }
}

#[inline(always)]
fn parse_unary(stream: &mut TokenStream<'_>, ctx: &mut ParseContext) -> ParseResult<Expr> {
    if let Some(tok) = stream.peek() {
        let op = match tok.node {
            Token::Minus => Some(UnaryOp::Neg),
            Token::Not => Some(UnaryOp::Not),
            _ => None,
        };

        if let Some(op) = op {
            let op_span = tok.span;
            stream.next(); // consume unary op
            let expr = parse_unary(stream, ctx)?; // handle stacked unaries
            let span = Span {
                start: op_span.start,
                end: expr.span.end,
                ..op_span
            };
            return Ok(Expr::un_op(expr, op, span, ctx.statement_id));
        }
    }

    parse_primary(stream, ctx)
}

#[inline(always)]
fn parse_primary(stream: &mut TokenStream<'_>, ctx: &mut ParseContext) -> ParseResult<Expr> {
    let tok = stream.next().ok_or_else(|| ParseError::eof("expression"))?;

    let span = tok.span;
    let id = ctx.statement_id;

    match &tok.node {
        Token::Int(val) => Ok(Expr::int(val.parse().unwrap_or(0), span, id)),
        Token::Float(val) => Ok(Expr::float(val.parse().unwrap_or(0.0), span, id)),
        Token::Bool(val) => Ok(Expr::bool(*val, span, id)),
        Token::String(s) => Ok(Expr::string(s.to_string(), span, id)),
        Token::Identifier(name) => Ok(Expr::variable(name.to_string(), span, id)),
        Token::LParen => {
            let expr = parse_expr(stream, ctx)?;
            stream.expect(&Token::RParen).map_err(|_| {
                ParseError::new(span, "Expected ')' after parenthesized expression")
            })?;
            Ok(expr)
        }
        _ => Err(ParseError::unexpected(tok, "primary expression")),
    }
}
