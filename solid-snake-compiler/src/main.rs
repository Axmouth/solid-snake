use ariadne::{Label, Report, ReportKind, Source};
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashSet;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct SolidSnakeParser;

#[derive(Debug)]
struct ASTNodeContainer {
    node: ASTNode,
    span: Span,
}

#[derive(Debug)]
enum ASTNode {
    VarDecl(String),
    // TODO named
    Assignment(String, String), // assignment = name = expr (we keep expr simple for now)
}

#[derive(Debug)]
struct Span {
    line: usize,
    column: usize,
    length: usize,
}

#[derive(Debug)]
enum CompileError {
    UndefinedVariable { name: String, span: Span },
}

fn main() {
    let source = r#"
x = 5
y = 10
"#;

    match SolidSnakeParser::parse(Rule::program, source) {
        Ok(pairs) => {
            let ast = build_ast(pairs);

            match analyze_ast(&ast) {
                Ok(()) => println!("Compilation succeeded! AST: {:#?}", ast),
                Err(err) => report_error(err, source),
            }
        }
        Err(e) => {
            println!("Syntax Error: {}", e);
        }
    }
}

fn build_ast(pairs: pest::iterators::Pairs<Rule>) -> Vec<ASTNode> {
    let mut ast = Vec::new();
    for pair in pairs {
        let mut inner = pair.clone().into_inner();
        if inner.len() < 1 {
            continue;
        }
        match pair.as_rule() {
            Rule::statement => match inner {
                _ => {}
            },
            Rule::var_decl => {
                let name = inner.next().unwrap().as_str().to_string();
                ast.push(ASTNode::VarDecl(name));
            }
            Rule::assignment => {
                let mut inner_rules = inner;
                let name = inner_rules.next().unwrap().as_str().to_string();
                let expr = inner_rules.next().unwrap().as_str().to_string();
                ast.push(ASTNode::Assignment(name, expr));
            }
            Rule::program => {
                let parse_iter = inner.flat_map(|pair| build_ast(pair.into_inner()));
                ast.extend(parse_iter);
            }
            _ => {}
        }
    }
    ast
}

fn analyze_ast(ast: &[ASTNode]) -> Result<(), CompileError> {
    let mut initialized_vars = HashSet::new();

    for node in ast {
        match node {
            ASTNode::VarDecl(name) => {
                initialized_vars.insert(name.clone());
            }
            ASTNode::Assignment(name, _) => {
                if !initialized_vars.contains(name) {
                    return Err(CompileError::UndefinedVariable {
                        name: name.clone(),
                        span: Span {
                            line: 3,
                            column: 1,
                            length: name.len(),
                        },
                    });
                }
            }
        }
    }

    Ok(())
}

fn report_error(error: CompileError, source: &str) {
    match error {
        CompileError::UndefinedVariable { name, span } => {
            let lines: Vec<&str> = source.lines().collect();
            let source_file = "src/test.ss";

            Report::build(
                ReportKind::Error,
                (source_file, (span.column - 1)..span.column),
            )
            .with_message(format!("Undefined variable '{}'", name))
            .with_label(
                Label::new((source_file, (span.line - 1)..span.line))
                    .with_message("Variable used before being declared.")
                    .with_color(ariadne::Color::Red),
            )
            .with_note("Declare the variable first with 'let'.")
            .finish()
            .print((source_file, Source::from(source)))
            .unwrap();
        }
    }
}
