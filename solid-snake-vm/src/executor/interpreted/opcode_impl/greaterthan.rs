use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_greaterthan_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode handler>]);

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
                    "GreaterThan: R{} <= {} R{} ({}) > R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 > val2) as u8;

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_greaterthan_instruction!(GreaterThanI8, i8);
impl_greaterthan_instruction!(GreaterThanI16, i16);
impl_greaterthan_instruction!(GreaterThanI32, i32);
impl_greaterthan_instruction!(GreaterThanI64, i64);
impl_greaterthan_instruction!(GreaterThanU8, u8);
impl_greaterthan_instruction!(GreaterThanU16, u16);
impl_greaterthan_instruction!(GreaterThanU32, u32);
impl_greaterthan_instruction!(GreaterThanU64, u64);

macro_rules! impl_greaterthan_float_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode:snake handler>]);

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
                    "GreaterThan: R{} <= {} R{} ({}) > R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 > val2) as u8;
                // Handle nan/inf?
                // ✅ In Rust, by IEEE-754 rules:
                // NaN > x is always false.
                // x > NaN is always false.
                // NaN > NaN is always false.
                // Thus "greater than" instruction is already correct for floats without any special NaN handling unless we want to error on NaNs later?

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_greaterthan_float_instruction!(GreaterThanF32, f32);
impl_greaterthan_float_instruction!(GreaterThanF64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        greaterthan_basic,
        [
            (GreaterThanU8, u8),
            (GreaterThanU16, u16),
            (GreaterThanU32, u32),
            (GreaterThanU64, u64),
            (GreaterThanI8, i8),
            (GreaterThanI16, i16),
            (GreaterThanI32, i32),
            (GreaterThanI64, i64)
        ],
        VmTest::new()
            .setup(10 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(1u8, R!(2)), // 10 > 5 -> true (1)
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthan_equal,
        [
            (GreaterThanU8, u8),
            (GreaterThanU16, u16),
            (GreaterThanU32, u32),
            (GreaterThanU64, u64),
            (GreaterThanI8, i8),
            (GreaterThanI16, i16),
            (GreaterThanI32, i32),
            (GreaterThanI64, i64)
        ],
        VmTest::new()
            .setup(42 as T, R!(0))
            .setup(42 as T, R!(1))
            .expect(0u8, R!(2)), // 42 > 42 -> false (0)
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthan_false,
        [
            (GreaterThanU8, u8),
            (GreaterThanU16, u16),
            (GreaterThanU32, u32),
            (GreaterThanU64, u64),
            (GreaterThanI8, i8),
            (GreaterThanI16, i16),
            (GreaterThanI32, i32),
            (GreaterThanI64, i64)
        ],
        VmTest::new()
            .setup(5 as T, R!(0))
            .setup(10 as T, R!(1))
            .expect(0u8, R!(2)), // 5 > 10 -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthan_negative,
        [
            (GreaterThanI8, i8),
            (GreaterThanI16, i16),
            (GreaterThanI32, i32),
            (GreaterThanI64, i64)
        ],
        VmTest::new()
            .setup(-5 as T, R!(0))
            .setup(-10 as T, R!(1))
            .expect(1u8, R!(2)), // -5 > -10 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthan_negative_false,
        [
            (GreaterThanI8, i8),
            (GreaterThanI16, i16),
            (GreaterThanI32, i32),
            (GreaterThanI64, i64)
        ],
        VmTest::new()
            .setup(-10 as T, R!(0))
            .setup(-5 as T, R!(1))
            .expect(0u8, R!(2)), // -10 > -5 -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthan_floats_basic,
        [(GreaterThanF32, f32), (GreaterThanF64, f64)],
        VmTest::new()
            .setup(2.5 as T, R!(0))
            .setup(1.5 as T, R!(1))
            .expect(1u8, R!(2)), // 2.5 > 1.5 -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthan_floats_equal,
        [(GreaterThanF32, f32), (GreaterThanF64, f64)],
        VmTest::new()
            .setup(1.68 as T, R!(0))
            .setup(1.68 as T, R!(1))
            .expect(0u8, R!(2)), // 1.68 > 1.68 -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthan_floats_false,
        [(GreaterThanF32, f32), (GreaterThanF64, f64)],
        VmTest::new()
            .setup(1.0 as T, R!(0))
            .setup(5.0 as T, R!(1))
            .expect(0u8, R!(2)), // 1.0 > 5.0 -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthan_floats_nan,
        [(GreaterThanF32, f32), (GreaterThanF64, f64)],
        VmTest::new()
            .setup(f32::NAN as T, R!(0))
            .setup(5.0 as T, R!(1))
            .expect(0u8, R!(2)), // NaN compared -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        greaterthan_floats_inf,
        [(GreaterThanF32, f32), (GreaterThanF64, f64)],
        VmTest::new()
            .setup(f32::INFINITY as T, R!(0))
            .setup(5.0 as T, R!(1))
            .expect(1u8, R!(2)), // inf > 5.0 -> true
        (R!(2), R!(0), R!(1))
    );
}
