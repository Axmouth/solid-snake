use log::debug;
use paste::paste;

use crate::executor::ext::{VmExecutionError, VmExecutorExt};
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::{RegisterType, VmErrorCode};
use crate::set_error_if;

macro_rules! impl_subtract_instruction {
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
                    "Subtract: R{} <= {} R{} ({}) - R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let (result, overflowed) = val1.overflowing_sub(val2);
                set_error_if!(executor, overflowed, VmErrorCode::Underflow);
                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;


                Ok(())
            }
        }
    };
}

impl_subtract_instruction!(SubtractI8, i8);
impl_subtract_instruction!(SubtractI16, i16);
impl_subtract_instruction!(SubtractI32, i32);
impl_subtract_instruction!(SubtractI64, i64);
impl_subtract_instruction!(SubtractU8, u8);
impl_subtract_instruction!(SubtractU16, u16);
impl_subtract_instruction!(SubtractU32, u32);
impl_subtract_instruction!(SubtractU64, u64);

macro_rules! impl_subtract_float_instruction {
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
                "subtract: R{} <= {} R{} ({}) - R{} ({})",
                dest,
                stringify!($ty),
                reg1,
                val1,
                reg2,
                val2
            );

            let result: $ty = val1 - val2;

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

impl_subtract_float_instruction!(SubtractF32, f32);
impl_subtract_float_instruction!(SubtractF64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        subtract_zero,
        [
            (SubtractU8, u8),
            (SubtractU16, u16),
            (SubtractU32, u32),
            (SubtractU64, u64),
            (SubtractI8, i8),
            (SubtractI16, i16),
            (SubtractI32, i32),
            (SubtractI64, i64),
            (SubtractF32, f32),
            (SubtractF64, f64)
        ],
        VmTest::new()
            .setup(20 as T, R!(0))
            .setup(0 as T, R!(1))
            .expect(20 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        subtract_from_zero,
        [
            (SubtractI8, i8),
            (SubtractI16, i16),
            (SubtractI32, i32),
            (SubtractI64, i64),
            (SubtractF32, f32),
            (SubtractF64, f64)
        ],
        VmTest::new()
            .setup(0 as T, R!(0))
            .setup(-20 as T, R!(1))
            .expect(20 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        subtract_small,
        [
            (SubtractU8, u8),
            (SubtractU16, u16),
            (SubtractU32, u32),
            (SubtractU64, u64),
            (SubtractI8, i8),
            (SubtractI16, i16),
            (SubtractI32, i32),
            (SubtractI64, i64),
            (SubtractF32, f32),
            (SubtractF64, f64)
        ],
        VmTest::new()
            .setup(20 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(15 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        subtract_overflow,
        [
            (SubtractU8, u8),
            (SubtractU16, u16),
            (SubtractU32, u32),
            (SubtractU64, u64),
            (SubtractI8, i8),
            (SubtractI16, i16),
            (SubtractI32, i32),
            (SubtractI64, i64)
        ],
        VmTest::new()
            .setup(10 as T, R!(1))
            .setup(T::MIN, R!(0))
            .expect_error(VmErrorCode::Underflow),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        subtract_underflow,
        [
            (SubtractI8, i8),
            (SubtractI16, i16),
            (SubtractI32, i32),
            (SubtractI64, i64)
        ],
        VmTest::new()
            .setup(-10 as T, R!(0))
            .setup(T::MAX, R!(1))
            .expect_error(VmErrorCode::Underflow),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        subtract_overflow_f,
        [(SubtractF32, f32), (SubtractF64, f64)],
        VmTest::new()
            .setup(T::MIN as T, R!(0))
            .setup(T::MAX, R!(1))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        subtract_nan_f,
        [(SubtractF32, f32), (SubtractF64, f64)],
        VmTest::new()
            .setup(10 as T, R!(0))
            .setup(T::NAN, R!(1))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        subtract_inf_f,
        [(SubtractF32, f32), (SubtractF64, f64)],
        VmTest::new()
            .setup(10 as T, R!(0))
            .setup(T::INFINITY, R!(1))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        subtract_same_registers,
        [(SubtractU8, u8), (SubtractI8, i8), (SubtractF32, f32)],
        VmTest::new().setup(10 as T, R!(0)).expect(0 as T, R!(0)),
        (R!(0), R!(0), R!(0))
    );

    define_vm_tests!(
        subtract_aliasing_output_input1,
        [(SubtractU8, u8), (SubtractI8, i8), (SubtractF32, f32)],
        VmTest::new()
            .setup(15 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(10 as T, R!(0)),
        (R!(0), R!(0), R!(1))
    );

    define_vm_tests!(
        subtract_aliasing_output_input2,
        [(SubtractU8, u8), (SubtractI8, i8), (SubtractF32, f32)],
        VmTest::new()
            .setup(5 as T, R!(0))
            .setup(15 as T, R!(1))
            .expect(10 as T, R!(0)),
        (R!(0), R!(1), R!(0))
    );

    define_vm_tests!(
        subtract_neg_neg_zero,
        [
            (SubtractI8, i8),
            (SubtractI16, i16),
            (SubtractI32, i32),
            (SubtractI64, i64)
        ],
        VmTest::new()
            .setup(-5 as T, R!(0))
            .setup(-5 as T, R!(1))
            .expect(0 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        subtract_neg_pos,
        [
            (SubtractI8, i8),
            (SubtractI16, i16),
            (SubtractI32, i32),
            (SubtractI64, i64)
        ],
        VmTest::new()
            .setup(5 as T, R!(0))
            .setup(-5 as T, R!(1))
            .expect(10 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        subtract_min_minus_min,
        [
            (SubtractI8, i8),
            (SubtractI16, i16),
            (SubtractI32, i32),
            (SubtractI64, i64)
        ],
        VmTest::new()
            .setup(T::MIN, R!(0))
            .setup(T::MAX, R!(1))
            .expect_error(VmErrorCode::Underflow),
        (R!(2), R!(0), R!(1))
    );

    #[test]
    fn subtract_pc() {
        use crate::executor::interpreted::opcode_impl::all::*;
        crate::asm_internal::VmProgramTest::new()
            .setup_register(30, R!(0))
            .setup_register(10, R!(1))
            .with_program(vec![
                SubtractU16Instruction::encode((R!(2), R!(0), R!(1))),
                HaltInstruction::encode(()),
            ])
            .expect_register(R!(2), 20)
            .expect_pc(1)
            .run()
            .unwrap();
    }
}
