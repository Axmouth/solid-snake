use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_lessthan_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                concat!("Compares two ", stringify!($ty), " registers. Sets 1 if the first is less than the second, else 0."),
                [
                    (dest: RegisterType, "Destination register for result (1 or 0)"),
                    (reg1: RegisterType, "First operand (left-hand side)"),
                    (reg2: RegisterType, "Second operand (right-hand side)")
                ],
                [Logical, Pure],
                [<$opcode handler>]
            );

            #[inline(always)]
            #[allow(non_snake_case)]
            pub fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (dest, reg1, reg2) = args;

                let val1: $ty = executor.registers().get_register_value(reg1)?;
                let val2: $ty = executor.registers().get_register_value(reg2)?;

                debug!(
                    "LessThan: R{} <= {} R{} ({}) < R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 < val2) as u8;

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_lessthan_instruction!(LessThanI8, i8);
impl_lessthan_instruction!(LessThanI16, i16);
impl_lessthan_instruction!(LessThanI32, i32);
impl_lessthan_instruction!(LessThanI64, i64);
impl_lessthan_instruction!(LessThanU8, u8);
impl_lessthan_instruction!(LessThanU16, u16);
impl_lessthan_instruction!(LessThanU32, u32);
impl_lessthan_instruction!(LessThanU64, u64);

macro_rules! impl_lessthan_float_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                concat!("Compares two ", stringify!($ty), " floating-point registers. Sets 1 if the first is less than the second, else 0."),
                [
                    (dest: RegisterType, "Destination register for result (1 or 0)"),
                    (reg1: RegisterType, "First operand (left-hand side)"),
                    (reg2: RegisterType, "Second operand (right-hand side)")
                ],
                [Logical, Pure],
                [<$opcode:snake handler>]
            );

            #[inline(always)]
            #[allow(non_snake_case)]
            pub fn [<$opcode:snake handler>](
            executor: &mut VmInterpretedExecutor,
            args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (dest, reg1, reg2) = args;

                let val1: $ty = executor.registers().get_register_value(reg1)?;
                let val2: $ty = executor.registers().get_register_value(reg2)?;

                debug!(
                    "LessThan: R{} <= {} R{} ({}) < R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 < val2) as u8;
                // Handle nan/inf?
                // ✅ In Rust, by IEEE-754 rules:
                // NaN < x is always false.
                // x < NaN is always false.
                // NaN < NaN is always false.
                // Thus "less than" instruction is already correct for floats without any special NaN handling unless we want to error on NaNs later?

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_lessthan_float_instruction!(LessThanF32, f32);
impl_lessthan_float_instruction!(LessThanF64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        lessthan_basic,
        [
            (LessThanU8, u8),
            (LessThanU16, u16),
            (LessThanU32, u32),
            (LessThanU64, u64),
            (LessThanI8, i8),
            (LessThanI16, i16),
            (LessThanI32, i32),
            (LessThanI64, i64)
        ],
        VmTest::new()
            .setup(5 as T, R!(0))
            .setup(10 as T, R!(1))
            .expect(1u8, R!(2)), // 5 < 10 -> true (1)
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthan_equal,
        [
            (LessThanU8, u8),
            (LessThanU16, u16),
            (LessThanU32, u32),
            (LessThanU64, u64),
            (LessThanI8, i8),
            (LessThanI16, i16),
            (LessThanI32, i32),
            (LessThanI64, i64)
        ],
        VmTest::new()
            .setup(42 as T, R!(0))
            .setup(42 as T, R!(1))
            .expect(0u8, R!(2)), // 42 < 42 -> false (0)
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthan_negative,
        [
            (LessThanI8, i8),
            (LessThanI16, i16),
            (LessThanI32, i32),
            (LessThanI64, i64)
        ],
        VmTest::new()
            .setup(-10 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(1u8, R!(2)), // -10 < 5 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthan_negative_false,
        [
            (LessThanI8, i8),
            (LessThanI16, i16),
            (LessThanI32, i32),
            (LessThanI64, i64)
        ],
        VmTest::new()
            .setup(10 as T, R!(0))
            .setup(-5 as T, R!(1))
            .expect(0u8, R!(2)), // 10 < -5 -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthan_floats_basic,
        [(LessThanF32, f32), (LessThanF64, f64)],
        VmTest::new()
            .setup(1.5 as T, R!(0))
            .setup(2.5 as T, R!(1))
            .expect(1u8, R!(2)), // 1.5 < 2.5 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthan_floats_equal,
        [(LessThanF32, f32), (LessThanF64, f64)],
        VmTest::new()
            .setup(1.618 as T, R!(0))
            .setup(1.618 as T, R!(1))
            .expect(0u8, R!(2)), // 1.618 < 1.618 -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthan_floats_nan,
        [(LessThanF32, f32), (LessThanF64, f64)],
        VmTest::new()
            .setup(f32::NAN as T, R!(0))
            .setup(5.0 as T, R!(1))
            .expect(0u8, R!(2)), // NaN compared -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthan_floats_inf,
        [(LessThanF32, f32), (LessThanF64, f64)],
        VmTest::new()
            .setup(1.0 as T, R!(0))
            .setup(f32::INFINITY as T, R!(1))
            .expect(1u8, R!(2)), // 1.0 < inf -> true
        (R!(2), R!(0), R!(1))
    );
}
