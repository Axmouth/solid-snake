use log::debug;

use crate::define_instruction;
use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{
    RegisterFileExt, VmHeapExt, VmInterpretedExecutor, VmMemorySectionExt,
};
use crate::executor::interpreted::opcode_decoder::RegisterType;

define_instruction!(Allocate, (RegisterType, RegisterType), allocate_handler);
define_instruction!(Deallocate, (RegisterType), deallocate_handler);
define_instruction!(
    Memcpy,
    (
        RegisterType,
        RegisterType,
        RegisterType,
        RegisterType,
        RegisterType
    ),
    memcpy_handler
);
define_instruction!(
    MemSet,
    (RegisterType, RegisterType, RegisterType),
    memset_handler
);

#[inline(always)]
pub fn allocate_handler(
    executor: &mut VmInterpretedExecutor,
    args: AllocateArgs,
) -> Result<(), VmExecutionError> {
    let (reg_target, reg_size) = args;

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
    args: DeallocateArgs,
) -> Result<(), VmExecutionError> {
    let (reg_target,) = args;

    let section_idx: u64 = executor.registers().get_register_value(reg_target)?;

    debug!("Deallocate: Section R{} ({})", reg_target, section_idx);

    executor.heap_mut().free(section_idx as usize)?;

    Ok(())
}

#[inline(always)]
pub fn memcpy_handler(
    executor: &mut VmInterpretedExecutor,
    args: MemcpyArgs,
) -> Result<(), VmExecutionError> {
    let (reg_dest, reg_dest_offset, reg_src, reg_src_offset, reg_size) = args;

    let dest_idx: u64 = executor.registers().get_register_value(reg_dest)?;
    let src_idx: u64 = executor.registers().get_register_value(reg_src)?;
    let dest_offset: u64 = executor.registers().get_register_value(reg_dest_offset)?;
    let src_offset: u64 = executor.registers().get_register_value(reg_src_offset)?;
    let size: u64 = executor.registers().get_register_value(reg_size)?;

    debug!(
        "Memcpy: Copy {} bytes from section R{} ({}) with offset R{} ({}) to section R{} ({}) with offset R{} ({})",
        size,
        reg_src,
        src_idx,
        reg_src_offset,
        src_offset,
        reg_dest,
        dest_idx,
        reg_dest_offset,
        dest_offset
    );

    let src = executor.heap().section(src_idx as usize)?;
    println!("src len {}", src.len());
    let src_bytes = src
        .bytes_n_with_offset(size as usize, src_offset as usize)?
        .to_vec();
    let dest = executor.heap_mut().section_mut(dest_idx as usize)?;
    println!("dest len {}", dest.len());

    let dest_bytes = dest.bytes_n_with_offset_mut(size as usize, dest_offset as usize)?;

    dest_bytes.copy_from_slice(&src_bytes);

    Ok(())
}

#[inline(always)]
pub fn memset_handler(
    executor: &mut VmInterpretedExecutor,
    args: MemSetArgs,
) -> Result<(), VmExecutionError> {
    let (reg_ptr, reg_value, reg_size) = args;

    let section_idx: u64 = executor.registers().get_register_value(reg_ptr)?;
    let value: u64 = executor.registers().get_register_value(reg_value)?;
    let size: u64 = executor.registers().get_register_value(reg_size)?;

    debug!(
        "MemSet: Fill section R{} ({}) with value 0x{:02X} for {} bytes",
        reg_ptr,
        section_idx,
        value & 0xFF,
        size
    );

    let section = executor.heap_mut().section_mut(section_idx as usize)?;
    let dest = section.bytes_n_mut(size as usize)?;

    dest.fill(value as u8);

    Ok(())
}

// todo tests
