use solid_snake_vm::executor::interpreted::opcode_decoder::RegisterType;
use solid_snake_vm::{
    executor::interpreted::implimentation::MAX_REGISTERS, opcodes::UnprocessedInstruction,
};

use paste::paste;

use crate::ast::{BinaryOp, IntermediateType, ProcessedType};
use crate::error_reporting::CompileError;
use crate::intermediate_pass1::{TypedIRExpr, TypedIRInstruction, TypedIRStmt};

use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BCType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

fn string_type() -> IntermediateType {
    let mut properties = BTreeMap::new();

    properties.insert("length".to_string(), IntermediateType::Int);

    let str_type = IntermediateType::Object { properties };

    str_type
}

impl BCType {
    pub fn from_ir_type(typ: ProcessedType) -> Self {
        match typ {
            ProcessedType::Boolean => Self::U8,
            ProcessedType::String => Self::U64,
            ProcessedType::Int => Self::I64,
            ProcessedType::UInt => Self::U64,
            ProcessedType::Float => Self::F64,
            ProcessedType::Byte => Self::U8,
            ProcessedType::Array { inner } => todo!(),
            ProcessedType::List { inner } => todo!(),
            ProcessedType::Object { properties } => todo!(),
            ProcessedType::Tuple { inner } => todo!(),
            ProcessedType::Indirect { target } => todo!(),
            ProcessedType::Enum { variants } => todo!(),
        }
    }
}

macro_rules! match_ops_for_type {
    ($op:expr, $target:ident, $lhs:ident, $rhs:ident, $type:ident, [$($opname:ident),*]) => {
        match $op {
            $(
                BinaryOp::$opname => {
                    paste! {
                        Some(UnprocessedInstruction::[<$opname $type>]((
                            $target.into(),
                            $lhs.into(),
                            $rhs.into(),
                        )))
                    }
                }
            )*
            _ => {
                None
            }
        }
    };
}

macro_rules! impl_op_match {
    ($op:expr, $typ:expr, $target:ident, $lhs:ident, $rhs:ident => {
        $($type:ident : [$($opname:ident),*]),* $(,)?
    }) => {{
        match $typ {
            $(
                BCType::$type => {
                    match_ops_for_type!($op, $target, $lhs, $rhs, $type, [$($opname),*])
                }
            )*
            _ => {
                None
            }
        }
    }};
}

pub const RESERVED_REGS: u8 = 4; // r0–r3: return, args, etc.
pub const SCRATCH_REGS: u8 = 6; // r4–r9: for immediate loads, alloc, math, etc.
pub const SPILL_TABLE_REG: u8 = 10; // r10: holds pointer to spill table
pub const FIRST_USABLE_REG: u8 = SPILL_TABLE_REG + 1;

#[derive(Debug)]
pub struct ScratchPool {
    pool: Vec<u8>,
}

impl ScratchPool {
    pub fn new(scratch_regs: &[u8]) -> Self {
        Self {
            pool: scratch_regs.to_vec(),
        }
    }

    pub fn take(&mut self) -> Option<u8> {
        self.pool.pop()
    }

    pub fn give_back(&mut self, reg: u8) {
        self.pool.push(reg);
    }

    pub fn take_n(&mut self, n: usize) -> Option<Vec<u8>> {
        if self.pool.len() >= n {
            Some((0..n).filter_map(|_| self.take()).collect())
        } else {
            None
        }
    }
}

pub struct RegisterAllocator {
    max_registers: usize,
    next_spill_offset: usize,
    reg_to_var: [Option<usize>; MAX_REGISTERS],
    var_to_reg: HashMap<usize, u8>,
    spilled_vars: HashMap<usize, usize>,
    free_regs: Vec<u8>,
    scratch_pool: ScratchPool,
}

impl RegisterAllocator {
    pub fn new(max_registers: usize) -> Self {
        assert!(max_registers <= MAX_REGISTERS);
        let usable_range = FIRST_USABLE_REG..(max_registers as u8);
        let scratch_range = RESERVED_REGS..(RESERVED_REGS + SCRATCH_REGS - 1);
        Self {
            max_registers,
            next_spill_offset: 0,
            reg_to_var: [None; MAX_REGISTERS],
            var_to_reg: HashMap::new(),
            spilled_vars: HashMap::new(),
            free_regs: usable_range.rev().collect(), // LIFO
            scratch_pool: ScratchPool::new(&scratch_range.rev().collect::<Vec<u8>>()), // LIFO
        }
    }

    // Turn to method?
    pub fn spill_table_register(&self) -> u8 {
        SPILL_TABLE_REG
    }
    pub fn allocate(&mut self, var_id: usize) -> Allocation {
        if let Some(&reg) = self.var_to_reg.get(&var_id) {
            return Allocation::Register(reg);
        }

        if let Some(reg) = self.free_regs.pop() {
            self.reg_to_var[reg as usize] = Some(var_id);
            self.var_to_reg.insert(var_id, reg);
            Allocation::Register(reg)
        } else {
            let offset = self.next_spill_offset;
            self.next_spill_offset += 1;
            self.spilled_vars.insert(var_id, offset);
            Allocation::Spilled(offset)
        }
    }

    pub fn free(&mut self, var_id: usize) {
        if let Some(reg) = self.var_to_reg.remove(&var_id) {
            self.reg_to_var[reg as usize] = None;
            self.free_regs.push(reg);
        } else {
            self.spilled_vars.remove(&var_id);
        }
    }

    pub fn is_spilled(&self, var_id: usize) -> bool {
        self.spilled_vars.contains_key(&var_id)
    }

    pub fn get_register(&self, var_id: usize) -> Option<u8> {
        self.var_to_reg.get(&var_id).copied()
    }

    pub fn get_spill_offset(&self, var_id: usize) -> Option<usize> {
        self.spilled_vars.get(&var_id).copied()
    }

    pub fn register_restored(&mut self, var_id: usize, reg: u8) {
        self.reg_to_var[reg as usize] = Some(var_id);
        self.var_to_reg.insert(var_id, reg);
        self.spilled_vars.remove(&var_id);
    }

    pub fn emit_spillage_table_init(
        &self,
        max_var_id: usize,
        scratch_reg: u8,
    ) -> Vec<UnprocessedInstruction> {
        vec![
            UnprocessedInstruction::LoadImmediateU64((scratch_reg.into(), (max_var_id * 8) as u64)),
            // Store into the dedicated SPILL_TABLE_REG
            UnprocessedInstruction::Allocate((
                self.spill_table_register().into(),
                scratch_reg.into(),
            )),
        ]
    }

    pub fn emit_spill_store(
        &self,
        var_id: usize,
        var_reg: u8,
        reg_alloc: &RegisterAllocator,
        scratch_reg: u8,
    ) -> Vec<UnprocessedInstruction> {
        let offset = reg_alloc.get_spill_offset(var_id).unwrap();

        vec![
            UnprocessedInstruction::LoadImmediateU64((scratch_reg.into(), (offset * 8) as u64)),
            UnprocessedInstruction::StoreIndirectWithOffsetU64((
                var_reg.into(),
                self.spill_table_register().into(),
                scratch_reg.into(),
            )),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Allocation {
    Register(u8),
    Spilled(usize),
}

pub fn emit_spill_store(
    var_id: usize,
    var_reg: u8,
    reg_alloc: &RegisterAllocator,
    scratch_reg: u8,
) -> Vec<UnprocessedInstruction> {
    let offset = reg_alloc.get_spill_offset(var_id).unwrap();

    vec![
        UnprocessedInstruction::LoadImmediateU64((scratch_reg.into(), (offset * 8) as u64)),
        UnprocessedInstruction::StoreIndirectWithOffsetU64((
            var_reg.into(),
            reg_alloc.spill_table_register().into(),
            scratch_reg.into(),
        )),
    ]
}

pub fn emit_spill_load(
    var_id: usize,
    target_reg: u8,
    reg_alloc: &RegisterAllocator,
    scratch_reg: u8,
) -> Vec<UnprocessedInstruction> {
    let offset = reg_alloc.get_spill_offset(var_id).unwrap();

    vec![
        UnprocessedInstruction::LoadImmediateU64((scratch_reg.into(), (offset * 8) as u64)),
        UnprocessedInstruction::LoadIndirectWithOffsetU64((
            target_reg.into(),
            reg_alloc.spill_table_register().into(),
            scratch_reg.into(),
        )),
    ]
}

/// Ensures var is in register, spilling others if needed.
/// Handles restoring from memory if previously spilled.
pub fn get_var_register(
    var_id: usize,
    reg_alloc: &mut RegisterAllocator,
) -> Result<(u8, Vec<UnprocessedInstruction>), CompileError> {
    if let Some(r) = reg_alloc.get_register(var_id) {
        return Ok((r, vec![])); // Already allocated
    }

    if reg_alloc.is_spilled(var_id) {
        // Need to load back into register
        let [addr_reg, offset_reg, dest_reg] = reg_alloc
            .scratch_pool
            .take_n(3)
            .ok_or(CompileError::internal_compiler_error(
                "Not enough scratch registers to restore spilled var",
            ))?
            .try_into()
            .unwrap();

        let offset = reg_alloc.get_spill_offset(var_id).unwrap();
        let spill_table_reg = reg_alloc.spill_table_register();

        let restore = vec![
            UnprocessedInstruction::LoadImmediateU64((offset_reg.into(), offset as u64)),
            UnprocessedInstruction::LoadIndirectWithOffsetU64((
                dest_reg.into(),
                spill_table_reg.into(),
                offset_reg.into(),
            )),
        ];

        // Mark as now "in register"
        reg_alloc.register_restored(var_id, dest_reg);

        // Free scratch regs used (except dest_reg — caller will use it)
        reg_alloc.scratch_pool.give_back(addr_reg);
        reg_alloc.scratch_pool.give_back(offset_reg);

        return Ok((dest_reg, restore));
    }

    // Not yet allocated nor spilled
    match reg_alloc.allocate(var_id) {
        Allocation::Register(r) => Ok((r, vec![])),
        Allocation::Spilled(offset) => {
            // Restore into scratch
            let [addr_reg, offset_reg, dest_reg] = reg_alloc
                .scratch_pool
                .take_n(3)
                .ok_or(CompileError::internal_compiler_error(
                    "Not enough scratch registers to restore spilled var",
                ))?
                .try_into()
                .unwrap();

            let spill_table_reg = reg_alloc.spill_table_register();
            let restore = vec![
                UnprocessedInstruction::LoadImmediateU64((offset_reg.into(), offset as u64)),
                UnprocessedInstruction::LoadIndirectWithOffsetU64((
                    dest_reg.into(),
                    spill_table_reg.into(),
                    offset_reg.into(),
                )),
            ];

            reg_alloc.register_restored(var_id, dest_reg);

            reg_alloc.scratch_pool.give_back(addr_reg);
            reg_alloc.scratch_pool.give_back(offset_reg);

            Ok((dest_reg, restore))
        }
    }
}

fn lower_expr_to_bytecode_stage_one(
    expr: &TypedIRExpr,
    target_reg: u8,
    reg_alloc: &mut RegisterAllocator,
    constants: &mut Vec<Vec<u8>>,
) -> Result<Vec<UnprocessedInstruction>, CompileError> {
    let mut instructions = Vec::with_capacity(10);

    match expr {
        TypedIRExpr::Var(var) => {
            let (reg_from, instrs) = get_var_register(var.id, reg_alloc)?;
            instructions.extend_from_slice(&instrs);
            instructions.push(UnprocessedInstruction::MoveU64((
                target_reg.into(),
                reg_from.into(),
            )));
        }
        TypedIRExpr::Bool(val) => {
            let byte = *val as u8;
            instructions.push(UnprocessedInstruction::LoadImmediateU8((
                target_reg.into(),
                byte,
            )));
        }
        TypedIRExpr::Int(val) => {
            instructions.push(UnprocessedInstruction::LoadImmediateI64((
                target_reg.into(),
                *val,
            )));
        }
        TypedIRExpr::UInt(val) => {
            instructions.push(UnprocessedInstruction::LoadImmediateU64((
                target_reg.into(),
                *val,
            )));
        }
        TypedIRExpr::Float(val) => {
            instructions.push(UnprocessedInstruction::LoadImmediateF64((
                target_reg.into(),
                *val,
            )));
        }
        // TODO StoreArrayImmediate?
        TypedIRExpr::String(val) => {
            let mut new_constant: Vec<u8> = Vec::with_capacity(val.len() + size_of::<u64>());
            let length_bytes = (val.len() as u64).to_be_bytes();
            new_constant.extend(length_bytes);
            new_constant.extend_from_slice(val.as_bytes());
            constants.push(new_constant);
            let const_id = (constants.len() - 1) as u64;
            instructions.push(UnprocessedInstruction::StoreConstantArray((
                target_reg.into(),
                const_id,
            )));
        }
        TypedIRExpr::Binary { op, left, right } => {
            let (reg_left, instrs) = get_var_register(left.id, reg_alloc)?;
            instructions.extend(instrs);
            let (reg_right, instrs) = get_var_register(right.id, reg_alloc)?;
            instructions.extend(instrs);

            if left.typ == ProcessedType::String && right.typ == ProcessedType::String {
                let instrs = emit_string_concat(target_reg, reg_left, reg_right, reg_alloc);
                instructions.extend(instrs);
            } else {
                let maybe_instr = impl_op_match!(
                    *op,       // BCType
                    BCType::from_ir_type(left.typ.clone()),
                    target_reg,
                    reg_left,
                    reg_right => {
                        I64: [Add, Subtract, Multiply, Divide, Modulo, Equal, GreaterThan, LessThan, BitwiseAnd, BitwiseOr],
                        F64: [Add, Subtract, Multiply, Divide, Modulo, Equal, GreaterThan, LessThan],
                        U8: [Add, Subtract, Multiply, Divide, Modulo, Equal, GreaterThan],
                        // ...
                    }
                );

                if let Some(instr) = maybe_instr {
                    instructions.push(instr);
                } else {
                    return Err(CompileError::internal_compiler_error(format!(
                        "derp {} {:?}",
                        op,
                        BCType::from_ir_type(left.typ.clone())
                    )));
                }
            }
        }
        TypedIRExpr::Unary { op, expr } => {
            let (reg, instrs) = get_var_register(expr.id, reg_alloc)?;
            instructions.extend(instrs);
            // TODO
            todo!()
        }
    }

    Ok(instructions)
}

pub fn lower_ir_to_bytecode_stage_one(
    ir: &[TypedIRInstruction],
    var_count: usize,
) -> Result<(Vec<UnprocessedInstruction>, Vec<Vec<u8>>), CompileError> {
    let mut instructions = Vec::with_capacity(ir.len());
    let mut constants: Vec<Vec<u8>> = Vec::new();

    let mut reg_alloc = RegisterAllocator::new(MAX_REGISTERS);
    let scratch_reg_1 = reg_alloc.scratch_pool.take().unwrap();
    instructions.extend_from_slice(&reg_alloc.emit_spillage_table_init(var_count, scratch_reg_1));
    reg_alloc.scratch_pool.give_back(scratch_reg_1);
    // let (reg, instrs) = get_var_register(var_id, &mut reg_alloc, &mut scratch_pool)?;
    for ir_instr in ir {
        match &ir_instr.kind {
            TypedIRStmt::Assign { target, value } => {
                let (reg, instrs) = get_var_register(target.id, &mut reg_alloc).unwrap();
                instructions.extend_from_slice(&instrs);
                let instrs =
                    lower_expr_to_bytecode_stage_one(value, reg, &mut reg_alloc, &mut constants)?;
                instructions.extend_from_slice(&instrs);
            }
            TypedIRStmt::Drop { target } => {
                // TODO logic to Deallocate heap types
                reg_alloc.free(target.id);
            }
            TypedIRStmt::Jump { target } => {
                let jump = UnprocessedInstruction::Jump((target.to_string(),));
                instructions.push(jump);
            }
            TypedIRStmt::JumpIf { condition, target } => {
                let (reg, instrs) = get_var_register(condition.id, &mut reg_alloc).unwrap();
                instructions.extend_from_slice(&instrs);
                let jump = UnprocessedInstruction::JumpIf((target.to_string(), reg.into()));
                instructions.push(jump);
            }
            TypedIRStmt::JumpIfNot { condition, target } => {
                let (reg, instrs) = get_var_register(condition.id, &mut reg_alloc).unwrap();
                instructions.extend_from_slice(&instrs);
                let jump = UnprocessedInstruction::JumpIfFalse((target.to_string(), reg.into()));
                instructions.push(jump);
            }
            TypedIRStmt::Label(label) => {
                let label_instr = UnprocessedInstruction::Label((label.to_string(),));
                instructions.push(label_instr);
            }
            TypedIRStmt::Call {
                function_id,
                args,
                dest,
            } => todo!(),
            TypedIRStmt::Return { value } => todo!(),
        }
    }

    instructions.push(UnprocessedInstruction::Halt(()));

    Ok((instructions, constants))
}

pub fn emit_string_concat(
    target_reg: u8,
    lhs_reg: u8,
    rhs_reg: u8,
    reg_alloc: &mut RegisterAllocator,
) -> Vec<UnprocessedInstruction> {
    assert!(
        reg_alloc.scratch_pool.pool.len() >= 5,
        "Not enough scratch registers for string concat"
    );

    let mut instrs = vec![];

    // Allocate scratch regs
    let len1 = reg_alloc.scratch_pool.take().unwrap(); // load lhs len
    let len2 = reg_alloc.scratch_pool.take().unwrap(); // load rhs len
    let total_len = reg_alloc.scratch_pool.take().unwrap();
    let alloc_size = reg_alloc.scratch_pool.take().unwrap();
    let offset_tmp = reg_alloc.scratch_pool.take().unwrap(); // for offset handling

    // Load lengths
    instrs.push(UnprocessedInstruction::LoadImmediateU64((
        offset_tmp.into(),
        0,
    )));
    instrs.push(UnprocessedInstruction::LoadIndirectWithOffsetU64((
        len1.into(),
        lhs_reg.into(),
        offset_tmp.into(),
    )));
    instrs.push(UnprocessedInstruction::LoadIndirectWithOffsetU64((
        len2.into(),
        rhs_reg.into(),
        offset_tmp.into(),
    )));

    // total_len = len1 + len2
    instrs.push(UnprocessedInstruction::AddU64((
        total_len.into(),
        len1.into(),
        len2.into(),
    )));

    instrs.push(UnprocessedInstruction::IncrementU64((
        total_len.into(),
        size_of::<u64>() as u64,
    )));

    instrs.push(UnprocessedInstruction::MoveU64((
        alloc_size.into(),
        total_len.into(),
    )));

    instrs.push(UnprocessedInstruction::IncrementU64((
        alloc_size.into(),
        size_of::<u64>() as u64,
    )));

    // Allocate target buffer
    instrs.push(UnprocessedInstruction::Allocate((
        target_reg.into(),
        alloc_size.into(),
    )));

    // Store length and capacity
    instrs.push(UnprocessedInstruction::StoreIndirectWithOffsetU64((
        target_reg.into(),
        total_len.into(),
        offset_tmp.into(),
    )));

    // reuse alloc_size, we have limited scratch
    instrs.push(UnprocessedInstruction::MoveU64((
        alloc_size.into(),
        len1.into(),
    )));
    instrs.push(UnprocessedInstruction::IncrementU64((
        alloc_size.into(),
        size_of::<u64>() as u64,
    )));
    instrs.push(UnprocessedInstruction::LoadImmediateU64((
        offset_tmp.into(),
        size_of::<u64>() as u64,
    )));

    // Copy lhs bytes to offset 16
    instrs.push(UnprocessedInstruction::Memcpy((
        target_reg.into(),
        offset_tmp.into(),
        lhs_reg.into(),
        offset_tmp.into(),
        len1.into(),
    )));

    instrs.push(UnprocessedInstruction::AddU64((
        offset_tmp.into(),
        offset_tmp.into(),
        len1.into(),
    )));
    instrs.push(UnprocessedInstruction::Memcpy((
        target_reg.into(),
        alloc_size.into(),
        rhs_reg.into(),
        offset_tmp.into(),
        len2.into(),
    )));

    // Free
    reg_alloc.scratch_pool.give_back(len1);
    reg_alloc.scratch_pool.give_back(len2);
    reg_alloc.scratch_pool.give_back(total_len);
    reg_alloc.scratch_pool.give_back(alloc_size);
    reg_alloc.scratch_pool.give_back(offset_tmp);

    instrs
}

pub fn builtin_prin(section_idx_reg: RegisterType) {}
