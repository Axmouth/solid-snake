use std::collections::BTreeMap;

use crate::{error_reporting::CompileError, new_parser::lexer::token::Token, parser::Rule};

#[derive(Debug)]
pub struct ASTNodeContainer {
    pub node: ASTNode,
    pub span: Span,
    pub statement_id: usize,
}

impl ASTNodeContainer {
    pub fn pretty(&self, indent: usize) -> String {
        let printed = self.node.pretty(indent);
        format!("[{}:{}] {}", self.span.line, self.statement_id, printed)
    }
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

    pub fn from_token(token: &Token) -> Option<BinaryOp> {
        Some(match token {
            Token::EqEq => BinaryOp::Equal,
            Token::NotEq => BinaryOp::NotEqual,
            Token::Gt => BinaryOp::GreaterThan,
            Token::Gte => BinaryOp::GreaterThanOrEqual,
            Token::Lt => BinaryOp::LessThan,
            Token::Lte => BinaryOp::LessThanOrEqual,
            Token::Plus => BinaryOp::Add,
            Token::Minus => BinaryOp::Subtract,
            Token::Star => BinaryOp::Multiply,
            Token::Slash => BinaryOp::Divide,
            Token::Percent => BinaryOp::Modulo,
            Token::And => BinaryOp::LogicalAnd,
            Token::Or => BinaryOp::LogicalOr,
            Token::LShift => BinaryOp::ShiftLeft,
            Token::RShift => BinaryOp::ShiftRight,
            Token::Ampersand => BinaryOp::BitwiseAnd,
            Token::Pipe => BinaryOp::BitwiseOr,
            _ => None?,
        })
    }

    pub fn pretty(&self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
            BinaryOp::Modulo => "%",
            BinaryOp::Equal => "==",
            BinaryOp::NotEqual => "!=",
            BinaryOp::LessThan => "<",
            BinaryOp::LessThanOrEqual => "<=",
            BinaryOp::GreaterThan => ">",
            BinaryOp::GreaterThanOrEqual => ">=",
            BinaryOp::LogicalAnd => "and",
            BinaryOp::LogicalOr => "or",
            BinaryOp::ShiftLeft => "<<",
            BinaryOp::ShiftRight => ">>",
            BinaryOp::BitwiseAnd => "&",
            BinaryOp::BitwiseOr => "|",
        }
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

impl UnaryOp {
    pub fn pretty(&self) -> &'static str {
        match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "not",
        }
    }
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
    pub statement_id: usize,
}

impl Expr {
    pub fn bool(val: bool, span: Span, statement_id: usize) -> Self {
        Self {
            kind: ExprKind::Bool(val),
            span,
            statement_id,
        }
    }

    pub fn string(val: String, span: Span, statement_id: usize) -> Self {
        Self {
            kind: ExprKind::String(val),
            span,
            statement_id,
        }
    }

    pub fn number(val: String, span: Span, statement_id: usize) -> Self {
        Self {
            kind: ExprKind::Number(val),
            span,
            statement_id,
        }
    }

    pub fn float(val: f64, span: Span, statement_id: usize) -> Self {
        Self {
            kind: ExprKind::Float(val),
            span,
            statement_id,
        }
    }

    pub fn int(val: i64, span: Span, statement_id: usize) -> Self {
        Self {
            kind: ExprKind::Int(val),
            span,
            statement_id,
        }
    }

    pub fn uint(val: u64, span: Span, statement_id: usize) -> Self {
        Self {
            kind: ExprKind::UInt(val),
            span,
            statement_id,
        }
    }

    pub fn variable(val: String, span: Span, statement_id: usize) -> Self {
        Self {
            kind: ExprKind::Variable(VarName::new(val, span)),
            span,
            statement_id,
        }
    }

    pub fn un_op(expr: Expr, op: UnaryOp, span: Span, statement_id: usize) -> Self {
        Self {
            kind: ExprKind::UnaryOp {
                op,
                expr: Box::new(expr),
            },
            span,
            statement_id,
        }
    }

    pub fn bin_op(left: Expr, right: Expr, op: BinaryOp, span: Span, statement_id: usize) -> Self {
        Self {
            kind: ExprKind::BinaryOp {
                left: Box::new(left),
                right: Box::new(right),
                op,
            },
            span,
            statement_id,
        }
    }

    pub fn invalid(span: Span, statement_id: usize) -> Self {
        Self {
            kind: ExprKind::Invalid,
            span,
            statement_id,
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

    pub fn pretty(&self) -> String {
        match &self.kind {
            ExprKind::Bool(v) => v.to_string(),
            ExprKind::Int(i) => i.to_string(),
            ExprKind::Float(f) => f.to_string(),
            ExprKind::String(s) => format!("{:?}", s),
            ExprKind::Variable(v) => v.name.clone(),
            ExprKind::UnaryOp { op, expr } => format!("({}{})", op.pretty(), expr.pretty()),
            ExprKind::BinaryOp { left, op, right } => {
                format!("({} {} {})", left.pretty(), op.pretty(), right.pretty())
            }
            ExprKind::Invalid => "<error>".to_string(),
            ExprKind::Number(n) => n.clone(),
            ExprKind::UInt(u) => u.to_string(),
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
    Invalid,
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
            Self::Invalid => false,
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
    pub statement_id: usize,
}

// TODO refactor to struct variants
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
    TypeDefinition { name: String, ty: IntermediateType },
    Continue,
    Break,
    Comment(String),
}

fn pretty_print_block(body: &[ASTNodeContainer], indent: usize) -> String {
    let mut main_print = body
        .iter()
        .map(|s| s.pretty(indent + 1))
        .collect::<Vec<String>>()
        .join("\n");
    main_print.push('\n');
    main_print
}

impl ASTNode {
    pub fn pretty(&self, indent: usize) -> String {
        let indentation = "    ".repeat(indent);
        match self {
            ASTNode::VarDecl(name, Some(expr)) => {
                        format!("{}let {} = {}", indentation, name.name, expr.pretty())
                    }
            ASTNode::VarDecl(name, None) => format!("{}let {}", indentation, name.name),
            ASTNode::Assignment(name, expr) => {
                        format!("{}{} = {}", indentation, name.name, expr.pretty())
                    }
            ASTNode::IfStmt {
                        if_block,
                        elif_blocks,
                        else_block,
                    } => {
                        let mut out = format!(
                            "{}if {}:\n{}",
                            indentation,
                            if_block.condition.pretty(),
                            pretty_print_block(&if_block.body, indent)
                        );
                        for elif in elif_blocks {
                            let prefix = format!("[{}:{}] ", elif.condition.span.line, elif.statement_id);
                            out += &format!(
                                "{}{}elif {}:\n{}",
                                prefix,
                                indentation,
                                elif.condition.pretty(),
                                pretty_print_block(&elif.body, indent)
                            );
                        }
                        if let Some(else_block) = else_block {
                            let prefix = format!(
                                "[{}:{}] ",
                                else_block.condition.span.line, else_block.statement_id,
                            );
                            out += &format!(
                                "{}{}else:\n{}",
                                prefix,
                                indentation,
                                pretty_print_block(&else_block.body, indent)
                            );
                        }
                        out
                    }
            ASTNode::WhileStmt { condition, body } => format!(
                        "{}while {}:\n{}",
                        indentation,
                        condition.pretty(),
                        pretty_print_block(body, indent)
                    ),
            ASTNode::Continue => "continue".to_string(),
            ASTNode::Break => "break".to_string(),
            ASTNode::TypeDefinition { name, ty } => format!("type {} = {}", name, ty),
            ASTNode::Comment(comment) => format!("#{}", comment),
        }
    }
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



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntermediateEnumVariant {
    Tuple { inner: Vec<IntermediateType> },
    Struct { properties: BTreeMap<String, IntermediateType> },
    Unit,
}

impl IntermediateEnumVariant {
    pub fn try_to_processed(&self, span: Span) -> Result<ProcessedEnumVariant, CompileError> {
        Ok(match self {
            IntermediateEnumVariant::Tuple { inner } => ProcessedEnumVariant::Tuple {
                inner: inner
                    .iter()
                    .map(|v| v.try_to_processed(span))
                    .collect::<Result<Vec<ProcessedType>, CompileError>>()?,
            },
            IntermediateEnumVariant::Struct { properties } => ProcessedEnumVariant::Struct {
                properties: properties
                    .iter()
                    .map(|(k, v)| v.try_to_processed(span).map(|v| (k.clone(), v)))
                    .collect::<Result<BTreeMap<String, ProcessedType>, CompileError>>()?,
            },
            IntermediateEnumVariant::Unit => ProcessedEnumVariant::Unit,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntermediateType {
    Indeterminate,
    Number,
    Boolean,
    String,
    Int,
    UInt,
    Float,
    Custom { name: String },
    Array {
        inner: Box<IntermediateType>,
    },
    List {
        inner: Box<IntermediateType>,
    },
    Object {
        properties: BTreeMap<String, IntermediateType>,
    },
    Tuple {
        inner: Vec<IntermediateType>,
    },
    Enum {
        variants: BTreeMap<String, IntermediateEnumVariant>,
    },
    Indirect {
        target: Box<IntermediateType>,
    },
    Byte,
}

impl std::fmt::Display for IntermediateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO implement all types
        match self {
            IntermediateType::Indeterminate => write!(f, "?"),
            IntermediateType::Number => write!(f, "Num"),
            IntermediateType::Boolean => write!(f, "Bool"),
            IntermediateType::String => write!(f, "Str"),
            IntermediateType::Int => write!(f, "Int"),
            IntermediateType::UInt => write!(f, "UInt"),
            IntermediateType::Float => write!(f, "Float"),
            IntermediateType::Custom { name } => write!(f, "{}", name),
            IntermediateType::Array { inner: _ } => write!(f, "[]"),
            IntermediateType::List { inner: _ } => write!(f, "[,]"),
            IntermediateType::Object { properties: _ } => write!(f, "{{}}"),
            IntermediateType::Tuple { inner: _ } => write!(f, "()"),
            IntermediateType::Indirect { target } => write!(f, "->{}", target),
            IntermediateType::Enum { variants: _ } => write!(f, "|"),
            IntermediateType::Byte => write!(f, "b"),
        }
    }
}

impl Default for IntermediateType {
    fn default() -> Self {
        Self::Indeterminate
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessedEnumVariant {
    Tuple { inner: Vec<ProcessedType> },
    Struct { properties: BTreeMap<String, ProcessedType> },
    Unit,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessedType {
    Boolean,
    String,
    Int,
    UInt,
    Float,
    Array {
        inner: Box<ProcessedType>,
    },
    List {
        inner: Box<ProcessedType>,
    },
    Object {
        properties: BTreeMap<String, ProcessedType>,
    },
    Tuple {
        inner: Vec<ProcessedType>,
    },
    Enum {
        variants: BTreeMap<String, ProcessedEnumVariant>,
    },
    Indirect {
        target: Box<ProcessedType>,
    },
    Byte,
}

impl std::fmt::Display for ProcessedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO implement all types
        match self {
            ProcessedType::Boolean => write!(f, "Bool"),
            ProcessedType::String => write!(f, "Str"),
            ProcessedType::Int => write!(f, "Int"),
            ProcessedType::UInt => write!(f, "UInt"),
            ProcessedType::Float => write!(f, "Float"),
            ProcessedType::Array { inner: _ } => write!(f, "[]"),
            ProcessedType::List { inner: _ } => write!(f, "[,]"),
            ProcessedType::Object { properties: _ } => write!(f, "{{}}"),
            ProcessedType::Tuple { inner: _ } => write!(f, "()"),
            ProcessedType::Indirect { target } => write!(f, "->{}", target),
            ProcessedType::Enum { variants } => write!(f, "|"),
            ProcessedType::Byte => write!(f, "b"),
        }
    }
}

impl IntermediateType {
    // TODO
    pub fn try_to_processed(&self, span: Span) -> Result<ProcessedType, CompileError> {
        match self {
            IntermediateType::Indeterminate => Err(CompileError::internal_compiler_error(
                        "Expected typed to be resolved at this point",
                    )),
            IntermediateType::Number => Ok(ProcessedType::Int),
            IntermediateType::Boolean => Ok(ProcessedType::Boolean),
            IntermediateType::String => Ok(ProcessedType::String),
            IntermediateType::Int => Ok(ProcessedType::Int),
            IntermediateType::UInt => Ok(ProcessedType::UInt),
            IntermediateType::Float => Ok(ProcessedType::Float),
            // TODO
            IntermediateType::Custom { name } => todo!(),
            IntermediateType::Array { inner } => Ok(ProcessedType::Array {
                        inner: Box::new(inner.try_to_processed(span)?),
                    }),
            IntermediateType::List { inner } => Ok(ProcessedType::List {
                        inner: Box::new(inner.try_to_processed(span)?),
                    }),
            IntermediateType::Object { properties } => Ok(ProcessedType::Object {
                        properties: properties
                            .iter()
                            .map(|(k, v)| v.try_to_processed(span).map(|v| (k.clone(), v)))
                            .collect::<Result<BTreeMap<String, ProcessedType>, CompileError>>()?,
                    }),
            IntermediateType::Tuple { inner } => Ok(ProcessedType::Tuple {
                        inner: inner
                            .iter()
                            .map(|v| v.try_to_processed(span))
                            .collect::<Result<Vec<ProcessedType>, CompileError>>()?,
                    }),
            IntermediateType::Indirect { target } => Ok(ProcessedType::Indirect {
                        target: Box::new(target.try_to_processed(span)?),
                    }),
            IntermediateType::Enum { variants } => Ok(ProcessedType::Enum {
                        variants: variants
                            .iter()
                            .map(|(k, v)| v.try_to_processed(span).map(|o| (k.to_string(), o)))
                            .collect::<Result<BTreeMap<String, ProcessedEnumVariant>, CompileError>>()?,
                    }),
            IntermediateType::Byte => Ok(ProcessedType::Byte),
        }
    }

    // TODO adjustments so errors reference the top level type
    pub fn try_apply_type_hint(&self, other: &Self, span: Span) -> Result<Self, CompileError> {
        const CONCRETE_NUMBERS: [IntermediateType; 3] = [
            IntermediateType::Int,
            IntermediateType::UInt,
            IntermediateType::Float,
        ];
        if self == other
            || &IntermediateType::Indeterminate == self
            || self == &IntermediateType::Number && CONCRETE_NUMBERS.contains(other)
            || other == &IntermediateType::Number && CONCRETE_NUMBERS.contains(self)
        {
            return Ok(other.clone());
        }
        if other == &IntermediateType::Indeterminate {
            return Ok(self.clone());
        }
        // TODO specially handle objects if we apply type inference to create object types, merge if other superset of self
        if let (
            IntermediateType::Object { properties: p_self },
            IntermediateType::Object {
                properties: p_other,
            },
        ) = (self, other)
        {
            let keys_match = p_self.keys().zip(p_other.keys()).all(|(a, b)| a == b);
            if !keys_match {
                return Err(CompileError::type_mismatch(
                    format!("{:?}", other),
                    format!("{:?}", self),
                    span,
                ));
            }

            let new_props = p_self
                .iter()
                .zip(p_other.iter())
                .map(|((key, type_self), (_, type_other))| {
                    type_self
                        .try_apply_type_hint(type_other, span)
                        .map(|typ| (key.to_string(), typ))
                })
                .collect::<Result<BTreeMap<String, Self>, CompileError>>()?;

            return Ok(IntermediateType::Object {
                properties: new_props,
            });
        }
        if let (
            IntermediateType::Tuple { inner: inner_self },
            IntermediateType::Tuple { inner: inner_other },
        ) = (self, other)
        {
            let new_props = inner_self
                .iter()
                .zip(inner_other.iter())
                .map(|(type_self, type_other)| type_self.try_apply_type_hint(type_other, span))
                .collect::<Result<Vec<Self>, CompileError>>()?;

            return Ok(IntermediateType::Tuple { inner: new_props });
        }
        if let (
            IntermediateType::List { inner: inner_self },
            IntermediateType::List { inner: inner_other },
        ) = (self, other)
        {
            return Ok(IntermediateType::List {
                inner: Box::new(inner_self.try_apply_type_hint(inner_other, span)?),
            });
        }
        if let (
            IntermediateType::Array { inner: inner_self },
            IntermediateType::Array { inner: inner_other },
        ) = (self, other)
        {
            return Ok(IntermediateType::Array {
                inner: Box::new(inner_self.try_apply_type_hint(inner_other, span)?),
            });
        }

        Err(CompileError::type_mismatch(
            format!("{:?}", other),
            format!("{:?}", self),
            span,
        ))
    }

    pub fn type_from_expr(expr: &Expr) -> Result<Self, CompileError> {
        match &expr.kind {
            ExprKind::Bool(_) => Ok(Self::Boolean),
            ExprKind::Number(_) => Ok(Self::Number),
            ExprKind::Float(_) => Ok(Self::Float),
            ExprKind::Int(_) => Ok(Self::Int),
            ExprKind::UInt(_) => Ok(Self::UInt),
            ExprKind::String(_) => Ok(Self::String),
            ExprKind::Variable(_) => Ok(Self::Indeterminate),
            ExprKind::UnaryOp { op: _, expr } => Self::type_from_expr(expr),
            ExprKind::BinaryOp { left, op, right } => {
                // match op {
                //     BinaryOp::Add => match (left.typ(), right.typ()) {
                //         (IntermediateType::Int, IntermediateType::Int) => Ok(IntermediateType::Int),
                //         (IntermediateType::Float, IntermediateType::Float) => Ok(IntermediateType::Float),
                //         (IntermediateType::String, IntermediateType::String) => Ok(IntermediateType::String),
                //         _ => Err(...),
                //     },
                //     ...
                // }

                if op.is_comparison() {
                    return Ok(Self::Boolean);
                }

                Self::type_from_expr(left)?
                    .try_apply_type_hint(&(Self::type_from_expr(right)?), left.span)
            }
            ExprKind::Invalid => Ok(Self::Indeterminate),
        }
    }
}