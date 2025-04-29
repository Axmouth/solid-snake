use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_bitwise_not_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                (RegisterType, RegisterType),
                [<$opcode handler>]
            );

            #[inline(always)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (dest, src) = args;
                let value: $ty = executor.registers().get_register_value(src)?;
                let result = !value;

                debug!(
                    "BitwiseNot: R{} <= {} ~R{} ({}) = {}",
                    dest,
                    stringify!($ty),
                    src,
                    value,
                    result
                );

                executor.registers_mut().set_register_value(dest, result)?;
                Ok(())
            }
        }
    };
}

impl_bitwise_not_instruction!(BitwiseNotU8, u8);
impl_bitwise_not_instruction!(BitwiseNotU16, u16);
impl_bitwise_not_instruction!(BitwiseNotU32, u32);
impl_bitwise_not_instruction!(BitwiseNotU64, u64);
impl_bitwise_not_instruction!(BitwiseNotI8, i8);
impl_bitwise_not_instruction!(BitwiseNotI16, i16);
impl_bitwise_not_instruction!(BitwiseNotI32, i32);
impl_bitwise_not_instruction!(BitwiseNotI64, i64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::interpreted::opcode_impl::all::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        bitwise_not_unsigned,
        [
            (BitwiseNotU8, u8),
            (BitwiseNotU16, u16),
            (BitwiseNotU32, u32),
            (BitwiseNotU64, u64)
        ],
        VmTest::new()
            .setup(0b10101010 as T, R!(1))
            .expect(!0b10101010 as T, R!(0)),
        (R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_not_signed,
        [
            (BitwiseNotI8, i8),
            (BitwiseNotI16, i16),
            (BitwiseNotI32, i32),
            (BitwiseNotI64, i64)
        ],
        VmTest::new()
            .setup(0b00001111 as T, R!(1))
            .expect(!0b00001111 as T, R!(0)),
        (R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_not_zero,
        [(BitwiseNotU8, u8), (BitwiseNotI8, i8)],
        VmTest::new().setup(0 as T, R!(1)).expect(!0 as T, R!(0)),
        (R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_not_all_ones_unsigned,
        [(BitwiseNotU8, u8)],
        VmTest::new()
            .setup(0xFF as T, R!(1))
            .expect(0x00 as T, R!(0)),
        (R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_not_all_ones_signed,
        [(BitwiseNotI8, i8)],
        VmTest::new()
            .setup(-1 as T, R!(1)) // all bits set
            .expect(0 as T, R!(0)), // result is 0
        (R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_not_same_register,
        [(BitwiseNotU8, u8), (BitwiseNotI8, i8)],
        VmTest::new()
            .setup(0b01010101 as T, R!(0))
            .expect(!0b01010101 as T, R!(0)),
        (R!(0), R!(0))
    );
}
