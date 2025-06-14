use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{executor::interpreted::opcode_decoder::RegisterType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArgDocsEntry {
    pub name: String,
    pub description: String,
    pub typ: ArgType,
    pub bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InstructionDocsEntry {
    pub name: String,
    pub description: String,
    pub opcode: u16,
    pub arg_bytes: usize,
    pub args: Vec<ArgDocsEntry>,
    pub tags: Box<[InstructionTag]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Docs {
    pub instructions: Vec<InstructionDocsEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstructionTag {
    /// Alters control flow: jumps, calls, returns, halts.
    ControlFlow,

    /// Reads from or writes to memory.
    Memory,

    /// Performs computation: arithmetic, bitwise, shifts, etc.
    Arithmetic,

    /// Performs logical or comparison operations.
    Logical,

    /// Has no side effects other than register output, enabling certain optimizations.
    Pure,

    /// Commutative operations, useful for reordering or common subexpression elimination.
    Commutative,

    /// Allocates or deallocates memory.
    Allocation,

    /// Moves data between registers or memory.
    DataMovement,

    /// Produces side effects visible outside the VM (e.g., print).
    SideEffects,
}

impl fmt::Display for InstructionTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstructionTag::ControlFlow => write!(f, "Control Flow"),
            InstructionTag::Memory => write!(f, "Memory"),
            InstructionTag::Arithmetic => write!(f, "Arithmetic"),
            InstructionTag::Logical => write!(f, "Logical"),
            InstructionTag::Pure => write!(f, "Pure"),
            InstructionTag::Commutative => write!(f, "Commutative"),
            InstructionTag::Allocation => write!(f, "Allocation"),
            InstructionTag::DataMovement => write!(f, "Data Movement"),
            InstructionTag::SideEffects => write!(f, "Side Effects"),
        }
    }
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
            markdown.push_str(&format!("**Arg Bytes**: {}\n\n", instruction.arg_bytes));
            markdown.push_str("### Instruction Details\n\n");

            if !instruction.args.is_empty() {
                markdown.push_str("### Arguments\n\n");
                for arg in &instruction.args {
                    markdown.push_str(&format!(
                        "- **{}**: {} (Type: `{}`, Bytes: `{}`)\n",
                        arg.name, arg.description, arg.typ, arg.bytes
                    ));
                }
                markdown.push('\n');
            }

            if !instruction.tags.is_empty() {
                markdown.push_str("### Tags\n\n");
                for tag in &instruction.tags {
                    markdown.push_str(&format!("- {}\n", tag));
                }
                markdown.push('\n');
            }
        }

        markdown
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("Failed to serialize docs to JSON")
    }
}


#[cfg(test)]
mod tests {
    use crate::opcodes::OpCode;

    use super::*;

    #[test]
    fn instructions_docs_are_up_to_date() {
        use std::fs;

        let expected = fs::read_to_string("INSTRUCTIONS.md").expect("Missing INSTRUCTIONS.md");
        let current = {
            let docs = Docs {
                instructions: OpCode::get_docs(),
            };
            docs.to_markdown()
        };

        if expected != current {
            panic!("Instruction docs are out of date. Run `cargo run --bin docgen`.");
        }
    }

    #[test]
    fn instruction_docs_json() {
        use std::fs;

        let expected = fs::read_to_string("docs.json").expect("Missing docs.json");
        let current = {
            let docs = Docs {
                instructions: OpCode::get_docs(),
            };
            docs.to_json()
        };

        if expected != current {
            panic!("Instruction docs are out of date. Run `cargo run --bin docgen`.");
        }
    }
}