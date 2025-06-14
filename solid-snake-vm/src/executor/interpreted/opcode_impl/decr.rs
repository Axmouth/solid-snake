use log::debug;
use paste::paste;

use crate::executor::ext::{VmExecutionError, VmExecutorExt};
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::{RegisterType, VmErrorCode};
use crate::set_error_if;

macro_rules! impl_incr_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                concat!("Decrements a ", stringify!($ty), " register by a constant value."),
                [
                    (dest: RegisterType, "Register to decrement"),
                    (decr_val: $ty, "Constant value to subtract")
                ],
                [Arithmetic, Pure],
                [<$opcode handler>]
            );

            #[inline(always)]
            #[allow(non_snake_case)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (dest, decr_val) = args;

                let val: $ty = executor.registers().get_register_value(dest)?;

                debug!(
                    "Decr: R{} <= {} R{} ({}) - ({})",
                    dest,
                    stringify!($ty),
                    dest,
                    val,
                    decr_val
                );

                let (result, overflowed) = val.overflowing_sub(decr_val);
                set_error_if!(executor, overflowed, VmErrorCode::Underflow);
                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_incr_instruction!(DecrementI8, i8);
impl_incr_instruction!(DecrementI16, i16);
impl_incr_instruction!(DecrementI32, i32);
impl_incr_instruction!(DecrementI64, i64);
impl_incr_instruction!(DecrementU8, u8);
impl_incr_instruction!(DecrementU16, u16);
impl_incr_instruction!(DecrementU32, u32);
impl_incr_instruction!(DecrementU64, u64);

macro_rules! impl_incr_float_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
        $crate::define_instruction!(
            $opcode,
            concat!("Decrements a ", stringify!($ty), " floating-point register by a constant value."),
            [
                (dest: RegisterType, "Register to decrement"),
                (incr_val: $ty, "Constant value to subtract")
            ],
            [Arithmetic, Pure],
            [<$opcode handler>]
        );

        #[inline(always)]
        #[allow(non_snake_case)]
        fn [<$opcode handler>](
            executor: &mut VmInterpretedExecutor,
            args: [<$opcode Args>],
        ) -> Result<(), VmExecutionError> {
            let (dest, incr_val) = args;

            let val: $ty = executor.registers().get_register_value(dest)?;

            debug!(
                "Decr: R{} <= {} R{} ({}) - ({})",
                dest,
                stringify!($ty),
                dest,
                val,
                incr_val
            );

            let result: $ty = val - incr_val;

            if result.is_nan() || result.is_infinite() {
                executor.set_error(VmErrorCode::FloatInvalidResult as i64);
            }

            executor
                .registers_mut()
                .set_register_value(dest, result)?;

            Ok(())
        }}
    };
}

impl_incr_float_instruction!(DecrementF32, f32);
impl_incr_float_instruction!(DecrementF64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        decr_zero,
        [
            (DecrementU8, u8),
            (DecrementU16, u16),
            (DecrementU32, u32),
            (DecrementU64, u64),
            (DecrementI8, i8),
            (DecrementI16, i16),
            (DecrementI32, i32),
            (DecrementI64, i64),
            (DecrementF32, f32),
            (DecrementF64, f64)
        ],
        VmTest::new().setup(20 as T, R!(2)).expect(20 as T, R!(2)),
        (R!(2), 0 as T)
    );

    define_vm_tests!(
        decr_small,
        [
            (DecrementU8, u8),
            (DecrementU16, u16),
            (DecrementU32, u32),
            (DecrementU64, u64),
            (DecrementI8, i8),
            (DecrementI16, i16),
            (DecrementI32, i32),
            (DecrementI64, i64),
            (DecrementF32, f32),
            (DecrementF64, f64)
        ],
        VmTest::new().setup(20 as T, R!(2)).expect(5 as T, R!(2)),
        (R!(2), 15 as T)
    );

    define_vm_tests!(
        decr_overflow,
        [
            (DecrementU8, u8),
            (DecrementU16, u16),
            (DecrementU32, u32),
            (DecrementU64, u64),
            (DecrementI8, i8),
            (DecrementI16, i16),
            (DecrementI32, i32),
            (DecrementI64, i64)
        ],
        VmTest::new()
            .setup(T::MIN, R!(2))
            .expect_error(VmErrorCode::Underflow),
        (R!(2), 10 as T)
    );

    define_vm_tests!(
        decr_underflow,
        [
            (DecrementI8, i8),
            (DecrementI16, i16),
            (DecrementI32, i32),
            (DecrementI64, i64)
        ],
        VmTest::new()
            .setup(T::MAX, R!(2))
            .expect_error(VmErrorCode::Underflow),
        (R!(2), -10 as T)
    );

    define_vm_tests!(
        dencr_overflow_f,
        [(DecrementF32, f32), (DecrementF64, f64)],
        VmTest::new()
            .setup(T::MIN, R!(2))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), T::MAX)
    );

    define_vm_tests!(
        incr_nan_f,
        [(DecrementF32, f32), (DecrementF64, f64)],
        VmTest::new()
            .setup(T::NAN, R!(2))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), 10 as T)
    );

    define_vm_tests!(
        decr_inf_f,
        [(DecrementF32, f32), (DecrementF64, f64)],
        VmTest::new()
            .setup(T::INFINITY, R!(2))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), 10 as T)
    );

    define_vm_tests!(
        decr_neg_pos_zero,
        [
            (DecrementI8, i8),
            (DecrementI16, i16),
            (DecrementI32, i32),
            (DecrementI64, i64)
        ],
        VmTest::new().setup(5 as T, R!(2)).expect(0 as T, R!(2)),
        (R!(2), 5 as T)
    );
    define_vm_tests!(
        decr_max_minus_max,
        [
            (DecrementI8, i8),
            (DecrementI16, i16),
            (DecrementI32, i32),
            (DecrementI64, i64)
        ],
        VmTest::new()
            .setup(T::MIN, R!(2))
            .expect_error(VmErrorCode::Underflow),
        (R!(2), T::MAX)
    );
}
