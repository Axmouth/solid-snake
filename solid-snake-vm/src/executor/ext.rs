use std::{error::Error, fmt};

use crate::opcodes::DecodedInstruction;

use super::interpreted::implimentation::VmInterpretedExecutor;

#[derive(Debug)]
pub enum VmExecutionError {
    InternalError(Box<dyn Error + Send + Sync>),
    StackOverflow,
    StackUnderflow,
    NullPointerException,
    SegmentationFault,
    InvalidOpCode,
    UnexpectedEOF,
}

impl Error for VmExecutionError {}

impl fmt::Display for VmExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmExecutionError::InternalError(err) => write!(f, "Internal error: {}", err),
            VmExecutionError::StackOverflow => write!(f, "Stack overflow"),
            VmExecutionError::StackUnderflow => write!(f, "Stack underflow"),
            VmExecutionError::NullPointerException => write!(f, "Null Pointer Exception"),
            VmExecutionError::SegmentationFault => write!(f, "Segmantation Fault"),
            VmExecutionError::InvalidOpCode => write!(f, "Invalid OpCode"),
            VmExecutionError::UnexpectedEOF => write!(f, "Unexpected End of File"),
        }
    }
}

pub trait VmExecutorExt {
    fn preprocess_bytecode(
        &mut self,
        bytecode: &[u8],
    ) -> Result<
        Vec<(
            DecodedInstruction,
            Box<dyn Fn(&mut VmInterpretedExecutor) -> Result<(), VmExecutionError>>,
        )>,
        VmExecutionError,
    >;
    fn execute_processeded_bytecode(
        &mut self,
        processed_bytecode: &[(
            DecodedInstruction,
            Box<dyn Fn(&mut VmInterpretedExecutor) -> Result<(), VmExecutionError>>,
        )],
    ) -> Result<i64, VmExecutionError>;
    fn set_error(&mut self, error_code: i64);
}
