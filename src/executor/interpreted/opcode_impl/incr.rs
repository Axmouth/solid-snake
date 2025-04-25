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
                    "Incr: R{} <= {} R{} ({}) + ({})",
                    dest,
                    stringify!($ty),
                    dest,
                    val,
                    incr_val
                );

                match val.checked_add(incr_val) {
                    Some(result) => {
                        executor
                            .registers_mut()
                            .set_register_value(dest, result)?;
                    }
                    None => {
                        executor
                            .registers_mut()
                            .set_register_value(dest, val.wrapping_add(incr_val))?;
                        executor.set_error(VmErrorCode::Overflow as i64);
                    }
                }

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
