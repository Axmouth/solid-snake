use log::debug;
use paste::paste;

use crate::define_instruction;
use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{
    RegisterFileExt, VmHeapExt, VmInterpretedExecutor, VmMemorySectionExt,
};
use crate::executor::interpreted::opcode_decoder::RegisterType;

define_instruction!(Allocate, (RegisterType, RegisterType), allocate_handler);
define_instruction!(Deallocate, (RegisterType), deallocate_handler);
define_instruction!(Memcpy, (RegisterType, RegisterType, RegisterType), memcpy_handler);
define_instruction!(MemSet, (RegisterType, RegisterType, RegisterType), memset_handler);

#[inline(always)]
pub fn allocate_handler(
    executor: &mut VmInterpretedExecutor,
    look_ahead: &[u8],
) -> Result<(), VmExecutionError> {
    assert_eq!(look_ahead.len(), AllocateInstruction::arg_size());
    let (reg_target, reg_size) = AllocateInstruction::parse_args(look_ahead);

    let size: u64 = executor.registers().get_register_value(reg_size)?;
    let section_idx = executor.heap_mut().alloc(size as usize)?;

    debug!(
        "Allocate: Section with R{} ({}) bytes, store pointer to R{} ({})",
        reg_target, section_idx, reg_size, size
    );

    executor
        .registers_mut()
        .set_register_value(reg_target, section_idx as u64)?;

    Ok(())
}

#[inline(always)]
pub fn deallocate_handler(
    executor: &mut VmInterpretedExecutor,
    look_ahead: &[u8],
) -> Result<(), VmExecutionError> {
    assert_eq!(look_ahead.len(), DeallocateInstruction::arg_size());
    let (reg_target,) = DeallocateInstruction::parse_args(look_ahead);

    let section_idx: u64 = executor.registers().get_register_value(reg_target)?;

    debug!("Deallocate: Section R{} ({})", reg_target, section_idx);

    executor.heap_mut().free(section_idx as usize)?;

    Ok(())
}

#[inline(always)]
pub fn memcpy_handler(
    executor: &mut VmInterpretedExecutor,
    look_ahead: &[u8],
) -> Result<(), VmExecutionError> {
    let (reg_dest, reg_src, reg_size) = MemcpyInstruction::parse_args(look_ahead);

    let dest_idx: u64 = executor.registers().get_register_value(reg_dest)?;
    let src_idx: u64 = executor.registers().get_register_value(reg_src)?;
    let size: u64 = executor.registers().get_register_value(reg_size)?;

    debug!("Memcpy: Copy {} bytes from section R{} ({}) to section R{} ({})", size, reg_src, src_idx, reg_dest, dest_idx);

    let src = executor.heap().section(src_idx as usize)?;
    let src_bytes = src.bytes_n(size as usize)?.to_vec();
    let dest = executor.heap_mut().section_mut(dest_idx as usize)?;

    let dest_bytes = dest.bytes_n_mut(size as usize)?;

    dest_bytes.copy_from_slice(&src_bytes);

    Ok(())
}

#[inline(always)]
pub fn memset_handler(
    executor: &mut VmInterpretedExecutor,
    look_ahead: &[u8],
) -> Result<(), VmExecutionError> {
    let (reg_ptr, reg_value, reg_size) = MemSetInstruction::parse_args(look_ahead);

    let section_idx: u64 = executor.registers().get_register_value(reg_ptr)?;
    let value: u64 = executor.registers().get_register_value(reg_value)?;
    let size: u64 = executor.registers().get_register_value(reg_size)?;

    debug!("MemSet: Fill section R{} ({}) with value 0x{:02X} for {} bytes", reg_ptr, section_idx, value & 0xFF, size);

    let section = executor.heap_mut().section_mut(section_idx as usize)?;
    let dest = section.bytes_n_mut(size as usize)?;

    dest.fill(value as u8);

    Ok(())
}