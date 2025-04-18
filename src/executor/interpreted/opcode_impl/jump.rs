use log::debug;

use crate::{
    define_instruction,
    executor::{
        ext::VmExecutionError,
        interpreted::{
            implimentation::{RegisterFileExt, VmInterpretedExecutor},
            opcode_decoder::RegisterType,
        },
    },
};

define_instruction!(Jump, (u64), jump);
define_instruction!(JumpIf, (u64, RegisterType), jumpif);
define_instruction!(JumpIfFalse, (u64, RegisterType), jumpiffalse);

#[inline(always)]
pub fn jump(
    executor: &mut VmInterpretedExecutor,
    look_ahead: &[u8],
) -> Result<(), VmExecutionError> {
    assert_eq!(look_ahead.len(), JumpInstruction::arg_size());
    let (target,) = JumpInstruction::parse_args(look_ahead);

    debug!("Jump: To {target}");

    executor.set_program_counter(target as usize)?;

    Ok(())
}

#[inline(always)]
pub fn jumpif(
    executor: &mut VmInterpretedExecutor,
    look_ahead: &[u8],
) -> Result<(), VmExecutionError> {
    assert_eq!(look_ahead.len(), JumpIfInstruction::arg_size());
    let (target, reg_val) = JumpIfInstruction::parse_args(look_ahead);

    let val: u8 = executor.registers().get_register_value(reg_val)?;

    debug!("Jump: To {target} if u8 R{reg_val} ({val}) is true");

    if val != 0 {
        executor.set_program_counter(target as usize)?;
    }

    Ok(())
}

#[inline(always)]
pub fn jumpiffalse(
    executor: &mut VmInterpretedExecutor,
    look_ahead: &[u8],
) -> Result<(), VmExecutionError> {
    assert_eq!(look_ahead.len(), JumpIfFalseInstruction::arg_size());
    let (target, reg_val) = JumpIfFalseInstruction::parse_args(look_ahead);

    let val: u8 = executor.registers().get_register_value(reg_val)?;

    debug!("Jump: To {target} if u8 R{reg_val} ({val}) is false");

    if val == 0 {
        executor.set_program_counter(target as usize)?;
    }

    Ok(())
}
