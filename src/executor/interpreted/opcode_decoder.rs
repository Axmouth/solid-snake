use std::str::FromStr;

use paste::paste;

use crate::executor::{ext::VmExecutionError, interpreted::implimentation::{MAX_REGISTERS, VmInterpretedExecutor}};
use crate::opcodes::OpCode;

pub const MAX_OPCODES: usize = 65536;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegisterType(u8);
pub const SIZE_OF_REGISTER: usize = size_of::<RegisterType>();

#[derive(Debug)]
pub enum VmParseError {
    InvalidRegister(String),
    InvalidArgument(String),
    UnknownInstruction(String),
    WrongArgumentCount { expected: usize, got: usize },
}

impl FromStr for RegisterType {
    type Err = VmParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix('R') {
            let idx = stripped.parse::<u8>()
                .map_err(|_| VmParseError::InvalidRegister(s.to_string()))?;
            if (idx as usize) >= MAX_REGISTERS {
                return Err(VmParseError::InvalidRegister(format!("Register index {} out of range", idx)));
            }
            Ok(RegisterType::from(idx))
        } else {
            Err(VmParseError::InvalidRegister(s.to_string()))
        }
    }
}


impl From<u8> for RegisterType {
    fn from(value: u8) -> Self {
        RegisterType(value)
    }
}

impl From<RegisterType> for u8 {
    fn from(RegisterType(value): RegisterType) -> Self {
        value
    }
}

impl From<RegisterType> for usize {
    fn from(RegisterType(value): RegisterType) -> Self {
        value as usize
    }
}

impl std::fmt::Display for RegisterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl RegisterType {
    pub fn to_be_bytes(self) -> [u8; SIZE_OF_REGISTER] {
        self.0.to_be_bytes()
    }
}

impl FromBytes for RegisterType {
    fn from_be_bytes(bytes: &[u8]) -> Self {
        RegisterType(u8::from_be_bytes(bytes.try_into().expect("Invalid bytes for RegisterType")))
    }

    fn from_le_bytes(bytes: &[u8]) -> Self {
        RegisterType(u8::from_le_bytes(bytes.try_into().expect("Invalid bytes for RegisterType")))
    }
}

impl ToBytes for RegisterType {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
    
    fn to_le_bytes(&self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }
}

pub trait RegisterValue {
    fn to_u64(&self) -> u64;

    fn from_u64(val: u64) -> Self;
}

pub trait VmParse: Sized {
    fn parse_vm(s: &str) -> Result<Self, VmParseError>;
}

macro_rules! impl_vm_parse_for_ints {
    ($($ty:ty),*) => {
        $(
            impl VmParse for $ty {
                fn parse_vm(s: &str) -> Result<Self, VmParseError> {
                    if let Some(stripped) = s.strip_prefix("0x") {
                        <$ty>::from_str_radix(stripped, 16)
                    } else if let Some(stripped) = s.strip_prefix("0b") {
                        <$ty>::from_str_radix(stripped, 2)
                    } else if let Some(stripped) = s.strip_prefix("0o") {
                        <$ty>::from_str_radix(stripped, 8)
                    } else {
                        s.parse::<$ty>()
                    }
                    .map_err(|_| VmParseError::InvalidArgument(s.to_string()))
                }
            }
        )*
    };
}

impl_vm_parse_for_ints!(
    u8, u16, u32, u64,
    i8, i16, i32, i64
);

macro_rules! impl_vm_parse_for_floats {
    ($($ty:ty),*) => {
        $(
            impl VmParse for $ty {
                fn parse_vm(s: &str) -> Result<Self, VmParseError> {
                    s.parse::<$ty>()
                        .map_err(|_| VmParseError::InvalidArgument(s.to_string()))
                }
            }
        )*
    };
}

impl_vm_parse_for_floats!(f32, f64);

impl VmParse for RegisterType {
    fn parse_vm(s: &str) -> Result<Self, VmParseError> {
        s.parse()
    }
}

pub trait InstructionArgsFromStr: Sized {
    fn parse_from_strs(strs: &[&str]) -> Result<Self, VmParseError>;
    fn encode_from_strs(strs: &[&str]) -> Result<Vec<u8>, VmParseError>;
}

macro_rules! count_args {
    () => { 0 };
    ($name:ident) => { 1 };
    ($first:ident, $($rest:ident),*) => {
        1 + count_args!($($rest),*)
    }
}

macro_rules! impl_instruction_args_from_str {
    () => {
        impl InstructionArgsFromStr for () {
            fn parse_from_strs(_strs: &[&str]) -> Result<Self, VmParseError> {
                Ok(())
            }
            
            fn encode_from_strs(_: &[&str]) -> Result<Vec<u8>, VmParseError> {
                Ok(vec![])
            }
        }
    };
    ($($T:ident),*) => {
        impl<$($T),*> InstructionArgsFromStr for ($($T,)*)
        where
            $($T: VmParse + FromBytes + ToBytes),*
        {
            fn parse_from_strs(args: &[&str]) -> Result<Self, VmParseError> {
                match args {
                    [$($T),*] => Ok(($($T::parse_vm($T)?,)*)),
                    _ => Err(VmParseError::WrongArgumentCount { expected: count_args!($($T),*), got: args.len() }),
                }
            }
            
            fn encode_from_strs(strs: &[&str]) -> Result<Vec<u8>, VmParseError> {
                Ok(Self::encode(Self::parse_from_strs(strs)?))
            }
        }
    };
}

impl_instruction_args_from_str!();
impl_instruction_args_from_str!(Arg1);
impl_instruction_args_from_str!(Arg1, Arg2);
impl_instruction_args_from_str!(Arg1, Arg2, Arg3);
impl_instruction_args_from_str!(Arg1, Arg2, Arg3, Arg4);
impl_instruction_args_from_str!(Arg1, Arg2, Arg3, Arg4, Arg5);


fn invalidop(_: &mut VmInterpretedExecutor, _: &[u8]) -> Result<(), VmExecutionError> {
    Err(VmExecutionError::InvalidOpCode)
}

macro_rules! add_instr {
    ($table:ident, $name:ident) => {
        paste!{
            $table[OpCode::$name as usize] = ([<$name Instruction>]::handler, <[<$name Args>] as InstructionArgsFromStr>::encode_from_strs, [<$name Instruction>]::arg_size());
        }
    }
}

pub fn initialize_dispatch_table() -> Vec<(OpcodeHandler, ParseHandler, usize)> {
    let mut dispatch_table: Vec<(OpcodeHandler, ParseHandler, usize)> = Vec::new();
    dispatch_table.resize(MAX_OPCODES, (invalidop, <()>::encode_from_strs, 0));

    // Assign function pointers for specific opcodes
    // Add other opcode handlers as needed
    use crate::executor::interpreted::opcode_impl::add::*;
    add_instr!(dispatch_table, AddI8);
    add_instr!(dispatch_table, AddI16);
    add_instr!(dispatch_table, AddI32);
    add_instr!(dispatch_table, AddI64);
    add_instr!(dispatch_table, AddU8);
    add_instr!(dispatch_table, AddU16);
    add_instr!(dispatch_table, AddU32);
    add_instr!(dispatch_table, AddU64);
    add_instr!(dispatch_table, AddF32);
    add_instr!(dispatch_table, AddF64);

    use crate::executor::interpreted::opcode_impl::subtract::*;
    add_instr!(dispatch_table, SubtractI8);
    add_instr!(dispatch_table, SubtractI16);
    add_instr!(dispatch_table, SubtractI32);
    add_instr!(dispatch_table, SubtractI64);
    add_instr!(dispatch_table, SubtractU8);
    add_instr!(dispatch_table, SubtractU16);
    add_instr!(dispatch_table, SubtractU32);
    add_instr!(dispatch_table, SubtractU64);
    add_instr!(dispatch_table, SubtractF32);
    add_instr!(dispatch_table, SubtractF64);

    use crate::executor::interpreted::opcode_impl::jump::*;
    add_instr!(dispatch_table, Jump);
    add_instr!(dispatch_table, JumpIf);
    add_instr!(dispatch_table, JumpIfFalse);

    use crate::executor::interpreted::opcode_impl::lessthan::*;
    add_instr!(dispatch_table, LessThanI8);
    add_instr!(dispatch_table, LessThanI16);
    add_instr!(dispatch_table, LessThanI32);
    add_instr!(dispatch_table, LessThanI64);
    add_instr!(dispatch_table, LessThanU8);
    add_instr!(dispatch_table, LessThanU16);
    add_instr!(dispatch_table, LessThanU32);
    add_instr!(dispatch_table, LessThanU64);
    add_instr!(dispatch_table, LessThanF32);
    add_instr!(dispatch_table, LessThanF64);

    use crate::executor::interpreted::opcode_impl::lessthanorequal::*;
    add_instr!(dispatch_table, LessThanOrEqualI8);
    add_instr!(dispatch_table, LessThanOrEqualI16);
    add_instr!(dispatch_table, LessThanOrEqualI32);
    add_instr!(dispatch_table, LessThanOrEqualI64);
    add_instr!(dispatch_table, LessThanOrEqualU8);
    add_instr!(dispatch_table, LessThanOrEqualU16);
    add_instr!(dispatch_table, LessThanOrEqualU32);
    add_instr!(dispatch_table, LessThanOrEqualU64);
    add_instr!(dispatch_table, LessThanOrEqualF32);
    add_instr!(dispatch_table, LessThanOrEqualF64);

    use crate::executor::interpreted::opcode_impl::load::*;
    add_instr!(dispatch_table, LoadImmediateI8);
    add_instr!(dispatch_table, LoadImmediateI16);
    add_instr!(dispatch_table, LoadImmediateI32);
    add_instr!(dispatch_table, LoadImmediateI64);
    add_instr!(dispatch_table, LoadImmediateU8);
    add_instr!(dispatch_table, LoadImmediateU16);
    add_instr!(dispatch_table, LoadImmediateU32);
    add_instr!(dispatch_table, LoadImmediateU64);
    add_instr!(dispatch_table, LoadImmediateF32);
    add_instr!(dispatch_table, LoadImmediateF64);

    add_instr!(dispatch_table, LoadIndirectI8);
    add_instr!(dispatch_table, LoadIndirectI16);
    add_instr!(dispatch_table, LoadIndirectI32);
    add_instr!(dispatch_table, LoadIndirectI64);
    add_instr!(dispatch_table, LoadIndirectU8);
    add_instr!(dispatch_table, LoadIndirectU16);
    add_instr!(dispatch_table, LoadIndirectU32);
    add_instr!(dispatch_table, LoadIndirectU64);
    add_instr!(dispatch_table, LoadIndirectF32);
    add_instr!(dispatch_table, LoadIndirectF64);

    add_instr!(dispatch_table, LoadIndirectWithOffsetI8);
    add_instr!(dispatch_table, LoadIndirectWithOffsetI16);
    add_instr!(dispatch_table, LoadIndirectWithOffsetI32);
    add_instr!(dispatch_table, LoadIndirectWithOffsetI64);
    add_instr!(dispatch_table, LoadIndirectWithOffsetU8);
    add_instr!(dispatch_table, LoadIndirectWithOffsetU16);
    add_instr!(dispatch_table, LoadIndirectWithOffsetU32);
    add_instr!(dispatch_table, LoadIndirectWithOffsetU64);
    add_instr!(dispatch_table, LoadIndirectWithOffsetF32);
    add_instr!(dispatch_table, LoadIndirectWithOffsetF64);

    add_instr!(dispatch_table, LoadFromImmediateI8);
    add_instr!(dispatch_table, LoadFromImmediateI16);
    add_instr!(dispatch_table, LoadFromImmediateI32);
    add_instr!(dispatch_table, LoadFromImmediateI64);
    add_instr!(dispatch_table, LoadFromImmediateU8);
    add_instr!(dispatch_table, LoadFromImmediateU16);
    add_instr!(dispatch_table, LoadFromImmediateU32);
    add_instr!(dispatch_table, LoadFromImmediateU64);
    add_instr!(dispatch_table, LoadFromImmediateF32);
    add_instr!(dispatch_table, LoadFromImmediateF64);

    use crate::executor::interpreted::opcode_impl::store::*;
    add_instr!(dispatch_table, StoreIndirectWithOffsetI8);
    add_instr!(dispatch_table, StoreIndirectWithOffsetI16);
    add_instr!(dispatch_table, StoreIndirectWithOffsetI32);
    add_instr!(dispatch_table, StoreIndirectWithOffsetI64);
    add_instr!(dispatch_table, StoreIndirectWithOffsetU8);
    add_instr!(dispatch_table, StoreIndirectWithOffsetU16);
    add_instr!(dispatch_table, StoreIndirectWithOffsetU32);
    add_instr!(dispatch_table, StoreIndirectWithOffsetU64);
    add_instr!(dispatch_table, StoreIndirectWithOffsetF32);
    add_instr!(dispatch_table, StoreIndirectWithOffsetF64);

    add_instr!(dispatch_table, StoreFromImmediateWithOffsetI8);
    add_instr!(dispatch_table, StoreFromImmediateWithOffsetI16);
    add_instr!(dispatch_table, StoreFromImmediateWithOffsetI32);
    add_instr!(dispatch_table, StoreFromImmediateWithOffsetI64);
    add_instr!(dispatch_table, StoreFromImmediateWithOffsetU8);
    add_instr!(dispatch_table, StoreFromImmediateWithOffsetU16);
    add_instr!(dispatch_table, StoreFromImmediateWithOffsetU32);
    add_instr!(dispatch_table, StoreFromImmediateWithOffsetU64);
    add_instr!(dispatch_table, StoreFromImmediateWithOffsetF32);
    add_instr!(dispatch_table, StoreFromImmediateWithOffsetF64);

    use crate::executor::interpreted::opcode_impl::mov::*;
    add_instr!(dispatch_table, MoveI8);
    add_instr!(dispatch_table, MoveI16);
    add_instr!(dispatch_table, MoveI32);
    add_instr!(dispatch_table, MoveI64);
    add_instr!(dispatch_table, MoveU8);
    add_instr!(dispatch_table, MoveU16);
    add_instr!(dispatch_table, MoveU32);
    add_instr!(dispatch_table, MoveU64);
    add_instr!(dispatch_table, MoveF32);
    add_instr!(dispatch_table, MoveF64);

    use crate::executor::interpreted::opcode_impl::incr::*;
    add_instr!(dispatch_table, IncrementI8);
    add_instr!(dispatch_table, IncrementI16);
    add_instr!(dispatch_table, IncrementI32);
    add_instr!(dispatch_table, IncrementI64);
    add_instr!(dispatch_table, IncrementU8);
    add_instr!(dispatch_table, IncrementU16);
    add_instr!(dispatch_table, IncrementU32);
    add_instr!(dispatch_table, IncrementU64);
    add_instr!(dispatch_table, IncrementF32);
    add_instr!(dispatch_table, IncrementF64);

    use crate::executor::interpreted::opcode_impl::decr::*;
    add_instr!(dispatch_table, DecrementI8);
    add_instr!(dispatch_table, DecrementI16);
    add_instr!(dispatch_table, DecrementI32);
    add_instr!(dispatch_table, DecrementI64);
    add_instr!(dispatch_table, DecrementU8);
    add_instr!(dispatch_table, DecrementU16);
    add_instr!(dispatch_table, DecrementU32);
    add_instr!(dispatch_table, DecrementU64);
    add_instr!(dispatch_table, DecrementF32);
    add_instr!(dispatch_table, DecrementF64);

    use crate::executor::interpreted::opcode_impl::function::*;
    add_instr!(dispatch_table, CallFunction);
    add_instr!(dispatch_table, Return);
    add_instr!(dispatch_table, Halt);

    use crate::executor::interpreted::opcode_impl::memory::*;
    add_instr!(dispatch_table, Allocate);
    add_instr!(dispatch_table, Deallocate);
    add_instr!(dispatch_table, MemSet);
    add_instr!(dispatch_table, Memcpy);

    use crate::executor::interpreted::opcode_impl::debug::*;
    add_instr!(dispatch_table, DebugPrintI8);
    add_instr!(dispatch_table, DebugPrintI16);
    add_instr!(dispatch_table, DebugPrintI32);
    add_instr!(dispatch_table, DebugPrintI64);
    add_instr!(dispatch_table, DebugPrintU8);
    add_instr!(dispatch_table, DebugPrintU16);
    add_instr!(dispatch_table, DebugPrintU32);
    add_instr!(dispatch_table, DebugPrintU64);
    add_instr!(dispatch_table, DebugPrintF32);
    add_instr!(dispatch_table, DebugPrintF64);
    add_instr!(dispatch_table, DebugPrintRaw);

    dispatch_table
}

pub type OpcodeHandler =
    fn(&mut VmInterpretedExecutor, look_ahead: &[u8]) -> Result<(), VmExecutionError>;

pub type ParseHandler =
    fn(strs: &[&str]) -> Result<Vec<u8>, VmParseError>;

pub struct OpCodeDecoder {}

pub trait FromBytes: Sized {
    fn from_be_bytes(bytes: &[u8]) -> Self;
    fn from_le_bytes(bytes: &[u8]) -> Self;
}

pub trait ToBytes: Sized {
    fn to_be_bytes(&self) -> Vec<u8>;
    fn to_le_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_from_bytes {
    ($($t:ty),*) => {
        $(
            impl FromBytes for $t {
                fn from_be_bytes(bytes: &[u8]) -> Self {
                    assert_eq!(bytes.len(), std::mem::size_of::<$t>());
                    <$t>::from_be_bytes(bytes.try_into().unwrap())
                }

                fn from_le_bytes(bytes: &[u8]) -> Self {
                    assert_eq!(bytes.len(), std::mem::size_of::<$t>());
                    <$t>::from_le_bytes(bytes.try_into().unwrap())
                }
            }
        )*
    };
}

macro_rules! impl_to_bytes {
    ($($t:ty),*) => {
        $(
            impl ToBytes for $t {
                fn to_be_bytes(&self) -> Vec<u8> {
                    <$t>::to_be_bytes(*self).to_vec()
                }

                fn to_le_bytes(&self) -> Vec<u8> {
                    <$t>::to_le_bytes(*self).to_vec()
                }
            }
        )*
    };
}

impl_from_bytes!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
impl_to_bytes!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

/// Generic function to read from slice
pub fn read_be<T: FromBytes>(bytes: &[u8]) -> T {
    assert!(bytes.len() >= size_of::<T>());
    T::from_be_bytes(&bytes[..size_of::<T>()])
}

pub fn read_le<T: FromBytes>(bytes: &[u8]) -> T {
    assert!(bytes.len() >= size_of::<T>());
    T::from_le_bytes(&bytes[..size_of::<T>()])
}

pub trait InstructionArgs: Sized {
    fn arg_size() -> usize;
    fn instr_size() -> usize {
        size_of::<u16>() + Self::arg_size()
    }
    fn parse_args(bytes: &[u8]) -> Self;
    fn encode(self) -> Vec<u8>;
}

#[macro_export]
macro_rules! define_instruction {
    (
        $name:ident,
        ($($arg:ty),*),
        $handler:expr
    ) => {
        paste::paste! {
        pub struct [<$name Instruction>];
        pub type [<$name Args>] = ($($arg,)*);

        impl [<$name Instruction>] {
            pub const OPCODE: $crate::OpCode = $crate::OpCode::$name;

            #[inline]
            pub fn arg_size() -> usize {
                pub use $crate::executor::interpreted::opcode_decoder::InstructionArgs;
                [<$name Args>]::arg_size()
            }

            #[inline]
            pub fn parse_args(bytes: &[u8]) -> [<$name Args>] {
                pub use $crate::executor::interpreted::opcode_decoder::InstructionArgs;
                [<$name Args>]::parse_args(bytes)
            }

            #[inline]
            pub fn encode(args: [<$name Args>]) -> Vec<u8> {
                let mut result = Vec::with_capacity(2 + Self::arg_size()); // 2 bytes for opcode
                result.extend_from_slice(&(Self::OPCODE as u16).to_be_bytes());
                result.extend_from_slice(&$crate::executor::interpreted::opcode_decoder::InstructionArgs::encode(args));
                result
            }

            #[inline]
            pub fn handler(
                executor: &mut VmInterpretedExecutor,
                look_ahead: &[u8],
            ) -> Result<(), VmExecutionError> {
                $handler(executor, look_ahead)
            }
        }}
    };
}

macro_rules! impl_instruction_args {
    // Empty tuple
    () => {
        impl InstructionArgs for () {
            fn arg_size() -> usize { 0 }
            fn parse_args(_bytes: &[u8]) -> Self {  }
            fn encode(self) -> Vec<u8> { Vec::new() }
        }
    };
    // Non-empty tuples
    ($($arg:ident),+) => {
        impl<$($arg: FromBytes + ToBytes),+> InstructionArgs for ($($arg),+,) {
            fn arg_size() -> usize {
                let mut total = 0;
                $(
                    total += size_of::<$arg>();
                )+
                total
            }

            fn parse_args(bytes: &[u8]) -> Self {
                let mut cursor = 0;
                (
                    $(
                        {
                            let value = read_be::<$arg>(&bytes[cursor..]);
                            cursor += size_of::<$arg>();
                            value
                        }
                    ),+,
                )
            }

            fn encode(self) -> Vec<u8> {
                let mut bytes = Vec::new();
                let ($($arg),+,) = self;

                $(
                    bytes.extend(ToBytes::to_be_bytes(& $arg));
                )+

                bytes
            }
        }
    };
}

// Generate implementations
impl_instruction_args!();
impl_instruction_args!(Arg1);
impl_instruction_args!(Arg1, Arg2);
impl_instruction_args!(Arg1, Arg2, Arg3);
impl_instruction_args!(Arg1, Arg2, Arg3, Arg4);
impl_instruction_args!(Arg1, Arg2, Arg3, Arg4, Arg5);

#[macro_export]
macro_rules! define_vm_tests {
    ($name:ident, [$(($instr:ident, $ty:ty)),+], $body:expr, $args:expr) => {
        paste::paste! {
            $(
                #[test]
                fn [<$instr:lower _ $name>]() {
                    use $crate::OpCode;
                    use $crate::asm_internal::VmTest;
                    type T = $ty;
                    let mut bc = [<$instr Instruction>]::encode($args);
                    bc.extend_from_slice(&mut (OpCode::Halt as u16).to_be_bytes());
                    $body.run(bc)
                }
            )+
        }
    };
}

#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VmErrorCode {
    None = 0,
    Overflow = 1,
    Underflow = 2,
    DivisionByZero = 3,
    InvalidRegisterAccess = 4,
    FloatInvalidResult = 5,
    // Add more as needed
}
