pub mod asm_internal;
pub mod bytecode_parser;
pub mod executor;
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
use rustc_version_runtime::version;
use sysinfo::{System, get_current_pid};
