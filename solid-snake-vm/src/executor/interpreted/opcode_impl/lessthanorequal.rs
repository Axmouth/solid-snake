use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_lessthanorequal_instruction {
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
                    "LessThan: R{} <= {} R{} ({}) <= R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 <= val2) as u8;

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_lessthanorequal_instruction!(LessThanOrEqualI8, i8);
impl_lessthanorequal_instruction!(LessThanOrEqualI16, i16);
impl_lessthanorequal_instruction!(LessThanOrEqualI32, i32);
impl_lessthanorequal_instruction!(LessThanOrEqualI64, i64);
impl_lessthanorequal_instruction!(LessThanOrEqualU8, u8);
impl_lessthanorequal_instruction!(LessThanOrEqualU16, u16);
impl_lessthanorequal_instruction!(LessThanOrEqualU32, u32);
impl_lessthanorequal_instruction!(LessThanOrEqualU64, u64);

macro_rules! impl_lessthan_float_instruction {
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
                    "LessThanOrEqual: R{} <= {} R{} ({}) <= R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 <= val2) as u8;
                // Handle nan/inf?
                // ✅ In Rust, by IEEE-754 rules:
                // NaN <= x is always false.
                // x <= NaN is always false.
                // NaN <= NaN is always false.
                // Thus "less than or equal" instruction is already correct for floats without any special NaN handling unless we want to error on NaNs later?

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_lessthan_float_instruction!(LessThanOrEqualF32, f32);
impl_lessthan_float_instruction!(LessThanOrEqualF64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::interpreted::opcode_impl::all::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        lessthanorequal_basic,
        [
            (LessThanOrEqualU8, u8),
            (LessThanOrEqualU16, u16),
            (LessThanOrEqualU32, u32),
            (LessThanOrEqualU64, u64),
            (LessThanOrEqualI8, i8),
            (LessThanOrEqualI16, i16),
            (LessThanOrEqualI32, i32),
            (LessThanOrEqualI64, i64)
        ],
        VmTest::new()
            .setup(5 as T, R!(0))
            .setup(10 as T, R!(1))
            .expect(1u8, R!(2)), // 5 <= 10 -> true (1)
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthanorequal_equal,
        [
            (LessThanOrEqualU8, u8),
            (LessThanOrEqualU16, u16),
            (LessThanOrEqualU32, u32),
            (LessThanOrEqualU64, u64),
            (LessThanOrEqualI8, i8),
            (LessThanOrEqualI16, i16),
            (LessThanOrEqualI32, i32),
            (LessThanOrEqualI64, i64)
        ],
        VmTest::new()
            .setup(42 as T, R!(0))
            .setup(42 as T, R!(1))
            .expect(1u8, R!(2)), // 42 <= 42 -> true (1)
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthanorequal_false,
        [
            (LessThanOrEqualU8, u8),
            (LessThanOrEqualU16, u16),
            (LessThanOrEqualU32, u32),
            (LessThanOrEqualU64, u64),
            (LessThanOrEqualI8, i8),
            (LessThanOrEqualI16, i16),
            (LessThanOrEqualI32, i32),
            (LessThanOrEqualI64, i64)
        ],
        VmTest::new()
            .setup(50 as T, R!(0))
            .setup(42 as T, R!(1))
            .expect(0u8, R!(2)), // 50 <= 42 -> false (0)
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthanorequal_negative,
        [
            (LessThanOrEqualI8, i8),
            (LessThanOrEqualI16, i16),
            (LessThanOrEqualI32, i32),
            (LessThanOrEqualI64, i64)
        ],
        VmTest::new()
            .setup(-10 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(1u8, R!(2)), // -10 <= 5 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthanorequal_negative_equal,
        [
            (LessThanOrEqualI8, i8),
            (LessThanOrEqualI16, i16),
            (LessThanOrEqualI32, i32),
            (LessThanOrEqualI64, i64)
        ],
        VmTest::new()
            .setup(-5 as T, R!(0))
            .setup(-5 as T, R!(1))
            .expect(1u8, R!(2)), // -5 <= -5 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthanorequal_floats_basic,
        [(LessThanOrEqualF32, f32), (LessThanOrEqualF64, f64)],
        VmTest::new()
            .setup(1.5 as T, R!(0))
            .setup(2.5 as T, R!(1))
            .expect(1u8, R!(2)), // 1.5 <= 2.5 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthanorequal_floats_equal,
        [(LessThanOrEqualF32, f32), (LessThanOrEqualF64, f64)],
        VmTest::new()
            .setup(1.618 as T, R!(0))
            .setup(1.618 as T, R!(1))
            .expect(1u8, R!(2)), // 1.618 <= 1.618 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthanorequal_floats_false,
        [(LessThanOrEqualF32, f32), (LessThanOrEqualF64, f64)],
        VmTest::new()
            .setup(5.0 as T, R!(0))
            .setup(1.0 as T, R!(1))
            .expect(0u8, R!(2)), // 5.0 <= 1.0 -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthanorequal_floats_nan,
        [(LessThanOrEqualF32, f32), (LessThanOrEqualF64, f64)],
        VmTest::new()
            .setup(f32::NAN as T, R!(0))
            .setup(5.0 as T, R!(1))
            .expect(0u8, R!(2)), // NaN comparison -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        lessthanorequal_floats_inf,
        [(LessThanOrEqualF32, f32), (LessThanOrEqualF64, f64)],
        VmTest::new()
            .setup(5.0 as T, R!(0))
            .setup(f32::INFINITY as T, R!(1))
            .expect(1u8, R!(2)), // 5.0 <= inf -> true
        (R!(2), R!(0), R!(1))
    );
}
