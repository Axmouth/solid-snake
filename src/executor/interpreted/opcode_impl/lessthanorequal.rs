use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_lessthanorequal_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode handler>]);

            #[inline(always)]
            pub fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                look_ahead: &[u8],
            ) -> Result<(), VmExecutionError> {
                assert!(look_ahead.len() == [<$opcode Instruction>]::arg_size());
                let (dest, reg1, reg2) = [<$opcode Instruction>]::parse_args(look_ahead);

                let val1: $ty = executor.registers().get_register_value(reg1)?;
                let val2: $ty = executor.registers().get_register_value(reg2)?;

                debug!(
                    "LessThan: R{} <= {} R{} ({}) <= R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 <= val2) as u8;

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_lessthanorequal_instruction!(LessThanOrEqualI8, i8);
impl_lessthanorequal_instruction!(LessThanOrEqualI16, i16);
impl_lessthanorequal_instruction!(LessThanOrEqualI32, i32);
impl_lessthanorequal_instruction!(LessThanOrEqualI64, i64);
impl_lessthanorequal_instruction!(LessThanOrEqualU8, u8);
impl_lessthanorequal_instruction!(LessThanOrEqualU16, u16);
impl_lessthanorequal_instruction!(LessThanOrEqualU32, u32);
impl_lessthanorequal_instruction!(LessThanOrEqualU64, u64);

macro_rules! impl_lessthan_float_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType, RegisterType), [<$opcode:snake handler>]);

            #[inline(always)]
            pub fn [<$opcode:snake handler>](
            executor: &mut VmInterpretedExecutor,
            look_ahead: &[u8],
            ) -> Result<(), VmExecutionError> {
                assert!(look_ahead.len() == [<$opcode Instruction>]::arg_size());
                let (dest, reg1, reg2) = [<$opcode Instruction>]::parse_args(look_ahead);

                let val1: $ty = executor.registers().get_register_value(reg1)?;
                let val2: $ty = executor.registers().get_register_value(reg2)?;

                debug!(
                    "LessThan: R{} <= {} R{} ({}) <= R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 <= val2) as u8;
                // TODO: handle nan/inf?

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_lessthan_float_instruction!(LessThanOrEqualF32, f32);
impl_lessthan_float_instruction!(LessThanOrEqualF64, f64);
