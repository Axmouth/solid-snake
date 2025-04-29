use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{
    RegisterFileExt, VmHeapExt, VmInterpretedExecutor, VmMemorySectionExt,
};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_store_indirect_with_offset {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode handler>]);

            #[inline(always)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (reg_ptr, reg_value, reg_offset) = args;

                let section_idx: u64 = executor.registers().get_register_value(reg_ptr)?;
                let offset: u64 = executor.registers().get_register_value(reg_offset)?;
                let value: $ty = executor.registers().get_register_value(reg_value)?;

                let bytes = &value.to_be_bytes();

                debug!("Store: {} R{} ({}) to address R{} ({}) with offset R{} {}", stringify!($ty), reg_value, value, reg_ptr, section_idx, reg_offset, offset);

                let section = executor.heap_mut().section_mut(section_idx as usize)?;
                let target = section.bytes_n_with_offset_mut(bytes.len(), offset as usize)?;

                target.copy_from_slice(bytes);

                Ok(())
            }
        }
    };
}

impl_store_indirect_with_offset!(StoreIndirectWithOffsetU8, u8);
impl_store_indirect_with_offset!(StoreIndirectWithOffsetU16, u16);
impl_store_indirect_with_offset!(StoreIndirectWithOffsetU32, u32);
impl_store_indirect_with_offset!(StoreIndirectWithOffsetU64, u64);
impl_store_indirect_with_offset!(StoreIndirectWithOffsetI8, i8);
impl_store_indirect_with_offset!(StoreIndirectWithOffsetI16, i16);
impl_store_indirect_with_offset!(StoreIndirectWithOffsetI32, i32);
impl_store_indirect_with_offset!(StoreIndirectWithOffsetI64, i64);
impl_store_indirect_with_offset!(StoreIndirectWithOffsetF32, f32);
impl_store_indirect_with_offset!(StoreIndirectWithOffsetF64, f64);

macro_rules! impl_store_from_imm_with_offset {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (u64, RegisterType, RegisterType), [<$opcode handler>]);

            #[inline(always)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (section_idx, reg_value, reg_offset) = args;

                let offset: u64 = executor.registers().get_register_value(reg_offset)?;
                let value: $ty = executor.registers().get_register_value(reg_value)?;

                debug!("Store: Directly {} R{} ({}) to address {} with offset R{} {}", stringify!($ty), reg_value, value, section_idx, reg_offset, offset);

                let bytes = &value.to_be_bytes();

                let section = executor.heap_mut().section_mut(section_idx as usize)?;
                let target = section.bytes_n_with_offset_mut(bytes.len(), offset as usize)?;

                target.copy_from_slice(bytes);

                Ok(())
            }
        }
    };
}

impl_store_from_imm_with_offset!(StoreFromImmediateWithOffsetU8, u8);
impl_store_from_imm_with_offset!(StoreFromImmediateWithOffsetU16, u16);
impl_store_from_imm_with_offset!(StoreFromImmediateWithOffsetU32, u32);
impl_store_from_imm_with_offset!(StoreFromImmediateWithOffsetU64, u64);
impl_store_from_imm_with_offset!(StoreFromImmediateWithOffsetI8, i8);
impl_store_from_imm_with_offset!(StoreFromImmediateWithOffsetI16, i16);
impl_store_from_imm_with_offset!(StoreFromImmediateWithOffsetI32, i32);
impl_store_from_imm_with_offset!(StoreFromImmediateWithOffsetI64, i64);
impl_store_from_imm_with_offset!(StoreFromImmediateWithOffsetF32, f32);
impl_store_from_imm_with_offset!(StoreFromImmediateWithOffsetF64, f64);

// todo more instrs, tests
