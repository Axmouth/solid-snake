use log::debug;
use paste::paste;

use crate::executor::ext::{VmExecutionError, VmExecutorExt};
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::{RegisterType, VmErrorCode};

macro_rules! impl_incr_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, $ty), [<$opcode handler>]);

            #[inline(always)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                look_ahead: &[u8],
            ) -> Result<(), VmExecutionError> {
                assert!(look_ahead.len() == [<$opcode Instruction>]::arg_size());
                let (dest, incr_val) = [<$opcode Instruction>]::parse_args(look_ahead);

                let val: $ty = executor.registers().get_register_value(dest)?;

                debug!(
                    "Decr: R{} <= {} R{} ({}) - ({})",
                    dest,
                    stringify!($ty),
                    dest,
                    val,
                    incr_val
                );

                match val.checked_sub(incr_val) {
                    Some(result) => {
                        executor
                            .registers_mut()
                            .set_register_value(dest, result)?;
                    }
                    None => {
                        executor
                            .registers_mut()
                            .set_register_value(dest, val.wrapping_sub(incr_val))?;
                        executor.set_error(VmErrorCode::Overflow as u64);
                    }
                }

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
            $crate::define_instruction!($opcode, (RegisterType, $ty), [<$opcode handler>]);

        #[inline(always)]
        fn [<$opcode handler>](
            executor: &mut VmInterpretedExecutor,
            look_ahead: &[u8],
        ) -> Result<(), VmExecutionError> {
            assert!(look_ahead.len() == [<$opcode Instruction>]::arg_size());
            let (dest, incr_val) = [<$opcode Instruction>]::parse_args(look_ahead);

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
                executor.set_error(VmErrorCode::FloatInvalidResult as u64);
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
