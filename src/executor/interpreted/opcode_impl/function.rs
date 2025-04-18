use log::debug;

use crate::{
    define_instruction,
    executor::{
        ext::VmExecutionError,
        interpreted::implimentation::{CallFrame, VmInterpretedExecutor},
    },
};

define_instruction!(CallFunction, (u64), callfunction);
define_instruction!(Return, (), funcreturn);
define_instruction!(Halt, (), halt);

#[inline(always)]
pub fn callfunction(
    executor: &mut VmInterpretedExecutor,
    look_ahead: &[u8],
) -> Result<(), VmExecutionError> {
    assert_eq!(look_ahead.len(), CallFunctionInstruction::arg_size());
    let (target,) = CallFunctionInstruction::parse_args(look_ahead);

    debug!("CallFunction: To {target}");

    if executor.frame_stack.len() == usize::MAX {
        return Err(VmExecutionError::StackOverflow);
    }

    let old_stack_top = executor.stack_top;

    executor.stack_top += 1;

    if executor.stack_top >= executor.frame_stack.len() - 1 {
        executor
            .frame_stack
            .resize(executor.frame_stack.len() + 256, CallFrame::new());
    }

    executor.frame_stack[executor.stack_top].return_address =
        executor.get_program_counter()? as u64;

    let (lower, upper) = executor.frame_stack.split_at_mut(executor.stack_top);
    let old_frame = &lower[old_stack_top];
    let new_frame = &mut upper[0];

    // Copy return argument(s)
    new_frame.registers.raw[1..4].copy_from_slice(&old_frame.registers.raw[1..4]);

    executor.set_program_counter(target as usize)?;

    Ok(())
}

#[inline(always)]
pub fn funcreturn(
    executor: &mut VmInterpretedExecutor,
    look_ahead: &[u8],
) -> Result<(), VmExecutionError> {
    assert_eq!(look_ahead.len(), ReturnInstruction::arg_size());

    if executor.stack_top == 0 {
        return Err(VmExecutionError::StackUnderflow);
    }

    debug!(
        "Return: From {} to {}",
        executor.get_program_counter()?,
        executor.frame_stack[executor.stack_top].return_address
    );

    let old_stack_top = executor.stack_top; // 5 | 1
    executor.stack_top -= 1; // 4 | 0

    if executor.stack_top + 1024 <= executor.frame_stack.len() {
        executor
            .frame_stack
            .resize(executor.frame_stack.len() - 512, CallFrame::new());
    }

    let (lower, upper) = executor.frame_stack.split_at_mut(old_stack_top); // [0, 5) - [5, len] | [0, 1) - [1, len]
    let new_frame = &mut lower[old_stack_top - 1]; // 0
    let old_frame = &upper[0]; // 1

    // Copy return value(s)
    new_frame.registers.raw[0] = old_frame.registers.raw[0];

    executor.set_program_counter(executor.frame_stack[old_stack_top].return_address as usize)?;

    Ok(())
}

#[inline(always)]
pub fn halt(
    _executor: &mut VmInterpretedExecutor,
    look_ahead: &[u8],
) -> Result<(), VmExecutionError> {
    assert_eq!(look_ahead.len(), ReturnInstruction::arg_size());

    debug!("Halt");

    Ok(())
}
