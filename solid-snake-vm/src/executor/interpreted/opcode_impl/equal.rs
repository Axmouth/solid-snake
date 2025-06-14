use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_equal_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                concat!("Checks equality between two ", stringify!($ty), " registers. Sets 1 if equal, 0 otherwise."),
                [
                    (dest: RegisterType, "Destination register for result (1 or 0)"),
                    (reg1: RegisterType, "First operand register"),
                    (reg2: RegisterType, "Second operand register")
                ],
                [Logical, Pure, Commutative],
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
                    "Equal: R{} <= {} R{} ({}) == R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 == val2) as u8;

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_equal_instruction!(EqualI8, i8);
impl_equal_instruction!(EqualI16, i16);
impl_equal_instruction!(EqualI32, i32);
impl_equal_instruction!(EqualI64, i64);
impl_equal_instruction!(EqualU8, u8);
impl_equal_instruction!(EqualU16, u16);
impl_equal_instruction!(EqualU32, u32);
impl_equal_instruction!(EqualU64, u64);

macro_rules! impl_equal_float_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                concat!("Checks equality between two ", stringify!($ty), " floating-point registers. Sets 1 if equal, 0 otherwise."),
                [
                    (dest: RegisterType, "Destination register for result (1 or 0)"),
                    (reg1: RegisterType, "First operand register"),
                    (reg2: RegisterType, "Second operand register")
                ],
                [Logical, Pure, Commutative],
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
                    "Equal: R{} <= {} R{} ({}) == R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 == val2) as u8;
                // Handle nan/inf?
                // ✅ In Rust, by IEEE-754 rules:
                // NaN == x is always false.
                // x == NaN is always false.
                // NaN == NaN is always false.
                // Thus "greater than or equal" instruction is already correct for floats without any special NaN handling unless we want to error on NaNs later?

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_equal_float_instruction!(EqualF32, f32);
impl_equal_float_instruction!(EqualF64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        equal_basic_true,
        [
            (EqualU8, u8),
            (EqualU16, u16),
            (EqualU32, u32),
            (EqualU64, u64),
            (EqualI8, i8),
            (EqualI16, i16),
            (EqualI32, i32),
            (EqualI64, i64)
        ],
        VmTest::new()
            .setup(123 as T, R!(0))
            .setup(123 as T, R!(1))
            .expect(1u8, R!(2)), // equal -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        equal_basic_false,
        [
            (EqualU8, u8),
            (EqualU16, u16),
            (EqualU32, u32),
            (EqualU64, u64),
            (EqualI8, i8),
            (EqualI16, i16),
            (EqualI32, i32),
            (EqualI64, i64)
        ],
        VmTest::new()
            .setup(123 as T, R!(0))
            .setup(42 as T, R!(1))
            .expect(0u8, R!(2)), // not equal -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        equal_negative,
        [
            (EqualI8, i8),
            (EqualI16, i16),
            (EqualI32, i32),
            (EqualI64, i64)
        ],
        VmTest::new()
            .setup(-5 as T, R!(0))
            .setup(-5 as T, R!(1))
            .expect(1u8, R!(2)), // negative equal -> true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        equal_negative_false,
        [
            (EqualI8, i8),
            (EqualI16, i16),
            (EqualI32, i32),
            (EqualI64, i64)
        ],
        VmTest::new()
            .setup(-5 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(0u8, R!(2)), // not equal -> false
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        equal_float_true,
        [(EqualF32, f32), (EqualF64, f64)],
        VmTest::new()
            .setup(1.618 as T, R!(0))
            .setup(1.618 as T, R!(1))
            .expect(1u8, R!(2)), // equal floats
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        equal_float_false,
        [(EqualF32, f32), (EqualF64, f64)],
        VmTest::new()
            .setup(1.618 as T, R!(0))
            .setup(3.0 as T, R!(1))
            .expect(0u8, R!(2)), // not equal
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        equal_float_nan,
        [(EqualF32, f32), (EqualF64, f64)],
        VmTest::new()
            .setup(f32::NAN as T, R!(0))
            .setup(f32::NAN as T, R!(1))
            .expect(0u8, R!(2)), // NaN == NaN is false in IEEE
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        equal_float_inf,
        [(EqualF32, f32), (EqualF64, f64)],
        VmTest::new()
            .setup(f32::INFINITY as T, R!(0))
            .setup(f32::INFINITY as T, R!(1))
            .expect(1u8, R!(2)), // Inf == Inf is true
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        equal_float_inf_false,
        [(EqualF32, f32), (EqualF64, f64)],
        VmTest::new()
            .setup(f32::INFINITY as T, R!(0))
            .setup(100.0 as T, R!(1))
            .expect(0u8, R!(2)), // Inf != finite
        (R!(2), R!(0), R!(1))
    );
}
