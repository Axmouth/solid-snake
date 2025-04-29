use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{
    RegisterFileExt, VmHeapExt, VmInterpretedExecutor, VmMemorySectionExt,
};
use crate::executor::interpreted::opcode_decoder::{FromBytes, RegisterType};

macro_rules! impl_load_immediate {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, $ty), [<$opcode handler>]);

            #[inline(always)]
            pub fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (reg, val) = args;

                debug!("Load: Directly {} R{} <= {}", stringify!($ty), reg, val);

                executor
                    .registers_mut()
                    .set_register_value(reg, val)?;
                Ok(())
            }
        }
    };
}

impl_load_immediate!(LoadImmediateU8, u8);
impl_load_immediate!(LoadImmediateU16, u16);
impl_load_immediate!(LoadImmediateU32, u32);
impl_load_immediate!(LoadImmediateU64, u64);
impl_load_immediate!(LoadImmediateI8, i8);
impl_load_immediate!(LoadImmediateI16, i16);
impl_load_immediate!(LoadImmediateI32, i32);
impl_load_immediate!(LoadImmediateI64, i64);
impl_load_immediate!(LoadImmediateF32, f32);
impl_load_immediate!(LoadImmediateF64, f64);

macro_rules! impl_load_indirect {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType), [<$opcode handler>]);

            #[inline(always)]
            pub fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (reg_ptr, dest) = args;

                // TODO
                // let addr: usize = executor.register_file.get_register_value(reg_ptr as usize)?;
                // let val = executor.memory.read_value::<$ty>(addr)?;

                // executor.register_file.set_register_value(dest as usize, val)?;
                Ok(())
            }
        }
    };
}

impl_load_indirect!(LoadIndirectU8, u8);
impl_load_indirect!(LoadIndirectU16, u16);
impl_load_indirect!(LoadIndirectU32, u32);
impl_load_indirect!(LoadIndirectU64, u64);
impl_load_indirect!(LoadIndirectI8, i8);
impl_load_indirect!(LoadIndirectI16, i16);
impl_load_indirect!(LoadIndirectI32, i32);
impl_load_indirect!(LoadIndirectI64, i64);
impl_load_indirect!(LoadIndirectF32, f32);
impl_load_indirect!(LoadIndirectF64, f64);

macro_rules! impl_load_from_imm_addr {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, u64), [<$opcode handler>]);

            #[inline(always)]
            pub fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (reg_dest, addr) = args;

                // TODO
                // let val = executor.memory.read_value::<$ty>(addr as usize)?;
                // executor.register_file.set_register_value(reg_dest as usize, val)?;

                Ok(())
            }
        }
    };
}

impl_load_from_imm_addr!(LoadFromImmediateU8, u8);
impl_load_from_imm_addr!(LoadFromImmediateU16, u16);
impl_load_from_imm_addr!(LoadFromImmediateU32, u32);
impl_load_from_imm_addr!(LoadFromImmediateU64, u64);
impl_load_from_imm_addr!(LoadFromImmediateI8, i8);
impl_load_from_imm_addr!(LoadFromImmediateI16, i16);
impl_load_from_imm_addr!(LoadFromImmediateI32, i32);
impl_load_from_imm_addr!(LoadFromImmediateI64, i64);
impl_load_from_imm_addr!(LoadFromImmediateF32, f32);
impl_load_from_imm_addr!(LoadFromImmediateF64, f64);

macro_rules! impl_load_indirect_with_offset {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode handler>]);

            #[inline(always)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (reg_dest, reg_ptr, reg_offset) = args;

                debug!("Load: Indirectly {} R{} <= From address {} with offset {}", stringify!($ty), reg_dest, reg_ptr, reg_offset);

                let section_idx: u64 = executor.registers().get_register_value(reg_ptr)?;
                let offset: u64 = executor.registers().get_register_value(reg_offset)?;

                let mem_section = executor.heap().section(section_idx as usize)?;
                let bytes = mem_section.bytes_n_with_offset(std::mem::size_of::<$ty>(), offset as usize)?;

                let value = <$ty as FromBytes>::from_be_bytes(bytes);
                executor.registers_mut().set_register_value(reg_dest, value)?;

                Ok(())
            }
        }
    };
}

impl_load_indirect_with_offset!(LoadIndirectWithOffsetU8, u8);
impl_load_indirect_with_offset!(LoadIndirectWithOffsetU16, u16);
impl_load_indirect_with_offset!(LoadIndirectWithOffsetU32, u32);
impl_load_indirect_with_offset!(LoadIndirectWithOffsetU64, u64);
impl_load_indirect_with_offset!(LoadIndirectWithOffsetI8, i8);
impl_load_indirect_with_offset!(LoadIndirectWithOffsetI16, i16);
impl_load_indirect_with_offset!(LoadIndirectWithOffsetI32, i32);
impl_load_indirect_with_offset!(LoadIndirectWithOffsetI64, i64);
impl_load_indirect_with_offset!(LoadIndirectWithOffsetF32, f32);
impl_load_indirect_with_offset!(LoadIndirectWithOffsetF64, f64);

// todo more instrs, tests
