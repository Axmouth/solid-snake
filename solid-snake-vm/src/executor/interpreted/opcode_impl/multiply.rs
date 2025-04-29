use log::debug;
use paste::paste;

use crate::executor::ext::{VmExecutionError, VmExecutorExt};
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::{RegisterType, VmErrorCode};
use crate::set_error_if;

macro_rules! impl_mul_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode handler>]);

            #[inline(always)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (dest, reg1, reg2) = args;

                let val1: $ty = executor.registers().get_register_value(reg1)?;
                let val2: $ty = executor.registers().get_register_value(reg2)?;

                debug!(
                    "Mul: R{} <= {} R{} ({}) * R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let (result, overflowed) = val1.overflowing_mul(val2);
                set_error_if!(executor, overflowed, VmErrorCode::Overflow);
                executor.registers_mut().set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_mul_instruction!(MultiplyI8, i8);
impl_mul_instruction!(MultiplyI16, i16);
impl_mul_instruction!(MultiplyI32, i32);
impl_mul_instruction!(MultiplyI64, i64);
impl_mul_instruction!(MultiplyU8, u8);
impl_mul_instruction!(MultiplyU16, u16);
impl_mul_instruction!(MultiplyU32, u32);
impl_mul_instruction!(MultiplyU64, u64);

macro_rules! impl_mul_float_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode handler>]);

            #[inline(always)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (dest, reg1, reg2) = args;

                let val1: $ty = executor.registers().get_register_value(reg1)?;
                let val2: $ty = executor.registers().get_register_value(reg2)?;

                debug!(
                    "Mul: R{} <= {} R{} ({}) * R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result = val1 * val2;

                if result.is_nan() || result.is_infinite() {
                    executor.set_error(VmErrorCode::FloatInvalidResult as i64);
                }

                executor.registers_mut().set_register_value(dest, result)?;
                Ok(())
            }
        }
    };
}

impl_mul_float_instruction!(MultiplyF32, f32);
impl_mul_float_instruction!(MultiplyF64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::interpreted::opcode_impl::all::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        multiply_basic,
        [
            (MultiplyU8, u8),
            (MultiplyU16, u16),
            (MultiplyU32, u32),
            (MultiplyU64, u64),
            (MultiplyI8, i8),
            (MultiplyI16, i16),
            (MultiplyI32, i32),
            (MultiplyI64, i64),
            (MultiplyF32, f32),
            (MultiplyF64, f64)
        ],
        VmTest::new()
            .setup(6 as T, R!(0))
            .setup(7 as T, R!(1))
            .expect(6 as T * 7 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        multiply_zero,
        [(MultiplyU8, u8), (MultiplyI8, i8), (MultiplyF32, f32)],
        VmTest::new()
            .setup(42 as T, R!(0))
            .setup(0 as T, R!(1))
            .expect(0 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        multiply_negative,
        [
            (MultiplyI8, i8),
            (MultiplyI16, i16),
            (MultiplyI32, i32),
            (MultiplyI64, i64)
        ],
        VmTest::new()
            .setup(-2 as T, R!(0))
            .setup(3 as T, R!(1))
            .expect(-6 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        multiply_overflow,
        [
            (MultiplyU8, u8),
            (MultiplyI8, i8),
            (MultiplyU16, u16),
            (MultiplyI16, i16)
        ],
        VmTest::new()
            .setup(T::MAX, R!(0))
            .setup(2 as T, R!(1))
            .expect_error(VmErrorCode::Overflow),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        multiply_float_inf,
        [(MultiplyF32, f32), (MultiplyF64, f64)],
        VmTest::new()
            .setup(T::MAX, R!(0))
            .setup(2.0 as T, R!(1))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        multiply_aliasing,
        [(MultiplyU8, u8), (MultiplyI8, i8), (MultiplyF32, f32)],
        VmTest::new().setup(3 as T, R!(0)).expect(9 as T, R!(0)),
        (R!(0), R!(0), R!(0))
    );
}
