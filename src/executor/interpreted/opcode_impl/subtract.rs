use log::debug;
use paste::paste;

use crate::executor::ext::{VmExecutionError, VmExecutorExt};
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::{RegisterType, VmErrorCode};

macro_rules! impl_subtract_instruction {
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
                    "Subtract: R{} <= {} R{} ({}) - R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                match val1.checked_sub(val2) {
                    Some(result) => {
                        executor
                            .registers_mut()
                            .set_register_value(dest, result)?;
                    }
                    None => {
                        executor
                            .registers_mut()
                            .set_register_value(dest, val1.wrapping_sub(val2))?;
                        executor.set_error(VmErrorCode::Underflow as u64);
                    }
                }

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
            look_ahead: &[u8],
        ) -> Result<(), VmExecutionError> {
            assert!(look_ahead.len() == [<$opcode Instruction>]::arg_size());
            let (dest, reg1, reg2) = [<$opcode Instruction>]::parse_args(look_ahead);

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
                executor.set_error(VmErrorCode::FloatInvalidResult as u64);
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
