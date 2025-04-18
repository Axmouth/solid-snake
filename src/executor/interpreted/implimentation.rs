use log::warn;

use crate::executor::ext::{VmExecutionError, VmExecutorExt};

use crate::executor::interpreted::opcode_decoder::{
    initialize_dispatch_table, OpcodeHandler, MAX_OPCODES,
};
use crate::opcodes::OpCode;

use super::opcode_decoder::RegisterType;
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

pub trait RegisterFileExt<T, R: Into<usize>> {
    fn get_register_value(&self, idx: R) -> Result<T, VmExecutionError>;
    fn set_register_value(&mut self, idx: R, value: T) -> Result<(), VmExecutionError>;
}

#[derive(Debug, Clone, Copy)]
pub struct RegisterFile {
    pub raw: [u64; 128],
}

impl RegisterFile {
    pub fn new() -> Self {
        Self { raw: [0u64; 128] }
    }
}

macro_rules! impl_register_file_ext {
    ($t:ty) => {
        impl<R: Into<usize>> RegisterFileExt<$t, R> for RegisterFile {
            fn get_register_value(&self, idx: R) -> Result<$t, VmExecutionError> {
                let idx: usize = idx.into();
                assert!(self.raw.len() > idx);
                Ok(self.raw[idx] as $t)
            }

            fn set_register_value(&mut self, idx: R, value: $t) -> Result<(), VmExecutionError> {
                let idx: usize = idx.into();
                assert!(self.raw.len() > idx);
                self.raw[idx] = value as u64;
                Ok(())
            }
        }
    };
}

impl_register_file_ext!(u8);
impl_register_file_ext!(u16);
impl_register_file_ext!(u32);
impl_register_file_ext!(u64);
impl_register_file_ext!(i8);
impl_register_file_ext!(i16);
impl_register_file_ext!(i32);
impl_register_file_ext!(i64);

impl<R: Into<usize>> RegisterFileExt<f64, R> for RegisterFile {
    fn get_register_value(&self, idx: R) -> Result<f64, VmExecutionError> {
        let idx: usize = idx.into();
        assert!(self.raw.len() > idx);
        Ok(f64::from_bits(self.raw[idx]))
    }

    fn set_register_value(&mut self, idx: R, value: f64) -> Result<(), VmExecutionError> {
        let idx: usize = idx.into();
        assert!(self.raw.len() > idx);
        self.raw[idx] = value.to_bits();
        Ok(())
    }
}

impl<R: Into<usize>> RegisterFileExt<f32, R> for RegisterFile {
    fn get_register_value(&self, idx: R) -> Result<f32, VmExecutionError> {
        let idx: usize = idx.into();
        assert!(self.raw.len() > idx);
        Ok(f32::from_bits(self.raw[idx] as u32))
    }

    fn set_register_value(&mut self, idx: R, value: f32) -> Result<(), VmExecutionError> {
        let idx: usize = idx.into();
        assert!(self.raw.len() > idx);
        self.raw[idx] = value.to_bits() as u64;
        Ok(())
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

#[derive(Debug, Clone)]
pub struct VmInterpretedExecutor {
    pub frame_stack: Vec<CallFrame>,
    pub stack_top: usize,
    pub error_code: u64,
    program_counter: usize,
    heap: VmHeap,
    dispatch_table: Vec<(OpcodeHandler, usize)>,
}

impl VmInterpretedExecutor {
    pub fn new() -> Self {
        Self {
            frame_stack: vec![CallFrame::new(); 256],
            stack_top: 0,
            error_code: 0,
            program_counter: 0,
            heap: VmHeap::new(),
            dispatch_table: initialize_dispatch_table(),
        }
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
    fn execute_bytecode(&mut self, bytecode: &[u8]) -> Result<(), VmExecutionError> {
        self.program_counter = 0;

        assert!(self.dispatch_table.len() == MAX_OPCODES);
        loop {
            // TODO: maybe remove with halt etc instructions
            let opcode_bytes: [u8; 2] = [
                bytecode[self.program_counter],
                bytecode[self.program_counter + 1],
            ];
            let opcode = u16::from_be_bytes(opcode_bytes);
            if opcode == OpCode::Halt as u16 {
                return Ok(());
            }
            self.program_counter += 2;
            // Lookup the function in the dispatch table and execute it
            assert!((opcode as usize) < self.dispatch_table.len());
            let (exec, look_ahead_bytes) = self.dispatch_table[opcode as usize];
            let look_ahead_counter = self.program_counter + look_ahead_bytes;
            let look_ahead = &bytecode[self.program_counter..look_ahead_counter];
            // TODO Better error handling in case untrue, or assert
            self.program_counter = look_ahead_counter;
            exec(self, look_ahead)?;
        }
    }

    fn set_error(&mut self, error_code: u64) {
        self.error_code = error_code
    }
}
