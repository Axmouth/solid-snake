use super::opcode_decoder::RegisterValue;
use crate::executor::ext::{VmExecutionError, VmExecutorExt};
use crate::opcodes::{DecodedInstruction, OpCode};

// TODO : thread related logic. Shared memory with atomic access. Ability to fork and run more instances on threads.

pub const INITIAL_FRAMES_CAPACITY: usize = usize::pow(2, 16);

pub trait VmMemorySectionExt {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;

    fn dereference_bytes(&self) -> Result<&[u8], VmExecutionError>;
    fn bytes_n(&self, n: usize) -> Result<&[u8], VmExecutionError>;
    fn bytes_with_offset(&self, offset: usize) -> Result<&[u8], VmExecutionError>;
    fn bytes_n_with_offset(&self, n: usize, offset: usize) -> Result<&[u8], VmExecutionError>;

    fn dereference_bytes_mut(&mut self) -> Result<&mut [u8], VmExecutionError>;
    fn bytes_n_mut(&mut self, n: usize) -> Result<&mut [u8], VmExecutionError>;
    fn bytes_with_offset_mut(&mut self, offset: usize) -> Result<&mut [u8], VmExecutionError>;
    fn bytes_n_with_offset_mut(
        &mut self,
        n: usize,
        offset: usize,
    ) -> Result<&mut [u8], VmExecutionError>;

    // TODO: stats method
    // section, allocated bytes, freed slots, allocations, etc
}

#[derive(Debug, Clone)]
pub struct VmMemorySection {
    bytes: Vec<u8>,
}

impl VmMemorySection {
    fn new_with_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    fn check_bounds(&self, offset: usize, size: usize) -> Result<(), VmExecutionError> {
        match offset.checked_add(size) {
            Some(end) if end <= self.bytes.len() => Ok(()),
            _ => Err(VmExecutionError::SegmentationFault),
        }
    }
}

impl VmMemorySectionExt for VmMemorySection {
    fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    fn len(&self) -> usize {
        self.bytes.len()
    }

    fn dereference_bytes(&self) -> Result<&[u8], VmExecutionError> {
        Ok(&self.bytes)
    }

    fn bytes_n(&self, n: usize) -> Result<&[u8], VmExecutionError> {
        self.bytes
            .get(..n)
            .ok_or(VmExecutionError::SegmentationFault)
    }

    fn bytes_with_offset(&self, offset: usize) -> Result<&[u8], VmExecutionError> {
        self.bytes
            .get(offset..)
            .ok_or(VmExecutionError::SegmentationFault)
    }

    fn bytes_n_with_offset(&self, n: usize, offset: usize) -> Result<&[u8], VmExecutionError> {
        self.check_bounds(offset, n)?;
        self.bytes
            .get(offset..offset + n)
            .ok_or(VmExecutionError::SegmentationFault)
    }

    fn dereference_bytes_mut(&mut self) -> Result<&mut [u8], VmExecutionError> {
        Ok(&mut self.bytes)
    }

    fn bytes_n_mut(&mut self, n: usize) -> Result<&mut [u8], VmExecutionError> {
        self.bytes
            .get_mut(..n)
            .ok_or(VmExecutionError::SegmentationFault)
    }

    fn bytes_with_offset_mut(&mut self, offset: usize) -> Result<&mut [u8], VmExecutionError> {
        self.bytes
            .get_mut(offset..)
            .ok_or(VmExecutionError::SegmentationFault)
    }

    fn bytes_n_with_offset_mut(
        &mut self,
        n: usize,
        offset: usize,
    ) -> Result<&mut [u8], VmExecutionError> {
        self.bytes
            .get_mut(offset..offset + n)
            .ok_or(VmExecutionError::SegmentationFault)
    }
}

pub trait VmHeapExt {
    // Allocates a memory section of n size and returns a "pointer" to it, the index of that sectiom
    fn alloc(&mut self, n: usize) -> Result<usize, VmExecutionError>;

    // Frees memory section inhabiting "point" (index)
    fn free(&mut self, idx: usize) -> Result<(), VmExecutionError>;

    // Returns a reference to the section
    fn section(&self, idx: usize) -> Result<&VmMemorySection, VmExecutionError>;
    fn section_mut(&mut self, idx: usize) -> Result<&mut VmMemorySection, VmExecutionError>;
}

#[derive(Debug, Clone)]
pub struct VmHeap {
    memory_sections: Vec<VmMemorySection>,
    freed_sections: Vec<usize>,
}

impl VmHeap {
    pub fn new() -> Self {
        Self {
            freed_sections: Vec::new(),
            memory_sections: Vec::new(),
        }
    }
}

impl VmHeapExt for VmHeap {
    fn alloc(&mut self, n: usize) -> Result<usize, VmExecutionError> {
        let memory_block: Vec<u8> = vec![0; n];
        let new_section = VmMemorySection::new_with_bytes(memory_block);
        if let Some(idx) = self.freed_sections.pop() {
            self.memory_sections[idx] = new_section;
            Ok(idx)
        } else {
            self.memory_sections.push(new_section);
            Ok(self.memory_sections.len() - 1)
        }
    }

    fn free(&mut self, idx: usize) -> Result<(), VmExecutionError> {
        // Erase section and add its "address" (index) to freed section for reuse
        *self
            .memory_sections
            .get_mut(idx)
            .ok_or(VmExecutionError::NullPointerException)? =
            VmMemorySection::new_with_bytes(Vec::new());
        self.freed_sections.push(idx);
        Ok(())
    }

    fn section(&self, idx: usize) -> Result<&VmMemorySection, VmExecutionError> {
        let section = self
            .memory_sections
            .get(idx)
            .ok_or(VmExecutionError::NullPointerException)?;
        if section.is_empty() {
            return Err(VmExecutionError::NullPointerException);
        }
        Ok(section)
    }

    fn section_mut(&mut self, idx: usize) -> Result<&mut VmMemorySection, VmExecutionError> {
        let section = self
            .memory_sections
            .get_mut(idx)
            .ok_or(VmExecutionError::NullPointerException)?;
        if section.is_empty() {
            return Err(VmExecutionError::NullPointerException);
        }
        Ok(section)
    }
}

pub trait RegisterFileExt<T: RegisterValue, R: Into<usize>> {
    fn get_register_value(&self, idx: R) -> Result<T, VmExecutionError>;
    fn set_register_value(&mut self, idx: R, value: T) -> Result<(), VmExecutionError>;
}

#[derive(Debug, Clone, Copy)]
pub struct RegisterFile {
    pub raw: [u64; 128],
}

pub const MAX_REGISTERS: usize = 128;

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            raw: [0u64; MAX_REGISTERS],
        }
    }
}

macro_rules! impl_register_value_ext {
    ($t:ty) => {
        impl RegisterValue for $t {
            fn to_u64(&self) -> u64 {
                *self as u64
            }

            fn from_u64(val: u64) -> Self {
                val as $t
            }
        }
    };
}

impl_register_value_ext!(u8);
impl_register_value_ext!(u16);
impl_register_value_ext!(u32);
impl_register_value_ext!(u64);
impl_register_value_ext!(i8);
impl_register_value_ext!(i16);
impl_register_value_ext!(i32);
impl_register_value_ext!(i64);

impl<T: RegisterValue, R: Into<usize>> RegisterFileExt<T, R> for RegisterFile {
    fn get_register_value(&self, idx: R) -> Result<T, VmExecutionError> {
        let idx: usize = idx.into();
        assert!(self.raw.len() > idx);
        Ok(RegisterValue::from_u64(self.raw[idx]))
    }

    fn set_register_value(&mut self, idx: R, value: T) -> Result<(), VmExecutionError> {
        let idx: usize = idx.into();
        assert!(self.raw.len() > idx);
        self.raw[idx] = value.to_u64();
        Ok(())
    }
}

impl RegisterValue for f32 {
    fn to_u64(&self) -> u64 {
        self.to_bits() as u64
    }

    fn from_u64(val: u64) -> Self {
        f32::from_bits(val as u32)
    }
}

impl RegisterValue for f64 {
    fn to_u64(&self) -> u64 {
        self.to_bits()
    }

    fn from_u64(val: u64) -> Self {
        f64::from_bits(val)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CallFrame {
    pub return_address: u64,

    pub registers: RegisterFile,
}

impl CallFrame {
    pub fn new_from(address: u64, registers: RegisterFile) -> Self {
        Self {
            return_address: address,
            registers,
        }
    }
    pub fn new() -> Self {
        Self {
            return_address: 0,
            registers: RegisterFile::new(),
        }
    }
}

pub struct VmInterpretedExecutor {
    pub frame_stack: Vec<CallFrame>,
    pub stack_top: usize,
    pub error_code: i64,
    pub prev_error_code: i64,
    max_stack_depth: usize,
    program_counter: usize,
    heap: VmHeap,
    bytecode_pc_to_instr_index: Vec<usize>,
    constants: Vec<Vec<u8>>,
}

impl VmInterpretedExecutor {
    pub fn new(constants: Option<Vec<Vec<u8>>>) -> Self {
        Self {
            frame_stack: vec![CallFrame::new(); INITIAL_FRAMES_CAPACITY],
            stack_top: 0,
            error_code: 0,
            prev_error_code: 0,
            program_counter: 0,
            heap: VmHeap::new(),
            max_stack_depth: (u32::MAX / 2048) as usize,
            bytecode_pc_to_instr_index: Vec::new(),
            constants: constants.unwrap_or_default(),
        }
    }

    pub fn get_constant(&self, id: usize) -> Option<&[u8]> {
        self.constants.get(id).map(|v| v.as_slice())
    }

    pub fn set_constants(&mut self, constants: Vec<Vec<u8>>) {
        self.constants = constants;
    }

    pub fn get_max_stack_depth(&self) -> usize {
        self.max_stack_depth
    }

    pub fn get_instr_idx_from_bc(&self, bc: usize) -> usize {
        self.bytecode_pc_to_instr_index[bc]
    }

    pub fn set_program_counter(&mut self, program_counter: usize) -> Result<(), VmExecutionError> {
        self.program_counter = program_counter;
        Ok(())
    }

    pub fn get_program_counter(&self) -> Result<usize, VmExecutionError> {
        Ok(self.program_counter)
    }

    pub fn registers(&self) -> &RegisterFile {
        &self.frame_stack[self.stack_top].registers
    }

    pub fn registers_mut(&mut self) -> &mut RegisterFile {
        &mut self.frame_stack[self.stack_top].registers
    }

    pub fn heap(&self) -> &VmHeap {
        &self.heap
    }

    pub fn heap_mut(&mut self) -> &mut VmHeap {
        &mut self.heap
    }
}

impl VmExecutorExt for VmInterpretedExecutor {
    // TODO Add Label fake instruction, that binds an address to an id, used for jumppoints. Pre processing can eliminate indirection. Cleaner for hot loading code especially
    fn preprocess_bytecode(
        &mut self,
        bytecode: &[u8],
    ) -> Result<
        Vec<(
            DecodedInstruction,
            Box<dyn Fn(&mut VmInterpretedExecutor) -> Result<(), VmExecutionError>>,
        )>,
        VmExecutionError,
    > {
        let mut processed_bytecode = Vec::with_capacity(bytecode.len());
        let mut bc_counter = 0;
        self.bytecode_pc_to_instr_index = vec![usize::MAX; bytecode.len()];
        loop {
            let jump_bc_counter = bc_counter;
            let forward_window: &[u8] = &bytecode[bc_counter..];

            if forward_window.is_empty() {
                break;
            } else if 2 > forward_window.len() {
                return Err(VmExecutionError::UnexpectedEOF);
            }

            let opcode_bytes: [u8; 2] = [forward_window[0], forward_window[1]];
            let opcode = u16::from_be_bytes(opcode_bytes);
            let opcode_enum =
                OpCode::try_from(opcode).map_err(|_| VmExecutionError::InvalidOpCode)?;
            bc_counter += size_of::<u16>();
            self.bytecode_pc_to_instr_index[jump_bc_counter] = processed_bytecode.len();
            let enum_opcode = OpCode::try_from(opcode).map_err(
                |_: num_enum::TryFromPrimitiveError<OpCode>| VmExecutionError::InvalidOpCode,
            )?;
            let look_ahead_bytes = opcode_enum.args_size();
            let look_ahead = &forward_window[2..2 + look_ahead_bytes];
            let decoded_instruction = DecodedInstruction::decode(enum_opcode, look_ahead);
            processed_bytecode.push(decoded_instruction);
            bc_counter += look_ahead_bytes;
        }

        // Preprocess jump address to the processed bytecode
        let processed_bytecode_second_pass = processed_bytecode
            .into_iter()
            .map(|instr| match instr {
                DecodedInstruction::Jump((target,)) => {
                    let resolved = self.get_instr_idx_from_bc(target as usize) as u64;
                    DecodedInstruction::Jump((resolved,))
                }
                DecodedInstruction::JumpIf((target, reg)) => {
                    let resolved = self.get_instr_idx_from_bc(target as usize) as u64;
                    DecodedInstruction::JumpIf((resolved, reg))
                }
                DecodedInstruction::JumpIfFalse((target, reg)) => {
                    let resolved = self.get_instr_idx_from_bc(target as usize) as u64;
                    DecodedInstruction::JumpIfFalse((resolved, reg))
                }
                DecodedInstruction::CallFunction((target,)) => {
                    let resolved = self.get_instr_idx_from_bc(target as usize) as u64;
                    DecodedInstruction::CallFunction((resolved,))
                }
                _ => instr,
            })
            .collect::<Vec<_>>();

        let final_pass = processed_bytecode_second_pass
            .into_iter()
            .map(|decoded_instr| (decoded_instr, decoded_instr.exec_instr_fn()))
            .collect::<Vec<_>>();

        Ok(final_pass)
    }

    fn execute_processeded_bytecode(
        &mut self,
        processed_bytecode: &[(
            DecodedInstruction,
            Box<dyn Fn(&mut VmInterpretedExecutor) -> Result<(), VmExecutionError>>,
        )],
    ) -> Result<(), VmExecutionError> {
        self.program_counter = 0;

        loop {
            let (decoded, exec_instr_fn) = &processed_bytecode[self.program_counter];

            if let DecodedInstruction::Halt(()) = decoded {
                return Ok(());
            }
            self.prev_error_code = self.error_code;
            self.error_code = 0; // TODO: examine if any instruction will need it kept set between multiple instructions.
            self.program_counter += 1;
            exec_instr_fn(self)?;
        }
    }

    fn set_error(&mut self, error_code: i64) {
        self.error_code = error_code
    }
}

#[macro_export]
macro_rules! set_error_if {
    ($executor:expr, $cond:expr, $error:expr) => {
        $executor.error_code |= ($cond as i64) * ($error as i64);
    };
}

pub trait ExecutableInstruction {
    fn execute(&self, executor: &mut VmInterpretedExecutor) -> Result<(), VmExecutionError>;
}

pub type ExecutableInstructionFn =
    Box<dyn Fn(&mut VmInterpretedExecutor) -> Result<(), VmExecutionError>>;
