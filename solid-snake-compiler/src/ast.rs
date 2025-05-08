use crate::{error_reporting::CompileError, parser::Rule};

#[derive(Debug)]
pub struct ASTNodeContainer {
    pub node: ASTNode,
    pub span: Span,
    pub statement_id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LogicalAnd,
    LogicalOr,
    ShiftLeft,
    ShiftRight,
    BitwiseAnd,
    BitwiseOr,
}

pub fn parse_bin_op(s: &str, span: Span) -> Result<BinaryOp, CompileError> {
    match s {
        "+" => Ok(BinaryOp::Add),
        "-" => Ok(BinaryOp::Subtract),
        "*" => Ok(BinaryOp::Multiply),
        "/" => Ok(BinaryOp::Divide),
        "%" => Ok(BinaryOp::Modulo),
        "==" => Ok(BinaryOp::Equal),
        "!=" => Ok(BinaryOp::NotEqual),
        "<" => Ok(BinaryOp::LessThan),
        ">" => Ok(BinaryOp::GreaterThan),
        "<=" => Ok(BinaryOp::LessThanOrEqual),
        ">=" => Ok(BinaryOp::GreaterThanOrEqual),
        "<<" => Ok(BinaryOp::ShiftLeft),
        ">>" => Ok(BinaryOp::ShiftRight),
        "&" => Ok(BinaryOp::BitwiseAnd),
        "|" => Ok(BinaryOp::BitwiseOr),
        "and" => Ok(BinaryOp::LogicalAnd),
        "or" => Ok(BinaryOp::LogicalOr),
        _ => Err(CompileError::invalid_operand(s.to_string(), span)),
    }
}

impl BinaryOp {
    pub fn is_comparison(&self) -> bool {
        matches!(
            self,
            BinaryOp::LessThan
                | BinaryOp::LessThanOrEqual
                | BinaryOp::GreaterThan
                | BinaryOp::GreaterThanOrEqual
                | BinaryOp::NotEqual
                | BinaryOp::Equal
        )
    }

    pub fn is_commutative_and_associative(&self) -> bool {
        matches!(
            self,
            BinaryOp::Add | BinaryOp::Multiply | BinaryOp::BitwiseAnd | BinaryOp::BitwiseOr
        )
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
            BinaryOp::Modulo => "%",
            BinaryOp::Equal => "==",
            BinaryOp::NotEqual => "-",
            BinaryOp::LessThan => "<",
            BinaryOp::GreaterThan => ">",
            BinaryOp::LessThanOrEqual => "=<",
            BinaryOp::GreaterThanOrEqual => ">=",
            BinaryOp::LogicalAnd => "and",
            BinaryOp::LogicalOr => "or",
            BinaryOp::ShiftLeft => "<<",
            BinaryOp::ShiftRight => ">>",
            BinaryOp::BitwiseAnd => "&",
            BinaryOp::BitwiseOr => "|",
        };
        write!(f, "{}", op_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

pub fn parse_un_op(s: &str, span: Span) -> Result<UnaryOp, CompileError> {
    match s {
        "-" => Ok(UnaryOp::Neg),
        "not" => Ok(UnaryOp::Not),
        _ => Err(CompileError::invalid_operand(s.to_string(), span)),
    }
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn bool(val: bool, span: Span) -> Self {
        Self {
            kind: ExprKind::Bool(val),
            span,
        }
    }

    pub fn string(val: String, span: Span) -> Self {
        Self {
            kind: ExprKind::String(val),
            span,
        }
    }

    pub fn number(val: String, span: Span) -> Self {
        Self {
            kind: ExprKind::Number(val),
            span,
        }
    }

    pub fn float(val: f64, span: Span) -> Self {
        Self {
            kind: ExprKind::Float(val),
            span,
        }
    }

    pub fn int(val: i64, span: Span) -> Self {
        Self {
            kind: ExprKind::Int(val),
            span,
        }
    }

    pub fn uint(val: u64, span: Span) -> Self {
        Self {
            kind: ExprKind::UInt(val),
            span,
        }
    }

    pub fn variable(val: String, span: Span) -> Self {
        Self {
            kind: ExprKind::Variable(VarName::new(val, span)),
            span,
        }
    }

    pub fn un_op(expr: Expr, op: UnaryOp, span: Span) -> Self {
        Self {
            kind: ExprKind::UnaryOp {
                op,
                expr: Box::new(expr),
            },
            span,
        }
    }

    pub fn bin_op(left: Expr, right: Expr, op: BinaryOp, span: Span) -> Self {
        Self {
            kind: ExprKind::BinaryOp {
                left: Box::new(left),
                right: Box::new(right),
                op,
            },
            span,
        }
    }

    pub fn is_numeric_literal(&self) -> bool {
        matches!(
            self.kind,
            ExprKind::Number(..) | ExprKind::Int(..) | ExprKind::UInt(..) | ExprKind::Float(..)
        )
    }

    pub fn is_bool_literal(&self) -> bool {
        matches!(self.kind, ExprKind::Bool(..))
    }

    pub fn normalized(&self) -> Self {
        Self {
            kind: self.kind.normalized(),
            ..self.clone()
        }
    }
}

// TODO more literal values like boolean, string, array, list, tuple dictionary/object
// TODO type declarations, first primitives, then complex types (ie List[Int]), then custom (anonymous structs effectively)
#[derive(Debug, Clone)]
pub enum ExprKind {
    Bool(bool),
    Number(String),
    Float(f64),
    Int(i64),
    UInt(u64),
    String(String),
    Variable(VarName),
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

impl ExprKind {
    pub fn is_const(&self) -> bool {
        match self {
            ExprKind::Bool(_) => true,
            ExprKind::Number(_) => true,
            ExprKind::String(_) => true,
            ExprKind::Float(_) => true,
            ExprKind::Int(_) => true,
            ExprKind::UInt(_) => true,
            ExprKind::Variable(var_name) => {
                // TODO
                false
            }
            ExprKind::UnaryOp { op: _, expr } => expr.kind.is_const(),
            ExprKind::BinaryOp { left, op: _, right } => {
                left.kind.is_const() && right.kind.is_const()
            }
        }
    }

    /// Pushes constants to the left for easier constant folding
    pub fn normalized(&self) -> Self {
        match self {
            ExprKind::BinaryOp { left, op, right } => {
                dbg!(op);
                // First normalize children
                let mut left = left.normalized();
                let mut right = right.normalized();

                if !op.is_commutative_and_associative() {
                    return ExprKind::BinaryOp {
                        left: Box::new(left.normalized()),
                        op: *op,
                        right: Box::new(right.normalized()),
                    };
                }

                // Push constant from right to left if possible
                if !left.kind.is_const() && right.kind.is_const() {
                    std::mem::swap(&mut left, &mut right);
                }

                // ⬅️ Look one level deeper in left: (a * x) * const → (a * const) * x
                if let ExprKind::BinaryOp {
                    left: _l2,
                    op: inner_op,
                    right: r2,
                } = &mut left.kind
                {
                    if *inner_op == *op && !r2.kind.is_const() && right.kind.is_const() {
                        std::mem::swap(r2, &mut Box::new(right.clone()));
                    }
                }

                // ➡️ Look one level deeper in right: x * (a * const) → (a * const) * x
                let right_clone = right.clone();
                if let ExprKind::BinaryOp {
                    left: l2,
                    op: inner_op,
                    right: r2,
                } = &mut right.kind
                {
                    if *inner_op == *op && !l2.kind.is_const() && r2.kind.is_const() {
                        std::mem::swap(l2, &mut Box::new(left.clone()));
                        return ExprKind::BinaryOp {
                            left: Box::new(right_clone), // now right is the folded constants
                            op: *op,
                            right: Box::new(*l2.clone()), // old left is moved
                        };
                    }
                }

                ExprKind::BinaryOp {
                    left: Box::new(left.normalized()),
                    op: *op,
                    right: Box::new(right.normalized()),
                }
            }
            _ => self.clone(),
        }
    }
}

pub enum ConstValue {}

#[derive(Debug)]
pub struct IfGroup {
    pub condition: Expr,
    pub body: Vec<ASTNodeContainer>,
}

#[derive(Debug)]
pub enum ASTNode {
    VarDecl(VarName, Option<Expr>),
    // TODO named
    Assignment(VarName, Expr), // assignment = name = expr (we keep expr simple for now)
    IfStmt {
        if_block: IfGroup,
        elif_blocks: Vec<IfGroup>,
        else_block: Option<IfGroup>,
    },
    WhileStmt {
        condition: Expr,
        body: Vec<ASTNodeContainer>,
    },
    Continue,
    Break,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub start: usize, // byte offset
    pub end: usize,   // byte offset
}

impl Span {
    pub fn from_pair(pair: &pest::iterators::Pair<Rule>) -> Self {
        let span = pair.as_span();
        let (line, column) = pair.line_col();
        let start = span.start();
        let end = span.end();

        Span {
            line,
            column,
            end,
            start,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VarName {
    name: String,
    span: Span,
}

impl VarName {
    pub fn new<S: Into<String>>(name: S, span: Span) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
