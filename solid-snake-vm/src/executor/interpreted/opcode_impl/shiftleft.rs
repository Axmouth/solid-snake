use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_shift_left_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                (RegisterType, RegisterType, RegisterType),
                [<$opcode handler>]
            );

            #[inline(always)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (dest, val_reg, shift_reg) = args;
                let value: $ty = executor.registers().get_register_value(val_reg)?;
                let shift: u32 = executor.registers().get_register_value(shift_reg)?; // safe default shift size

                let result = value.wrapping_shl(shift);

                debug!(
                    "ShiftLeft: R{} <= {} R{} ({}) << R{} ({}) = {}",
                    dest,
                    stringify!($ty),
                    val_reg,
                    value,
                    shift_reg,
                    shift,
                    result
                );

                executor.registers_mut().set_register_value(dest, result)?;
                Ok(())
            }
        }
    };
}

impl_shift_left_instruction!(ShiftLeftU8, u8);
impl_shift_left_instruction!(ShiftLeftU16, u16);
impl_shift_left_instruction!(ShiftLeftU32, u32);
impl_shift_left_instruction!(ShiftLeftU64, u64);
impl_shift_left_instruction!(ShiftLeftI8, i8);
impl_shift_left_instruction!(ShiftLeftI16, i16);
impl_shift_left_instruction!(ShiftLeftI32, i32);
impl_shift_left_instruction!(ShiftLeftI64, i64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        shift_left_unsigned,
        [
            (ShiftLeftU8, u8),
            (ShiftLeftU16, u16),
            (ShiftLeftU32, u32),
            (ShiftLeftU64, u64)
        ],
        VmTest::new()
            .setup(0b00001111 as T, R!(0))
            .setup(2 as T, R!(1)) // Shift by 2
            .expect(0b00111100 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        shift_left_signed,
        [
            (ShiftLeftI8, i8),
            (ShiftLeftI16, i16),
            (ShiftLeftI32, i32),
            (ShiftLeftI64, i64)
        ],
        VmTest::new()
            .setup(0b00000011 as T, R!(0))
            .setup(3 as T, R!(1)) // Shift by 3
            .expect((0b00000011 as T) << 3, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        shift_left_zero,
        [(ShiftLeftU8, u8)],
        VmTest::new()
            .setup(0 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(0 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        shift_left_by_zero,
        [(ShiftLeftU8, u8)],
        VmTest::new()
            .setup(0b11110000 as T, R!(0))
            .setup(0 as T, R!(1))
            .expect(0b11110000 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        shift_left_aliasing_output_input1,
        [(ShiftLeftU8, u8)],
        VmTest::new()
            .setup(0b00001111 as T, R!(0))
            .setup(2 as T, R!(1))
            .expect(0b00111100 as T, R!(0)),
        (R!(0), R!(0), R!(1))
    );

    define_vm_tests!(
        shift_left_aliasing_output_input2,
        [(ShiftLeftI16, i16)],
        VmTest::new()
            .setup(3 as T, R!(0))
            .setup(1 as T, R!(1))
            .expect((3 as T) << 1, R!(1)),
        (R!(1), R!(0), R!(1))
    );

    define_vm_tests!(
        shift_left_same_registers,
        [(ShiftLeftU8, u8)],
        VmTest::new()
            .setup(0b00000001 as T, R!(0))
            .expect(0b00000010 as T, R!(0)),
        (R!(0), R!(0), R!(0))
    );
}
