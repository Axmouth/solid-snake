use std::borrow::Cow;

use crate::executor::{
    ext::VmExecutionError,
    interpreted::{
        implimentation::{VmHeapExt, VmInterpretedExecutor, VmMemorySectionExt},
        opcode_decoder::RegisterType,
    },
};

crate::define_instruction!(
    Print,
    "Prints a UTF-8 string from memory to standard output. Reads `length` bytes from `section_id` starting at `offset`, decodes as UTF-8, and prints the resulting string.",
    [
        (reg_section_id: RegisterType, "Register containing the heap section index"),
        (reg_offset: RegisterType, "Register containing the byte offset into the section"),
        (reg_length: RegisterType, "Register containing the number of bytes to read")
    ],
    [SideEffects],
    print_raw
);

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
