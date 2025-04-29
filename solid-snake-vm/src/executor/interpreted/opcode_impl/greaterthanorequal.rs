use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_greaterthanorequal_instruction {
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
                    "GreaterThanOrEqual: R{} <= {} R{} ({}) >= R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 >= val2) as u8;

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_greaterthanorequal_instruction!(GreaterThanOrEqualI8, i8);
impl_greaterthanorequal_instruction!(GreaterThanOrEqualI16, i16);
impl_greaterthanorequal_instruction!(GreaterThanOrEqualI32, i32);
impl_greaterthanorequal_instruction!(GreaterThanOrEqualI64, i64);
impl_greaterthanorequal_instruction!(GreaterThanOrEqualU8, u8);
impl_greaterthanorequal_instruction!(GreaterThanOrEqualU16, u16);
impl_greaterthanorequal_instruction!(GreaterThanOrEqualU32, u32);
impl_greaterthanorequal_instruction!(GreaterThanOrEqualU64, u64);

macro_rules! impl_greaterthanorequal_float_instruction {
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
                    "GreaterThanOrEqual: R{} <= {} R{} ({}) >= R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 >= val2) as u8;
                // Handle nan/inf?
                // ✅ In Rust, by IEEE-754 rules:
                // NaN >= x is always false.
                // x >= NaN is always false.
                // NaN >= NaN is always false.
                // Thus "greater than or equal" instruction is already correct for floats without any special NaN handling unless we want to error on NaNs later?

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_greaterthanorequal_float_instruction!(GreaterThanOrEqualF32, f32);
impl_greaterthanorequal_float_instruction!(GreaterThanOrEqualF64, f64);

#[cfg(test)]
mod tests {
    use crate::executor::interpreted::opcode_impl::all::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        greaterthanorequal_basic,
        [
            (GreaterThanOrEqualU8, u8),
            (GreaterThanOrEqualU16, u16),
            (GreaterThanOrEqualU32, u32),
            (GreaterThanOrEqualU64, u64),
            (GreaterThanOrEqualI8, i8),
            (GreaterThanOrEqualI16, i16),
            (GreaterThanOrEqualI32, i32),
            (GreaterThanOrEqualI64, i64)
        ],
        VmTest::new()
            .setup(10 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(1u8, R!(2)), // 10 >= 5 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthanorequal_equal,
        [
            (GreaterThanOrEqualU8, u8),
            (GreaterThanOrEqualU16, u16),
            (GreaterThanOrEqualU32, u32),
            (GreaterThanOrEqualU64, u64),
            (GreaterThanOrEqualI8, i8),
            (GreaterThanOrEqualI16, i16),
            (GreaterThanOrEqualI32, i32),
            (GreaterThanOrEqualI64, i64)
        ],
        VmTest::new()
            .setup(42 as T, R!(0))
            .setup(42 as T, R!(1))
            .expect(1u8, R!(2)), // 42 >= 42 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthanorequal_false,
        [
            (GreaterThanOrEqualU8, u8),
            (GreaterThanOrEqualU16, u16),
            (GreaterThanOrEqualU32, u32),
            (GreaterThanOrEqualU64, u64),
            (GreaterThanOrEqualI8, i8),
            (GreaterThanOrEqualI16, i16),
            (GreaterThanOrEqualI32, i32),
            (GreaterThanOrEqualI64, i64)
        ],
        VmTest::new()
            .setup(5 as T, R!(0))
            .setup(10 as T, R!(1))
            .expect(0u8, R!(2)), // 5 >= 10 -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthanorequal_negative,
        [
            (GreaterThanOrEqualI8, i8),
            (GreaterThanOrEqualI16, i16),
            (GreaterThanOrEqualI32, i32),
            (GreaterThanOrEqualI64, i64)
        ],
        VmTest::new()
            .setup(-5 as T, R!(0))
            .setup(-10 as T, R!(1))
            .expect(1u8, R!(2)), // -5 >= -10 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthanorequal_negative_equal,
        [
            (GreaterThanOrEqualI8, i8),
            (GreaterThanOrEqualI16, i16),
            (GreaterThanOrEqualI32, i32),
            (GreaterThanOrEqualI64, i64)
        ],
        VmTest::new()
            .setup(-5 as T, R!(0))
            .setup(-5 as T, R!(1))
            .expect(1u8, R!(2)), // -5 >= -5 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthanorequal_floats_basic,
        [(GreaterThanOrEqualF32, f32), (GreaterThanOrEqualF64, f64)],
        VmTest::new()
            .setup(2.5 as T, R!(0))
            .setup(1.5 as T, R!(1))
            .expect(1u8, R!(2)), // 2.5 >= 1.5 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthanorequal_floats_equal,
        [(GreaterThanOrEqualF32, f32), (GreaterThanOrEqualF64, f64)],
        VmTest::new()
            .setup(1.68 as T, R!(0))
            .setup(1.68 as T, R!(1))
            .expect(1u8, R!(2)), // 1.68 >= 1.68 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthanorequal_floats_false,
        [(GreaterThanOrEqualF32, f32), (GreaterThanOrEqualF64, f64)],
        VmTest::new()
            .setup(1.0 as T, R!(0))
            .setup(5.0 as T, R!(1))
            .expect(0u8, R!(2)), // 1.0 >= 5.0 -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthanorequal_floats_nan,
        [(GreaterThanOrEqualF32, f32), (GreaterThanOrEqualF64, f64)],
        VmTest::new()
            .setup(f32::NAN as T, R!(0))
            .setup(5.0 as T, R!(1))
            .expect(0u8, R!(2)), // NaN compared -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthanorequal_floats_inf,
        [(GreaterThanOrEqualF32, f32), (GreaterThanOrEqualF64, f64)],
        VmTest::new()
            .setup(f32::INFINITY as T, R!(0))
            .setup(5.0 as T, R!(1))
            .expect(1u8, R!(2)), // inf >= 5.0 -> true
        (R!(2), R!(0), R!(1))
    );
}
