use log::debug;
use paste::paste;

use crate::executor::ext::{VmExecutionError, VmExecutorExt};
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::{RegisterType, VmErrorCode};

macro_rules! impl_add_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode handler>]);

            #[inline(always)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                look_ahead: &[u8],
            ) -> Result<(), VmExecutionError> {
                assert!(look_ahead.len() == [<$opcode Instruction>]::arg_size());
                let (dest, reg1, reg2) = [<$opcode Instruction>]::parse_args(look_ahead);

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

                match val1.checked_add(val2) {
                    Some(result) => {
                        executor
                            .registers_mut()
                            .set_register_value(dest, result)?;
                    }
                    None => {
                        executor
                            .registers_mut()
                            .set_register_value(dest, val1.wrapping_add(val2))?;
                        executor.set_error(VmErrorCode::Overflow as u64);
                    }
                }

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
        $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode handler>]);

        #[inline(always)]
        fn [<$opcode handler>](
            executor: &mut VmInterpretedExecutor,
            look_ahead: &[u8],
        ) -> Result<(), VmExecutionError> {
            assert!(look_ahead.len() == [<$opcode Instruction>]::arg_size());
            let (dest, reg1, reg2) = [<$opcode Instruction>]::parse_args(look_ahead);

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
                executor.set_error(VmErrorCode::FloatInvalidResult as u64);
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

