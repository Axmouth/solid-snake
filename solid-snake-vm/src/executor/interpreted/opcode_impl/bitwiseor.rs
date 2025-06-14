use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_bitwise_or_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                concat!("Performs a bitwise OR between two ", stringify!($ty), " registers and stores the result."),
                [
                    (dest: RegisterType, "Destination register"),
                    (r1: RegisterType, "First operand"),
                    (r2: RegisterType, "Second operand")
                ],
                [Arithmetic, Logical, Pure, Commutative],
                [<$opcode handler>]
            );

            #[inline(always)]
            #[allow(non_snake_case)]
            fn [<$opcode handler>](
                executor: &mut VmInterpretedExecutor,
                args: [<$opcode Args>],
            ) -> Result<(), VmExecutionError> {
                let (dest, r1, r2) = args;
                let v1: $ty = executor.registers().get_register_value(r1)?;
                let v2: $ty = executor.registers().get_register_value(r2)?;

                let result = v1 | v2;

                debug!(
                    "BitwiseOr: R{} <= {} R{} ({}) | R{} ({}) = {}",
                    dest,
                    stringify!($ty),
                    r1,
                    v1,
                    r2,
                    v2,
                    result
                );

                executor.registers_mut().set_register_value(dest, result)?;
                Ok(())
            }
        }
    };
}

impl_bitwise_or_instruction!(BitwiseOrU8, u8);
impl_bitwise_or_instruction!(BitwiseOrU16, u16);
impl_bitwise_or_instruction!(BitwiseOrU32, u32);
impl_bitwise_or_instruction!(BitwiseOrU64, u64);
impl_bitwise_or_instruction!(BitwiseOrI8, i8);
impl_bitwise_or_instruction!(BitwiseOrI16, i16);
impl_bitwise_or_instruction!(BitwiseOrI32, i32);
impl_bitwise_or_instruction!(BitwiseOrI64, i64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        bitwise_or_unsigned,
        [
            (BitwiseOrU8, u8),
            (BitwiseOrU16, u16),
            (BitwiseOrU32, u32),
            (BitwiseOrU64, u64)
        ],
        VmTest::new()
            .setup(0b10100000 as T, R!(0))
            .setup(0b00001111 as T, R!(1))
            .expect((0b10100000 as T) | (0b00001111 as T), R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_or_signed,
        [
            (BitwiseOrI8, i8),
            (BitwiseOrI16, i16),
            (BitwiseOrI32, i32),
            (BitwiseOrI64, i64)
        ],
        VmTest::new()
            .setup(0b00000101 as T, R!(0)) // safe for i8
            .setup(0b00000011 as T, R!(1))
            .expect((0b00000101 as T) | (0b00000011 as T), R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_or_zero,
        [(BitwiseOrU8, u8)],
        VmTest::new()
            .setup(0b00000000 as T, R!(0))
            .setup(0b11110000 as T, R!(1))
            .expect(0b11110000 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_or_zero,
        [(BitwiseOrI8, i8)],
        VmTest::new()
            .setup(0b0000000 as T, R!(0))
            .setup(-0b1110000 as T, R!(1))
            .expect(-0b1110000 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_or_all_ones_unsigned,
        [(BitwiseOrU8, u8), (BitwiseOrU16, u16)],
        VmTest::new()
            .setup(!0 as T, R!(0))
            .setup(0b00000000 as T, R!(1))
            .expect(!0 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_or_all_ones_signed,
        [(BitwiseOrI8, i8), (BitwiseOrI16, i16)],
        VmTest::new()
            .setup(!0 as T, R!(0))
            .setup(0b00000000 as T, R!(1))
            .expect(!0 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_or_aliasing_output_input1,
        [(BitwiseOrU8, u8), (BitwiseOrI16, i16)],
        VmTest::new()
            .setup(0b00000001 as T, R!(0))
            .setup(0b00000100 as T, R!(1))
            .expect(0b00000101 as T, R!(0)),
        (R!(0), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_or_aliasing_output_input2,
        [(BitwiseOrU8, u8), (BitwiseOrI32, i32)],
        VmTest::new()
            .setup(0b00000100 as T, R!(0))
            .setup(0b00001000 as T, R!(1))
            .expect(0b00001100 as T, R!(1)),
        (R!(1), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_or_same_registers,
        [(BitwiseOrU8, u8), (BitwiseOrI8, i8)],
        VmTest::new()
            .setup(0b01010101 as T, R!(0))
            .expect(0b01010101 as T, R!(0)),
        (R!(0), R!(0), R!(0))
    );
}
