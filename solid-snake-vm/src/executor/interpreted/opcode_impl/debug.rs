use log::debug;
use paste::paste;

use crate::executor::{
    ext::VmExecutionError,
    interpreted::{
        implimentation::{RegisterFileExt, VmInterpretedExecutor},
        opcode_decoder::RegisterType,
    },
};

macro_rules! impl_mov_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType), [<$opcode handler>]);

            #[inline(always)]
            #[allow(non_snake_case)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (source, ) = args;

                let val: $ty = executor.registers().get_register_value(source)?;

                debug!("Debug: R{source}");

                println!("R{} : {}", source, val);

                Ok(())
            }
        }
    };
}

impl_mov_instruction!(DebugPrintI8, i8);
impl_mov_instruction!(DebugPrintI16, i16);
impl_mov_instruction!(DebugPrintI32, i32);
impl_mov_instruction!(DebugPrintI64, i64);
impl_mov_instruction!(DebugPrintU8, u8);
impl_mov_instruction!(DebugPrintU16, u16);
impl_mov_instruction!(DebugPrintU32, u32);
impl_mov_instruction!(DebugPrintU64, u64);
impl_mov_instruction!(DebugPrintF32, f32);
impl_mov_instruction!(DebugPrintF64, f64);

crate::define_instruction!(DebugPrintRaw, (RegisterType), debug_print_raw);

fn debug_print_raw(
    executor: &mut VmInterpretedExecutor,
    args: DebugPrintRawArgs,
) -> Result<(), VmExecutionError> {
    let (reg,) = args;
    let val = executor.registers().raw[usize::from(reg)];
    println!("R{} raw bits: {:#018X}", reg, val);
    Ok(())
}
