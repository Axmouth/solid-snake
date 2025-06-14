use log::debug;

use crate::{
    define_instruction,
    executor::{
        ext::VmExecutionError,
        interpreted::implimentation::{CallFrame, INITIAL_FRAMES_CAPACITY, VmInterpretedExecutor},
    },
};

define_instruction!(
    CallFunction,
    "Calls a function at the specified bytecode address. Saves the return address and switches stack frame.",
    [
        (target: u64, "Bytecode address (offset) to jump to for the function")
    ],
    [ControlFlow, SideEffects],
    callfunction
);

define_instruction!(
    Return,
    "Returns from the current function by restoring the previous frame and program counter.",
    [],
    [ControlFlow, SideEffects],
    funcreturn
);

define_instruction!(
    Halt,
    "Halts execution of the virtual machine immediately.",
    [(exit_code: i64, "Exit code for the VM (default is 0)")],
    [ControlFlow, SideEffects],
    halt
);


pub const FRAME_ALLOCATION_CHUNK: usize = usize::pow(2, 16);

#[inline(always)]
pub fn callfunction(
    executor: &mut VmInterpretedExecutor,
    args: CallFunctionArgs,
) -> Result<(), VmExecutionError> {
    let (target,) = args;

    debug!("CallFunction: To {target}");

    if executor.frame_stack.len() >= executor.get_max_stack_depth() {
        return Err(VmExecutionError::StackOverflow);
    }

    let old_stack_top = executor.stack_top;

    executor.stack_top += 1;

    if executor.stack_top >= executor.frame_stack.len() - 1 {
        executor.frame_stack.resize(
            executor.frame_stack.len() + FRAME_ALLOCATION_CHUNK,
            CallFrame::new(),
        );
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
    _: ReturnArgs,
) -> Result<(), VmExecutionError> {
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

    if executor.stack_top + FRAME_ALLOCATION_CHUNK * 2 <= executor.frame_stack.len()
        && executor.frame_stack.len() > INITIAL_FRAMES_CAPACITY + FRAME_ALLOCATION_CHUNK * 2
    {
        executor
            .frame_stack
            .resize(executor.frame_stack.len() - 512, CallFrame::new());
    }

    let (lower, upper) = executor.frame_stack.split_at_mut(old_stack_top); // [0, 5) - [5, len] | [0, 1) - [1, len]
    let new_frame = &mut lower[old_stack_top - 1]; // 0
    let old_frame = &upper[0]; // 1

    // Copy return value(s)
    new_frame.registers.raw[0] = old_frame.registers.raw[0];

    let return_target = executor.frame_stack[old_stack_top].return_address as usize;

    executor.set_program_counter(return_target)?;

    Ok(())
}

#[inline(always)]
pub fn halt(
    _executor: &mut VmInterpretedExecutor,
    args: HaltArgs,
) -> Result<(), VmExecutionError> {
    let (_exit_code,) = args;
    debug!("Halt");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{R, define_vm_program_test, executor::ext::VmExecutionError};

    #[test]
    fn function_call_and_return() {
        use crate::executor::interpreted::opcode_impl::all::*;

        crate::asm_internal::VmProgramTest::new()
            .with_program(vec![
                LoadImmediateI64Instruction::encode((R!(1), 5i64)), // R1 = 5
                CallFunctionInstruction::encode((31u64,)),          // Call function at offset 31
                HaltInstruction::encode((0,)),
                // function "func" at offset 31
                IncrementU8Instruction::encode((R!(1), 1u8)), // R1 += 1 --> 6
                MoveI64Instruction::encode((R!(0), R!(1))),   // R0 = R1
                ReturnInstruction::encode(()),
            ])
            .expect_register(R!(0), 6i64)
            .run()
            .unwrap();
    }

    #[test]
    fn stack_underflow() {
        use crate::executor::interpreted::opcode_impl::all::*;

        let err = crate::asm_internal::VmProgramTest::new()
            .with_program(vec![
                ReturnInstruction::encode(()), // Return immediately, no call
            ])
            .run()
            .unwrap_err();
        assert!(matches!(err, VmExecutionError::StackUnderflow))
    }

    #[test]
    fn stack_overflow_manual() {
        use crate::asm_internal::VmProgramTest;
        use crate::executor::interpreted::opcode_impl::all::*;

        let mut test = VmProgramTest::new();
        let vm = test.vm_mut();

        // Artificially make frame stack max out
        vm.frame_stack.resize(
            vm.get_max_stack_depth(),
            crate::executor::interpreted::implimentation::CallFrame::new(),
        );
        vm.stack_top = vm.frame_stack.len() - 1;

        let err = test
            .with_program(vec![
                CallFunctionInstruction::encode((0u64,)), // Any address, doesn't matter
            ])
            .run()
            .unwrap_err();

        assert!(matches!(err, VmExecutionError::StackOverflow))
    }

    define_vm_program_test!(
        stack_underflow_test,
        program => [
            ReturnInstruction::encode(())
        ],
        expect_fail => VmExecutionError::StackUnderflow,
    );
}
