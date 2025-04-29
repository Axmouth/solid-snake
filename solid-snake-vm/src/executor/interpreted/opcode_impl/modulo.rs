use log::debug;
use paste::paste;

use crate::executor::ext::{VmExecutionError, VmExecutorExt};
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::{RegisterType, VmErrorCode};

macro_rules! impl_mod_instruction {
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
                    "Mod: R{} <= {} R{} ({}) % R{} ({})",
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

                let result = val1 % val2;
                executor.registers_mut().set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_mod_instruction!(ModuloI8, i8);
impl_mod_instruction!(ModuloI16, i16);
impl_mod_instruction!(ModuloI32, i32);
impl_mod_instruction!(ModuloI64, i64);
impl_mod_instruction!(ModuloU8, u8);
impl_mod_instruction!(ModuloU16, u16);
impl_mod_instruction!(ModuloU32, u32);
impl_mod_instruction!(ModuloU64, u64);

macro_rules! impl_mod_float_instruction {
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
                    "Mod: R{} <= {} R{} ({}) % R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result = val1 % val2;

                if result.is_nan() || result.is_infinite() {
                    executor.set_error(VmErrorCode::FloatInvalidResult as i64);
                }

                executor.registers_mut().set_register_value(dest, result)?;
                Ok(())
            }
        }
    };
}

impl_mod_float_instruction!(ModuloF32, f32);
impl_mod_float_instruction!(ModuloF64, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::interpreted::opcode_impl::all::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        modulo_basic,
        [
            (ModuloU8, u8),
            (ModuloU16, u16),
            (ModuloU32, u32),
            (ModuloU64, u64),
            (ModuloI8, i8),
            (ModuloI16, i16),
            (ModuloI32, i32),
            (ModuloI64, i64),
            (ModuloF32, f32),
            (ModuloF64, f64)
        ],
        VmTest::new()
            .setup(17 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(17 as T % 5 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        modulo_zero,
        [
            (ModuloU8, u8),
            (ModuloI8, i8),
            (ModuloU32, u32),
            (ModuloI32, i32)
        ],
        VmTest::new()
            .setup(42 as T, R!(0))
            .setup(0 as T, R!(1))
            .expect_error(VmErrorCode::DivisionByZero),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        modulo_negative,
        [
            (ModuloI8, i8),
            (ModuloI16, i16),
            (ModuloI32, i32),
            (ModuloI64, i64)
        ],
        VmTest::new()
            .setup(-17 as T, R!(0))
            .setup(5 as T, R!(1))
            .expect(-17 as T % 5 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        modulo_float_special,
        [(ModuloF32, f32), (ModuloF64, f64)],
        VmTest::new()
            .setup(5.5 as T, R!(0))
            .setup(2.0 as T, R!(1))
            .expect(5.5 as T % 2.0 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        modulo_float_zero,
        [(ModuloF32, f32), (ModuloF64, f64)],
        VmTest::new()
            .setup(5.5 as T, R!(0))
            .setup(0.0 as T, R!(1))
            .expect_error(VmErrorCode::FloatInvalidResult),
        (R!(2), R!(0), R!(1))
    );
}
