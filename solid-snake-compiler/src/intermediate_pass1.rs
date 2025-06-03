use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    usize,
};

use crate::{
    ast::{ASTNode, ASTNodeContainer, BinaryOp, Expr, ExprKind, IntermediateType, ProcessedType, Span, UnaryOp},
    error_reporting::{CompileError, CompileErrorList},
};

pub struct Register(u8);

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Label(String);

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Label(txt) = self;
        write!(f, "{txt}:")
    }
}

#[derive(Debug, Default, Clone)]
struct LabelAllocator {
    counters: HashMap<LabelKind, usize>,
}

impl LabelAllocator {
    fn new() -> Self {
        Self {
            counters: HashMap::new(),
        }
    }

    fn fresh_label(&mut self, kind: LabelKind) -> Label {
        let prefix = match kind {
            LabelKind::If => "if",
            LabelKind::Loop => "loop",
            LabelKind::Else => "else",
            LabelKind::Elif => "elif",
            LabelKind::End => "end",
        };
        let count = self.counters.entry(kind).or_default();
        let label = Label(format!("{}_{}", prefix, *count));
        *count += 1;
        label
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
enum LabelKind {
    If,
    Loop,
    Else,
    Elif,
    End,
}

#[derive(Debug, Clone)]
pub struct IRInstruction {
    pub kind: IRStmt,
    pub span: Span,
    pub statement_id: usize,
    pub scope_id: usize,
}

impl std::fmt::Display for IRInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl IRInstruction {
    pub fn drop(var: IRVar, span: Span, statement_id: usize, scope_id: usize) -> Self {
        Self {
            kind: IRStmt::Drop { target: var },
            span,
            statement_id,
            scope_id,
        }
    }

    pub fn label(label: Label, span: Span, statement_id: usize, scope_id: usize) -> Self {
        Self {
            kind: IRStmt::Label(label),
            span,
            statement_id,
            scope_id,
        }
    }

    pub fn jump(target: Label, span: Span, statement_id: usize, scope_id: usize) -> Self {
        Self {
            kind: IRStmt::Jump { target },
            span,
            statement_id,
            scope_id,
        }
    }

    pub fn jump_if(
        target: Label,
        condition: IRVar,
        span: Span,
        statement_id: usize,
        scope_id: usize,
    ) -> Self {
        Self {
            kind: IRStmt::JumpIf { condition, target },
            span,
            statement_id,
            scope_id,
        }
    }

    pub fn jump_if_not(
        target: Label,
        condition: IRVar,
        span: Span,
        statement_id: usize,
        scope_id: usize,
    ) -> Self {
        Self {
            kind: IRStmt::JumpIfNot { condition, target },
            span,
            statement_id,
            scope_id,
        }
    }

    pub fn to_typed_ir_instruction(
        &self,
        ctx: &mut AnalysisContext,
    ) -> Result<TypedIRInstruction, CompileError> {
        Ok(TypedIRInstruction {
            kind: self
                .kind
                .try_to_typed_ir_stmt(ctx, self.statement_id, self.scope_id)?,
            span: self.span,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TypedIRInstruction {
    pub kind: TypedIRStmt,
    pub span: Span,
}

impl std::fmt::Display for TypedIRInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, Clone)]
pub struct FuncId {
    id: usize,
}

// Convert labels to ids too? Nah?
#[derive(Debug, Clone)]
pub enum TypedIRStmt {
    Assign {
        target: TypedIRVar,
        value: TypedIRExpr,
    },
    Drop {
        target: TypedIRVar,
    },
    Jump {
        target: Label,
    },
    JumpIf {
        condition: TypedIRVar,
        target: Label,
    },
    JumpIfNot {
        condition: TypedIRVar,
        target: Label,
    },
    Label(Label),
    Call {
        function_id: FuncId, // or later a FuncId
        args: Vec<TypedIRVar>,
        dest: TypedIRVar, // where to store return, optional
    },
    Return {
        value: Option<TypedIRVar>,
    },
}

impl std::fmt::Display for TypedIRStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypedIRStmt::Assign { target, value } => write!(f, "{} = {}", target, value),
            TypedIRStmt::Drop { target } => write!(f, "(x) {}", target),
            TypedIRStmt::Jump { target } => write!(f, "jump {}", target),
            TypedIRStmt::JumpIf { condition, target } => {
                write!(f, "jump {} if {}", target, condition)
            }
            TypedIRStmt::JumpIfNot { condition, target } => {
                write!(f, "jump {} if not {}", target, condition)
            }
            TypedIRStmt::Label(label) => write!(f, "{}", label),
            TypedIRStmt::Call {
                function_id,
                args,
                dest,
            } => todo!(),
            TypedIRStmt::Return { value } => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IRStmt {
    Assign {
        target: IRVar,
        value: IRExpr,
    },
    Drop {
        target: IRVar,
    },
    Jump {
        target: Label,
    },
    JumpIf {
        condition: IRVar,
        target: Label,
    },
    JumpIfNot {
        condition: IRVar,
        target: Label,
    },
    Label(Label),
    Call {
        function_name: String, // or later a FuncId
        args: Vec<IRVar>,
        dest: IRVar, // where to store return, optional
    },
    Return {
        value: Option<IRVar>,
    },
}

impl std::fmt::Display for IRStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRStmt::Assign { target, value } => write!(f, "{} = {}", target, value),
            IRStmt::Drop { target } => write!(f, "(x) {}", target),
            IRStmt::Jump { target } => write!(f, "jump {}", target),
            IRStmt::JumpIf { condition, target } => write!(f, "jump {} if {}", target, condition),
            IRStmt::JumpIfNot { condition, target } => {
                write!(f, "jump {} if not {}", target, condition)
            }
            IRStmt::Label(label) => write!(f, "{}", label),
            IRStmt::Call {
                function_name,
                args,
                dest,
            } => todo!(),
            IRStmt::Return { value } => todo!(),
        }
    }
}

impl IRStmt {
    pub fn try_to_typed_ir_stmt(
        &self,
        ctx: &mut AnalysisContext,
        statement_id: usize,
        scope_id: usize,
    ) -> Result<TypedIRStmt, CompileError> {
        match self {
            IRStmt::Assign { target, value } => Ok(TypedIRStmt::Assign {
                target: target.try_to_typed_ir_var(ctx)?,
                value: value.try_to_typed_ir_expr(
                    target,
                    target.span(),
                    ctx,
                    statement_id,
                    scope_id,
                )?,
            }),
            IRStmt::Drop { target } => Ok(TypedIRStmt::Drop {
                target: target.try_to_typed_ir_var(ctx)?,
            }),
            IRStmt::Jump { target } => Ok(TypedIRStmt::Jump {
                target: target.clone(),
            }),
            IRStmt::JumpIf { condition, target } => Ok(TypedIRStmt::JumpIf {
                condition: condition.try_to_typed_ir_var(ctx)?,
                target: target.clone(),
            }),
            IRStmt::JumpIfNot { condition, target } => Ok(TypedIRStmt::JumpIfNot {
                condition: condition.try_to_typed_ir_var(ctx)?,
                target: target.clone(),
            }),
            IRStmt::Label(label) => Ok(TypedIRStmt::Label(label.clone())),
            IRStmt::Call {
                function_name,
                args,
                dest,
            } => todo!(),
            IRStmt::Return { value } => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypedIRExpr {
    Var(TypedIRVar),
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(Box<str>),
    Binary {
        op: BinaryOp,
        left: TypedIRVar,
        right: TypedIRVar,
    },
    Unary {
        op: UnaryOp,
        expr: TypedIRVar,
    },
}

impl TypedIRExpr {
    pub fn read_vars(&self) -> Vec<usize> {
        match self {
            TypedIRExpr::Var(irvar) => vec![irvar.id()],
            TypedIRExpr::Binary { op: _, left, right } => vec![left.id(), right.id()],
            TypedIRExpr::Unary { op: _, expr } => vec![expr.id()],
            _ => Vec::new(),
        }
    }
}

impl std::fmt::Display for TypedIRExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypedIRExpr::Var(irvar) => write!(f, "{}", irvar),
            TypedIRExpr::Bool(v) => write!(f, "{}", v),
            TypedIRExpr::Int(v) => write!(f, "{}", v),
            TypedIRExpr::UInt(v) => write!(f, "{}", v),
            TypedIRExpr::Float(v) => write!(f, "{}", v),
            TypedIRExpr::String(v) => write!(f, "{}", v),
            TypedIRExpr::Binary { op, left, right } => write!(f, "{} {} {}", left, op, right),
            TypedIRExpr::Unary { op, expr } => write!(f, "{:?} {}", op, expr),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IRExpr {
    VagueValue(String),
    Var(IRVar),
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(Box<str>),
    Binary {
        op: BinaryOp,
        left: IRVar,
        right: IRVar,
    },
    Unary {
        op: UnaryOp,
        expr: IRVar,
    },
}

impl std::fmt::Display for IRExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRExpr::VagueValue(v) => write!(f, "{}", v),
            IRExpr::Var(irvar) => write!(f, "{}", irvar),
            IRExpr::Bool(v) => write!(f, "{}", v),
            IRExpr::UInt(v) => write!(f, "{}", v),
            IRExpr::Int(v) => write!(f, "{}", v),
            IRExpr::Float(v) => write!(f, "{}", v),
            IRExpr::String(v) => write!(f, "{}", v),
            IRExpr::Binary { op, left, right } => write!(f, "{} {} {}", left, op, right),
            IRExpr::Unary { op, expr } => write!(f, "{:?} {}", op, expr),
        }
    }
}

impl IRExpr {
    pub fn try_to_typed_ir_expr(
        &self,
        ir_var: &IRVar,
        span: Span,
        ctx: &mut AnalysisContext,
        statement_id: usize,
        scope_id: usize,
    ) -> Result<TypedIRExpr, CompileError> {
        // Folding
        // println!("{self}");
        // dbg!(statement_id);
        // dbg!(scope_id);
        // dbg!(ctx.get_repeat_entered_at(ir_var.id()));
        // dbg!(ctx.get_var_scope_id(ir_var.id()));

        let expression = match TypedConstVal::from_ir_expr(self, ctx, statement_id, scope_id)? {
            Some(const_val) => {
                ctx.var_const_map()
                    .set(ir_var.id(), Some(const_val.clone()));
                const_val.to_ir_expr()
            }
            _ => self.clone(),
        };

        match expression {
            IRExpr::VagueValue(var) => Err(CompileError::untyped_variable(var, span)),
            IRExpr::Var(irvar) => Ok(TypedIRExpr::Var(irvar.try_to_typed_ir_var(ctx)?)),
            IRExpr::Bool(val) => Ok(TypedIRExpr::Bool(val)),
            IRExpr::Int(val) => Ok(TypedIRExpr::Int(val)),
            IRExpr::UInt(val) => Ok(TypedIRExpr::UInt(val)),
            IRExpr::Float(val) => Ok(TypedIRExpr::Float(val)),
            IRExpr::String(val) => Ok(TypedIRExpr::String(val.clone())),
            IRExpr::Binary { op, left, right } => {
                let left = left.try_to_typed_ir_var(ctx)?;
                let right = right.try_to_typed_ir_var(ctx)?;

                Ok(TypedIRExpr::Binary { op, left, right })
            }
            IRExpr::Unary { op, expr } => Ok(TypedIRExpr::Unary {
                op,
                expr: expr.try_to_typed_ir_var(ctx)?,
            }),
        }
    }

    pub fn is_comparison(&self) -> bool {
        match self {
            IRExpr::Binary {
                op,
                left: _,
                right: _,
            } => op.is_comparison(),
            _ => false,
        }
    }

    pub fn typ(&self, span: Span) -> Result<IntermediateType, CompileError> {
        match self {
            IRExpr::VagueValue(_) => Ok(IntermediateType::Indeterminate),
            IRExpr::Var(irvar) => Ok(irvar.typ().clone()),
            IRExpr::Bool(_) => Ok(IntermediateType::Boolean),
            IRExpr::Int(_) => Ok(IntermediateType::Int),
            IRExpr::UInt(_) => Ok(IntermediateType::UInt),
            IRExpr::Float(_) => Ok(IntermediateType::Float),
            IRExpr::String(_) => Ok(IntermediateType::String),
            IRExpr::Binary { op: _, left, right } => {
                left.typ().try_apply_type_hint(right.typ(), span)
            }
            IRExpr::Unary { op: _, expr } => Ok(expr.typ().clone()),
        }
    }

    pub fn read_vars(&self) -> Vec<usize> {
        match self {
            IRExpr::Var(irvar) => vec![irvar.id()],
            IRExpr::Binary { op: _, left, right } => vec![left.id(), right.id()],
            IRExpr::Unary { op: _, expr } => vec![expr.id()],
            _ => Vec::new(),
        }
    }
}

macro_rules! impl_numeric_bin_ops {
    ($op:expr, $a:expr, $b:expr, $type_variant:ident, $wrap:ident) => {
        match $op {
            BinaryOp::Add => Ok(Some(TypedConstVal::$wrap($a + $b))),
            BinaryOp::Subtract => Ok(Some(TypedConstVal::$wrap($a - $b))),
            BinaryOp::Multiply => Ok(Some(TypedConstVal::$wrap($a * $b))),
            BinaryOp::Divide => Ok(Some(TypedConstVal::$wrap($a / $b))),
            BinaryOp::Modulo => Ok(Some(TypedConstVal::$wrap($a % $b))),
            BinaryOp::Equal => Ok(Some(TypedConstVal::Bool($a == $b))),
            BinaryOp::NotEqual => Ok(Some(TypedConstVal::Bool($a != $b))),
            BinaryOp::LessThan => Ok(Some(TypedConstVal::Bool($a < $b))),
            BinaryOp::GreaterThan => Ok(Some(TypedConstVal::Bool($a > $b))),
            BinaryOp::LessThanOrEqual => Ok(Some(TypedConstVal::Bool($a <= $b))),
            BinaryOp::GreaterThanOrEqual => Ok(Some(TypedConstVal::Bool($a >= $b))),
            _ => Err(CompileError::internal_compiler_error(concat!(
                "Invalid binary op for ",
                stringify!($type_variant)
            ))),
        }
    };
}

pub fn eval_binary_op(
    op: BinaryOp,
    lhs: &TypedConstVal,
    rhs: &TypedConstVal,
) -> Result<Option<TypedConstVal>, CompileError> {
    use TypedConstVal::*;
    match (lhs, rhs) {
        (Int(a), Int(b)) => {
            let (a, b) = (*a, *b);
            impl_numeric_bin_ops!(op, a, b, Int, Int)
        }
        (Float(a), Float(b)) => {
            let (a, b) = (*a, *b);
            impl_numeric_bin_ops!(op, a, b, Float, Float)
        }
        (Bool(a), Bool(b)) => match op {
            BinaryOp::LogicalAnd => Ok(Some(Bool(*a && *b))),
            BinaryOp::LogicalOr => Ok(Some(Bool(*a || *b))),
            BinaryOp::Equal => Ok(Some(Bool(*a == *b))),
            BinaryOp::NotEqual => Ok(Some(Bool(*a != *b))),
            _ => Err(CompileError::internal_compiler_error(
                "Invalid binary op for Bool",
            )),
        },
        (String(a), String(b)) => match op {
            BinaryOp::Add => {
                let mut s = a.to_string();
                s.push_str(b);
                Ok(Some(String(s.into())))
            }
            BinaryOp::Equal => Ok(Some(Bool(*a == *b))),
            BinaryOp::NotEqual => Ok(Some(Bool(*a != *b))),
            _ => Err(CompileError::internal_compiler_error(
                "Invalid binary op for String",
            )),
        },
        _ => Err(CompileError::internal_compiler_error(
            "Type mismatch in binary operation",
        )),
    }
}

macro_rules! impl_numeric_unary_ops {
    ($op:expr, $val:expr, $type_variant:ident, $wrap:ident) => {
        match $op {
            UnaryOp::Neg => Ok(Some(TypedConstVal::$wrap(-$val))),
            _ => Err(CompileError::internal_compiler_error(concat!(
                "Invalid unary op for ",
                stringify!($type_variant)
            ))),
        }
    };
}

pub fn eval_unary_op(
    op: UnaryOp,
    val: &TypedConstVal,
) -> Result<Option<TypedConstVal>, CompileError> {
    use TypedConstVal::*;
    match val {
        Int(v) => {
            let val = *v;
            impl_numeric_unary_ops!(op, val, Int, Int)
        }
        Float(v) => {
            let val = *v;
            impl_numeric_unary_ops!(op, val, Float, Float)
        }
        Bool(v) => match op {
            UnaryOp::Not => Ok(Some(Bool(!v))),
            _ => Err(CompileError::internal_compiler_error(
                "Invalid unary op for Bool",
            )),
        },
        _ => Err(CompileError::internal_compiler_error(
            "Invalid unary op for type",
        )),
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TypedConstVal {
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(Box<str>),
    // TODO expand to arrays, objects and more
}

impl TypedConstVal {
    pub fn to_ir_expr(&self) -> IRExpr {
        match self {
            TypedConstVal::Bool(val) => IRExpr::Bool(*val),
            TypedConstVal::Int(val) => IRExpr::Int(*val),
            TypedConstVal::UInt(val) => IRExpr::UInt(*val),
            TypedConstVal::Float(val) => IRExpr::Float(*val),
            TypedConstVal::String(val) => IRExpr::String(val.clone()),
        }
    }

    pub fn from_ir_expr(
        irxpr: &IRExpr,
        ctx: &mut AnalysisContext,
        statement_id: usize,
        scope_id: usize,
    ) -> Result<Option<Self>, CompileError> {
        match irxpr {
            IRExpr::VagueValue(_) => Err(CompileError::internal_compiler_error(
                "Derp, should not be vague by now",
            )),
            IRExpr::Var(irvar) => {
                if ctx.can_fold_var(irvar.id(), statement_id) {
                    Ok(ctx.var_const_map().get(irvar.id()))
                } else {
                    Ok(None)
                }
            }
            IRExpr::Bool(val) => Ok(Some(TypedConstVal::Bool(*val))),
            IRExpr::Int(val) => Ok(Some(TypedConstVal::Int(*val))),
            IRExpr::UInt(val) => Ok(Some(TypedConstVal::UInt(*val))),
            IRExpr::Float(val) => Ok(Some(TypedConstVal::Float(*val))),
            IRExpr::String(val) => Ok(Some(TypedConstVal::String(val.clone()))),
            IRExpr::Binary { op, left, right } => {
                let lhs = match ctx.var_const_map().get(left.id()) {
                    Some(v) if ctx.can_fold_var(left.id(), statement_id) => v,
                    _ => return Ok(None),
                };
                let rhs = match ctx.var_const_map().get(right.id()) {
                    Some(v) if ctx.can_fold_var(right.id(), statement_id) => v,
                    _ => return Ok(None),
                };
                let result = eval_binary_op(*op, &lhs, &rhs)?;
                Ok(result)
            }
            IRExpr::Unary { op, expr } => {
                let val = match ctx.var_const_map().get(expr.id()) {
                    Some(v) if ctx.can_fold_var(expr.id(), statement_id) => v,
                    _ => return Ok(None),
                };
                eval_unary_op(*op, &val)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TypedIRVar {
    pub scope_id: usize,
    pub name: Option<String>, // None if temp
    pub id: usize,
    pub span: Span,
    pub typ: ProcessedType,
}

impl std::fmt::Display for TypedIRVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ":{}{}:{}",
            self.id,
            self.name
                .as_ref()
                .map(|v| format!("({})", v))
                .unwrap_or_default(),
            self.typ
        )
    }
}

impl TypedIRVar {
    pub fn is_temp(&self) -> bool {
        self.name.is_none()
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum IRVar {
    Var { metadata: VariableMetadata },
    Temp { metadata: TempVariableMetadata },
}

impl IRVar {
    pub fn try_to_typed_ir_var(&self, ctx: &AnalysisContext) -> Result<TypedIRVar, CompileError> {
        Ok(match self {
            IRVar::Var { metadata } => TypedIRVar {
                scope_id: metadata.scope_id,
                name: Some(metadata.name.clone()),
                id: metadata.id,
                span: metadata.span,
                typ: ctx.get_type(metadata.id).try_to_processed(metadata.span)?,
            },
            IRVar::Temp { metadata } => TypedIRVar {
                scope_id: metadata.scope_id,
                name: None,
                id: metadata.id,
                span: metadata.span,
                typ: ctx.get_type(metadata.id).try_to_processed(metadata.span)?,
            },
        })
    }

    pub fn span(&self) -> Span {
        match self {
            IRVar::Var { metadata } => metadata.span,
            IRVar::Temp { metadata } => metadata.span,
        }
    }

    pub fn typ(&self) -> &IntermediateType {
        match self {
            IRVar::Var { metadata } => &metadata.typ,
            IRVar::Temp { metadata } => &metadata.typ,
        }
    }

    pub fn id(&self) -> usize {
        match self {
            IRVar::Var { metadata } => metadata.id,
            IRVar::Temp { metadata } => metadata.id,
        }
    }

    pub fn set_type(&mut self, new_type: IntermediateType) {
        match self {
            IRVar::Var { metadata } => metadata.typ = new_type,
            IRVar::Temp { metadata } => metadata.typ = new_type,
        }
    }

    pub fn resolved_typ(&self, ctx: &AnalysisContext) -> IntermediateType {
        ctx.get_type(self.id())
    }

    pub fn is_temp(&self) -> bool {
        matches!(self, IRVar::Temp { .. })
    }
}

impl std::fmt::Display for IRVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRVar::Var { metadata } => write!(f, ":{}:{}", metadata.name, metadata.typ),
            IRVar::Temp { metadata } => write!(f, "::temp{}:{}", metadata.id, metadata.typ),
        }
    }
}

// TODO property of a sort of usage score, based on number of usages, existing in loops, especially ones with few statements, etc. To use during register allocation, prioritize what to push out
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub id: usize,
    pub declared_span: Span,
    pub declared_at: usize, // statement id
    pub last_used: Option<usize>,
    pub last_mutated: Option<usize>,
    // pub typ: IntermediateType,
    pub moved: bool,
    pub scope_id: usize,
    pub is_init_scope: bool,
    pub shadowed: bool,
}

impl VariableInfo {
    pub fn last_access(&self) -> usize {
        match (self.last_used, self.last_mutated) {
            (None, None) => 0,
            (None, Some(mutated)) => mutated,
            (Some(used), None) => used,
            (Some(used), Some(mutated)) => std::cmp::max(used, mutated),
        }
    }

    pub fn metadata(&self, typ: IntermediateType, span: Span) -> VariableMetadata {
        VariableMetadata {
            name: self.name.clone(),
            span,
            scope_id: self.scope_id,
            id: self.id,
            declared_at: self.declared_at,
            typ,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VariableMetadata {
    pub name: String,
    pub scope_id: usize,
    pub span: Span,
    pub id: usize,
    pub declared_at: usize,
    pub typ: IntermediateType,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TempVariableMetadata {
    pub scope_id: usize,
    pub span: Span,
    pub id: usize,
    pub declared_at: usize,
    pub typ: IntermediateType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarConstValMap {
    map: BTreeMap<usize, Option<TypedConstVal>>,
}

impl VarConstValMap {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn get(&self, id: usize) -> Option<TypedConstVal> {
        self.map.get(&id).cloned().flatten()
    }

    pub fn set(&mut self, id: usize, val: Option<TypedConstVal>) {
        self.map.insert(id, val);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarTypeMap {
    map: BTreeMap<usize, VarTypeMapData>,
}

impl VarTypeMap {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn get_type(&self, id: usize) -> IntermediateType {
        self.map
            .get(&id)
            .cloned()
            .map(|v| v.typ)
            .unwrap_or(IntermediateType::Indeterminate)
    }

    pub fn get_repeat_entered_at(&self, id: usize) -> Option<usize> {
        self.map.get(&id).cloned().and_then(|v| v.repeat_entered_at)
    }

    pub fn get_repeat_write_at(&self, id: usize) -> Option<usize> {
        self.map.get(&id).cloned().and_then(|v| v.repeat_write_at)
    }

    pub fn set_type(
        &mut self,
        id: usize,
        typ: IntermediateType,
        span: Span,
        ctx: &mut AnalysisContext,
    ) {
        let initial_typ = self.map.get_mut(&id);
        if let Some(var) = initial_typ {
            match var.typ.try_apply_type_hint(&typ, span) {
                Ok(new_typ) => {
                    var.typ = new_typ;
                }
                Err(error) => {
                    ctx.errors.push_error(error);
                }
            }
        } else {
            let data = VarTypeMapData {
                typ,
                last_accessed: None,
                repeat_write_at: None,
                repeat_entered_at: None,
                scope_id: 0,
            };
            self.map.insert(id, data);
        }
    }

    pub fn set_last_accessed(&mut self, id: usize, accessed: usize) {
        let initial_typ = self.map.get_mut(&id);
        if let Some(var) = initial_typ {
            var.last_accessed = Some(accessed);
        } else {
            let data = VarTypeMapData {
                typ: IntermediateType::Indeterminate,
                last_accessed: Some(accessed),
                repeat_write_at: None,
                repeat_entered_at: None,
                scope_id: 0,
            };
            self.map.insert(id, data);
        }
    }

    pub fn set_last_repeat_entered_at(&mut self, id: usize, entered: usize) {
        let initial_typ = self.map.get_mut(&id);
        if let Some(var) = initial_typ {
            var.repeat_entered_at = Some(entered);
        } else {
            let data = VarTypeMapData {
                typ: IntermediateType::Indeterminate,
                last_accessed: Some(entered),
                repeat_write_at: None,
                repeat_entered_at: Some(entered),
                scope_id: 0,
            };
            self.map.insert(id, data);
        }
    }

    pub fn set_last_repeat_write_at(&mut self, id: usize, written: usize) {
        let initial_typ = self.map.get_mut(&id);
        if let Some(var) = initial_typ {
            var.repeat_write_at = Some(written);
        } else {
            let data = VarTypeMapData {
                typ: IntermediateType::Indeterminate,
                last_accessed: Some(written),
                repeat_write_at: Some(written),
                repeat_entered_at: None,
                scope_id: 0,
            };
            self.map.insert(id, data);
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VarTypeMapData {
    pub typ: IntermediateType,
    pub scope_id: usize,
    pub last_accessed: Option<usize>,
    pub repeat_entered_at: Option<usize>,
    pub repeat_write_at: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct ScopeTree {
    pub scopes: Vec<Scope>,
    pub root_index: usize,
}

impl ScopeTree {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
            root_index: 0,
        }
    }

    // Checks if the var name exists in the
    // Return the scope where it was declared or none
    pub fn var_access_from_scope(&self, vname: &str, scope_idx: usize) -> Option<usize> {
        if let Some(scope) = self.scopes.get(scope_idx) {
            if scope.vars.contains_key(vname) {
                Some(scope_idx)
            } else if let Some(parent_index) = scope.parent_index {
                self.var_access_from_scope(vname, parent_index)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub id: usize,
    pub parent_index: Option<usize>,
    pub vars: HashMap<String, VariableInfo>,
    pub children: Vec<usize>,
    pub repeated: bool,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            id: 0,
            parent_index: None,
            vars: HashMap::new(),
            children: Vec::new(),
            repeated: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UseKind {
    Declare,
    Init,
    Read,
    Write,
    Shadowed,
}

#[derive(Debug, Clone, Default)]
pub struct UseMap {
    pub uses: HashMap<String, Vec<BTreeSet<UseSite>>>,
}

impl UseMap {
    pub fn new() -> Self {
        Self {
            uses: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseSite {
    pub span: Span,
    pub statement_id: usize,
    pub kind: UseKind, // Read / Write / Shadow
}

impl PartialOrd for UseSite {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.span.line, self.span.column).cmp(&(other.span.line, other.span.column)))
    }
}

impl Ord for UseSite {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.span.line, self.span.column).cmp(&(other.span.line, other.span.column))
    }
}

fn var_not_found_err(name: String, span: Span, use_kind: UseKind) -> CompileError {
    if UseKind::Write == use_kind {
        CompileError::assign_undefined_variable(name, span)
    } else {
        CompileError::read_undefined_variable(name, span)
    }
}

impl UseMap {
    pub fn record_use(
        &mut self,
        name: String,
        statement_id: usize,
        span: Span,
        kind: UseKind,
    ) -> Result<(), CompileError> {
        let entry: &mut Vec<BTreeSet<UseSite>> = self.uses.entry(name.clone()).or_default();
        let use_site = UseSite {
            kind,
            statement_id,
            span,
        };
        if UseKind::Declare == kind || UseKind::Init == kind {
            entry.push(BTreeSet::from([use_site]));
        } else {
            entry
                .last_mut()
                .ok_or(var_not_found_err(name, span, kind))?
                .insert(use_site);
        }

        Ok(())
    }

    pub fn merge(&mut self, other: UseMap) {
        for (name, sites) in other.uses {
            self.uses.entry(name).or_default().extend(sites);
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisContext {
    scopes: ScopeTree,
    current_scope: usize,
    stmt_counter: usize,
    var_counter: usize,
    var_type_map: VarTypeMap,
    var_const_map: VarConstValMap,
    errors: CompileErrorList,
    ir_output: Vec<IRInstruction>,
    typed_ir_output: Vec<TypedIRInstruction>,
    label_allocator: LabelAllocator,
    loop_break_labels: Vec<Label>,
    loop_continue_labels: Vec<Label>,
    var_reads: HashSet<usize>,
    var_assigns: HashMap<usize, usize>,
}

impl AnalysisContext {
    pub fn new() -> Self {
        Self {
            scopes: ScopeTree::new(),
            current_scope: 0,
            stmt_counter: 0,
            var_counter: 0,
            var_type_map: VarTypeMap::new(),
            var_const_map: VarConstValMap::new(),
            errors: CompileErrorList::new(),
            ir_output: Vec::new(),
            typed_ir_output: Vec::new(),
            label_allocator: LabelAllocator::new(),
            loop_break_labels: Vec::new(),
            loop_continue_labels: Vec::new(),
            var_reads: HashSet::new(),
            var_assigns: HashMap::new(),
        }
    }

    pub fn ir(&self) -> &[IRInstruction] {
        &self.ir_output
    }

    pub fn errors(&self) -> &CompileErrorList {
        &self.errors
    }

    pub fn errors_mut(&mut self) -> &mut CompileErrorList {
        &mut self.errors
    }

    pub fn typed_ir(&self) -> &[TypedIRInstruction] {
        &self.typed_ir_output
    }

    pub fn var_type_map(&self) -> &VarTypeMap {
        &self.var_type_map
    }

    pub fn var_const_map(&mut self) -> &mut VarConstValMap {
        &mut self.var_const_map
    }

    pub fn var_count(&self) -> usize {
        self.var_counter
    }

    pub fn can_fold_var(&self, id: usize, statement_id: usize) -> bool {
        let repeat_entered_at = self.get_repeat_entered_at(id);

        dbg!(repeat_entered_at, statement_id);

        if let Some(repeat_entered_at) = repeat_entered_at {
            statement_id < repeat_entered_at
        } else {
            true
        }
    }

    pub fn get_type(&self, id: usize) -> IntermediateType {
        self.var_type_map
            .map
            .get(&id)
            .cloned()
            .map(|v| v.typ)
            .unwrap_or(IntermediateType::Indeterminate)
    }

    pub fn set_type(&mut self, id: usize, typ: IntermediateType, span: Span) {
        let initial_typ = self.var_type_map.map.get_mut(&id);
        if let Some(var) = initial_typ {
            match var.typ.try_apply_type_hint(&typ, span) {
                Ok(new_typ) => {
                    var.typ = new_typ;
                }
                Err(error) => {
                    self.errors.push_error(error);
                }
            }
        } else {
            let data = VarTypeMapData {
                typ,
                last_accessed: None,
                repeat_entered_at: None,
                repeat_write_at: None,
                scope_id: 0,
            };
            self.var_type_map.map.insert(id, data);
        }
    }

    pub fn set_last_accessed(&mut self, id: usize, accessed: usize) {
        let initial_typ = self.var_type_map.map.get_mut(&id);
        if let Some(var) = initial_typ {
            var.last_accessed = Some(accessed);
        } else {
            let data = VarTypeMapData {
                typ: IntermediateType::Indeterminate,
                last_accessed: Some(accessed),
                repeat_entered_at: None,
                repeat_write_at: None,
                scope_id: 0,
            };
            self.var_type_map.map.insert(id, data);
        }
    }

    pub fn get_var_scope_id(&self, id: usize) -> Option<usize> {
        self.var_type_map.map.get(&id).cloned().map(|v| v.scope_id)
    }

    pub fn get_repeat_entered_at(&self, id: usize) -> Option<usize> {
        self.var_type_map
            .map
            .get(&id)
            .cloned()
            .and_then(|v| v.repeat_entered_at)
    }

    pub fn get_repeat_write_at(&self, id: usize) -> Option<usize> {
        self.var_type_map
            .map
            .get(&id)
            .cloned()
            .and_then(|v| v.repeat_write_at)
    }

    pub fn set_repeat_entered_at(&mut self, id: usize, entered: usize) {
        let initial_typ = self.var_type_map.map.get_mut(&id);
        if let Some(var) = initial_typ {
            var.repeat_entered_at = Some(entered);
        } else {
            let data = VarTypeMapData {
                typ: IntermediateType::Indeterminate,
                last_accessed: None,
                repeat_write_at: None,
                repeat_entered_at: Some(entered),
                scope_id: 0,
            };
            self.var_type_map.map.insert(id, data);
        }
    }

    pub fn set_repeat_write_at(&mut self, id: usize, written: usize) {
        let initial_typ = self.var_type_map.map.get_mut(&id);
        if let Some(var) = initial_typ {
            var.repeat_write_at = Some(written);
        } else {
            let data = VarTypeMapData {
                typ: IntermediateType::Indeterminate,
                last_accessed: Some(written),
                repeat_write_at: Some(written),
                repeat_entered_at: None,
                scope_id: 0,
            };
            self.var_type_map.map.insert(id, data);
        }
    }

    pub fn set_scope_id(&mut self, id: usize, scope_id: usize) {
        let initial_typ = self.var_type_map.map.get_mut(&id);
        if let Some(var) = initial_typ {
            var.scope_id = scope_id;
        } else {
            let data = VarTypeMapData {
                typ: IntermediateType::Indeterminate,
                last_accessed: None,
                repeat_write_at: None,
                repeat_entered_at: None,
                scope_id,
            };
            self.var_type_map.map.insert(id, data);
        }
    }
}

fn sync_irvar_type(ir_var: &mut IRVar, ctx: &AnalysisContext) {
    let id = ir_var.id();
    let correct_type = ctx.get_type(id);

    if ir_var.typ() != &correct_type {
        ir_var.set_type(correct_type);
    }
}

macro_rules! unwrap_or_push {
    ($result:expr, $ctx:expr, $span:expr) => {
        match $result {
            Ok(val) => val,
            Err(err) => {
                $ctx.errors.push_error(err);
                continue;
            }
        }
    };
}

fn resolve_types(ctx: &mut AnalysisContext) {
    let mut new_ir_output = Vec::with_capacity(ctx.ir_output.len());

    for instr in ctx.ir_output.clone().iter() {
        let mut updated_instr = instr.clone();

        match &mut updated_instr.kind {
            IRStmt::Assign { target, value } => {
                // TODO handle unwraps
                let assign_type: IntermediateType =
                    unwrap_or_push!(value.typ(target.span()), ctx, target.span());
                let init_type: IntermediateType = ctx.get_type(target.id());
                let new_type: IntermediateType = if value.is_comparison() {
                    assign_type.clone()
                } else {
                    unwrap_or_push!(
                        init_type.try_apply_type_hint(&assign_type, target.span()),
                        ctx,
                        target.span()
                    )
                };
                if value.is_comparison() {
                    ctx.set_type(target.id(), IntermediateType::Boolean, target.span());
                } else {
                    ctx.set_type(target.id(), new_type, target.span());
                }
                sync_irvar_type(target, ctx);

                if let IRExpr::Var(var) = value {
                    sync_irvar_type(var, ctx);
                } else if let IRExpr::Binary { left, right, .. } = value {
                    sync_irvar_type(left, ctx);
                    sync_irvar_type(right, ctx);
                } else if let IRExpr::Unary { expr, .. } = value {
                    sync_irvar_type(expr, ctx);
                }

                if matches!(target.typ(), IntermediateType::Indeterminate) {
                    ctx.errors.push_error(CompileError::untyped_variable(
                        format!("{target}"),
                        instr.span,
                    ));
                }
            }

            IRStmt::Drop { target } => {
                sync_irvar_type(target, ctx);
            }

            IRStmt::JumpIf { condition, .. } | IRStmt::JumpIfNot { condition, .. } => {
                sync_irvar_type(condition, ctx);

                if condition.typ() != &IntermediateType::Boolean {
                    ctx.errors.push_error(CompileError::type_mismatch(
                        "Bool".to_string(),
                        format!("{:?}", condition.typ()),
                        instr.span,
                    ));
                }
            }

            IRStmt::Call { args, dest, .. } => {
                for arg in args {
                    sync_irvar_type(arg, ctx);
                }
                sync_irvar_type(dest, ctx);
            }

            IRStmt::Return { value } => {
                if let Some(var) = value {
                    sync_irvar_type(var, ctx);
                }
            }

            IRStmt::Label(_) | IRStmt::Jump { .. } => {}
        }

        new_ir_output.push(updated_instr);
    }

    ctx.ir_output = new_ir_output;

    let typed_ir = match ctx
        .ir_output
        .clone()
        .iter()
        .map(|v: &IRInstruction| v.to_typed_ir_instruction(ctx))
        .collect::<Result<Vec<_>, CompileError>>()
    {
        Ok(v) => v,
        Err(error) => {
            ctx.errors.push_error(error);
            Vec::new()
        }
    };

    let dce_ir = perform_dead_code_elimination(&typed_ir);

    ctx.typed_ir_output = dce_ir;
}

fn perform_dead_code_elimination(typed_ir: &[TypedIRInstruction]) -> Vec<TypedIRInstruction> {
    let mut live_vars = HashSet::new();
    let mut result = Vec::with_capacity(typed_ir.len());

    for instr in typed_ir.iter().rev() {
        let keep = match &instr.kind {
            TypedIRStmt::Assign { target, value } => {
                let var_id = target.id();

                if live_vars.contains(&var_id) || !target.is_temp() {
                    // Mark inputs as live
                    live_vars.extend(value.read_vars());
                    true
                } else {
                    // Temp was never used
                    false
                }
            }
            TypedIRStmt::JumpIf {
                condition: target, ..
            }
            | TypedIRStmt::JumpIfNot {
                condition: target, ..
            } => {
                live_vars.insert(target.id());
                true
            }
            TypedIRStmt::Drop { target } => {
                if target.is_temp() && !live_vars.contains(&target.id()) {
                    false // Dead drop of dead temp
                } else {
                    true
                }
            }
            TypedIRStmt::Call { args, dest, .. } => {
                for arg in args {
                    live_vars.insert(arg.id());
                }
                live_vars.insert(dest.id()); // assume return is used
                true
            }
            TypedIRStmt::Return { value: Some(var) } => {
                live_vars.insert(var.id());
                true
            }
            _ => true, // Jump, Label, Return(None), etc.
        };

        if keep {
            result.push(instr.clone());
        }
    }

    result.reverse(); // We built it backwards
    result
}

pub fn analyze_ast(ast: &[ASTNodeContainer]) -> AnalysisContext {
    let mut ctx = AnalysisContext::new();

    create_scopes_map_ast(ast, &mut ctx, None);

    resolve_types(&mut ctx);

    ctx
}

fn fresh_temp_var(
    expr: &Expr,
    ctx: &mut AnalysisContext,
    hint: Option<&IntermediateType>,
) -> (IRVar, usize) {
    let id = ctx.var_counter;
    ctx.var_counter += 1;

    let typ = match IntermediateType::type_from_expr(expr) {
        Ok(t) => {
            if let Some(h) = hint {
                match h.try_apply_type_hint(&t, expr.span) {
                    Ok(new_t) => new_t,
                    Err(e) => {
                        ctx.errors.push_error(e.clone());
                        t
                    }
                }
            } else {
                t
            }
        }
        Err(error) => {
            ctx.errors.push_error(error);
            IntermediateType::Indeterminate
        }
    };

    let metadata = TempVariableMetadata {
        declared_at: ctx.stmt_counter,
        span: expr.span,
        id,
        scope_id: ctx.current_scope,
        typ,
    };

    ctx.set_type(metadata.id, metadata.typ.clone(), expr.span);

    (IRVar::Temp { metadata }, id)
}

pub fn flatten_expr_to_var(
    expr: &Expr,
    ctx: &mut AnalysisContext,
    target: IRVar,
    hint: Option<&IntermediateType>,
    statement_id: usize,
) -> Result<(IRVar, Vec<IRInstruction>), CompileError> {
    match &expr.kind {
        ExprKind::Bool(bool) => {
            let instr = IRInstruction {
                span: expr.span,
                kind: IRStmt::Assign {
                    target: target.clone(),
                    value: IRExpr::Bool(*bool),
                },
                statement_id,
                scope_id: ctx.current_scope,
            };

            Ok((target, vec![instr]))
        }
        ExprKind::Number(num) => {
            let parsed_expr = match hint {
                Some(IntermediateType::Int) if expr.is_numeric_literal() => {
                    num.parse::<i64>().ok().map(IRExpr::Int)
                }
                Some(IntermediateType::UInt) if expr.is_numeric_literal() => {
                    num.parse::<u64>().ok().map(IRExpr::UInt)
                }
                Some(IntermediateType::Float) if expr.is_numeric_literal() => {
                    num.parse::<f64>().ok().map(IRExpr::Float)
                }
                _ => {
                    // try all
                    if let Ok(i) = num.parse::<i64>() {
                        Some(IRExpr::Int(i))
                    } else if let Ok(i) = num.parse::<u64>() {
                        Some(IRExpr::UInt(i))
                    } else if let Ok(f) = num.parse::<f64>() {
                        Some(IRExpr::Float(f))
                    } else {
                        None
                    }
                }
            };

            let value =
                parsed_expr.ok_or_else(|| CompileError::invalid_numeric(num.clone(), expr.span))?;

            let instr = IRInstruction {
                span: expr.span,
                kind: IRStmt::Assign {
                    target: target.clone(),
                    value,
                },
                statement_id,
                scope_id: ctx.current_scope,
            };

            Ok((target, vec![instr]))
        }
        ExprKind::Float(num) => {
            let instr = IRInstruction {
                span: expr.span,
                kind: IRStmt::Assign {
                    target: target.clone(),
                    value: IRExpr::Float(*num),
                },
                statement_id,
                scope_id: ctx.current_scope,
            };

            Ok((target, vec![instr]))
        }
        ExprKind::Int(num) => {
            let instr = IRInstruction {
                span: expr.span,
                kind: IRStmt::Assign {
                    target: target.clone(),
                    value: IRExpr::Int(*num),
                },
                statement_id,
                scope_id: ctx.current_scope,
            };

            Ok((target, vec![instr]))
        }
        ExprKind::UInt(num) => {
            let instr = IRInstruction {
                span: expr.span,
                kind: IRStmt::Assign {
                    target: target.clone(),
                    value: IRExpr::UInt(*num),
                },
                statement_id,
                scope_id: ctx.current_scope,
            };

            Ok((target, vec![instr]))
        }
        ExprKind::String(string) => {
            let instr = IRInstruction {
                span: expr.span,
                kind: IRStmt::Assign {
                    target: target.clone(),
                    value: IRExpr::String(string.clone().into_boxed_str()),
                },
                statement_id,
                scope_id: ctx.current_scope,
            };

            Ok((target, vec![instr]))
        }
        ExprKind::Variable(varname) => {
            let scope_idx = if let Some(idx) = ctx
                .scopes
                .var_access_from_scope(varname.name(), ctx.current_scope)
            {
                idx
            } else {
                return Err(CompileError::read_undefined_variable(
                    varname.name(),
                    varname.span(),
                ));
            };

            let vinfo = &ctx.scopes.scopes[scope_idx].vars[varname.name()];
            let typ = ctx.get_type(vinfo.id);
            let var = IRVar::Var {
                metadata: vinfo.metadata(typ, varname.span()),
            };
            Ok((var, vec![]))
        }
        ExprKind::UnaryOp { op, expr: inner } => {
            let (operand_var, mut inner_instrs) =
                flatten_expr_to_var(inner, ctx, target.clone(), hint, statement_id)?;

            let ir_expr = IRExpr::Unary {
                op: *op,
                expr: operand_var,
            };

            let assign_instr = IRInstruction {
                span: expr.span,
                kind: IRStmt::Assign {
                    target: target.clone(),
                    value: ir_expr,
                },
                statement_id,
                scope_id: ctx.current_scope,
            };

            inner_instrs.push(assign_instr);
            Ok((target, inner_instrs))
        }
        ExprKind::BinaryOp { left, op, right } => {
            let type_hint_left =
                IntermediateType::type_from_expr(left).unwrap_or(IntermediateType::Indeterminate);
            let type_hint_right =
                IntermediateType::type_from_expr(right).unwrap_or(IntermediateType::Indeterminate);
            let common_hint = type_hint_left
                .try_apply_type_hint(&type_hint_right, expr.span)
                .or_else(|_| type_hint_right.try_apply_type_hint(&type_hint_left, expr.span))
                .unwrap_or(IntermediateType::Indeterminate);
            let (temp_left, _temp_left_id) = fresh_temp_var(left, ctx, Some(&common_hint));
            let (temp_right, _temp_right_id) = fresh_temp_var(right, ctx, Some(&common_hint));
            ctx.set_scope_id(temp_left.id(), ctx.current_scope);
            ctx.set_scope_id(temp_right.id(), ctx.current_scope);
            let (left_var, mut left_instrs) =
                flatten_expr_to_var(left, ctx, temp_left.clone(), hint, statement_id)?;
            let (right_var, right_instrs) =
                flatten_expr_to_var(right, ctx, temp_right.clone(), hint, statement_id)?;

            let ir_expr = IRExpr::Binary {
                op: *op,
                left: left_var.clone(),
                right: right_var.clone(),
            };

            let assign_instr = IRInstruction {
                span: expr.span,
                kind: IRStmt::Assign {
                    target: target.clone(),
                    value: ir_expr,
                },
                statement_id,
                scope_id: ctx.current_scope,
            };

            left_instrs.extend(right_instrs);
            left_instrs.push(assign_instr);
            if left_var.is_temp() {
                let drop_temp_left =
                    IRInstruction::drop(left_var, expr.span, statement_id, ctx.current_scope);
                left_instrs.push(drop_temp_left);
            }
            if right_var.is_temp() {
                let drop_temp_right =
                    IRInstruction::drop(right_var, expr.span, statement_id, ctx.current_scope);
                left_instrs.push(drop_temp_right);
            }
            Ok((target, left_instrs))
        }
        ExprKind::Invalid => Ok((target, vec![])),
    }
}

fn emit_assign_var(
    var_data: VariableMetadata,
    expr: &Expr,
    ctx: &mut AnalysisContext,
    span: Span,
    statement_id: usize,
) -> Vec<IRInstruction> {
    let target_var = IRVar::Var { metadata: var_data };
    let (result_var, mut instrs) = match flatten_expr_to_var(
        expr,
        ctx,
        target_var.clone(),
        Some(target_var.typ()),
        statement_id,
    ) {
        Ok(res) => res,
        Err(err) => {
            ctx.errors.push_error(err);
            return vec![];
        }
    };

    // Only add explicit assignment if flatten_expr didn't already do it into target
    if result_var != target_var {
        instrs.push(IRInstruction {
            kind: IRStmt::Assign {
                target: target_var,
                value: IRExpr::Var(result_var),
            },
            span,
            statement_id,
            scope_id: ctx.current_scope,
        });
    }

    instrs
}

pub fn create_scopes_map_ast(
    ast: &[ASTNodeContainer],
    ctx: &mut AnalysisContext,
    repeated: Option<usize>,
) {
    for node_c in ast {
        match &node_c.node {
            ASTNode::VarDecl(varname, expr) => {
                                if let Some(decl_expr) = expr {
                                    create_scopes_map_expr(decl_expr, ctx, node_c.statement_id);
                                }
                                let mut typ = IntermediateType::Indeterminate;
                                if let Some(assign_expr) = expr {
                                    match IntermediateType::type_from_expr(assign_expr) {
                                        Ok(t) => typ = t,
                                        Err(error) => {
                                            ctx.errors.push_error(error);
                                        }
                                    }
                                }
                                let vinfo = VariableInfo {
                                    name: varname.name().to_string(),
                                    id: ctx.var_counter,
                                    declared_span: node_c.span,
                                    declared_at: node_c.statement_id,
                                    is_init_scope: true,
                                    last_used: None,
                                    moved: false,
                                    last_mutated: expr.is_some().then_some(node_c.statement_id),
                                    scope_id: ctx.current_scope,
                                    shadowed: ctx
                                        .scopes
                                        .var_access_from_scope(varname.name(), ctx.current_scope)
                                        .is_some(),
                                };
                                ctx.set_type(vinfo.id, typ.clone(), node_c.span);
                                ctx.set_scope_id(vinfo.id, ctx.current_scope);
                                ctx.var_counter += 1;
                                if let Some(assign_expr) = expr {
                                    let ir = emit_assign_var(
                                        vinfo.metadata(typ, varname.span()),
                                        &assign_expr.normalized(),
                                        ctx,
                                        assign_expr.span,
                                        node_c.statement_id,
                                    );
                                    ctx.ir_output.extend_from_slice(&ir);
                                }
                                ctx.scopes.scopes[ctx.current_scope]
                                    .vars
                                    .insert(varname.name().to_string(), vinfo);
                            }
            ASTNode::Assignment(varname, assign_expr) => {
                                create_scopes_map_expr(assign_expr, ctx, node_c.statement_id);
                                let scope_opt = ctx
                                    .scopes
                                    .var_access_from_scope(varname.name(), ctx.current_scope);
                                if let Some(scope_idx) = scope_opt {
                                    let scopes = &mut ctx.scopes;
                                    if let Some(mut vinfo) = scopes.scopes[scope_idx]
                                        .vars
                                        .get_mut(varname.name())
                                        .cloned()
                                    {
                                        let mut typ = IntermediateType::Indeterminate;
                                        match IntermediateType::type_from_expr(assign_expr) {
                                            Ok(t) => typ = t,
                                            Err(error) => {
                                                ctx.errors.push_error(error);
                                            }
                                        }
                                        ctx.set_type(vinfo.id, typ.clone(), node_c.span);
                                        ctx.set_last_accessed(vinfo.id, node_c.statement_id);

                                        if let Some(repeat_stmt) = repeated {
                                            if ctx.current_scope != vinfo.scope_id
                                                && ctx.get_repeat_entered_at(vinfo.id).is_none()
                                                && ctx.get_repeat_write_at(vinfo.id).is_none()
                                            {
                                                ctx.set_repeat_entered_at(vinfo.id, repeat_stmt);
                                                ctx.set_repeat_write_at(vinfo.id, repeat_stmt);
                                            }
                                        }

                                        let ir = emit_assign_var(
                                            vinfo.metadata(typ, varname.span()),
                                            &assign_expr.normalized(),
                                            ctx,
                                            assign_expr.span,
                                            node_c.statement_id,
                                        );

                                        ctx.ir_output.extend_from_slice(&ir);
                                    } else {
                                        ctx.errors
                                            .push_error(CompileError::assign_undefined_variable(
                                                varname.name().to_string(),
                                                varname.span(),
                                            ));
                                    }
                                } else {
                                    ctx.errors
                                        .push_error(CompileError::assign_undefined_variable(
                                            varname.name().to_string(),
                                            varname.span(),
                                        ));
                                }
                            }
            ASTNode::IfStmt {
                                if_block,
                                elif_blocks,
                                else_block,
                            } => {
                                let mut branches = Vec::new();
                                let exit_label = ctx.label_allocator.fresh_label(LabelKind::End);
                                let mut branch_id = 0;

                                // Collect if and elif branches
                                branches.push(lower_conditional_branch(
                                    &if_block.condition,
                                    &if_block.body,
                                    ctx,
                                    LabelKind::If,
                                    branch_id,
                                    node_c.statement_id,
                                    repeated,
                                ));
                                branch_id += 1;
                                for elif in elif_blocks {
                                    branches.push(lower_conditional_branch(
                                        &elif.condition,
                                        &elif.body,
                                        ctx,
                                        LabelKind::Elif,
                                        branch_id,
                                        node_c.statement_id,
                                        repeated,
                                    ));
                                    branch_id += 1;
                                }

                                // Emit condition checks
                                for b in &branches {
                                    ctx.ir_output.extend_from_slice(&b.cond_instrs);
                                    ctx.ir_output.push(b.jump_if_true.clone());
                                }

                                // If no condition matched, jump to else or end
                                let else_label = ctx.label_allocator.fresh_label(LabelKind::Else);
                                if else_block.is_none() {
                                    ctx.ir_output.push(IRInstruction::jump(
                                        exit_label.clone(),
                                        if_block.condition.span,
                                        node_c.statement_id,
                                        ctx.current_scope,
                                    ));
                                } else {
                                    // Ensure we jump into the else block
                                    ctx.ir_output.push(IRInstruction::jump(
                                        else_label.clone(),
                                        if_block.condition.span,
                                        node_c.statement_id,
                                        ctx.current_scope,
                                    ));
                                }

                                // Emit branch bodies
                                for b in branches {
                                    ctx.ir_output.push(IRInstruction::label(
                                        b.block_label.clone(),
                                        if_block.condition.span,
                                        node_c.statement_id,
                                        ctx.current_scope,
                                    ));
                                    ctx.ir_output.extend_from_slice(&b.block_instrs);
                                    ctx.ir_output.push(IRInstruction::jump(
                                        exit_label.clone(),
                                        if_block.condition.span,
                                        node_c.statement_id,
                                        ctx.current_scope,
                                    ));
                                }

                                // Emit else block if any
                                if let Some(else_grp) = else_block {
                                    let else_scope = create_new_scope(ctx, repeated.is_some());
                                    ctx.current_scope = else_scope;

                                    ctx.ir_output.push(IRInstruction::label(
                                        else_label.clone(),
                                        else_grp.condition.span,
                                        node_c.statement_id,
                                        ctx.current_scope,
                                    ));

                                    create_scopes_map_ast(&else_grp.body, ctx, repeated);
                                    // TODO handle error
                                    ctx.current_scope = match ctx.scopes.scopes[else_scope].parent_index {
                                        Some(v) => v,
                                        None => {
                                            ctx.errors.push_error(CompileError::internal_compiler_error(
                                                "Attempted to access invalid scope",
                                            ));
                                            continue;
                                        }
                                    };
                                }

                                // Finally, the end label
                                ctx.ir_output.push(IRInstruction::label(
                                    exit_label,
                                    if_block.condition.span,
                                    node_c.statement_id,
                                    ctx.current_scope,
                                ));
                            }
            ASTNode::WhileStmt { condition, body } => {
                                let cond_span = condition.span;
                                let start_label = ctx.label_allocator.fresh_label(LabelKind::Loop);
                                let end_label = ctx.label_allocator.fresh_label(LabelKind::End);

                                // Emit start label
                                ctx.ir_output.push(IRInstruction::label(
                                    start_label.clone(),
                                    cond_span,
                                    node_c.statement_id,
                                    ctx.current_scope,
                                ));

                                // Evaluate condition
                                let (cond_temp, _) =
                                    fresh_temp_var(condition, ctx, Some(&IntermediateType::Boolean));
                                let (cond_var, cond_instrs) = match flatten_expr_to_var(
                                    condition,
                                    ctx,
                                    cond_temp.clone(),
                                    None,
                                    node_c.statement_id,
                                ) {
                                    Ok(res) => res,
                                    Err(err) => {
                                        ctx.errors.push_error(err);
                                        continue;
                                    }
                                };
                                let cond_var_id = cond_var.id();

                                if cond_var.typ() != &IntermediateType::Boolean {
                                    ctx.errors.push_error(CompileError::type_mismatch(
                                        "Bool",
                                        format!("{:?}", cond_var.typ()),
                                        cond_span,
                                    ));
                                }

                                // Add condition + jump-if-false to end
                                ctx.ir_output.extend(cond_instrs);
                                ctx.ir_output.push(IRInstruction::jump_if_not(
                                    end_label.clone(),
                                    cond_var,
                                    cond_span,
                                    node_c.statement_id,
                                    ctx.current_scope,
                                ));

                                // Enter loop body
                                let loop_scope = create_new_scope(ctx, true);
                                let saved_scope = ctx.current_scope;
                                ctx.current_scope = loop_scope;
                                ctx.set_scope_id(cond_var_id, loop_scope);
                                ctx.set_scope_id(cond_temp.id(), loop_scope);

                                // Push labels
                                ctx.loop_break_labels.push(end_label.clone());
                                ctx.loop_continue_labels.push(start_label.clone());

                                // Lower loop body
                                create_scopes_map_ast(body, ctx, Some(repeated.unwrap_or(node_c.statement_id)));

                                // Pop labels
                                ctx.loop_break_labels.pop();
                                ctx.loop_continue_labels.pop();
                                ctx.current_scope = saved_scope;

                                // Unconditional jump back to start
                                ctx.ir_output.push(IRInstruction::jump(
                                    start_label,
                                    cond_span,
                                    node_c.statement_id,
                                    ctx.current_scope,
                                ));

                                // Emit loop end label
                                ctx.ir_output.push(IRInstruction::label(
                                    end_label,
                                    cond_span,
                                    node_c.statement_id,
                                    ctx.current_scope,
                                ));
                            }
            ASTNode::Break => {
                                let span = node_c.span;
                                if let Some(label) = ctx.loop_break_labels.last() {
                                    ctx.ir_output.push(IRInstruction::jump(
                                        label.clone(),
                                        span,
                                        node_c.statement_id,
                                        ctx.current_scope,
                                    ));
                                } else {
                                    ctx.errors
                                        .push_error(CompileError::syntax_error("`break` outside of loop", span));
                                }
                            }
            ASTNode::Continue => {
                                let span = node_c.span;
                                if let Some(label) = ctx.loop_continue_labels.last() {
                                    ctx.ir_output.push(IRInstruction::jump(
                                        label.clone(),
                                        span,
                                        node_c.statement_id,
                                        ctx.current_scope,
                                    ));
                                } else {
                                    ctx.errors.push_error(CompileError::unexpected_error(
                                        "`continue` outside of loop",
                                        span,
                                    ));
                                }
                            }
            ASTNode::TypeDefinition { name, ty } => {
                        // TODO
                    },
            ASTNode::Comment(_comment ) => (),
        }
    }

    let drop_scope = ctx.current_scope;
    let current_scope_opt = ctx.scopes.scopes.get(drop_scope);

    // TODO evaluate if shadowing needs extra handling. Properly setting moved should theoratically resolve it
    if let Some(current_scope) = current_scope_opt {
        let mut drop_instructions = Vec::new();
        for varinfo in current_scope.vars.values() {
            if varinfo.moved {
                continue;
            }
            if varinfo.scope_id == current_scope.id {
                let typ = ctx.get_type(varinfo.id);
                let span = if let Some(last_node) = ast.last() {
                    Span {
                        start: last_node.span.end,
                        end: last_node.span.end,
                        line: last_node.span.line,
                        column: last_node.span.column,
                    }
                } else {
                    Span {
                        start: 0,
                        end: 0,
                        line: 0,
                        column: 0,
                    }
                };
                let var = IRVar::Var {
                    metadata: varinfo.metadata(typ, span),
                };
                // Figure a better id
                let drop_instr = IRInstruction::drop(var, span, usize::MAX, ctx.current_scope);
                drop_instructions.push(drop_instr);
            }
        }

        ctx.ir_output.extend_from_slice(&drop_instructions);
    }
}

pub fn create_scopes_map_expr(expr: &Expr, ctx: &mut AnalysisContext, statement_id: usize) {
    match &expr.kind {
        ExprKind::Bool(_) => {}
        ExprKind::Number(_) => {}
        ExprKind::Float(_) => {}
        ExprKind::Int(_) => {}
        ExprKind::UInt(_) => {}
        ExprKind::String(_) => {}
        ExprKind::Variable(varname) => {
            if let Some(scope_idx) = ctx
                .scopes
                .var_access_from_scope(varname.name(), ctx.current_scope)
            {
                if let Some(vinfo) = ctx.scopes.scopes[scope_idx].vars.get_mut(varname.name()) {
                    vinfo.last_used = Some(statement_id);
                }
            } else {
                ctx.errors.push_error(CompileError::read_undefined_variable(
                    varname.name().to_string(),
                    varname.span(),
                ));
            }
        }
        ExprKind::UnaryOp {
            op: _,
            expr: inner_exp,
        } => {
            create_scopes_map_expr(inner_exp, ctx, statement_id);
        }
        ExprKind::BinaryOp { left, op: _, right } => {
            create_scopes_map_expr(left, ctx, statement_id);
            create_scopes_map_expr(right, ctx, statement_id);
        }
        ExprKind::Invalid => {}
    }
}

struct LoweredBranch {
    cond_instrs: Vec<IRInstruction>,
    jump_if_true: IRInstruction,
    block_label: Label,
    block_instrs: Vec<IRInstruction>,
}

fn lower_conditional_branch(
    cond: &Expr,
    body: &[ASTNodeContainer],
    ctx: &mut AnalysisContext,
    kind: LabelKind,
    _branch_id: usize,
    statement_id: usize,
    repeated: Option<usize>,
) -> LoweredBranch {
    // Evaluate the condition
    let (temp_cond, _) = fresh_temp_var(cond, ctx, Some(&IntermediateType::Boolean));
    let (result_var, cond_instrs) =
        match flatten_expr_to_var(cond, ctx, temp_cond.clone(), None, statement_id) {
            Ok(res) => res,
            Err(e) => {
                ctx.errors.push_error(e);
                return LoweredBranch {
                    cond_instrs: vec![],
                    jump_if_true: IRInstruction::jump(
                        Label("error".into()),
                        cond.span,
                        statement_id,
                        ctx.current_scope,
                    ),
                    block_label: Label("error".into()),
                    block_instrs: vec![],
                };
            }
        };

    if result_var.typ() != &IntermediateType::Boolean {
        ctx.errors.push_error(CompileError::type_mismatch(
            "Bool",
            format!("{:?}", result_var.typ()),
            cond.span,
        ));
    }

    let block_label = ctx.label_allocator.fresh_label(kind);
    let jump = IRInstruction::jump_if(
        block_label.clone(),
        temp_cond.clone(),
        cond.span,
        statement_id,
        ctx.current_scope,
    );

    // Lower the body
    let block_scope = create_new_scope(ctx, repeated.is_some());
    let parent_scope = ctx.current_scope; // Save parent scope
    ctx.current_scope = block_scope;

    let mut block_instrs_start = ctx.ir_output.len();
    create_scopes_map_ast(body, ctx, repeated);
    let block_instrs = ctx.ir_output.split_off(block_instrs_start);

    ctx.current_scope = parent_scope;

    LoweredBranch {
        cond_instrs,
        jump_if_true: jump,
        block_label,
        block_instrs,
    }
}

fn create_new_scope(ctx: &mut AnalysisContext, repeated: bool) -> usize {
    let new_idx = ctx.scopes.scopes.len();
    let new_scope = Scope {
        id: new_idx,
        parent_index: Some(ctx.current_scope),
        repeated,
        ..Default::default()
    };
    ctx.scopes.scopes.push(new_scope);
    ctx.scopes.scopes[ctx.current_scope].children.push(new_idx);

    new_idx
}
