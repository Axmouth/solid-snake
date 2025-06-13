pub mod asm_internal;
pub mod bytecode_parser;
pub mod executor;
pub mod docs;
pub mod opcodes;

use std::{error::Error, time::Instant};

use bytecode_parser::parse_byte_code_from_txt;
use executor::{
    ext::VmExecutorExt,
    interpreted::{
        implimentation::{RegisterFileExt, VmInterpretedExecutor},
        opcode_decoder::RegisterType,
    },
};

use opcodes::OpCode;

#[test]
fn instructions_docs_are_up_to_date() {
    use std::fs;

    let expected = fs::read_to_string("../INSTRUCTIONS.md").expect("Missing INSTRUCTIONS.md");
    let current = {
        let docs = docs::Docs {
            instructions: OpCode::get_docs(),
        };
        docs.to_markdown()
    };

    assert_eq!(expected, current, "Instruction docs are out of date. Run `cargo run --bin docgen`.");
}
