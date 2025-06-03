use crate::ast::{ASTNode, ASTNodeContainer, Expr, IfGroup, Span, VarName};
use crate::error_reporting::CompileErrorList;
use crate::new_parser::expression::parse_expr;
use crate::new_parser::lexer::token::{Spanned, Token};
use crate::new_parser::{ParseContext, ParseError, ParseResult, TokenStream};

use super::typing::parse_type;

pub fn parse_program(stream: &mut TokenStream<'_>) -> (Vec<ASTNodeContainer>, CompileErrorList) {
    let mut ctx = ParseContext {
        errors: vec![],
        statement_id: 0,
    };

    let mut nodes = Vec::new();

    while !stream.at_end() {
        match parse_statement(stream, &mut ctx) {
            Ok(Some(node)) => nodes.push(node),
            Ok(None) => {}
            Err(err) => {
                ctx.record(err);
                // basic recovery: skip one token
                stream.next();
            }
        }
    }

    (nodes, ctx.to_compile_errors())
}

// TODO use a stack to avoid recursion
fn parse_statement(
    stream: &mut TokenStream<'_>,
    ctx: &mut ParseContext,
) -> Result<Option<ASTNodeContainer>, ParseError> {
    match stream.peek() {
        Some(tok) => match tok.node {
            Token::Newline => {
                stream.next();
                Ok(None)
            }
            Token::Let => parse_var_decl(stream, ctx).map(Some),
            Token::Identifier(_) => parse_assignment(stream, ctx).map(Some),
            Token::If => parse_if_stmt(stream, ctx).map(Some),
            Token::While => parse_while_stmt(stream, ctx).map(Some),
            Token::Break => {
                let span = stream.expect(&Token::Break)?;
                if !stream.at_end() {
                    stream.expect(&Token::Newline)?; // enforce end of statement
                }
                Ok(Some(ASTNodeContainer {
                    node: ASTNode::Break,
                    span,
                    statement_id: ctx.next_id(),
                }))
            }
            Token::Continue => {
                let span = stream.expect(&Token::Continue)?;
                if !stream.at_end() {
                    stream.expect(&Token::Newline)?; // enforce end of statement
                }
                Ok(Some(ASTNodeContainer {
                    node: ASTNode::Continue,
                    span,
                    statement_id: ctx.next_id(),
                }))
            }
            Token::TypeDef => parse_typedef_stmt(stream, ctx).map(Some),
            Token::Elif => Err(ParseError::new(
                tok.span,
                "Unexpected 'elif' header without matching 'if'",
            )),
            Token::Else => Err(ParseError::new(
                tok.span,
                "Unexpected 'else' header without matching 'if'",
            )),
            Token::Comment(comment) => Ok(Some(ASTNodeContainer {
                node: ASTNode::Comment(comment.to_string()),
                span: tok.span,
                statement_id: ctx.next_id(),
            })),
            _ => {
                let span = tok.span;
                Err(ParseError::new(span, "Unexpected token in statement"))
            }
        },
        None => Err(ParseError::eof("statement")),
    }
}

#[inline(always)]
fn parse_var_decl(
    stream: &mut TokenStream<'_>,
    ctx: &mut ParseContext,
) -> ParseResult<ASTNodeContainer> {
    let id = ctx.next_id();

    let let_tok_span = stream.expect(&Token::Let)?;
    let ident = stream
        .next()
        .ok_or_else(|| ParseError::eof("variable name"))?;

    let (name, ident_span) = match &ident.node {
        Token::Identifier(name) => (name.to_string(), ident.span),
        _ => return Err(ParseError::unexpected(ident, "identifier")),
    };

    let name = VarName::new(name, ident_span);

    let expr = if let Some(eq_tok) = stream.peek() {
        if eq_tok.node == Token::Assign {
            stream.next(); // consume '='
            Some(parse_expr(stream, ctx)?)
        } else {
            None
        }
    } else {
        None
    };

    let span = Span {
        start: let_tok_span.start,
        end: expr.as_ref().map(|e| e.span.end).unwrap_or(ident_span.end),
        line: let_tok_span.line,
        column: let_tok_span.column,
    };

    Ok(ASTNodeContainer {
        node: ASTNode::VarDecl(name, expr),
        span,
        statement_id: id,
    })
}

#[inline(always)]
fn parse_assignment(
    stream: &mut TokenStream<'_>,
    ctx: &mut ParseContext,
) -> ParseResult<ASTNodeContainer> {
    let id = ctx.next_id();
    let ident = stream
        .next()
        .ok_or_else(|| ParseError::eof("variable name"))?;

    let (name, name_span) = match &ident.node {
        Token::Identifier(name) => (name.to_string(), ident.span),
        _ => return Err(ParseError::unexpected(ident, "identifier")),
    };

    stream.expect(&Token::Assign)?;
    let expr = parse_expr(stream, ctx)?;

    let span = Span {
        start: name_span.start,
        end: expr.span.end,
        line: name_span.line,
        column: name_span.column,
    };

    Ok(ASTNodeContainer {
        node: ASTNode::Assignment(VarName::new(name, name_span), expr),
        span,
        statement_id: id,
    })
}

#[inline(always)]
fn parse_if_stmt(
    stream: &mut TokenStream<'_>,
    ctx: &mut ParseContext,
) -> ParseResult<ASTNodeContainer> {
    let if_id = ctx.next_id();
    let if_group = parse_if_group(stream, Token::If, ctx)?;

    let mut elif_groups = Vec::new();
    while let Some(tok) = stream.peek() {
        if tok.node == Token::Elif {
            elif_groups.push(parse_if_group(stream, Token::Elif, ctx)?);
        } else {
            break;
        }
    }

    let else_block = if let Some(tok) = stream.peek() {
        if tok.node == Token::Else {
            Some(parse_else_group(stream, ctx)?)
        } else {
            None
        }
    } else {
        None
    };

    let span = Span {
        start: if_group.condition.span.start,
        end: else_block
            .as_ref()
            .map(|g| {
                g.body
                    .last()
                    .map(|n| n.span.end)
                    .unwrap_or(g.condition.span.end)
            })
            .or_else(|| {
                elif_groups
                    .last()
                    .and_then(|g| g.body.last().map(|n| n.span.end))
            })
            .unwrap_or(if_group.condition.span.end),
        line: if_group.condition.span.line,
        column: if_group.condition.span.column,
    };

    Ok(ASTNodeContainer {
        node: ASTNode::IfStmt {
            if_block: if_group,
            elif_blocks: elif_groups,
            else_block,
        },
        span,
        statement_id: if_id,
    })
}

#[inline(always)]
fn parse_if_group(
    stream: &mut TokenStream<'_>,
    keyword: Token,
    ctx: &mut ParseContext,
) -> ParseResult<IfGroup> {
    let statement_id = ctx.next_id();
    stream.expect(&keyword)?;
    let condition = parse_expr(stream, ctx)?;
    stream.expect(&Token::Colon)?;
    stream.expect(&Token::Newline)?;
    let body = parse_block(stream, ctx)?;

    Ok(IfGroup {
        condition,
        body,
        statement_id,
    })
}

#[inline(always)]
fn parse_else_group(stream: &mut TokenStream<'_>, ctx: &mut ParseContext) -> ParseResult<IfGroup> {
    let statement_id = ctx.next_id();
    let else_span = stream.expect(&Token::Else)?;
    stream.expect(&Token::Colon)?;
    stream.expect(&Token::Newline)?;
    let dummy_condition = Expr::bool(true, else_span, statement_id);
    let body = parse_block(stream, ctx)?;
    Ok(IfGroup {
        condition: dummy_condition,
        body,
        statement_id,
    })
}

#[inline(always)]
fn parse_block(
    stream: &mut TokenStream<'_>,
    ctx: &mut ParseContext,
) -> ParseResult<Vec<ASTNodeContainer>> {
    stream.expect(&Token::Indent)?;

    let mut items = Vec::new();

    while !stream.at_end() {
        if let Some(tok) = stream.peek() {
            if tok.node == Token::Newline {
                stream.next(); // skip empty line
                continue;
            }

            if tok.node == Token::Dedent {
                stream.next(); // consume dedent
                break;
            }

            if let Some(stmt) = parse_statement(stream, ctx)? {
                items.push(stmt);
            }
        } else {
            return Err(ParseError::eof("block"));
        }
    }

    skip_newlines(stream);

    Ok(items)
}

#[inline(always)]
fn parse_while_stmt(
    stream: &mut TokenStream<'_>,
    ctx: &mut ParseContext,
) -> ParseResult<ASTNodeContainer> {
    let id = ctx.next_id();
    let while_tok = stream.expect(&Token::While)?;
    let condition = parse_expr(stream, ctx)?;
    stream.expect(&Token::Colon)?;
    stream.expect(&Token::Newline)?;
    let body = parse_block(stream, ctx)?;

    let end = body
        .last()
        .map(|n| n.span.end)
        .unwrap_or(condition.span.end);

    let span = Span {
        start: while_tok.start,
        end,
        line: while_tok.line,
        column: while_tok.column,
    };

    Ok(ASTNodeContainer {
        node: ASTNode::WhileStmt { condition, body },
        span,
        statement_id: id,
    })
}

fn parse_typedef_stmt(
    stream: &mut TokenStream<'_>,
    ctx: &mut ParseContext,
) -> ParseResult<ASTNodeContainer> {
    let statement_id = ctx.next_id();
    let start_span = stream.expect(&Token::TypeDef)?;

    let ident = stream
        .next()
        .ok_or_else(|| ParseError::eof("type name"))?;
    let name = match &ident.node {
        Token::Identifier(name) => name.to_string(),
        _ => return Err(ParseError::unexpected(ident, "identifier")),
    };

    if let Some(next) = stream.peek() {
        if next.node == Token::Assign {
            stream.next(); // consume '='
        }
    }

    let ty = parse_type(stream, ctx);

    let span = Span {
        start: start_span.start,
        end: stream.peek().map(|t| t.span.end).unwrap_or(start_span.end),
        line: start_span.line,
        column: start_span.column,
    };

    Ok(ASTNodeContainer {
        node: ASTNode::TypeDefinition { name, ty },
        span,
        statement_id,
    })
}

#[inline(always)]
fn skip_newlines(stream: &mut TokenStream<'_>) {
    while let Some(Spanned {
        node: Token::Newline,
        ..
    }) = stream.peek()
    {
        stream.next();
    }
}
