use log::debug;
use paste::paste;

use crate::executor::ext::VmExecutionError;
use crate::executor::interpreted::implimentation::{RegisterFileExt, VmInterpretedExecutor};
use crate::executor::interpreted::opcode_decoder::RegisterType;

macro_rules! impl_bitwise_and_instruction {
    ($opcode:ident, $ty:ty) => {
        paste! {
            $crate::define_instruction!(
                $opcode,
                concat!("Performs a bitwise AND between two ", stringify!($ty), " registers and stores the result."),
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

                let result = v1 & v2;

                debug!(
                    "BitwiseAnd: R{} <= {} R{} ({}) & R{} ({}) = {}",
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

impl_bitwise_and_instruction!(BitwiseAndU8, u8);
impl_bitwise_and_instruction!(BitwiseAndU16, u16);
impl_bitwise_and_instruction!(BitwiseAndU32, u32);
impl_bitwise_and_instruction!(BitwiseAndU64, u64);
impl_bitwise_and_instruction!(BitwiseAndI8, i8);
impl_bitwise_and_instruction!(BitwiseAndI16, i16);
impl_bitwise_and_instruction!(BitwiseAndI32, i32);
impl_bitwise_and_instruction!(BitwiseAndI64, i64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{R, define_vm_tests};

    // Unsigned types can use full binary masks
    define_vm_tests!(
        bitwise_and_unsigned,
        [
            (BitwiseAndU8, u8),
            (BitwiseAndU16, u16),
            (BitwiseAndU32, u32),
            (BitwiseAndU64, u64)
        ],
        VmTest::new()
            .setup(0b10101010 as T, R!(0))
            .setup(0b11001100 as T, R!(1))
            .expect((0b10101010 as T) & (0b11001100 as T), R!(2)),
        (R!(2), R!(0), R!(1))
    );

    // Signed types must use small positive literals
    define_vm_tests!(
        bitwise_and_signed,
        [
            (BitwiseAndI8, i8),
            (BitwiseAndI16, i16),
            (BitwiseAndI32, i32),
            (BitwiseAndI64, i64)
        ],
        VmTest::new()
            .setup(0b00001111 as T, R!(0)) // <= safe for i8
            .setup(0b00111100 as T, R!(1))
            .expect((0b00001111 as T) & (0b00111100 as T), R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_and_zero_unsigned,
        [(BitwiseAndU8, u8), (BitwiseAndU32, u32)],
        VmTest::new()
            .setup(0b10101010 as T, R!(0))
            .setup(0 as T, R!(1))
            .expect(0 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_and_zero_signed,
        [(BitwiseAndI8, i8), (BitwiseAndI32, i32)],
        VmTest::new()
            .setup(0b00001111 as T, R!(0))
            .setup(0 as T, R!(1))
            .expect(0 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_and_all_ones_unsigned,
        [(BitwiseAndU8, u8), (BitwiseAndU16, u16)],
        VmTest::new()
            .setup(!0 as T, R!(0))
            .setup(0b11110000 as T, R!(1))
            .expect(0b11110000 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_and_all_ones_signed,
        [(BitwiseAndI8, i8), (BitwiseAndI16, i16)],
        VmTest::new()
            .setup(!0 as T, R!(0))
            .setup(0b00001111 as T, R!(1))
            .expect(0b00001111 as T, R!(2)),
        (R!(2), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_and_aliasing_output_input1,
        [(BitwiseAndU8, u8), (BitwiseAndI16, i16)],
        VmTest::new()
            .setup(0b11110000 as T, R!(0))
            .setup(0b11001100 as T, R!(1))
            .expect((0b11110000 as T) & (0b11001100 as T), R!(0)),
        (R!(0), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_and_aliasing_output_input2,
        [(BitwiseAndU8, u8), (BitwiseAndI32, i32)],
        VmTest::new()
            .setup(0b10101010 as T, R!(0))
            .setup(0b11110000 as T, R!(1))
            .expect((0b10101010 as T) & (0b11110000 as T), R!(1)),
        (R!(1), R!(0), R!(1))
    );

    define_vm_tests!(
        bitwise_and_same_registers,
        [(BitwiseAndU8, u8), (BitwiseAndI8, i8)],
        VmTest::new()
            .setup(0b00001111 as T, R!(0))
            .expect(0b00001111 as T, R!(0)),
        (R!(0), R!(0), R!(0))
    );
}
