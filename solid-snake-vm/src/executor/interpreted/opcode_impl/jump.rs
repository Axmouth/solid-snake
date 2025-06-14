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

define_instruction!(
    Jump,
    "Unconditionally jumps to the specified bytecode address.",
    [
        (target: u64, "Bytecode address(byte offset) to jump to")
    ], [ControlFlow, SideEffects],
    jump);
define_instruction!(
    JumpIf,
    "Jumps to the target address if the register is non-zero (true).",
    [
        (target: u64, "Bytecode address(byte offset) to jump to"),
        (reg: RegisterType, "Register to check")
        ],
    [ControlFlow, SideEffects],
    jumpif);
define_instruction!(
    JumpIfFalse,
    "Jumps to the target address if the register is zero (false).",
    [
        (target: u64, "Bytecode address(byte offset) to jump to"),
        (reg: RegisterType, "Register to check")
        ],
    [ControlFlow, SideEffects],
    jumpiffalse);

#[inline(always)]
pub fn jump(executor: &mut VmInterpretedExecutor, args: JumpArgs) -> Result<(), VmExecutionError> {
    let (target,) = args;

    debug!("Jump: To {target}");

    executor.set_program_counter(target as usize)?;

    Ok(())
}

#[inline(always)]
pub fn jumpif(
    executor: &mut VmInterpretedExecutor,
    args: JumpIfArgs,
) -> Result<(), VmExecutionError> {
    let (target, reg_val) = args;

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
    args: JumpIfFalseArgs,
) -> Result<(), VmExecutionError> {
    let (target, reg_val) = args;

    let val: u8 = executor.registers().get_register_value(reg_val)?;

    debug!("Jump: To {target} if u8 R{reg_val} ({val}) is false");

    if val == 0 {
        executor.set_program_counter(target as usize)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::R;

    #[test]
    fn jump_unconditional() {
        use crate::executor::interpreted::opcode_impl::all::*;

        crate::asm_internal::VmProgramTest::new()
            .with_program(vec![
                JumpInstruction::encode((14u64,)), // Jump forward to Halt
                IncrementU8Instruction::encode((R!(0), 1u8)), // <-- skipped
                HaltInstruction::encode(()),
            ])
            .expect_pc(2)
            .expect_register(R!(0), 0u8)
            .run()
            .unwrap();
    }

    #[test]
    fn jump_if_not_taken() {
        use crate::executor::interpreted::opcode_impl::all::*;

        crate::asm_internal::VmProgramTest::new()
            .setup_register(0u8, R!(0)) // false
            .setup_register(0u8, R!(1))
            .with_program(vec![
                JumpIfInstruction::encode((0u64, R!(0))),     // No jump
                IncrementU8Instruction::encode((R!(1), 1u8)), // executed
                HaltInstruction::encode(()),
            ])
            .expect_pc(2) // JumpIf + Incr
            .expect_register(R!(1), 1u8)
            .run()
            .unwrap();
    }

    #[test]
    fn jump_if_taken() {
        use crate::executor::interpreted::opcode_impl::all::*;

        crate::asm_internal::VmProgramTest::new()
            .setup_register(1u8, R!(0)) // true
            .setup_register(0u8, R!(1))
            .with_program(vec![
                JumpIfInstruction::encode((15u64, R!(0))),    // No jump
                IncrementU8Instruction::encode((R!(1), 1u8)), // skipped
                HaltInstruction::encode(()),
            ])
            .expect_pc(2) // JumpIf + Incr
            .expect_register(R!(1), 0u8)
            .run()
            .unwrap();
    }

    #[test]
    fn jump_if_false_not_taken() {
        use crate::executor::interpreted::opcode_impl::all::*;

        crate::asm_internal::VmProgramTest::new()
            .setup_register(1u8, R!(0)) // true
            .setup_register(0u8, R!(1))
            .with_program(vec![
                JumpIfFalseInstruction::encode((0u64, R!(0))), // No jump
                IncrementU8Instruction::encode((R!(1), 1u8)),  // executed
                HaltInstruction::encode(()),
            ])
            .expect_pc(2) // JumpIf + Incr
            .expect_register(R!(1), 1u8)
            .run()
            .unwrap();
    }

    #[test]
    fn jump_if_false_taken() {
        use crate::executor::interpreted::opcode_impl::all::*;

        crate::asm_internal::VmProgramTest::new()
            .setup_register(0u8, R!(0)) // false
            .setup_register(0u8, R!(1))
            .with_program(vec![
                JumpIfFalseInstruction::encode((15u64, R!(0))), // No jump
                IncrementU8Instruction::encode((R!(1), 1u8)),   // skipped
                HaltInstruction::encode(()),
            ])
            .expect_pc(2) // JumpIf + Incr
            .expect_register(R!(1), 0u8)
            .run()
            .unwrap();
    }
}
