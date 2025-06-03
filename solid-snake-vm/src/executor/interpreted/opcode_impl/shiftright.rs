use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_shift_right_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                (RegisterType, RegisterType, RegisterType),
                [<$opcode handler>]
            );

            #[inline(always)]
            #[allow(non_snake_case)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (dest, val_reg, shift_reg) = args;
                let value: $ty = executor.registers().get_register_value(val_reg)?;
                let shift: u32 = executor.registers().get_register_value(shift_reg)?;

                let result = value.wrapping_shr(shift);

                debug!(
                    "ShiftRight: R{} <= {} R{} ({}) >> R{} ({}) = {}",
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

impl_shift_right_instruction!(ShiftRightU8, u8);
impl_shift_right_instruction!(ShiftRightU16, u16);
impl_shift_right_instruction!(ShiftRightU32, u32);
impl_shift_right_instruction!(ShiftRightU64, u64);
impl_shift_right_instruction!(ShiftRightI8, i8);
impl_shift_right_instruction!(ShiftRightI16, i16);
impl_shift_right_instruction!(ShiftRightI32, i32);
impl_shift_right_instruction!(ShiftRightI64, i64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        shift_right_unsigned,
        [
            (ShiftRightU8, u8),
            (ShiftRightU16, u16),
            (ShiftRightU32, u32),
            (ShiftRightU64, u64)
        ],
        VmTest::new()
            .setup(240 as T, R!(0)) // 0b11110000
            .setup(2 as T, R!(1)) // shift by 2
            .expect(60 as T, R!(2)), // 0b00111100
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        shift_right_signed,
        [
            (ShiftRightI8, i8),
            (ShiftRightI16, i16),
            (ShiftRightI32, i32),
            (ShiftRightI64, i64)
        ],
        VmTest::new()
            .setup(-8 as T, R!(0)) // 0b11111000 (sign-extended)
            .setup(2 as T, R!(1))
            .expect((-8 as T).wrapping_shr(2), R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        shift_right_zero,
        [(ShiftRightU8, u8)],
        VmTest::new()
            .setup(0 as T, R!(0))
            .setup(3 as T, R!(1))
            .expect(0 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        shift_right_by_zero,
        [(ShiftRightU8, u8)],
        VmTest::new()
            .setup(0b11110000 as T, R!(0))
            .setup(0 as T, R!(1))
            .expect(0b11110000 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        shift_right_aliasing_output_input1,
        [(ShiftRightU8, u8)],
        VmTest::new()
            .setup(0b10000000 as T, R!(0)) // 128
            .setup(3 as T, R!(1))
            .expect((128 as T) >> 3, R!(0)), // 16
        (R!(0), R!(0), R!(1))
    );

    define_vm_tests!(
        shift_right_aliasing_output_input2,
        [(ShiftRightI16, i16)],
        VmTest::new()
            .setup(64 as T, R!(0))
            .setup(2 as T, R!(1))
            .expect((64 as T) >> 2, R!(1)), // 16
        (R!(1), R!(0), R!(1))
    );

    define_vm_tests!(
        shift_right_same_registers,
        [(ShiftRightU8, u8)],
        VmTest::new()
            .setup(16 as T, R!(0)) // 0b00010000
            .setup(1 as T, R!(1))
            .expect(8 as T, R!(0)),
        (R!(0), R!(0), R!(1))
    );
}
