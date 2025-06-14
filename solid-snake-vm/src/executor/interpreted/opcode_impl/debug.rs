use log::debug;
use paste::paste;

use crate::executor::{
    ext::VmExecutionError,
    interpreted::{
        implimentation::{RegisterFileExt, VmInterpretedExecutor},
        opcode_decoder::RegisterType,
    },
};

macro_rules! impl_dbg_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                concat!("Prints the value of a ", stringify!($ty), " register to stdout for debugging."),
                [
                    (source: RegisterType, "Register to print")
                ],
                [SideEffects],
                [<$opcode handler>]
            );


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

impl_dbg_instruction!(DebugPrintI8, i8);
impl_dbg_instruction!(DebugPrintI16, i16);
impl_dbg_instruction!(DebugPrintI32, i32);
impl_dbg_instruction!(DebugPrintI64, i64);
impl_dbg_instruction!(DebugPrintU8, u8);
impl_dbg_instruction!(DebugPrintU16, u16);
impl_dbg_instruction!(DebugPrintU32, u32);
impl_dbg_instruction!(DebugPrintU64, u64);
impl_dbg_instruction!(DebugPrintF32, f32);
impl_dbg_instruction!(DebugPrintF64, f64);

crate::define_instruction!(
    DebugPrintRaw,
    "Prints the raw 64-bit value of a register in hexadecimal for debugging.",
    [
        (reg: RegisterType, "Register to inspect as raw bits")
    ],
    [SideEffects],
    debug_print_raw
);

fn debug_print_raw(
    executor: &mut VmInterpretedExecutor,
    args: DebugPrintRawArgs,
) -> Result<(), VmExecutionError> {
    let (reg,) = args;
    let val = executor.registers().raw[usize::from(reg)];
    println!("R{} raw bits: {:#018X}", reg, val);
    Ok(())
}
