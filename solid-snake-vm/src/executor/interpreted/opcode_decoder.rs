use std::str::FromStr;

use crate::executor::{
    ext::VmExecutionError,
    interpreted::implimentation::{MAX_REGISTERS, VmInterpretedExecutor},
};
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
            let idx = stripped
                .parse::<u8>()
                .map_err(|_| VmParseError::InvalidRegister(s.to_string()))?;
            if (idx as usize) >= MAX_REGISTERS {
                return Err(VmParseError::InvalidRegister(format!(
                    "Register index {} out of range",
                    idx
                )));
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
        RegisterType(u8::from_be_bytes(
            bytes.try_into().expect("Invalid bytes for RegisterType"),
        ))
    }

    fn from_le_bytes(bytes: &[u8]) -> Self {
        RegisterType(u8::from_le_bytes(
            bytes.try_into().expect("Invalid bytes for RegisterType"),
        ))
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

impl_vm_parse_for_ints!(u8, u16, u32, u64, i8, i16, i32, i64);

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
        #[allow(non_snake_case)]
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

pub fn initialize_dispatch_table() -> Vec<(OpcodeHandler, ParseHandler, usize)> {
    let mut dispatch_table: Vec<(OpcodeHandler, ParseHandler, usize)> = Vec::new();
    dispatch_table.resize(MAX_OPCODES, (invalidop, <()>::encode_from_strs, 0));

    OpCode::init_dispatch_table(&mut dispatch_table);

    dispatch_table
}

pub type OpcodeHandler =
    fn(&mut VmInterpretedExecutor, look_ahead: &[u8]) -> Result<(), VmExecutionError>;

pub type ParseHandler = fn(strs: &[&str]) -> Result<Vec<u8>, VmParseError>;

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
    fn args_size() -> usize;
    fn parse_args(bytes: &[u8]) -> Self;
    fn encode(self) -> Vec<u8>;
}

#[macro_export]
macro_rules! define_instruction {
    // --- Rich form with doc metadata and flags ---
    (
        $name:ident,
        $description:expr,
        [ $( ($arg_name:ident : $arg_ty:ty, $arg_desc:expr) ),* ],
        [$( $tag:ident ),*],
        $handler:expr
    ) => {
        paste::paste! {
            pub struct [<$name Instruction>];
            pub type [<$name Args>] = ($($arg_ty,)*);

            impl [<$name Instruction>] {
                pub const OPCODE: $crate::OpCode = $crate::OpCode::$name;

                #[allow(dead_code)]
                pub fn docs() -> $crate::docs::InstructionDocsEntry {
                    pub use $crate::docs::{
                        ArgDocsEntry,
                        Docs,
                        GetArgType,
                        InstructionDocsEntry,
                    };

                    let args: Vec<ArgDocsEntry> = vec![
                        $(
                            ArgDocsEntry {
                                name: stringify!($arg_name).to_string(),
                                description: $arg_desc.to_string(),
                                typ: <$arg_ty as GetArgType>::arg_type(),
                                bytes: size_of::<$arg_ty>(),
                            }
                        ),*
                    ];

                    InstructionDocsEntry {
                        name: stringify!($name).to_string(),
                        description: $description.to_string(),
                        args,
                        opcode: Self::OPCODE as u16,
                        arg_bytes: Self::args_size(),
                        tags: Box::new([
                                $( $crate::docs::InstructionTag::$tag ),*
                            ]),
                    }
                }

                #[allow(dead_code)]
                #[inline]
                pub fn tags() -> &'static [$crate::docs::InstructionTag] {
                    return &[
                        $( $crate::docs::InstructionTag::$tag ),*
                    ];
                }

                #[allow(dead_code)]
                #[inline]
                pub fn args_size() -> usize {
                    pub use $crate::executor::interpreted::opcode_decoder::InstructionArgs;
                    [<$name Args>]::args_size()
                }

                #[allow(dead_code)]
                pub fn instr_size() -> usize {
                    size_of::<u16>() + Self::args_size()
                }

                #[allow(dead_code)]
                #[inline]
                pub fn parse_args(bytes: &[u8]) -> [<$name Args>] {
                    pub use $crate::executor::interpreted::opcode_decoder::InstructionArgs;
                    [<$name Args>]::parse_args(bytes)
                }

                #[allow(dead_code)]
                #[inline]
                pub fn encode(args: [<$name Args>]) -> Vec<u8> {
                    let mut result = Vec::with_capacity(2 + Self::args_size()); // 2 bytes for opcode
                    result.extend_from_slice(&(Self::OPCODE as u16).to_be_bytes());
                    result.extend_from_slice(&$crate::executor::interpreted::opcode_decoder::InstructionArgs::encode(args));
                    result
                }

                #[inline]
                pub fn handler(
                    executor: &mut $crate::executor::interpreted::implimentation::VmInterpretedExecutor,
                    look_ahead: &[u8],
                ) -> Result<(), $crate::executor::ext::VmExecutionError> {
                    let args = Self::parse_args(look_ahead);
                    $handler(executor, args)
                }

                #[inline]
                pub fn decoded_handler(
                    executor: &mut $crate::executor::interpreted::implimentation::VmInterpretedExecutor,
                    args: [<$name Args>],
                ) -> Result<(), $crate::executor::ext::VmExecutionError> {
                    $handler(executor, args)
                }
            }

            #[allow(dead_code)]
            #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
            pub struct [<Executable $name>] {
                args: [<$name Args>]
            }

            #[allow(dead_code)]
            impl [<Executable $name>] {
                pub fn new(args: [<$name Args>]) -> Self {
                    Self {
                        args
                    }
                }
            }

            impl $crate::executor::interpreted::implimentation::ExecutableInstruction for [<Executable $name>] {
                fn execute(&self, executor: &mut $crate::executor::interpreted::implimentation::VmInterpretedExecutor) -> std::result::Result<(), $crate::executor::ext::VmExecutionError> {
                    [<$name Instruction>]::decoded_handler(executor, self.args)
                }
            }
        }
    };
}

macro_rules! impl_instruction_args {
    // Empty tuple
    () => {
        impl InstructionArgs for () {
            fn args_size() -> usize { 0 }
            fn parse_args(_bytes: &[u8]) -> Self {  }
            fn encode(self) -> Vec<u8> { Vec::new() }
        }
    };
    // Non-empty tuples
    ($($arg:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($arg: FromBytes + ToBytes),+> InstructionArgs for ($($arg),+,) {
            fn args_size() -> usize {
                let mut total = 0;
                $(
                    total += size_of::<$arg>();
                )+
                total
            }

            #[allow(unused_assignments)]
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
                #[allow(dead_code)]
                fn [<$instr:lower _ $name>]() {
                    use $crate::OpCode;
                    use $crate::asm_internal::VmTest;
                    type T = $ty;
                    let mut bc = [<$instr Instruction>]::encode($args);
                    bc.extend_from_slice(&mut (OpCode::Halt as u16).to_be_bytes());
                    bc.extend_from_slice(&mut (0 as i64).to_be_bytes());
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
    // ...
}
