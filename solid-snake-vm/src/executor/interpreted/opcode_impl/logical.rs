use log::debug;

use crate::define_instruction;
use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_logical_binop {
    ($name:ident, $opcode:ident, $op:tt) => {
        $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), $name);

        #[inline(always)]
        fn $name(
            executor: &mut VmInterpretedExecutor,
            args: ($crate::executor::interpreted::opcode_decoder::RegisterType,
                   $crate::executor::interpreted::opcode_decoder::RegisterType,
                   $crate::executor::interpreted::opcode_decoder::RegisterType),
        ) -> Result<(), VmExecutionError> {
            let (dest, reg1, reg2) = args;
            let v1: u64 = executor.registers().get_register_value(reg1)?;
            let v2: u64 = executor.registers().get_register_value(reg2)?;
            let r = ((v1 != 0) $op (v2 != 0)) as u8;


            debug!(
                "{}: R{} <= R{} ({}) {} R{} ({}) = {}",
                stringify!($name),
                dest,
                reg1,
                v1,
                stringify!($op),
                reg2,
                v2,
                r
            );

            executor.registers_mut().set_register_value(dest, r)?;
            Ok(())
        }
    };
}

define_instruction!(LogicalNot, (RegisterType, RegisterType), logical_not);

#[inline(always)]
fn logical_not(
    executor: &mut VmInterpretedExecutor,
    args: (RegisterType, RegisterType),
) -> Result<(), VmExecutionError> {
    let (dest, source) = args;
    let v: u64 = executor.registers().get_register_value(source)?;
    let result = (v == 0) as u8;

    debug!(
        "LogicalNot: R{} <= ! R{} ({}) = {}",
        dest, source, v, result
    );

    executor.registers_mut().set_register_value(dest, result)?;
    Ok(())
}

impl_logical_binop!(logical_and, LogicalAnd, &&);
impl_logical_binop!(logical_or, LogicalOr, ||);
impl_logical_binop!(logical_xor, LogicalXor, ^);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        logical_and,
        [(LogicalAnd, u8)],
        VmTest::new()
            .setup(1u64, R!(0))
            .setup(1u64, R!(1))
            .expect(1u8, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        logical_and_false,
        [(LogicalAnd, u8)],
        VmTest::new()
            .setup(0u64, R!(0))
            .setup(42u64, R!(1))
            .expect(0u8, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        logical_or,
        [(LogicalOr, u8)],
        VmTest::new()
            .setup(0u64, R!(0))
            .setup(99u64, R!(1))
            .expect(1u8, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        logical_or_false,
        [(LogicalOr, u8)],
        VmTest::new()
            .setup(0u64, R!(0))
            .setup(0u64, R!(1))
            .expect(0u8, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        logical_xor_true,
        [(LogicalXor, u8)],
        VmTest::new()
            .setup(1u64, R!(0))
            .setup(0u64, R!(1))
            .expect(1u8, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        logical_xor_false,
        [(LogicalXor, u8)],
        VmTest::new()
            .setup(1u64, R!(0))
            .setup(1u64, R!(1))
            .expect(0u8, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        logical_not_true,
        [(LogicalNot, u8)],
        VmTest::new().setup(1u64, R!(1)).expect(0u8, R!(0)),
        (R!(0), R!(1))
    );

    define_vm_tests!(
        logical_not_false,
        [(LogicalNot, u8)],
        VmTest::new().setup(0u64, R!(1)).expect(1u8, R!(0)),
        (R!(0), R!(1))
    );

    define_vm_tests!(
        logical_aliasing,
        [(LogicalOr, u8)],
        VmTest::new().setup(1u64, R!(0)).expect(1u8, R!(0)),
        (R!(0), R!(0), R!(0))
    );
}
