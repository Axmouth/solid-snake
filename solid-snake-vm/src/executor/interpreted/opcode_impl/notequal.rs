use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_notequal_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode handler>]);

            #[inline(always)]
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

                let result: u8 = (val1 != val2) as u8;

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_notequal_instruction!(NotEqualI8, i8);
impl_notequal_instruction!(NotEqualI16, i16);
impl_notequal_instruction!(NotEqualI32, i32);
impl_notequal_instruction!(NotEqualI64, i64);
impl_notequal_instruction!(NotEqualU8, u8);
impl_notequal_instruction!(NotEqualU16, u16);
impl_notequal_instruction!(NotEqualU32, u32);
impl_notequal_instruction!(NotEqualU64, u64);

macro_rules! impl_notequal_float_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode:snake handler>]);

            #[inline(always)]
            pub fn [<$opcode:snake handler>](
            executor: &mut VmInterpretedExecutor,
            args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (dest, reg1, reg2) = args;

                let val1: $ty = executor.registers().get_register_value(reg1)?;
                let val2: $ty = executor.registers().get_register_value(reg2)?;

                debug!(
                    "NotEqual: R{} <= {} R{} ({}) != R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 != val2) as u8;
                // Handle nan/inf?
                // ✅ In Rust, by IEEE-754 rules:
                // NaN != x is always false.
                // x != NaN is always false.
                // NaN != NaN is always false.
                // Thus "greater than or equal" instruction is already correct for floats without any special NaN handling unless we want to error on NaNs later?

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_notequal_float_instruction!(NotEqualF32, f32);
impl_notequal_float_instruction!(NotEqualF64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        notequal_basic_true,
        [
            (NotEqualU8, u8),
            (NotEqualU16, u16),
            (NotEqualU32, u32),
            (NotEqualU64, u64),
            (NotEqualI8, i8),
            (NotEqualI16, i16),
            (NotEqualI32, i32),
            (NotEqualI64, i64)
        ],
        VmTest::new()
            .setup(123 as T, R!(0))
            .setup(42 as T, R!(1))
            .expect(1u8, R!(2)), // 123 != 42 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        notequal_basic_false,
        [
            (NotEqualU8, u8),
            (NotEqualU16, u16),
            (NotEqualU32, u32),
            (NotEqualU64, u64),
            (NotEqualI8, i8),
            (NotEqualI16, i16),
            (NotEqualI32, i32),
            (NotEqualI64, i64)
        ],
        VmTest::new()
            .setup(123 as T, R!(0))
            .setup(123 as T, R!(1))
            .expect(0u8, R!(2)), // 123 != 123 -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        notequal_negative_true,
        [
            (NotEqualI8, i8),
            (NotEqualI16, i16),
            (NotEqualI32, i32),
            (NotEqualI64, i64)
        ],
        VmTest::new()
            .setup(-5 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(1u8, R!(2)), // -5 != 5 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        notequal_negative_false,
        [
            (NotEqualI8, i8),
            (NotEqualI16, i16),
            (NotEqualI32, i32),
            (NotEqualI64, i64)
        ],
        VmTest::new()
            .setup(-5 as T, R!(0))
            .setup(-5 as T, R!(1))
            .expect(0u8, R!(2)), // -5 != -5 -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        notequal_float_true,
        [(NotEqualF32, f32), (NotEqualF64, f64)],
        VmTest::new()
            .setup(1.618 as T, R!(0))
            .setup(3.0 as T, R!(1))
            .expect(1u8, R!(2)), // 1.618 != 3.0 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        notequal_float_false,
        [(NotEqualF32, f32), (NotEqualF64, f64)],
        VmTest::new()
            .setup(1.618 as T, R!(0))
            .setup(1.618 as T, R!(1))
            .expect(0u8, R!(2)), // 1.618 != 1.618 -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        notequal_float_nan,
        [(NotEqualF32, f32), (NotEqualF64, f64)],
        VmTest::new()
            .setup(f32::NAN as T, R!(0))
            .setup(f32::NAN as T, R!(1))
            .expect(1u8, R!(2)), // NaN != NaN -> true (IEEE)
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        notequal_float_inf_false,
        [(NotEqualF32, f32), (NotEqualF64, f64)],
        VmTest::new()
            .setup(f32::INFINITY as T, R!(0))
            .setup(f32::INFINITY as T, R!(1))
            .expect(0u8, R!(2)), // inf == inf -> false for !=
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        notequal_float_inf_true,
        [(NotEqualF32, f32), (NotEqualF64, f64)],
        VmTest::new()
            .setup(f32::INFINITY as T, R!(0))
            .setup(100.0 as T, R!(1))
            .expect(1u8, R!(2)), // inf != finite -> true
        (R!(2), R!(0), R!(1))
    );
}
