use std::{error::Error, fmt};

#[derive(Debug)]
pub enum VmExecutionError {
    InternalError(Box<dyn Error + Send + Sync>),
    StackOverflow,
    StackUnderflow,
    NullPointerException,
    SegmentationFault,
    InvalidOpCode,
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
        }
    }
}

pub trait VmExecutorExt {
    fn execute_bytecode(&mut self, bytecode: &[u8]) -> Result<(), VmExecutionError>;
    fn set_error(&mut self, error_code: i64);
}
