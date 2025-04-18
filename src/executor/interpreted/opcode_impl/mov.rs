use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

// TODO: remove conversions, just copy bytes.
macro_rules! impl_mov_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType), [<$opcode handler>]);

            #[inline(always)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                look_ahead: &[u8],
            ) -> Result<(), VmExecutionError> {
                assert!(look_ahead.len() == [<$opcode Instruction>]::arg_size());
                let (dest, source) = [<$opcode Instruction>]::parse_args(look_ahead);

                let val: $ty = executor.registers().get_register_value(source)?;

                debug!(
                    "Move: R{} <= {} R{} ({})",
                    dest,
                    stringify!($ty),
                    source,
                    val,
                );

                executor
                    .registers_mut()
                    .set_register_value(dest, val)?;

                Ok(())
            }
        }
    };
}

impl_mov_instruction!(MoveI8, i8);
impl_mov_instruction!(MoveI16, i16);
impl_mov_instruction!(MoveI32, i32);
impl_mov_instruction!(MoveI64, i64);
impl_mov_instruction!(MoveU8, u8);
impl_mov_instruction!(MoveU16, u16);
impl_mov_instruction!(MoveU32, u32);
impl_mov_instruction!(MoveU64, u64);
impl_mov_instruction!(MoveF32, f32);
impl_mov_instruction!(MoveF64, f64);
