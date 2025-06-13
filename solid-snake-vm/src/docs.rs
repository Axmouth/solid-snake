use std::fmt;

use crate::executor::interpreted::opcode_decoder::RegisterType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Register
}

impl fmt::Display for ArgType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait GetArgType {
    fn arg_type() -> ArgType;
}

macro_rules! impl_arg_type {
    ($type_name:ident, $type_value:ident) => {
        impl GetArgType for $type_name {
            fn arg_type() -> ArgType {
                ArgType::$type_value
            }
        }
    };
}

impl_arg_type!(i8, I8);
impl_arg_type!(i16, I16);
impl_arg_type!(i32, I32);
impl_arg_type!(i64, I64);
impl_arg_type!(u8, U8);
impl_arg_type!(u16, U16);
impl_arg_type!(u32, U32);
impl_arg_type!(u64, U64);
impl_arg_type!(f32, F32);
impl_arg_type!(f64, F64);
impl_arg_type!(bool, Bool);
impl_arg_type!(RegisterType, Register);

// TODO Use static strs?

pub struct ArgDocsEntry {
    pub name: String,
    pub description: String,
    pub typ: ArgType,
}

pub struct InstructionDocsEntry {
    pub name: String,
    pub description: String,
    pub opcode: u16,
    pub args: Vec<ArgDocsEntry>,
    pub commutative: bool,
}

pub struct Docs {
    pub instructions: Vec<InstructionDocsEntry>,
}

impl Docs {
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str("# Solid Snake Bytecode Instructions\n\n");
        markdown.push_str("This document provides a comprehensive overview of the bytecode instructions used in the Solid Snake virtual machine.\n\n");

        for instruction in &self.instructions {
            markdown.push_str(&format!("## {}\n\n", instruction.name));
            markdown.push_str(&format!("{}\n\n", instruction.description));
            markdown.push_str(&format!("**Opcode**: `0x{:04X}`\n\n", instruction.opcode));
            markdown.push_str("### Instruction Details\n\n");

            if !instruction.args.is_empty() {
                markdown.push_str("### Arguments\n\n");
                for arg in &instruction.args {
                    markdown.push_str(&format!(
                        "- **{}**: {} (Type: `{}`)\n",
                        arg.name, arg.description, arg.typ
                    ));
                }
                markdown.push('\n');
            }

            if instruction.commutative {
                markdown.push_str("This instruction is commutative.\n\n");
            } else {
                markdown.push_str("This instruction is not commutative.\n\n");
            }
        }

        markdown
    }
}