use log::debug;

use crate::{
    define_instruction,
    executor::{
        ext::VmExecutionError,
        interpreted::{
            implimentation::{RegisterFileExt, VmInterpretedExecutor},
            opcode_decoder::RegisterType,
        },
    }, R,
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

#[test]
fn jump_unconditional() {
    use crate::executor::interpreted::opcode_impl::all::*;

    crate::asm_internal::VmProgramTest::new()
        .with_program(vec![
            JumpInstruction::encode((14u64,)), // Jump forward to Halt
            IncrementU8Instruction::encode((R!(0), 1u8)), // <-- skipped
            HaltInstruction::encode(()),
        ])
        .expect_pc(14)
        .expect_register(R!(0), 0u8)
        .run();
}

#[test]
fn jump_if_not_taken() {
    use crate::executor::interpreted::opcode_impl::all::*;

    crate::asm_internal::VmProgramTest::new()
        .setup_register(0u8, R!(0)) // false
        .setup_register(0u8, R!(1))
        .with_program(vec![
            JumpIfInstruction::encode((64u64, R!(0))), // No jump
            IncrementU8Instruction::encode((R!(1), 1u8)), // executed
            HaltInstruction::encode(()),
        ])
        .expect_pc(15) // JumpIf + Incr
        .expect_register(R!(1), 1u8)
        .run();
}

#[test]
fn jump_if_taken() {
    use crate::executor::interpreted::opcode_impl::all::*;

    crate::asm_internal::VmProgramTest::new()
        .setup_register(1u8, R!(0)) // true
        .setup_register(0u8, R!(1))
        .with_program(vec![
            JumpIfInstruction::encode((15u64, R!(0))), // No jump
            IncrementU8Instruction::encode((R!(1), 1u8)), // skipped
            HaltInstruction::encode(()),
        ])
        .expect_pc(15) // JumpIf + Incr
        .expect_register(R!(1), 0u8)
        .run();
}

#[test]
fn jump_if_false_not_taken() {
    use crate::executor::interpreted::opcode_impl::all::*;

    crate::asm_internal::VmProgramTest::new()
        .setup_register(1u8, R!(0)) // true
        .setup_register(0u8, R!(1))
        .with_program(vec![
            JumpIfFalseInstruction::encode((64u64, R!(0))), // No jump
            IncrementU8Instruction::encode((R!(1), 1u8)), // executed
            HaltInstruction::encode(()),
        ])
        .expect_pc(15) // JumpIf + Incr
        .expect_register(R!(1), 1u8)
        .run();
}

#[test]
fn jump_if_false_taken() {
    use crate::executor::interpreted::opcode_impl::all::*;

    crate::asm_internal::VmProgramTest::new()
        .setup_register(0u8, R!(0)) // false
        .setup_register(0u8, R!(1))
        .with_program(vec![
            JumpIfFalseInstruction::encode((15u64, R!(0))), // No jump
            IncrementU8Instruction::encode((R!(1), 1u8)), // skipped
            HaltInstruction::encode(()),
        ])
        .expect_pc(15) // JumpIf + Incr
        .expect_register(R!(1), 0u8)
        .run();
}
