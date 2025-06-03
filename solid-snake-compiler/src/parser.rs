use pest_derive::Parser;

use crate::{
    ast::{ASTNode, ASTNodeContainer, Expr, IfGroup, Span, VarName, parse_bin_op, parse_un_op},
    error_reporting::{CompileError, CompileErrorList},
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SolidSnakeParser;

pub fn to_span(span: pest::Span) -> Span {
    let (line, column) = span.start_pos().line_col();
    Span {
        line,
        column,
        start: span.start(),
        end: span.end(),
    }
}

macro_rules! expect_or_push {
    ($opt:expr, $error_list:expr, $error:expr) => {
        match $opt {
            Some(val) => val,
            None => {
                $error_list.push_error($error);
                continue;
            }
        }
    };
}

macro_rules! handle_error_soft {
    ($opt:expr, $error_list:expr) => {
        match $opt {
            Ok(val) => val,
            Err(error) => {
                $error_list.extend(error);
                continue;
            }
        }
    };
}

macro_rules! handle_missing {
    ($opt:expr, $span:expr) => {
        $opt.ok_or(CompileErrorList::new().with_error(CompileError::unexpected_eof($span)))
    };
}

pub fn build_ast(
    pairs: pest::iterators::Pairs<Rule>,
    mut statement_id: usize,
) -> (Vec<ASTNodeContainer>, CompileErrorList, usize) {
    let mut ast = Vec::new();
    let mut error_list = CompileErrorList::new();
    for pair in pairs {
        let inner = pair.clone().into_inner();
        if inner.len() < 1 {
            // continue;
        }

        match pair.as_rule() {
            Rule::COMMENT => continue, // ✅ skip comments!
            Rule::VarDecl => {
                let mut inner = pair.clone().into_inner();
                let next_pair = expect_or_push!(
                    inner.next(),
                    error_list,
                    CompileError::unexpected_eof(Span::from_pair(&pair))
                );

                let name = next_pair.as_str().to_string();

                let initializer = match inner.next() {
                    Some(expr_or_eq) => {
                        if expr_or_eq.as_rule() == Rule::Expr {
                            Some(handle_error_soft!(
                                parse_expr(expr_or_eq, statement_id),
                                error_list
                            ))
                        } else {
                            // Probably matched "=" → skip it
                            let expr = inner.next().expect("Expected expression after '='");
                            Some(handle_error_soft!(
                                parse_expr(expr, statement_id),
                                error_list
                            ))
                        }
                    }
                    None => None,
                };

                let span = to_span(pair.as_span());
                let name_span = to_span(next_pair.as_span());
                statement_id += 1;

                ast.push(ASTNodeContainer {
                    node: ASTNode::VarDecl(VarName::new(name, name_span), initializer),
                    span,
                    statement_id,
                });
            }
            Rule::Assignment => {
                let mut inner_rules = inner;
                let next_pair = expect_or_push!(
                    inner_rules.next(),
                    error_list,
                    CompileError::unexpected_eof(Span::from_pair(&pair))
                );
                let name = next_pair.as_str().to_string();
                let expr_pair = expect_or_push!(
                    inner_rules.next(),
                    error_list,
                    CompileError::unexpected_eof(Span::from_pair(&pair))
                );

                let expr = handle_error_soft!(parse_expr(expr_pair, statement_id), error_list);
                let span = to_span(pair.as_span());
                let name_span = to_span(next_pair.as_span());
                statement_id += 1;

                ast.push(ASTNodeContainer {
                    node: ASTNode::Assignment(VarName::new(name, name_span), expr),
                    span,
                    statement_id,
                });
            }
            Rule::IfStmt => {
                let mut inner = pair.clone().into_inner();
                let mut if_block: Option<IfGroup> = None;
                let mut elif_blocks = Vec::new();
                let mut else_block = None;
                let if_statement_id = statement_id;
                statement_id += 1;

                for part in inner {
                    match part.as_rule() {
                        Rule::IfBlock | Rule::ElifBlock => {
                            let mut part_inner = part.clone().into_inner();
                            let cond_pair = expect_or_push!(
                                part_inner.next(),
                                error_list,
                                CompileError::unexpected_eof(Span::from_pair(&part))
                            );
                            let condition =
                                handle_error_soft!(parse_expr(cond_pair, statement_id), error_list);
                            let block_pair = expect_or_push!(
                                part_inner.next(),
                                error_list,
                                CompileError::unexpected_eof(Span::from_pair(&part))
                            );
                            statement_id += 1;
                            let header_id = statement_id;
                            let (body, suberrors, new_statement_id) =
                                build_ast(block_pair.into_inner(), statement_id);
                            statement_id = new_statement_id;
                            error_list.extend(suberrors);
                            let group = IfGroup {
                                condition,
                                body,
                                statement_id: header_id,
                            };

                            if part.as_rule() == Rule::IfBlock {
                                if_block = Some(group);
                            } else {
                                elif_blocks.push(group);
                            }
                        }
                        Rule::ElseBlock => {
                            let block_pair = expect_or_push!(
                                part.clone().into_inner().next(),
                                error_list,
                                CompileError::unexpected_eof(Span::from_pair(&part))
                            );
                            statement_id += 1;
                            let header_id = statement_id;
                            let (body, suberrors, new_statement_id) =
                                build_ast(block_pair.into_inner(), statement_id);
                            statement_id = new_statement_id;
                            error_list.extend(suberrors);
                            else_block = Some(IfGroup {
                                condition: Expr::bool(true, Span::from_pair(&part), statement_id), // dummy
                                body,
                                statement_id: header_id,
                            });
                        }
                        _ => {
                            println!("Unexpected part in IfStmt: {:?}", part.as_rule());
                        }
                    }

                    statement_id += 1;
                }

                let span = to_span(pair.as_span());

                let if_block = if_block.expect("if_block must exist in IfStmt");
                ast.push(ASTNodeContainer {
                    node: ASTNode::IfStmt {
                        if_block,
                        elif_blocks,
                        else_block,
                    },
                    span,
                    statement_id: if_statement_id,
                });
            }
            Rule::WhileStmt => {
                let mut inner = pair.clone().into_inner();
                let cond_pair = expect_or_push!(
                    inner.next(),
                    error_list,
                    CompileError::unexpected_eof(Span::from_pair(&pair))
                );
                let condition = handle_error_soft!(parse_expr(cond_pair, statement_id), error_list);
                statement_id += 1;
                let while_statement_id = statement_id;
                statement_id += 1;

                let block_pair = expect_or_push!(
                    inner.next(),
                    error_list,
                    CompileError::unexpected_eof(Span::from_pair(&pair))
                );

                let (body, suberrors, new_statement_id) =
                    build_ast(block_pair.into_inner(), statement_id);
                statement_id = new_statement_id;
                error_list.extend(suberrors);

                let span = to_span(pair.as_span());
                ast.push(ASTNodeContainer {
                    node: ASTNode::WhileStmt { condition, body },
                    span,
                    statement_id: while_statement_id,
                });
            }
            Rule::BreakStmt => {
                let span = to_span(pair.as_span());
                statement_id += 1;
                ast.push(ASTNodeContainer {
                    node: ASTNode::Break,
                    span,
                    statement_id,
                });
            }
            Rule::ContinueStmt => {
                let span = to_span(pair.as_span());
                statement_id += 1;
                ast.push(ASTNodeContainer {
                    node: ASTNode::Continue,
                    span,
                    statement_id,
                });
            }
            Rule::Statement => {
                // Recurse into the content of the statement
                let (sub_ast, sub_errors, new_statement_id) =
                    build_ast(pair.into_inner(), statement_id);
                statement_id = new_statement_id;
                ast.extend(sub_ast);
                error_list.extend(sub_errors);
            }
            Rule::INDENT | Rule::DEDENT | Rule::BlankLine => {
                // skip — structural only
            }
            Rule::Program => {
                //let parse_iter = inner.flat_map(|pair| build_ast(pair.into_inner(), source)).collect::<Result<Vec<_>, CompileError>>()?;
                let statement_id_snap = statement_id;
                // for (new_ast, errors, new_statement_id) in
                //     inner.map(|pair| build_ast(pair.into_inner(), statement_id_snap))
                // {
                //     ast.extend(new_ast);
                //     error_list.extend(errors);
                //     statement_id = new_statement_id;
                // }
                for pair in inner {
                    let (new_ast, errors, new_statement_id) =
                        build_ast(pair.into_inner(), statement_id_snap);
                    ast.extend(new_ast);
                    error_list.extend(errors);
                    statement_id = new_statement_id;
                }
                //ast.extend(parse_iter);
            }
            _ => {
                println!("WARNING : unhandled {:?}", pair.as_rule())
            }
        }
    }
    (ast, error_list, statement_id)
}

fn parse_expr(
    pair: pest::iterators::Pair<Rule>,
    statement_id: usize,
) -> Result<Expr, CompileErrorList> {
    let span = Span::from_pair(&pair);
    match pair.as_rule() {
        Rule::Int => pair
            .as_str()
            .parse()
            .map_err(|_| {
                CompileErrorList::new().with_error(CompileError::invalid_numeric(
                    pair.as_str().to_string(),
                    Span::from_pair(&pair),
                ))
            })
            .map(|val| Expr::number(val, span, statement_id)),
        Rule::Float => pair
            .as_str()
            .parse()
            .map_err(|_| {
                CompileErrorList::new().with_error(CompileError::invalid_numeric(
                    pair.as_str().to_string(),
                    Span::from_pair(&pair),
                ))
            })
            .map(|val| Expr::float(val, span, statement_id)),
        Rule::Number => pair
            .as_str()
            .parse()
            .map_err(|_| {
                CompileErrorList::new().with_error(CompileError::invalid_numeric(
                    pair.as_str().to_string(),
                    Span::from_pair(&pair),
                ))
            })
            .map(|val| Expr::number(val, span, statement_id)),
        Rule::BoolValue => pair
            .as_str()
            .parse()
            .map_err(|_| {
                CompileErrorList::new().with_error(CompileError::unexpected_error(
                    format!("Expected valid boolean value, got '{}'", pair.as_str()),
                    Span::from_pair(&pair),
                ))
            })
            .map(|val| Expr::bool(val, span, statement_id)),
        Rule::Identifier => Ok(Expr::variable(
            pair.as_str().to_string(),
            span,
            statement_id,
        )),
        Rule::String => Ok(Expr::string(pair.as_str().to_string(), span, statement_id)),
        Rule::LogicalOr
        | Rule::LogicalAnd
        | Rule::Equality
        | Rule::AddSub
        | Rule::MulDiv
        | Rule::Comparison => {
            let mut inner = pair.into_inner();
            // TODO less early fail?
            let inner_next = handle_missing!(inner.next(), span)?;
            let mut left = parse_expr(inner_next, statement_id)?;

            fn next_non_comment<'a>(
                iter: &mut pest::iterators::Pairs<'a, Rule>,
            ) -> Option<pest::iterators::Pair<'a, Rule>> {
                iter.find(|pair| pair.as_rule() != Rule::COMMENT)
            }
            while let Some(op) = next_non_comment(&mut inner) {
                // TODO less early fail?
                let right_pair = handle_missing!(next_non_comment(&mut inner), span)?;
                let right = parse_expr(right_pair, statement_id)?;
                let op_span = Span::from_pair(&op);
                left = Expr::bin_op(
                    left,
                    right,
                    parse_bin_op(op.as_str(), op_span)?,
                    span,
                    statement_id,
                );
            }

            Ok(left)
        }
        Rule::UnaryExpr => {
            let mut inner = pair.into_inner();
            let mut unary_ops = vec![];

            while let Some(next) = inner.peek() {
                if next.as_rule() == Rule::UnaryOp {
                    unary_ops.push((
                        handle_missing!(inner.next(), span)?.as_str().to_string(),
                        span,
                    ));
                } else {
                    break;
                }
            }

            let mut expr = parse_expr(handle_missing!(inner.next(), span)?, statement_id)?;
            for (op, un_span) in unary_ops.into_iter().rev() {
                let expr_span = expr.span;
                expr = Expr::un_op(expr, parse_un_op(&op, un_span)?, expr_span, statement_id);
            }
            Ok(expr)
        }
        Rule::Primary => parse_expr(
            handle_missing!(pair.into_inner().next(), span)?,
            statement_id,
        ),
        Rule::Expr => {
            // Just forward the call to its only child
            parse_expr(
                handle_missing!(pair.into_inner().next(), span)?,
                statement_id,
            )
        }
        _ => panic!("Unhandled expr rule: {:?}", pair.as_rule()),
    }
}
