use std::borrow::Cow;

use crate::executor::{
    ext::VmExecutionError,
    interpreted::{
        implimentation::{VmHeapExt, VmInterpretedExecutor, VmMemorySectionExt},
        opcode_decoder::RegisterType,
    },
};

crate::define_instruction!(Print, (RegisterType, RegisterType, RegisterType), print_raw);

fn print_raw(
    executor: &mut VmInterpretedExecutor,
    args: PrintArgs,
) -> Result<(), VmExecutionError> {
    let (reg_section_id, reg_offset, reg_length) = args;
    let section_id = executor.registers().raw[usize::from(reg_section_id)];
    let offset = executor.registers().raw[usize::from(reg_offset)];
    let length = executor.registers().raw[usize::from(reg_length)];

    let bytes = executor
        .heap()
        .section(section_id as usize)?
        .bytes_n_with_offset(length as usize, offset as usize)?;

    let text: Cow<str> = String::from_utf8_lossy(bytes);

    print!("{}", text);

    Ok(())
}

// formatting instrs
