use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

// TODO: remove conversions, just copy bytes?
macro_rules! impl_mov_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!($opcode, (RegisterType, RegisterType), [<$opcode handler>]);

            #[inline(always)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (dest, source) = args;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        move_simple,
        [
            (MoveU8, u8),
            (MoveU16, u16),
            (MoveU32, u32),
            (MoveU64, u64),
            (MoveI8, i8),
            (MoveI16, i16),
            (MoveI32, i32),
            (MoveI64, i64)
        ],
        VmTest::new().setup(42 as T, R!(1)).expect(42 as T, R!(0)),
        (R!(0), R!(1))
    );

    define_vm_tests!(
        move_negative,
        [(MoveI8, i8), (MoveI16, i16), (MoveI32, i32), (MoveI64, i64)],
        VmTest::new()
            .setup(-123 as T, R!(1))
            .expect(-123 as T, R!(0)),
        (R!(0), R!(1))
    );

    define_vm_tests!(
        move_zero,
        [
            (MoveU8, u8),
            (MoveU16, u16),
            (MoveU32, u32),
            (MoveU64, u64),
            (MoveI8, i8),
            (MoveI16, i16),
            (MoveI32, i32),
            (MoveI64, i64),
            (MoveF32, f32),
            (MoveF64, f64)
        ],
        VmTest::new().setup(0 as T, R!(1)).expect(0 as T, R!(0)),
        (R!(0), R!(1))
    );

    define_vm_tests!(
        move_aliasing,
        [(MoveU8, u8), (MoveI8, i8), (MoveF32, f32)],
        VmTest::new().setup(42 as T, R!(0)).expect(42 as T, R!(0)),
        (R!(0), R!(0))
    );

    define_vm_tests!(
        move_float_basic,
        [(MoveF32, f32), (MoveF64, f64)],
        VmTest::new()
            .setup(1.618 as T, R!(1))
            .expect(1.618 as T, R!(0)),
        (R!(0), R!(1))
    );

    define_vm_tests!(
        move_float_extreme,
        [(MoveF32, f32), (MoveF64, f64)],
        VmTest::new().setup(T::MAX, R!(1)).expect(T::MAX, R!(0)),
        (R!(0), R!(1))
    );

    define_vm_tests!(
        move_float_nan_inf,
        [(MoveF32, f32), (MoveF64, f64)],
        VmTest::new()
            .setup(f32::INFINITY as T, R!(1))
            .expect(f32::INFINITY as T, R!(0)),
        (R!(0), R!(1))
    );
}
