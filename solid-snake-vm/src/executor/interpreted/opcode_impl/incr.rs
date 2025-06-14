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
                concat!("Adds an immediate ", stringify!($ty), " value to the destination register."),
                [
                    (dest: RegisterType, "Target register to be incremented"),
                    (incr_val: $ty, "Immediate value to add to the register")
                ],
                [Arithmetic],
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
                    "Incr: R{} <= {} R{} ({}) + ({})",
                    dest,
                    stringify!($ty),
                    dest,
                    val,
                    incr_val
                );

                let (result, overflowed) = val.overflowing_add(incr_val);
                set_error_if!(executor, overflowed, VmErrorCode::Overflow);
                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_incr_instruction!(IncrementI8, i8);
impl_incr_instruction!(IncrementI16, i16);
impl_incr_instruction!(IncrementI32, i32);
impl_incr_instruction!(IncrementI64, i64);
impl_incr_instruction!(IncrementU8, u8);
impl_incr_instruction!(IncrementU16, u16);
impl_incr_instruction!(IncrementU32, u32);
impl_incr_instruction!(IncrementU64, u64);

macro_rules! impl_incr_float_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
        $crate::define_instruction!(
            $opcode,
            concat!("Adds an immediate ", stringify!($ty), " floating-point value to the destination register."),
            [
                (dest: RegisterType, "Target register to be incremented"),
                (incr_val: $ty, "Immediate value to add to the register")
            ],
            [Arithmetic],
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
                "Incr: R{} <= {} R{} ({}) + ({})",
                dest,
                stringify!($ty),
                dest,
                val,
                incr_val
            );

            let result: $ty = val + incr_val;

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

impl_incr_float_instruction!(IncrementF32, f32);
impl_incr_float_instruction!(IncrementF64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        incr_zero,
        [
            (IncrementU8, u8),
            (IncrementU16, u16),
            (IncrementU32, u32),
            (IncrementU64, u64),
            (IncrementI8, i8),
            (IncrementI16, i16),
            (IncrementI32, i32),
            (IncrementI64, i64),
            (IncrementF32, f32),
            (IncrementF64, f64)
        ],
        VmTest::new().setup(20 as T, R!(2)).expect(20 as T, R!(2)),
        (R!(2), 0 as T)
    );

    define_vm_tests!(
        icr_small,
        [
            (IncrementU8, u8),
            (IncrementU16, u16),
            (IncrementU32, u32),
            (IncrementU64, u64),
            (IncrementI8, i8),
            (IncrementI16, i16),
            (IncrementI32, i32),
            (IncrementI64, i64),
            (IncrementF32, f32),
            (IncrementF64, f64)
        ],
        VmTest::new().setup(20 as T, R!(2)).expect(30 as T, R!(2)),
        (R!(2), 10 as T)
    );

    define_vm_tests!(
        incr_overflow,
        [
            (IncrementU8, u8),
            (IncrementU16, u16),
            (IncrementU32, u32),
            (IncrementU64, u64),
            (IncrementI8, i8),
            (IncrementI16, i16),
            (IncrementI32, i32),
            (IncrementI64, i64)
        ],
        VmTest::new()
            .setup(T::MAX, R!(2))
            .expect_error(VmErrorCode::Overflow),
        (R!(2), 10 as T)
    );

    define_vm_tests!(
        incr_underflow,
        [
            (IncrementI8, i8),
            (IncrementI16, i16),
            (IncrementI32, i32),
            (IncrementI64, i64)
        ],
        VmTest::new()
            .setup(T::MIN, R!(2))
            .expect_error(VmErrorCode::Overflow),
        (R!(2), -10 as T)
    );

    define_vm_tests!(
        incr_overflow_f,
        [(IncrementF32, f32), (IncrementF64, f64)],
        VmTest::new()
            .setup(T::MAX, R!(2))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), T::MAX)
    );

    define_vm_tests!(
        incr_nan_f,
        [(IncrementF32, f32), (IncrementF64, f64)],
        VmTest::new()
            .setup(T::NAN, R!(2))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), 10 as T)
    );

    define_vm_tests!(
        incr_inf_f,
        [(IncrementF32, f32), (IncrementF64, f64)],
        VmTest::new()
            .setup(T::INFINITY, R!(2))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), 10 as T)
    );

    define_vm_tests!(
        incr_neg_pos_zero,
        [
            (IncrementI8, i8),
            (IncrementI16, i16),
            (IncrementI32, i32),
            (IncrementI64, i64)
        ],
        VmTest::new().setup(5 as T, R!(2)).expect(0 as T, R!(2)),
        (R!(2), -5 as T)
    );
    define_vm_tests!(
        incr_min_plus_min,
        [
            (IncrementI8, i8),
            (IncrementI16, i16),
            (IncrementI32, i32),
            (IncrementI64, i64)
        ],
        VmTest::new()
            .setup(T::MIN, R!(2))
            .expect_error(VmErrorCode::Overflow),
        (R!(2), T::MIN)
    );

    define_vm_tests!(
        incr_mixed_bits,
        [(IncrementU8, u8)],
        VmTest::new()
            .setup(0b01010101 as T, R!(2))
            .expect(0b11111111 as T, R!(2)),
        (R!(2), 0b10101010 as T)
    );
}
