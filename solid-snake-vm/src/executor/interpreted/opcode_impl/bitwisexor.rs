use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_bitwise_xor_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                concat!("Performs a bitwise XOR between two ", stringify!($ty), " registers and stores the result."),
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

                let result = v1 ^ v2;

                debug!(
                    "BitwiseXor: R{} <= {} R{} ({}) ^ R{} ({}) = {}",
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

impl_bitwise_xor_instruction!(BitwiseXorU8, u8);
impl_bitwise_xor_instruction!(BitwiseXorU16, u16);
impl_bitwise_xor_instruction!(BitwiseXorU32, u32);
impl_bitwise_xor_instruction!(BitwiseXorU64, u64);
impl_bitwise_xor_instruction!(BitwiseXorI8, i8);
impl_bitwise_xor_instruction!(BitwiseXorI16, i16);
impl_bitwise_xor_instruction!(BitwiseXorI32, i32);
impl_bitwise_xor_instruction!(BitwiseXorI64, i64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    define_vm_tests!(
        bitwise_xor_unsigned,
        [
            (BitwiseXorU8, u8),
            (BitwiseXorU16, u16),
            (BitwiseXorU32, u32),
            (BitwiseXorU64, u64)
        ],
        VmTest::new()
            .setup(0b10100000 as T, R!(0))
            .setup(0b11110000 as T, R!(1))
            .expect((0b10100000 as T) ^ (0b11110000 as T), R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_xor_signed,
        [
            (BitwiseXorI8, i8),
            (BitwiseXorI16, i16),
            (BitwiseXorI32, i32),
            (BitwiseXorI64, i64)
        ],
        VmTest::new()
            .setup(0b00001111 as T, R!(0))
            .setup(0b00000101 as T, R!(1))
            .expect((0b00001111 as T) ^ (0b00000101 as T), R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_xor_zero,
        [(BitwiseXorU8, u8)],
        VmTest::new()
            .setup(0b10101010 as T, R!(0))
            .setup(0 as T, R!(1))
            .expect(0b10101010 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_xor_zero_i8,
        [(BitwiseXorI8, i8)],
        VmTest::new()
            .setup(0b00001010 as T, R!(0))
            .setup(0 as T, R!(1))
            .expect(0b00001010 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_xor_self,
        [(BitwiseXorU8, u8)],
        VmTest::new()
            .setup(0b11111111 as T, R!(0))
            .expect(0 as T, R!(0)),
        (R!(0), R!(0), R!(0))
    );

    define_vm_tests!(
        bitwise_xor_self,
        [(BitwiseXorI8, i8)],
        VmTest::new()
            .setup(-0b1111111 as T, R!(0))
            .expect(0 as T, R!(0)),
        (R!(0), R!(0), R!(0))
    );

    define_vm_tests!(
        bitwise_xor_aliasing_output_input1,
        [(BitwiseXorU8, u8)],
        VmTest::new()
            .setup(0b10100000 as T, R!(0))
            .setup(0b11000000 as T, R!(1))
            .expect((0b10100000 as T) ^ (0b11000000 as T), R!(0)),
        (R!(0), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_xor_aliasing_output_input2,
        [(BitwiseXorI16, i16)],
        VmTest::new()
            .setup(0b01010101 as T, R!(0))
            .setup(0b00110011 as T, R!(1))
            .expect((0b01010101 as T) ^ (0b00110011 as T), R!(1)),
        (R!(1), R!(0), R!(1))
    );
}
