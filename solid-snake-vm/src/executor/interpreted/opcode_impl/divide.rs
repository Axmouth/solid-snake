use log::debug;
use paste::paste;

use crate::executor::ext::{VmExecutionError, VmExecutorExt};
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::{RegisterType, VmErrorCode};

macro_rules! impl_div_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode handler>]);

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
                    "Div: R{} <= {} R{} ({}) / R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                if val2 == 0 {
                    executor.set_error(VmErrorCode::DivisionByZero as i64);
                    executor.registers_mut().set_register_value(dest, $ty::default())?;
                    return Ok(());
                }

                let result = val1 / val2;
                executor.registers_mut().set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_div_instruction!(DivideI8, i8);
impl_div_instruction!(DivideI16, i16);
impl_div_instruction!(DivideI32, i32);
impl_div_instruction!(DivideI64, i64);
impl_div_instruction!(DivideU8, u8);
impl_div_instruction!(DivideU16, u16);
impl_div_instruction!(DivideU32, u32);
impl_div_instruction!(DivideU64, u64);

macro_rules! impl_div_float_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode handler>]);

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
                    "Div: R{} <= {} R{} ({}) / R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: $ty = val1 / val2;

                if result.is_nan() || result.is_infinite() {
                    executor.set_error(VmErrorCode::FloatInvalidResult as i64);
                }

                executor.registers_mut().set_register_value(dest, result)?;
                Ok(())
            }
        }
    };
}

impl_div_float_instruction!(DivideF32, f32);
impl_div_float_instruction!(DivideF64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        divide_basic,
        [
            (DivideU8, u8),
            (DivideU16, u16),
            (DivideU32, u32),
            (DivideU64, u64),
            (DivideI8, i8),
            (DivideI16, i16),
            (DivideI32, i32),
            (DivideI64, i64),
            (DivideF32, f32),
            (DivideF64, f64)
        ],
        VmTest::new()
            .setup(100 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect((100 as T) / (5 as T), R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        divide_by_one,
        [(DivideU8, u8), (DivideI8, i8), (DivideF32, f32)],
        VmTest::new()
            .setup(42 as T, R!(0))
            .setup(1 as T, R!(1))
            .expect(42 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        divide_by_zero_int,
        [
            (DivideU8, u8),
            (DivideI8, i8),
            (DivideU32, u32),
            (DivideI32, i32)
        ],
        VmTest::new()
            .setup(42 as T, R!(0))
            .setup(0 as T, R!(1))
            .expect_error(VmErrorCode::DivisionByZero),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        divide_by_zero_float,
        [(DivideF32, f32), (DivideF64, f64)],
        VmTest::new()
            .setup(42.0 as T, R!(0))
            .setup(0.0 as T, R!(1))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        divide_negative,
        [
            (DivideI8, i8),
            (DivideI16, i16),
            (DivideI32, i32),
            (DivideI64, i64)
        ],
        VmTest::new()
            .setup(-100 as T, R!(0))
            .setup(2 as T, R!(1))
            .expect(-50 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        divide_float_precision,
        [(DivideF32, f32), (DivideF64, f64)],
        VmTest::new()
            .setup(1.0 as T, R!(0))
            .setup(3.0 as T, R!(1))
            .expect(1.0 as T / 3.0 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );
}
