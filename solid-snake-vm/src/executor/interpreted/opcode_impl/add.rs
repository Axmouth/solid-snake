use log::debug;
use paste::paste;

use crate::executor::ext::{VmExecutionError, VmExecutorExt};
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::{RegisterType, VmErrorCode};
use crate::set_error_if;

macro_rules! impl_add_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                concat!("Adds two ", stringify!($ty), " registers and stores the result in the destination register."),
                [
                    (dest: RegisterType, "Destination register"),
                    (reg1: RegisterType, "First operand"),
                    (reg2: RegisterType, "Second operand")
                ],
                [Arithmetic, Pure, Commutative],
                [<$opcode handler>]
            );

            #[inline(always)]
            #[allow(non_snake_case)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (dest, reg1, reg2) = args;

                let val1: $ty = executor.registers().get_register_value(reg1)?;
                let val2: $ty = executor.registers().get_register_value(reg2)?;

                debug!(
                    "Add: R{} <= {} R{} ({}) + R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let (result, overflowed) = val1.overflowing_add(val2);
                set_error_if!(executor, overflowed, VmErrorCode::Overflow);
                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_add_instruction!(AddI8, i8);
impl_add_instruction!(AddI16, i16);
impl_add_instruction!(AddI32, i32);
impl_add_instruction!(AddI64, i64);
impl_add_instruction!(AddU8, u8);
impl_add_instruction!(AddU16, u16);
impl_add_instruction!(AddU32, u32);
impl_add_instruction!(AddU64, u64);

macro_rules! impl_add_float_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
        $crate::define_instruction!(
            $opcode,
            concat!("Adds two ", stringify!($ty), " floating-point registers and stores the result in the destination register."),
            [
                (dest: RegisterType, "Destination register"),
                (reg1: RegisterType, "First operand"),
                (reg2: RegisterType, "Second operand")
            ],
            [Arithmetic, Pure, Commutative],
            [<$opcode handler>]
        );

        #[inline(always)]
        #[allow(non_snake_case)]
        fn [<$opcode handler>](
            executor: &mut VmInterpretedExecutor,
            args: [<$opcode Args>],
        ) -> Result<(), VmExecutionError> {
            let (dest, reg1, reg2) = args;

            let val1: $ty = executor.registers().get_register_value(reg1)?;
            let val2: $ty = executor.registers().get_register_value(reg2)?;

            debug!(
                "Add: R{} <= {} R{} ({}) + R{} ({})",
                dest,
                stringify!($ty),
                reg1,
                val1,
                reg2,
                val2
            );

            let result: $ty = val1 + val2;

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

impl_add_float_instruction!(AddF32, f32);
impl_add_float_instruction!(AddF64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::interpreted::opcode_impl::all::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        add_zero,
        [
            (AddU8, u8),
            (AddU16, u16),
            (AddU32, u32),
            (AddU64, u64),
            (AddI8, i8),
            (AddI16, i16),
            (AddI32, i32),
            (AddI64, i64),
            (AddF32, f32),
            (AddF64, f64)
        ],
        VmTest::new()
            .setup(0 as T, R!(0))
            .setup(20 as T, R!(1))
            .expect(20 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        add_small,
        [
            (AddU8, u8),
            (AddU16, u16),
            (AddU32, u32),
            (AddU64, u64),
            (AddI8, i8),
            (AddI16, i16),
            (AddI32, i32),
            (AddI64, i64),
            (AddF32, f32),
            (AddF64, f64)
        ],
        VmTest::new()
            .setup(10 as T, R!(0))
            .setup(20 as T, R!(1))
            .expect(30 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        add_overflow,
        [
            (AddU8, u8),
            (AddU16, u16),
            (AddU32, u32),
            (AddU64, u64),
            (AddI8, i8),
            (AddI16, i16),
            (AddI32, i32),
            (AddI64, i64)
        ],
        VmTest::new()
            .setup(10 as T, R!(0))
            .setup(T::MAX, R!(1))
            .expect_error(VmErrorCode::Overflow),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        add_underflow,
        [(AddI8, i8), (AddI16, i16), (AddI32, i32), (AddI64, i64)],
        VmTest::new()
            .setup(-10 as T, R!(0))
            .setup(T::MIN, R!(1))
            .expect_error(VmErrorCode::Overflow),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        add_overflow_f,
        [(AddF32, f32), (AddF64, f64)],
        VmTest::new()
            .setup(T::MAX as T, R!(0))
            .setup(T::MAX, R!(1))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        add_nan_f,
        [(AddF32, f32), (AddF64, f64)],
        VmTest::new()
            .setup(10 as T, R!(0))
            .setup(T::NAN, R!(1))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        add_inf_f,
        [(AddF32, f32), (AddF64, f64)],
        VmTest::new()
            .setup(10 as T, R!(0))
            .setup(T::INFINITY, R!(1))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        add_same_registers,
        [(AddU8, u8), (AddI8, i8), (AddF32, f32)],
        VmTest::new().setup(10 as T, R!(0)).expect(20 as T, R!(0)),
        (R!(0), R!(0), R!(0))
    );

    define_vm_tests!(
        add_aliasing_output_input1,
        [(AddU8, u8), (AddI8, i8), (AddF32, f32)],
        VmTest::new()
            .setup(15 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(20 as T, R!(0)),
        (R!(0), R!(0), R!(1))
    );

    define_vm_tests!(
        add_aliasing_output_input2,
        [(AddU8, u8), (AddI8, i8), (AddF32, f32)],
        VmTest::new()
            .setup(5 as T, R!(0))
            .setup(15 as T, R!(1))
            .expect(20 as T, R!(0)),
        (R!(0), R!(1), R!(0))
    );

    define_vm_tests!(
        add_neg_pos_zero,
        [(AddI8, i8), (AddI16, i16), (AddI32, i32), (AddI64, i64)],
        VmTest::new()
            .setup(-5 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(0 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );
    define_vm_tests!(
        add_min_plus_min,
        [(AddI8, i8), (AddI16, i16), (AddI32, i32), (AddI64, i64)],
        VmTest::new()
            .setup(T::MIN, R!(0))
            .setup(T::MIN, R!(1))
            .expect_error(VmErrorCode::Overflow),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        add_mixed_bits,
        [(AddU8, u8)],
        VmTest::new()
            .setup(0b10101010 as T, R!(0))
            .setup(0b01010101 as T, R!(1))
            .expect(0b11111111 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    #[test]
    fn add_pc() {
        crate::asm_internal::VmProgramTest::new()
            .setup_register(10, R!(0))
            .setup_register(20, R!(1))
            .with_program(vec![
                AddU16Instruction::encode((R!(2), R!(0), R!(1))),
                HaltInstruction::encode((0,)),
            ])
            .expect_register(R!(2), 30)
            .expect_pc(1)
            .run()
            .unwrap();
    }
}
