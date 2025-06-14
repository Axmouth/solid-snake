# Solid Snake Bytecode Instructions

This document provides a comprehensive overview of the bytecode instructions used in the Solid Snake virtual machine.

## JumpIfFalse

Jumps to the target address if the register is zero (false).

**Opcode**: `0x0001`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **target**: Bytecode address(byte offset) to jump to (Type: `U64`)
- **reg**: Register to check (Type: `Register`)

### Tags

- Control Flow
- Side Effects

## JumpIf

Jumps to the target address if the register is non-zero (true).

**Opcode**: `0x0002`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **target**: Bytecode address(byte offset) to jump to (Type: `U64`)
- **reg**: Register to check (Type: `Register`)

### Tags

- Control Flow
- Side Effects

## Jump

Unconditionally jumps to the specified bytecode address.

**Opcode**: `0x0003`

**Arg Bytes**: 8

### Instruction Details

### Arguments

- **target**: Bytecode address(byte offset) to jump to (Type: `U64`)

### Tags

- Control Flow
- Side Effects

## LoadIndirectU8

Loads a u8 value from memory at address stored in a register.

**Opcode**: `0x0014`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg_ptr**: Register holding memory address (Type: `Register`)
- **dest**: Target register to store the loaded value (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectU16

Loads a u16 value from memory at address stored in a register.

**Opcode**: `0x0015`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg_ptr**: Register holding memory address (Type: `Register`)
- **dest**: Target register to store the loaded value (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectU32

Loads a u32 value from memory at address stored in a register.

**Opcode**: `0x0016`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg_ptr**: Register holding memory address (Type: `Register`)
- **dest**: Target register to store the loaded value (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectU64

Loads a u64 value from memory at address stored in a register.

**Opcode**: `0x0017`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg_ptr**: Register holding memory address (Type: `Register`)
- **dest**: Target register to store the loaded value (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectI8

Loads a i8 value from memory at address stored in a register.

**Opcode**: `0x0018`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg_ptr**: Register holding memory address (Type: `Register`)
- **dest**: Target register to store the loaded value (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectI16

Loads a i16 value from memory at address stored in a register.

**Opcode**: `0x0019`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg_ptr**: Register holding memory address (Type: `Register`)
- **dest**: Target register to store the loaded value (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectI32

Loads a i32 value from memory at address stored in a register.

**Opcode**: `0x001A`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg_ptr**: Register holding memory address (Type: `Register`)
- **dest**: Target register to store the loaded value (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectI64

Loads a i64 value from memory at address stored in a register.

**Opcode**: `0x001B`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg_ptr**: Register holding memory address (Type: `Register`)
- **dest**: Target register to store the loaded value (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectF32

Loads a f32 value from memory at address stored in a register.

**Opcode**: `0x001C`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg_ptr**: Register holding memory address (Type: `Register`)
- **dest**: Target register to store the loaded value (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectF64

Loads a f64 value from memory at address stored in a register.

**Opcode**: `0x001D`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg_ptr**: Register holding memory address (Type: `Register`)
- **dest**: Target register to store the loaded value (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectWithOffsetU8

Loads a u8 value from a memory section with a runtime-computed offset.

**Opcode**: `0x001E`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **reg_ptr**: Register holding section index (Type: `Register`)
- **reg_offset**: Register holding byte offset within section (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectWithOffsetU16

Loads a u16 value from a memory section with a runtime-computed offset.

**Opcode**: `0x001F`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **reg_ptr**: Register holding section index (Type: `Register`)
- **reg_offset**: Register holding byte offset within section (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectWithOffsetU32

Loads a u32 value from a memory section with a runtime-computed offset.

**Opcode**: `0x0020`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **reg_ptr**: Register holding section index (Type: `Register`)
- **reg_offset**: Register holding byte offset within section (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectWithOffsetU64

Loads a u64 value from a memory section with a runtime-computed offset.

**Opcode**: `0x0021`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **reg_ptr**: Register holding section index (Type: `Register`)
- **reg_offset**: Register holding byte offset within section (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectWithOffsetI8

Loads a i8 value from a memory section with a runtime-computed offset.

**Opcode**: `0x0022`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **reg_ptr**: Register holding section index (Type: `Register`)
- **reg_offset**: Register holding byte offset within section (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectWithOffsetI16

Loads a i16 value from a memory section with a runtime-computed offset.

**Opcode**: `0x0023`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **reg_ptr**: Register holding section index (Type: `Register`)
- **reg_offset**: Register holding byte offset within section (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectWithOffsetI32

Loads a i32 value from a memory section with a runtime-computed offset.

**Opcode**: `0x0024`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **reg_ptr**: Register holding section index (Type: `Register`)
- **reg_offset**: Register holding byte offset within section (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectWithOffsetI64

Loads a i64 value from a memory section with a runtime-computed offset.

**Opcode**: `0x0025`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **reg_ptr**: Register holding section index (Type: `Register`)
- **reg_offset**: Register holding byte offset within section (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectWithOffsetF32

Loads a f32 value from a memory section with a runtime-computed offset.

**Opcode**: `0x0026`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **reg_ptr**: Register holding section index (Type: `Register`)
- **reg_offset**: Register holding byte offset within section (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadIndirectWithOffsetF64

Loads a f64 value from a memory section with a runtime-computed offset.

**Opcode**: `0x0027`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **reg_ptr**: Register holding section index (Type: `Register`)
- **reg_offset**: Register holding byte offset within section (Type: `Register`)

### Tags

- Data Movement
- Memory

## LoadImmediateU8

Loads an immediate u8 value into the given register.

**Opcode**: `0x0028`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg**: Target register to store the value (Type: `Register`)
- **val**: Immediate value to load (Type: `U8`)

### Tags

- Data Movement
- Pure

## LoadImmediateU16

Loads an immediate u16 value into the given register.

**Opcode**: `0x0029`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg**: Target register to store the value (Type: `Register`)
- **val**: Immediate value to load (Type: `U16`)

### Tags

- Data Movement
- Pure

## LoadImmediateU32

Loads an immediate u32 value into the given register.

**Opcode**: `0x002A`

**Arg Bytes**: 5

### Instruction Details

### Arguments

- **reg**: Target register to store the value (Type: `Register`)
- **val**: Immediate value to load (Type: `U32`)

### Tags

- Data Movement
- Pure

## LoadImmediateU64

Loads an immediate u64 value into the given register.

**Opcode**: `0x002B`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg**: Target register to store the value (Type: `Register`)
- **val**: Immediate value to load (Type: `U64`)

### Tags

- Data Movement
- Pure

## LoadImmediateI8

Loads an immediate i8 value into the given register.

**Opcode**: `0x002C`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg**: Target register to store the value (Type: `Register`)
- **val**: Immediate value to load (Type: `I8`)

### Tags

- Data Movement
- Pure

## LoadImmediateI16

Loads an immediate i16 value into the given register.

**Opcode**: `0x002D`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg**: Target register to store the value (Type: `Register`)
- **val**: Immediate value to load (Type: `I16`)

### Tags

- Data Movement
- Pure

## LoadImmediateI32

Loads an immediate i32 value into the given register.

**Opcode**: `0x002E`

**Arg Bytes**: 5

### Instruction Details

### Arguments

- **reg**: Target register to store the value (Type: `Register`)
- **val**: Immediate value to load (Type: `I32`)

### Tags

- Data Movement
- Pure

## LoadImmediateI64

Loads an immediate i64 value into the given register.

**Opcode**: `0x002F`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg**: Target register to store the value (Type: `Register`)
- **val**: Immediate value to load (Type: `I64`)

### Tags

- Data Movement
- Pure

## LoadImmediateF32

Loads an immediate f32 value into the given register.

**Opcode**: `0x0030`

**Arg Bytes**: 5

### Instruction Details

### Arguments

- **reg**: Target register to store the value (Type: `Register`)
- **val**: Immediate value to load (Type: `F32`)

### Tags

- Data Movement
- Pure

## LoadImmediateF64

Loads an immediate f64 value into the given register.

**Opcode**: `0x0031`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg**: Target register to store the value (Type: `Register`)
- **val**: Immediate value to load (Type: `F64`)

### Tags

- Data Movement
- Pure

## LoadFromImmediateU8

Loads a u8 value from the specified immediate memory address.

**Opcode**: `0x003C`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **addr**: Immediate memory address to read from (Type: `U64`)

### Tags

- Data Movement
- Memory

## LoadFromImmediateU16

Loads a u16 value from the specified immediate memory address.

**Opcode**: `0x003D`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **addr**: Immediate memory address to read from (Type: `U64`)

### Tags

- Data Movement
- Memory

## LoadFromImmediateU32

Loads a u32 value from the specified immediate memory address.

**Opcode**: `0x003E`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **addr**: Immediate memory address to read from (Type: `U64`)

### Tags

- Data Movement
- Memory

## LoadFromImmediateU64

Loads a u64 value from the specified immediate memory address.

**Opcode**: `0x003F`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **addr**: Immediate memory address to read from (Type: `U64`)

### Tags

- Data Movement
- Memory

## LoadFromImmediateI8

Loads a i8 value from the specified immediate memory address.

**Opcode**: `0x0040`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **addr**: Immediate memory address to read from (Type: `U64`)

### Tags

- Data Movement
- Memory

## LoadFromImmediateI16

Loads a i16 value from the specified immediate memory address.

**Opcode**: `0x0041`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **addr**: Immediate memory address to read from (Type: `U64`)

### Tags

- Data Movement
- Memory

## LoadFromImmediateI32

Loads a i32 value from the specified immediate memory address.

**Opcode**: `0x0042`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **addr**: Immediate memory address to read from (Type: `U64`)

### Tags

- Data Movement
- Memory

## LoadFromImmediateI64

Loads a i64 value from the specified immediate memory address.

**Opcode**: `0x0043`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **addr**: Immediate memory address to read from (Type: `U64`)

### Tags

- Data Movement
- Memory

## LoadFromImmediateF32

Loads a f32 value from the specified immediate memory address.

**Opcode**: `0x0044`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **addr**: Immediate memory address to read from (Type: `U64`)

### Tags

- Data Movement
- Memory

## LoadFromImmediateF64

Loads a f64 value from the specified immediate memory address.

**Opcode**: `0x0045`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg_dest**: Target register to store the loaded value (Type: `Register`)
- **addr**: Immediate memory address to read from (Type: `U64`)

### Tags

- Data Movement
- Memory

## StoreIndirectWithOffsetU8

Stores a u8 value from `reg_value` into the heap section at the index in `reg_ptr`, at the byte offset given in `reg_offset`.

**Opcode**: `0x005A`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_ptr**: Register containing the target section index (Type: `Register`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreIndirectWithOffsetU16

Stores a u16 value from `reg_value` into the heap section at the index in `reg_ptr`, at the byte offset given in `reg_offset`.

**Opcode**: `0x005B`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_ptr**: Register containing the target section index (Type: `Register`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreIndirectWithOffsetU32

Stores a u32 value from `reg_value` into the heap section at the index in `reg_ptr`, at the byte offset given in `reg_offset`.

**Opcode**: `0x005C`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_ptr**: Register containing the target section index (Type: `Register`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreIndirectWithOffsetU64

Stores a u64 value from `reg_value` into the heap section at the index in `reg_ptr`, at the byte offset given in `reg_offset`.

**Opcode**: `0x005D`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_ptr**: Register containing the target section index (Type: `Register`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreIndirectWithOffsetI8

Stores a i8 value from `reg_value` into the heap section at the index in `reg_ptr`, at the byte offset given in `reg_offset`.

**Opcode**: `0x005E`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_ptr**: Register containing the target section index (Type: `Register`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreIndirectWithOffsetI16

Stores a i16 value from `reg_value` into the heap section at the index in `reg_ptr`, at the byte offset given in `reg_offset`.

**Opcode**: `0x005F`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_ptr**: Register containing the target section index (Type: `Register`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreIndirectWithOffsetI32

Stores a i32 value from `reg_value` into the heap section at the index in `reg_ptr`, at the byte offset given in `reg_offset`.

**Opcode**: `0x0060`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_ptr**: Register containing the target section index (Type: `Register`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreIndirectWithOffsetI64

Stores a i64 value from `reg_value` into the heap section at the index in `reg_ptr`, at the byte offset given in `reg_offset`.

**Opcode**: `0x0061`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_ptr**: Register containing the target section index (Type: `Register`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreIndirectWithOffsetF32

Stores a f32 value from `reg_value` into the heap section at the index in `reg_ptr`, at the byte offset given in `reg_offset`.

**Opcode**: `0x0062`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_ptr**: Register containing the target section index (Type: `Register`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreIndirectWithOffsetF64

Stores a f64 value from `reg_value` into the heap section at the index in `reg_ptr`, at the byte offset given in `reg_offset`.

**Opcode**: `0x0063`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_ptr**: Register containing the target section index (Type: `Register`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreFromImmediateWithOffsetU8

Stores a u8 value from `reg_value` into the heap section at immediate index `section_idx`, with byte offset from `reg_offset`.

**Opcode**: `0x0082`

**Arg Bytes**: 10

### Instruction Details

### Arguments

- **section_idx**: Immediate index of the heap section (Type: `U64`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreFromImmediateWithOffsetU16

Stores a u16 value from `reg_value` into the heap section at immediate index `section_idx`, with byte offset from `reg_offset`.

**Opcode**: `0x0083`

**Arg Bytes**: 10

### Instruction Details

### Arguments

- **section_idx**: Immediate index of the heap section (Type: `U64`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreFromImmediateWithOffsetU32

Stores a u32 value from `reg_value` into the heap section at immediate index `section_idx`, with byte offset from `reg_offset`.

**Opcode**: `0x0084`

**Arg Bytes**: 10

### Instruction Details

### Arguments

- **section_idx**: Immediate index of the heap section (Type: `U64`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreFromImmediateWithOffsetU64

Stores a u64 value from `reg_value` into the heap section at immediate index `section_idx`, with byte offset from `reg_offset`.

**Opcode**: `0x0085`

**Arg Bytes**: 10

### Instruction Details

### Arguments

- **section_idx**: Immediate index of the heap section (Type: `U64`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreFromImmediateWithOffsetI8

Stores a i8 value from `reg_value` into the heap section at immediate index `section_idx`, with byte offset from `reg_offset`.

**Opcode**: `0x0086`

**Arg Bytes**: 10

### Instruction Details

### Arguments

- **section_idx**: Immediate index of the heap section (Type: `U64`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreFromImmediateWithOffsetI16

Stores a i16 value from `reg_value` into the heap section at immediate index `section_idx`, with byte offset from `reg_offset`.

**Opcode**: `0x0087`

**Arg Bytes**: 10

### Instruction Details

### Arguments

- **section_idx**: Immediate index of the heap section (Type: `U64`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreFromImmediateWithOffsetI32

Stores a i32 value from `reg_value` into the heap section at immediate index `section_idx`, with byte offset from `reg_offset`.

**Opcode**: `0x0088`

**Arg Bytes**: 10

### Instruction Details

### Arguments

- **section_idx**: Immediate index of the heap section (Type: `U64`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreFromImmediateWithOffsetI64

Stores a i64 value from `reg_value` into the heap section at immediate index `section_idx`, with byte offset from `reg_offset`.

**Opcode**: `0x0089`

**Arg Bytes**: 10

### Instruction Details

### Arguments

- **section_idx**: Immediate index of the heap section (Type: `U64`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreFromImmediateWithOffsetF32

Stores a f32 value from `reg_value` into the heap section at immediate index `section_idx`, with byte offset from `reg_offset`.

**Opcode**: `0x008A`

**Arg Bytes**: 10

### Instruction Details

### Arguments

- **section_idx**: Immediate index of the heap section (Type: `U64`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## StoreFromImmediateWithOffsetF64

Stores a f64 value from `reg_value` into the heap section at immediate index `section_idx`, with byte offset from `reg_offset`.

**Opcode**: `0x008B`

**Arg Bytes**: 10

### Instruction Details

### Arguments

- **section_idx**: Immediate index of the heap section (Type: `U64`)
- **reg_value**: Register containing the value to store (Type: `Register`)
- **reg_offset**: Register containing the byte offset within the section (Type: `Register`)

### Tags

- Memory

## LogicalAnd

Performs logical && on the truthiness of two registers.

**Opcode**: `0x0096`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Register to store the result (0 or 1) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure

## LogicalOr

Performs logical || on the truthiness of two registers.

**Opcode**: `0x0097`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Register to store the result (0 or 1) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure

## LogicalNot

Performs logical negation (!), storing 1 if the input is zero, else 0.

**Opcode**: `0x0098`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Target register to store result (0 or 1) (Type: `Register`)
- **source**: Register containing value to logically negate (Type: `Register`)

### Tags

- Logical
- Pure

## LogicalXor

Performs logical ^ on the truthiness of two registers.

**Opcode**: `0x0099`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Register to store the result (0 or 1) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure

## AddU8

Adds two u8 registers and stores the result in the destination register.

**Opcode**: `0x00AA`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: First operand (Type: `Register`)
- **reg2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Pure
- Commutative

## AddU16

Adds two u16 registers and stores the result in the destination register.

**Opcode**: `0x00AB`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: First operand (Type: `Register`)
- **reg2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Pure
- Commutative

## AddU32

Adds two u32 registers and stores the result in the destination register.

**Opcode**: `0x00AC`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: First operand (Type: `Register`)
- **reg2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Pure
- Commutative

## AddU64

Adds two u64 registers and stores the result in the destination register.

**Opcode**: `0x00AD`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: First operand (Type: `Register`)
- **reg2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Pure
- Commutative

## AddI8

Adds two i8 registers and stores the result in the destination register.

**Opcode**: `0x00AE`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: First operand (Type: `Register`)
- **reg2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Pure
- Commutative

## AddI16

Adds two i16 registers and stores the result in the destination register.

**Opcode**: `0x00AF`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: First operand (Type: `Register`)
- **reg2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Pure
- Commutative

## AddI32

Adds two i32 registers and stores the result in the destination register.

**Opcode**: `0x00B0`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: First operand (Type: `Register`)
- **reg2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Pure
- Commutative

## AddI64

Adds two i64 registers and stores the result in the destination register.

**Opcode**: `0x00B1`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: First operand (Type: `Register`)
- **reg2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Pure
- Commutative

## AddF32

Adds two f32 floating-point registers and stores the result in the destination register.

**Opcode**: `0x00B2`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: First operand (Type: `Register`)
- **reg2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Pure
- Commutative

## AddF64

Adds two f64 floating-point registers and stores the result in the destination register.

**Opcode**: `0x00B3`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: First operand (Type: `Register`)
- **reg2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Pure
- Commutative

## SubtractU8

Subtracts two u8 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00C8`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (minuend) (Type: `Register`)
- **reg2**: Second operand register (subtrahend) (Type: `Register`)

### Tags

- Arithmetic

## SubtractU16

Subtracts two u16 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00C9`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (minuend) (Type: `Register`)
- **reg2**: Second operand register (subtrahend) (Type: `Register`)

### Tags

- Arithmetic

## SubtractU32

Subtracts two u32 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00CA`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (minuend) (Type: `Register`)
- **reg2**: Second operand register (subtrahend) (Type: `Register`)

### Tags

- Arithmetic

## SubtractU64

Subtracts two u64 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00CB`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (minuend) (Type: `Register`)
- **reg2**: Second operand register (subtrahend) (Type: `Register`)

### Tags

- Arithmetic

## SubtractI8

Subtracts two i8 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00CC`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (minuend) (Type: `Register`)
- **reg2**: Second operand register (subtrahend) (Type: `Register`)

### Tags

- Arithmetic

## SubtractI16

Subtracts two i16 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00CD`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (minuend) (Type: `Register`)
- **reg2**: Second operand register (subtrahend) (Type: `Register`)

### Tags

- Arithmetic

## SubtractI32

Subtracts two i32 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00CE`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (minuend) (Type: `Register`)
- **reg2**: Second operand register (subtrahend) (Type: `Register`)

### Tags

- Arithmetic

## SubtractI64

Subtracts two i64 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00CF`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (minuend) (Type: `Register`)
- **reg2**: Second operand register (subtrahend) (Type: `Register`)

### Tags

- Arithmetic

## SubtractF32

Subtracts two f32 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00D0`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (minuend) (Type: `Register`)
- **reg2**: Second operand register (subtrahend) (Type: `Register`)

### Tags

- Arithmetic

## SubtractF64

Subtracts two f64 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00D1`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (minuend) (Type: `Register`)
- **reg2**: Second operand register (subtrahend) (Type: `Register`)

### Tags

- Arithmetic

## MultiplyU8

Multiplies two u8 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00E6`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Arithmetic

## MultiplyU16

Multiplies two u16 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00E7`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Arithmetic

## MultiplyU32

Multiplies two u32 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00E8`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Arithmetic

## MultiplyU64

Multiplies two u64 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00E9`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Arithmetic

## MultiplyI8

Multiplies two i8 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00EA`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Arithmetic

## MultiplyI16

Multiplies two i16 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00EB`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Arithmetic

## MultiplyI32

Multiplies two i32 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00EC`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Arithmetic

## MultiplyI64

Multiplies two i64 values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00ED`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Arithmetic

## MultiplyF32

Multiplies two f32 floating-point values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00EE`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Arithmetic

## MultiplyF64

Multiplies two f64 floating-point values from `reg1` and `reg2`, storing the result in `dest`.

**Opcode**: `0x00EF`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Arithmetic

## DivideU8

Divides one u8 register by another and stores the result.

**Opcode**: `0x0104`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Numerator register (Type: `Register`)
- **reg2**: Denominator register (Type: `Register`)

### Tags

- Arithmetic
- Pure

## DivideU16

Divides one u16 register by another and stores the result.

**Opcode**: `0x0105`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Numerator register (Type: `Register`)
- **reg2**: Denominator register (Type: `Register`)

### Tags

- Arithmetic
- Pure

## DivideU32

Divides one u32 register by another and stores the result.

**Opcode**: `0x0106`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Numerator register (Type: `Register`)
- **reg2**: Denominator register (Type: `Register`)

### Tags

- Arithmetic
- Pure

## DivideU64

Divides one u64 register by another and stores the result.

**Opcode**: `0x0107`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Numerator register (Type: `Register`)
- **reg2**: Denominator register (Type: `Register`)

### Tags

- Arithmetic
- Pure

## DivideI8

Divides one i8 register by another and stores the result.

**Opcode**: `0x0108`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Numerator register (Type: `Register`)
- **reg2**: Denominator register (Type: `Register`)

### Tags

- Arithmetic
- Pure

## DivideI16

Divides one i16 register by another and stores the result.

**Opcode**: `0x0109`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Numerator register (Type: `Register`)
- **reg2**: Denominator register (Type: `Register`)

### Tags

- Arithmetic
- Pure

## DivideI32

Divides one i32 register by another and stores the result.

**Opcode**: `0x010A`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Numerator register (Type: `Register`)
- **reg2**: Denominator register (Type: `Register`)

### Tags

- Arithmetic
- Pure

## DivideI64

Divides one i64 register by another and stores the result.

**Opcode**: `0x010B`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Numerator register (Type: `Register`)
- **reg2**: Denominator register (Type: `Register`)

### Tags

- Arithmetic
- Pure

## DivideF32

Divides one f32 floating-point register by another and stores the result.

**Opcode**: `0x010C`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Numerator register (Type: `Register`)
- **reg2**: Denominator register (Type: `Register`)

### Tags

- Arithmetic
- Pure

## DivideF64

Divides one f64 floating-point register by another and stores the result.

**Opcode**: `0x010D`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Numerator register (Type: `Register`)
- **reg2**: Denominator register (Type: `Register`)

### Tags

- Arithmetic
- Pure

## ModuloU8

Computes the remainder of a u8 division. `dest = reg1 % reg2`. Sets error on division by zero.

**Opcode**: `0x0122`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Dividend register (Type: `Register`)
- **reg2**: Divisor register (Type: `Register`)

### Tags

- Arithmetic

## ModuloU16

Computes the remainder of a u16 division. `dest = reg1 % reg2`. Sets error on division by zero.

**Opcode**: `0x0123`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Dividend register (Type: `Register`)
- **reg2**: Divisor register (Type: `Register`)

### Tags

- Arithmetic

## ModuloU32

Computes the remainder of a u32 division. `dest = reg1 % reg2`. Sets error on division by zero.

**Opcode**: `0x0124`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Dividend register (Type: `Register`)
- **reg2**: Divisor register (Type: `Register`)

### Tags

- Arithmetic

## ModuloU64

Computes the remainder of a u64 division. `dest = reg1 % reg2`. Sets error on division by zero.

**Opcode**: `0x0125`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Dividend register (Type: `Register`)
- **reg2**: Divisor register (Type: `Register`)

### Tags

- Arithmetic

## ModuloI8

Computes the remainder of a i8 division. `dest = reg1 % reg2`. Sets error on division by zero.

**Opcode**: `0x0126`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Dividend register (Type: `Register`)
- **reg2**: Divisor register (Type: `Register`)

### Tags

- Arithmetic

## ModuloI16

Computes the remainder of a i16 division. `dest = reg1 % reg2`. Sets error on division by zero.

**Opcode**: `0x0127`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Dividend register (Type: `Register`)
- **reg2**: Divisor register (Type: `Register`)

### Tags

- Arithmetic

## ModuloI32

Computes the remainder of a i32 division. `dest = reg1 % reg2`. Sets error on division by zero.

**Opcode**: `0x0128`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Dividend register (Type: `Register`)
- **reg2**: Divisor register (Type: `Register`)

### Tags

- Arithmetic

## ModuloI64

Computes the remainder of a i64 division. `dest = reg1 % reg2`. Sets error on division by zero.

**Opcode**: `0x0129`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Dividend register (Type: `Register`)
- **reg2**: Divisor register (Type: `Register`)

### Tags

- Arithmetic

## ModuloF32

Computes the remainder of a f32 division using `%`. Result is undefined if inputs are NaN or infinite.

**Opcode**: `0x012A`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Dividend register (Type: `Register`)
- **reg2**: Divisor register (Type: `Register`)

### Tags

- Arithmetic

## ModuloF64

Computes the remainder of a f64 division using `%`. Result is undefined if inputs are NaN or infinite.

**Opcode**: `0x012B`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **reg1**: Dividend register (Type: `Register`)
- **reg2**: Divisor register (Type: `Register`)

### Tags

- Arithmetic

## EqualU8

Checks equality between two u8 registers. Sets 1 if equal, 0 otherwise.

**Opcode**: `0x015E`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure
- Commutative

## EqualU16

Checks equality between two u16 registers. Sets 1 if equal, 0 otherwise.

**Opcode**: `0x015F`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure
- Commutative

## EqualU32

Checks equality between two u32 registers. Sets 1 if equal, 0 otherwise.

**Opcode**: `0x0160`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure
- Commutative

## EqualU64

Checks equality between two u64 registers. Sets 1 if equal, 0 otherwise.

**Opcode**: `0x0161`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure
- Commutative

## EqualI8

Checks equality between two i8 registers. Sets 1 if equal, 0 otherwise.

**Opcode**: `0x0162`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure
- Commutative

## EqualI16

Checks equality between two i16 registers. Sets 1 if equal, 0 otherwise.

**Opcode**: `0x0163`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure
- Commutative

## EqualI32

Checks equality between two i32 registers. Sets 1 if equal, 0 otherwise.

**Opcode**: `0x0164`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure
- Commutative

## EqualI64

Checks equality between two i64 registers. Sets 1 if equal, 0 otherwise.

**Opcode**: `0x0165`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure
- Commutative

## EqualF32

Checks equality between two f32 floating-point registers. Sets 1 if equal, 0 otherwise.

**Opcode**: `0x0166`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure
- Commutative

## EqualF64

Checks equality between two f64 floating-point registers. Sets 1 if equal, 0 otherwise.

**Opcode**: `0x0167`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical
- Pure
- Commutative

## NotEqualU8

Compares two u8 values for inequality. Sets `dest` to 1 if not equal, 0 otherwise.

**Opcode**: `0x017C`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical

## NotEqualU16

Compares two u16 values for inequality. Sets `dest` to 1 if not equal, 0 otherwise.

**Opcode**: `0x017D`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical

## NotEqualU32

Compares two u32 values for inequality. Sets `dest` to 1 if not equal, 0 otherwise.

**Opcode**: `0x017E`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical

## NotEqualU64

Compares two u64 values for inequality. Sets `dest` to 1 if not equal, 0 otherwise.

**Opcode**: `0x017F`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical

## NotEqualI8

Compares two i8 values for inequality. Sets `dest` to 1 if not equal, 0 otherwise.

**Opcode**: `0x0180`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical

## NotEqualI16

Compares two i16 values for inequality. Sets `dest` to 1 if not equal, 0 otherwise.

**Opcode**: `0x0181`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical

## NotEqualI32

Compares two i32 values for inequality. Sets `dest` to 1 if not equal, 0 otherwise.

**Opcode**: `0x0182`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical

## NotEqualI64

Compares two i64 values for inequality. Sets `dest` to 1 if not equal, 0 otherwise.

**Opcode**: `0x0183`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical

## NotEqualF32

Compares two f32 floating-point values for inequality. Sets `dest` to 1 if not equal, 0 otherwise.

**Opcode**: `0x0184`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical

## NotEqualF64

Compares two f64 floating-point values for inequality. Sets `dest` to 1 if not equal, 0 otherwise.

**Opcode**: `0x0185`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register to store the result (1 or 0) (Type: `Register`)
- **reg1**: First operand register (Type: `Register`)
- **reg2**: Second operand register (Type: `Register`)

### Tags

- Logical

## LessThanU8

Compares two u8 registers. Sets 1 if the first is less than the second, else 0.

**Opcode**: `0x019A`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanU16

Compares two u16 registers. Sets 1 if the first is less than the second, else 0.

**Opcode**: `0x019B`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanU32

Compares two u32 registers. Sets 1 if the first is less than the second, else 0.

**Opcode**: `0x019C`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanU64

Compares two u64 registers. Sets 1 if the first is less than the second, else 0.

**Opcode**: `0x019D`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanI8

Compares two i8 registers. Sets 1 if the first is less than the second, else 0.

**Opcode**: `0x019E`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanI16

Compares two i16 registers. Sets 1 if the first is less than the second, else 0.

**Opcode**: `0x019F`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanI32

Compares two i32 registers. Sets 1 if the first is less than the second, else 0.

**Opcode**: `0x01A0`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanI64

Compares two i64 registers. Sets 1 if the first is less than the second, else 0.

**Opcode**: `0x01A1`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanF32

Compares two f32 floating-point registers. Sets 1 if the first is less than the second, else 0.

**Opcode**: `0x01A2`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanF64

Compares two f64 floating-point registers. Sets 1 if the first is less than the second, else 0.

**Opcode**: `0x01A3`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanOrEqualU8

Compares two u8 registers. Sets 1 if the first is less than or equal to the second, else 0.

**Opcode**: `0x01B8`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanOrEqualU16

Compares two u16 registers. Sets 1 if the first is less than or equal to the second, else 0.

**Opcode**: `0x01B9`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanOrEqualU32

Compares two u32 registers. Sets 1 if the first is less than or equal to the second, else 0.

**Opcode**: `0x01BA`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanOrEqualU64

Compares two u64 registers. Sets 1 if the first is less than or equal to the second, else 0.

**Opcode**: `0x01BB`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanOrEqualI8

Compares two i8 registers. Sets 1 if the first is less than or equal to the second, else 0.

**Opcode**: `0x01BC`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanOrEqualI16

Compares two i16 registers. Sets 1 if the first is less than or equal to the second, else 0.

**Opcode**: `0x01BD`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanOrEqualI32

Compares two i32 registers. Sets 1 if the first is less than or equal to the second, else 0.

**Opcode**: `0x01BE`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanOrEqualI64

Compares two i64 registers. Sets 1 if the first is less than or equal to the second, else 0.

**Opcode**: `0x01BF`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanOrEqualF32

Compares two f32 floating-point registers. Sets 1 if the first is less than or equal to the second, else 0.

**Opcode**: `0x01C0`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## LessThanOrEqualF64

Compares two f64 floating-point registers. Sets 1 if the first is less than or equal to the second, else 0.

**Opcode**: `0x01C1`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanU8

Compares two u8 registers. Sets 1 if the first is greater than the second, else 0.

**Opcode**: `0x01D6`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanU16

Compares two u16 registers. Sets 1 if the first is greater than the second, else 0.

**Opcode**: `0x01D7`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanU32

Compares two u32 registers. Sets 1 if the first is greater than the second, else 0.

**Opcode**: `0x01D8`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanU64

Compares two u64 registers. Sets 1 if the first is greater than the second, else 0.

**Opcode**: `0x01D9`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanI8

Compares two i8 registers. Sets 1 if the first is greater than the second, else 0.

**Opcode**: `0x01DA`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanI16

Compares two i16 registers. Sets 1 if the first is greater than the second, else 0.

**Opcode**: `0x01DB`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanI32

Compares two i32 registers. Sets 1 if the first is greater than the second, else 0.

**Opcode**: `0x01DC`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanI64

Compares two i64 registers. Sets 1 if the first is greater than the second, else 0.

**Opcode**: `0x01DD`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanF32

Compares two f32 floating-point registers. Sets 1 if the first is greater than the second, else 0.

**Opcode**: `0x01DE`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanF64

Compares two f64 floating-point registers. Sets 1 if the first is greater than the second, else 0.

**Opcode**: `0x01DF`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanOrEqualU8

Compares two u8 registers. Sets 1 if the first is greater than or equal to the second, else 0.

**Opcode**: `0x01F4`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanOrEqualU16

Compares two u16 registers. Sets 1 if the first is greater than or equal to the second, else 0.

**Opcode**: `0x01F5`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanOrEqualU32

Compares two u32 registers. Sets 1 if the first is greater than or equal to the second, else 0.

**Opcode**: `0x01F6`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanOrEqualU64

Compares two u64 registers. Sets 1 if the first is greater than or equal to the second, else 0.

**Opcode**: `0x01F7`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanOrEqualI8

Compares two i8 registers. Sets 1 if the first is greater than or equal to the second, else 0.

**Opcode**: `0x01F8`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanOrEqualI16

Compares two i16 registers. Sets 1 if the first is greater than or equal to the second, else 0.

**Opcode**: `0x01F9`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanOrEqualI32

Compares two i32 registers. Sets 1 if the first is greater than or equal to the second, else 0.

**Opcode**: `0x01FA`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanOrEqualI64

Compares two i64 registers. Sets 1 if the first is greater than or equal to the second, else 0.

**Opcode**: `0x01FB`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanOrEqualF32

Compares two f32 floating-point registers. Sets 1 if the first is greater than or equal to the second, else 0.

**Opcode**: `0x01FC`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## GreaterThanOrEqualF64

Compares two f64 floating-point registers. Sets 1 if the first is greater than or equal to the second, else 0.

**Opcode**: `0x01FD`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for result (1 or 0) (Type: `Register`)
- **reg1**: First operand (left-hand side) (Type: `Register`)
- **reg2**: Second operand (right-hand side) (Type: `Register`)

### Tags

- Logical
- Pure

## CallFunction

Calls a function at the specified bytecode address. Saves the return address and switches stack frame.

**Opcode**: `0x0258`

**Arg Bytes**: 8

### Instruction Details

### Arguments

- **target**: Bytecode address (offset) to jump to for the function (Type: `U64`)

### Tags

- Control Flow
- Side Effects

## Return

Returns from the current function by restoring the previous frame and program counter.

**Opcode**: `0x0259`

**Arg Bytes**: 0

### Instruction Details

### Tags

- Control Flow
- Side Effects

## Allocate

Allocates a heap section of size from `reg_size` and stores the section index in `reg_target`.

**Opcode**: `0x025A`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **reg_target**: Register to store the section index (Type: `Register`)
- **reg_size**: Register containing the allocation size in bytes (Type: `Register`)

### Tags

- Allocation

## Deallocate

Frees the heap section at the index given in `reg_target`.

**Opcode**: `0x025B`

**Arg Bytes**: 1

### Instruction Details

### Arguments

- **reg_target**: Register containing the section index to free (Type: `Register`)

### Tags

- Allocation

## Memcpy

Copies memory from a source heap section to a destination heap section.

**Opcode**: `0x025C`

**Arg Bytes**: 5

### Instruction Details

### Arguments

- **reg_dest**: Register with destination section index (Type: `Register`)
- **reg_dest_offset**: Offset in destination section (Type: `Register`)
- **reg_src**: Register with source section index (Type: `Register`)
- **reg_src_offset**: Offset in source section (Type: `Register`)
- **reg_size**: Number of bytes to copy (Type: `Register`)

### Tags

- Memory

## MemSet

Fills a heap section with a repeated byte value.

**Opcode**: `0x025D`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_ptr**: Register with section index to fill (Type: `Register`)
- **reg_value**: Register with byte value to fill (only lowest 8 bits used) (Type: `Register`)
- **reg_size**: Register with number of bytes to fill (Type: `Register`)

### Tags

- Memory

## Halt

Halts execution of the virtual machine immediately.

**Opcode**: `0x025E`

**Arg Bytes**: 8

### Instruction Details

### Arguments

- **exit_code**: Exit code for the VM (default is 0) (Type: `I64`)

### Tags

- Control Flow
- Side Effects

## MoveU8

Moves a u8 value from one register to another. `dest = source`.

**Opcode**: `0x0335`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **source**: Source register (Type: `Register`)

### Tags

- Data Movement

## MoveU16

Moves a u16 value from one register to another. `dest = source`.

**Opcode**: `0x0336`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **source**: Source register (Type: `Register`)

### Tags

- Data Movement

## MoveU32

Moves a u32 value from one register to another. `dest = source`.

**Opcode**: `0x0337`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **source**: Source register (Type: `Register`)

### Tags

- Data Movement

## MoveU64

Moves a u64 value from one register to another. `dest = source`.

**Opcode**: `0x0338`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **source**: Source register (Type: `Register`)

### Tags

- Data Movement

## MoveI8

Moves a i8 value from one register to another. `dest = source`.

**Opcode**: `0x0339`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **source**: Source register (Type: `Register`)

### Tags

- Data Movement

## MoveI16

Moves a i16 value from one register to another. `dest = source`.

**Opcode**: `0x033A`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **source**: Source register (Type: `Register`)

### Tags

- Data Movement

## MoveI32

Moves a i32 value from one register to another. `dest = source`.

**Opcode**: `0x033B`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **source**: Source register (Type: `Register`)

### Tags

- Data Movement

## MoveI64

Moves a i64 value from one register to another. `dest = source`.

**Opcode**: `0x033C`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **source**: Source register (Type: `Register`)

### Tags

- Data Movement

## MoveF32

Moves a f32 value from one register to another. `dest = source`.

**Opcode**: `0x033D`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **source**: Source register (Type: `Register`)

### Tags

- Data Movement

## MoveF64

Moves a f64 value from one register to another. `dest = source`.

**Opcode**: `0x033E`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **source**: Source register (Type: `Register`)

### Tags

- Data Movement

## IncrementU8

Adds an immediate u8 value to the destination register.

**Opcode**: `0x0349`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Target register to be incremented (Type: `Register`)
- **incr_val**: Immediate value to add to the register (Type: `U8`)

### Tags

- Arithmetic

## IncrementU16

Adds an immediate u16 value to the destination register.

**Opcode**: `0x034A`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Target register to be incremented (Type: `Register`)
- **incr_val**: Immediate value to add to the register (Type: `U16`)

### Tags

- Arithmetic

## IncrementU32

Adds an immediate u32 value to the destination register.

**Opcode**: `0x034B`

**Arg Bytes**: 5

### Instruction Details

### Arguments

- **dest**: Target register to be incremented (Type: `Register`)
- **incr_val**: Immediate value to add to the register (Type: `U32`)

### Tags

- Arithmetic

## IncrementU64

Adds an immediate u64 value to the destination register.

**Opcode**: `0x034C`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **dest**: Target register to be incremented (Type: `Register`)
- **incr_val**: Immediate value to add to the register (Type: `U64`)

### Tags

- Arithmetic

## IncrementI8

Adds an immediate i8 value to the destination register.

**Opcode**: `0x034D`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Target register to be incremented (Type: `Register`)
- **incr_val**: Immediate value to add to the register (Type: `I8`)

### Tags

- Arithmetic

## IncrementI16

Adds an immediate i16 value to the destination register.

**Opcode**: `0x034E`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Target register to be incremented (Type: `Register`)
- **incr_val**: Immediate value to add to the register (Type: `I16`)

### Tags

- Arithmetic

## IncrementI32

Adds an immediate i32 value to the destination register.

**Opcode**: `0x034F`

**Arg Bytes**: 5

### Instruction Details

### Arguments

- **dest**: Target register to be incremented (Type: `Register`)
- **incr_val**: Immediate value to add to the register (Type: `I32`)

### Tags

- Arithmetic

## IncrementI64

Adds an immediate i64 value to the destination register.

**Opcode**: `0x0350`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **dest**: Target register to be incremented (Type: `Register`)
- **incr_val**: Immediate value to add to the register (Type: `I64`)

### Tags

- Arithmetic

## IncrementF32

Adds an immediate f32 floating-point value to the destination register.

**Opcode**: `0x0351`

**Arg Bytes**: 5

### Instruction Details

### Arguments

- **dest**: Target register to be incremented (Type: `Register`)
- **incr_val**: Immediate value to add to the register (Type: `F32`)

### Tags

- Arithmetic

## IncrementF64

Adds an immediate f64 floating-point value to the destination register.

**Opcode**: `0x0352`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **dest**: Target register to be incremented (Type: `Register`)
- **incr_val**: Immediate value to add to the register (Type: `F64`)

### Tags

- Arithmetic

## DecrementU8

Decrements a u8 register by a constant value.

**Opcode**: `0x035D`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Register to decrement (Type: `Register`)
- **decr_val**: Constant value to subtract (Type: `U8`)

### Tags

- Arithmetic
- Pure

## DecrementU16

Decrements a u16 register by a constant value.

**Opcode**: `0x035E`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Register to decrement (Type: `Register`)
- **decr_val**: Constant value to subtract (Type: `U16`)

### Tags

- Arithmetic
- Pure

## DecrementU32

Decrements a u32 register by a constant value.

**Opcode**: `0x035F`

**Arg Bytes**: 5

### Instruction Details

### Arguments

- **dest**: Register to decrement (Type: `Register`)
- **decr_val**: Constant value to subtract (Type: `U32`)

### Tags

- Arithmetic
- Pure

## DecrementU64

Decrements a u64 register by a constant value.

**Opcode**: `0x0360`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **dest**: Register to decrement (Type: `Register`)
- **decr_val**: Constant value to subtract (Type: `U64`)

### Tags

- Arithmetic
- Pure

## DecrementI8

Decrements a i8 register by a constant value.

**Opcode**: `0x0361`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Register to decrement (Type: `Register`)
- **decr_val**: Constant value to subtract (Type: `I8`)

### Tags

- Arithmetic
- Pure

## DecrementI16

Decrements a i16 register by a constant value.

**Opcode**: `0x0362`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Register to decrement (Type: `Register`)
- **decr_val**: Constant value to subtract (Type: `I16`)

### Tags

- Arithmetic
- Pure

## DecrementI32

Decrements a i32 register by a constant value.

**Opcode**: `0x0363`

**Arg Bytes**: 5

### Instruction Details

### Arguments

- **dest**: Register to decrement (Type: `Register`)
- **decr_val**: Constant value to subtract (Type: `I32`)

### Tags

- Arithmetic
- Pure

## DecrementI64

Decrements a i64 register by a constant value.

**Opcode**: `0x0364`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **dest**: Register to decrement (Type: `Register`)
- **decr_val**: Constant value to subtract (Type: `I64`)

### Tags

- Arithmetic
- Pure

## DecrementF32

Decrements a f32 floating-point register by a constant value.

**Opcode**: `0x0365`

**Arg Bytes**: 5

### Instruction Details

### Arguments

- **dest**: Register to decrement (Type: `Register`)
- **incr_val**: Constant value to subtract (Type: `F32`)

### Tags

- Arithmetic
- Pure

## DecrementF64

Decrements a f64 floating-point register by a constant value.

**Opcode**: `0x0366`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **dest**: Register to decrement (Type: `Register`)
- **incr_val**: Constant value to subtract (Type: `F64`)

### Tags

- Arithmetic
- Pure

## BitwiseAndU8

Performs a bitwise AND between two u8 registers and stores the result.

**Opcode**: `0x0384`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseAndU16

Performs a bitwise AND between two u16 registers and stores the result.

**Opcode**: `0x0385`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseAndU32

Performs a bitwise AND between two u32 registers and stores the result.

**Opcode**: `0x0386`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseAndU64

Performs a bitwise AND between two u64 registers and stores the result.

**Opcode**: `0x0387`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseAndI8

Performs a bitwise AND between two i8 registers and stores the result.

**Opcode**: `0x0388`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseAndI16

Performs a bitwise AND between two i16 registers and stores the result.

**Opcode**: `0x0389`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseAndI32

Performs a bitwise AND between two i32 registers and stores the result.

**Opcode**: `0x038A`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseAndI64

Performs a bitwise AND between two i64 registers and stores the result.

**Opcode**: `0x038B`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseOrU8

Performs a bitwise OR between two u8 registers and stores the result.

**Opcode**: `0x038E`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseOrU16

Performs a bitwise OR between two u16 registers and stores the result.

**Opcode**: `0x038F`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseOrU32

Performs a bitwise OR between two u32 registers and stores the result.

**Opcode**: `0x0390`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseOrU64

Performs a bitwise OR between two u64 registers and stores the result.

**Opcode**: `0x0391`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseOrI8

Performs a bitwise OR between two i8 registers and stores the result.

**Opcode**: `0x0392`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseOrI16

Performs a bitwise OR between two i16 registers and stores the result.

**Opcode**: `0x0393`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseOrI32

Performs a bitwise OR between two i32 registers and stores the result.

**Opcode**: `0x0394`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseOrI64

Performs a bitwise OR between two i64 registers and stores the result.

**Opcode**: `0x0395`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseXorU8

Performs a bitwise XOR between two u8 registers and stores the result.

**Opcode**: `0x0398`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseXorU16

Performs a bitwise XOR between two u16 registers and stores the result.

**Opcode**: `0x0399`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseXorU32

Performs a bitwise XOR between two u32 registers and stores the result.

**Opcode**: `0x039A`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseXorU64

Performs a bitwise XOR between two u64 registers and stores the result.

**Opcode**: `0x039B`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseXorI8

Performs a bitwise XOR between two i8 registers and stores the result.

**Opcode**: `0x039C`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseXorI16

Performs a bitwise XOR between two i16 registers and stores the result.

**Opcode**: `0x039D`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseXorI32

Performs a bitwise XOR between two i32 registers and stores the result.

**Opcode**: `0x039E`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseXorI64

Performs a bitwise XOR between two i64 registers and stores the result.

**Opcode**: `0x039F`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **r1**: First operand (Type: `Register`)
- **r2**: Second operand (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure
- Commutative

## BitwiseNotU8

Performs a bitwise NOT on a u8 register and stores the result.

**Opcode**: `0x03A2`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **src**: Source register (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure

## BitwiseNotU16

Performs a bitwise NOT on a u16 register and stores the result.

**Opcode**: `0x03A3`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **src**: Source register (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure

## BitwiseNotU32

Performs a bitwise NOT on a u32 register and stores the result.

**Opcode**: `0x03A4`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **src**: Source register (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure

## BitwiseNotU64

Performs a bitwise NOT on a u64 register and stores the result.

**Opcode**: `0x03A5`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **src**: Source register (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure

## BitwiseNotI8

Performs a bitwise NOT on a i8 register and stores the result.

**Opcode**: `0x03A6`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **src**: Source register (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure

## BitwiseNotI16

Performs a bitwise NOT on a i16 register and stores the result.

**Opcode**: `0x03A7`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **src**: Source register (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure

## BitwiseNotI32

Performs a bitwise NOT on a i32 register and stores the result.

**Opcode**: `0x03A8`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **src**: Source register (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure

## BitwiseNotI64

Performs a bitwise NOT on a i64 register and stores the result.

**Opcode**: `0x03A9`

**Arg Bytes**: 2

### Instruction Details

### Arguments

- **dest**: Destination register (Type: `Register`)
- **src**: Source register (Type: `Register`)

### Tags

- Arithmetic
- Logical
- Pure

## ShiftLeftU8

Performs left bit shift on a u8 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03AC`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftLeftU16

Performs left bit shift on a u16 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03AD`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftLeftU32

Performs left bit shift on a u32 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03AE`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftLeftU64

Performs left bit shift on a u64 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03AF`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftLeftI8

Performs left bit shift on a i8 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03B0`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftLeftI16

Performs left bit shift on a i16 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03B1`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftLeftI32

Performs left bit shift on a i32 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03B2`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftLeftI64

Performs left bit shift on a i64 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03B3`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftRightU8

Performs right bit shift on a u8 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03B6`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftRightU16

Performs right bit shift on a u16 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03B7`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftRightU32

Performs right bit shift on a u32 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03B8`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftRightU64

Performs right bit shift on a u64 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03B9`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftRightI8

Performs right bit shift on a i8 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03BA`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftRightI16

Performs right bit shift on a i16 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03BB`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftRightI32

Performs right bit shift on a i32 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03BC`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## ShiftRightI64

Performs right bit shift on a i64 value from `val_reg` by amount in `shift_reg`, storing result in `dest`.

**Opcode**: `0x03BD`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **dest**: Destination register for the shifted result (Type: `Register`)
- **val_reg**: Register containing the value to shift (Type: `Register`)
- **shift_reg**: Register containing the shift amount (Type: `Register`)

### Tags

- Arithmetic

## Print

Prints a UTF-8 string from memory to standard output. Reads `length` bytes from `section_id` starting at `offset`, decodes as UTF-8, and prints the resulting string.

**Opcode**: `0x03E8`

**Arg Bytes**: 3

### Instruction Details

### Arguments

- **reg_section_id**: Register containing the heap section index (Type: `Register`)
- **reg_offset**: Register containing the byte offset into the section (Type: `Register`)
- **reg_length**: Register containing the number of bytes to read (Type: `Register`)

### Tags

- Side Effects

## StoreConstantArray

Copies a constant array (identified by `const_id`) into a newly allocated heap section, storing the section index in `reg_ptr`.

**Opcode**: `0x0406`

**Arg Bytes**: 9

### Instruction Details

### Arguments

- **reg_ptr**: Register to store the resulting section index (Type: `Register`)
- **const_id**: Identifier of the constant array to store (Type: `U64`)

### Tags

- Memory
- Allocation

## DebugPrintU8

Prints the value of a u8 register to stdout for debugging.

**Opcode**: `0x07D0`

**Arg Bytes**: 1

### Instruction Details

### Arguments

- **source**: Register to print (Type: `Register`)

### Tags

- Side Effects

## DebugPrintU16

Prints the value of a u16 register to stdout for debugging.

**Opcode**: `0x07D1`

**Arg Bytes**: 1

### Instruction Details

### Arguments

- **source**: Register to print (Type: `Register`)

### Tags

- Side Effects

## DebugPrintU32

Prints the value of a u32 register to stdout for debugging.

**Opcode**: `0x07D2`

**Arg Bytes**: 1

### Instruction Details

### Arguments

- **source**: Register to print (Type: `Register`)

### Tags

- Side Effects

## DebugPrintU64

Prints the value of a u64 register to stdout for debugging.

**Opcode**: `0x07D3`

**Arg Bytes**: 1

### Instruction Details

### Arguments

- **source**: Register to print (Type: `Register`)

### Tags

- Side Effects

## DebugPrintI8

Prints the value of a i8 register to stdout for debugging.

**Opcode**: `0x4E24`

**Arg Bytes**: 1

### Instruction Details

### Arguments

- **source**: Register to print (Type: `Register`)

### Tags

- Side Effects

## DebugPrintI16

Prints the value of a i16 register to stdout for debugging.

**Opcode**: `0x07D5`

**Arg Bytes**: 1

### Instruction Details

### Arguments

- **source**: Register to print (Type: `Register`)

### Tags

- Side Effects

## DebugPrintI32

Prints the value of a i32 register to stdout for debugging.

**Opcode**: `0x07D6`

**Arg Bytes**: 1

### Instruction Details

### Arguments

- **source**: Register to print (Type: `Register`)

### Tags

- Side Effects

## DebugPrintI64

Prints the value of a i64 register to stdout for debugging.

**Opcode**: `0x07D7`

**Arg Bytes**: 1

### Instruction Details

### Arguments

- **source**: Register to print (Type: `Register`)

### Tags

- Side Effects

## DebugPrintF32

Prints the value of a f32 register to stdout for debugging.

**Opcode**: `0x07D8`

**Arg Bytes**: 1

### Instruction Details

### Arguments

- **source**: Register to print (Type: `Register`)

### Tags

- Side Effects

## DebugPrintF64

Prints the value of a f64 register to stdout for debugging.

**Opcode**: `0x07D9`

**Arg Bytes**: 1

### Instruction Details

### Arguments

- **source**: Register to print (Type: `Register`)

### Tags

- Side Effects

## DebugPrintRaw

Prints the raw 64-bit value of a register in hexadecimal for debugging.

**Opcode**: `0x07DA`

**Arg Bytes**: 1

### Instruction Details

### Arguments

- **reg**: Register to inspect as raw bits (Type: `Register`)

### Tags

- Side Effects

