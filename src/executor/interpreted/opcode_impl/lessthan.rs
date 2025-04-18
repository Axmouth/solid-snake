use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_lessthan_instruction {
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
                    "LessThan: R{} <= {} R{} ({}) < R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 < val2) as u8;

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_lessthan_instruction!(LessThanI8, i8);
impl_lessthan_instruction!(LessThanI16, i16);
impl_lessthan_instruction!(LessThanI32, i32);
impl_lessthan_instruction!(LessThanI64, i64);
impl_lessthan_instruction!(LessThanU8, u8);
impl_lessthan_instruction!(LessThanU16, u16);
impl_lessthan_instruction!(LessThanU32, u32);
impl_lessthan_instruction!(LessThanU64, u64);

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
                    "LessThan: R{} <= {} R{} ({}) < R{} ({})",
                    dest,
                    stringify!($ty),
                    reg1,
                    val1,
                    reg2,
                    val2
                );

                let result: u8 = (val1 < val2) as u8;
                // TODO: handle nan/inf?

                executor
                    .registers_mut()
                    .set_register_value(dest, result)?;

                Ok(())
            }
        }
    };
}

impl_lessthan_float_instruction!(LessThanF32, f32);
impl_lessthan_float_instruction!(LessThanF64, f64);
