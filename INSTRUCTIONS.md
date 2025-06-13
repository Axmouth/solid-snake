# Solid Snake Bytecode Instructions

This document provides a comprehensive overview of the bytecode instructions used in the Solid Snake virtual machine.

## JumpIfFalse

Instruction for JumpIfFalse

**Opcode**: `0x0001`

### Instruction Details

### Arguments

- **target**: Bytecode address(byte offset) to jump to (Type: `U64`)
- **reg**: Register to check (Type: `Register`)

This instruction is not commutative.

## JumpIf

Instruction for JumpIf

**Opcode**: `0x0002`

### Instruction Details

### Arguments

- **target**: Bytecode address(byte offset) to jump to (Type: `U64`)
- **reg**: Register to check (Type: `Register`)

This instruction is not commutative.

## Jump

Instruction for Jump

**Opcode**: `0x0003`

### Instruction Details

### Arguments

- **target**: Bytecode address(byte offset) to jump to (Type: `U64`)

This instruction is not commutative.

## LoadIndirectU8

Instruction for LoadIndirectU8

**Opcode**: `0x0014`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectU16

Instruction for LoadIndirectU16

**Opcode**: `0x0015`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectU32

Instruction for LoadIndirectU32

**Opcode**: `0x0016`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectU64

Instruction for LoadIndirectU64

**Opcode**: `0x0017`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectI8

Instruction for LoadIndirectI8

**Opcode**: `0x0018`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectI16

Instruction for LoadIndirectI16

**Opcode**: `0x0019`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectI32

Instruction for LoadIndirectI32

**Opcode**: `0x001A`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectI64

Instruction for LoadIndirectI64

**Opcode**: `0x001B`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectF32

Instruction for LoadIndirectF32

**Opcode**: `0x001C`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectF64

Instruction for LoadIndirectF64

**Opcode**: `0x001D`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectWithOffsetU8

Instruction for LoadIndirectWithOffsetU8

**Opcode**: `0x001E`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectWithOffsetU16

Instruction for LoadIndirectWithOffsetU16

**Opcode**: `0x001F`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectWithOffsetU32

Instruction for LoadIndirectWithOffsetU32

**Opcode**: `0x0020`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectWithOffsetU64

Instruction for LoadIndirectWithOffsetU64

**Opcode**: `0x0021`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectWithOffsetI8

Instruction for LoadIndirectWithOffsetI8

**Opcode**: `0x0022`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectWithOffsetI16

Instruction for LoadIndirectWithOffsetI16

**Opcode**: `0x0023`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectWithOffsetI32

Instruction for LoadIndirectWithOffsetI32

**Opcode**: `0x0024`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectWithOffsetI64

Instruction for LoadIndirectWithOffsetI64

**Opcode**: `0x0025`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectWithOffsetF32

Instruction for LoadIndirectWithOffsetF32

**Opcode**: `0x0026`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadIndirectWithOffsetF64

Instruction for LoadIndirectWithOffsetF64

**Opcode**: `0x0027`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LoadImmediateU8

Instruction for LoadImmediateU8

**Opcode**: `0x0028`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u8**: Argument of type u8 (Type: `U8`)

This instruction is not commutative.

## LoadImmediateU16

Instruction for LoadImmediateU16

**Opcode**: `0x0029`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u16**: Argument of type u16 (Type: `U16`)

This instruction is not commutative.

## LoadImmediateU32

Instruction for LoadImmediateU32

**Opcode**: `0x002A`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u32**: Argument of type u32 (Type: `U32`)

This instruction is not commutative.

## LoadImmediateU64

Instruction for LoadImmediateU64

**Opcode**: `0x002B`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## LoadImmediateI8

Instruction for LoadImmediateI8

**Opcode**: `0x002C`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **i8**: Argument of type i8 (Type: `I8`)

This instruction is not commutative.

## LoadImmediateI16

Instruction for LoadImmediateI16

**Opcode**: `0x002D`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **i16**: Argument of type i16 (Type: `I16`)

This instruction is not commutative.

## LoadImmediateI32

Instruction for LoadImmediateI32

**Opcode**: `0x002E`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **i32**: Argument of type i32 (Type: `I32`)

This instruction is not commutative.

## LoadImmediateI64

Instruction for LoadImmediateI64

**Opcode**: `0x002F`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **i64**: Argument of type i64 (Type: `I64`)

This instruction is not commutative.

## LoadImmediateF32

Instruction for LoadImmediateF32

**Opcode**: `0x0030`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **f32**: Argument of type f32 (Type: `F32`)

This instruction is not commutative.

## LoadImmediateF64

Instruction for LoadImmediateF64

**Opcode**: `0x0031`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **f64**: Argument of type f64 (Type: `F64`)

This instruction is not commutative.

## LoadFromImmediateU8

Instruction for LoadFromImmediateU8

**Opcode**: `0x003C`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## LoadFromImmediateU16

Instruction for LoadFromImmediateU16

**Opcode**: `0x003D`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## LoadFromImmediateU32

Instruction for LoadFromImmediateU32

**Opcode**: `0x003E`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## LoadFromImmediateU64

Instruction for LoadFromImmediateU64

**Opcode**: `0x003F`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## LoadFromImmediateI8

Instruction for LoadFromImmediateI8

**Opcode**: `0x0040`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## LoadFromImmediateI16

Instruction for LoadFromImmediateI16

**Opcode**: `0x0041`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## LoadFromImmediateI32

Instruction for LoadFromImmediateI32

**Opcode**: `0x0042`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## LoadFromImmediateI64

Instruction for LoadFromImmediateI64

**Opcode**: `0x0043`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## LoadFromImmediateF32

Instruction for LoadFromImmediateF32

**Opcode**: `0x0044`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## LoadFromImmediateF64

Instruction for LoadFromImmediateF64

**Opcode**: `0x0045`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## StoreIndirectWithOffsetU8

Instruction for StoreIndirectWithOffsetU8

**Opcode**: `0x005A`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreIndirectWithOffsetU16

Instruction for StoreIndirectWithOffsetU16

**Opcode**: `0x005B`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreIndirectWithOffsetU32

Instruction for StoreIndirectWithOffsetU32

**Opcode**: `0x005C`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreIndirectWithOffsetU64

Instruction for StoreIndirectWithOffsetU64

**Opcode**: `0x005D`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreIndirectWithOffsetI8

Instruction for StoreIndirectWithOffsetI8

**Opcode**: `0x005E`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreIndirectWithOffsetI16

Instruction for StoreIndirectWithOffsetI16

**Opcode**: `0x005F`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreIndirectWithOffsetI32

Instruction for StoreIndirectWithOffsetI32

**Opcode**: `0x0060`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreIndirectWithOffsetI64

Instruction for StoreIndirectWithOffsetI64

**Opcode**: `0x0061`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreIndirectWithOffsetF32

Instruction for StoreIndirectWithOffsetF32

**Opcode**: `0x0062`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreIndirectWithOffsetF64

Instruction for StoreIndirectWithOffsetF64

**Opcode**: `0x0063`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreFromImmediateWithOffsetU8

Instruction for StoreFromImmediateWithOffsetU8

**Opcode**: `0x0082`

### Instruction Details

### Arguments

- **u64**: Argument of type u64 (Type: `U64`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreFromImmediateWithOffsetU16

Instruction for StoreFromImmediateWithOffsetU16

**Opcode**: `0x0083`

### Instruction Details

### Arguments

- **u64**: Argument of type u64 (Type: `U64`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreFromImmediateWithOffsetU32

Instruction for StoreFromImmediateWithOffsetU32

**Opcode**: `0x0084`

### Instruction Details

### Arguments

- **u64**: Argument of type u64 (Type: `U64`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreFromImmediateWithOffsetU64

Instruction for StoreFromImmediateWithOffsetU64

**Opcode**: `0x0085`

### Instruction Details

### Arguments

- **u64**: Argument of type u64 (Type: `U64`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreFromImmediateWithOffsetI8

Instruction for StoreFromImmediateWithOffsetI8

**Opcode**: `0x0086`

### Instruction Details

### Arguments

- **u64**: Argument of type u64 (Type: `U64`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreFromImmediateWithOffsetI16

Instruction for StoreFromImmediateWithOffsetI16

**Opcode**: `0x0087`

### Instruction Details

### Arguments

- **u64**: Argument of type u64 (Type: `U64`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreFromImmediateWithOffsetI32

Instruction for StoreFromImmediateWithOffsetI32

**Opcode**: `0x0088`

### Instruction Details

### Arguments

- **u64**: Argument of type u64 (Type: `U64`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreFromImmediateWithOffsetI64

Instruction for StoreFromImmediateWithOffsetI64

**Opcode**: `0x0089`

### Instruction Details

### Arguments

- **u64**: Argument of type u64 (Type: `U64`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreFromImmediateWithOffsetF32

Instruction for StoreFromImmediateWithOffsetF32

**Opcode**: `0x008A`

### Instruction Details

### Arguments

- **u64**: Argument of type u64 (Type: `U64`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreFromImmediateWithOffsetF64

Instruction for StoreFromImmediateWithOffsetF64

**Opcode**: `0x008B`

### Instruction Details

### Arguments

- **u64**: Argument of type u64 (Type: `U64`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LogicalAnd

Instruction for LogicalAnd

**Opcode**: `0x0096`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LogicalOr

Instruction for LogicalOr

**Opcode**: `0x0097`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LogicalNot

Instruction for LogicalNot

**Opcode**: `0x0098`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LogicalXor

Instruction for LogicalXor

**Opcode**: `0x0099`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## AddU8

Instruction for AddU8

**Opcode**: `0x00AA`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## AddU16

Instruction for AddU16

**Opcode**: `0x00AB`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## AddU32

Instruction for AddU32

**Opcode**: `0x00AC`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## AddU64

Instruction for AddU64

**Opcode**: `0x00AD`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## AddI8

Instruction for AddI8

**Opcode**: `0x00AE`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## AddI16

Instruction for AddI16

**Opcode**: `0x00AF`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## AddI32

Instruction for AddI32

**Opcode**: `0x00B0`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## AddI64

Instruction for AddI64

**Opcode**: `0x00B1`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## AddF32

Instruction for AddF32

**Opcode**: `0x00B2`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## AddF64

Instruction for AddF64

**Opcode**: `0x00B3`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## SubtractU8

Instruction for SubtractU8

**Opcode**: `0x00C8`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## SubtractU16

Instruction for SubtractU16

**Opcode**: `0x00C9`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## SubtractU32

Instruction for SubtractU32

**Opcode**: `0x00CA`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## SubtractU64

Instruction for SubtractU64

**Opcode**: `0x00CB`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## SubtractI8

Instruction for SubtractI8

**Opcode**: `0x00CC`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## SubtractI16

Instruction for SubtractI16

**Opcode**: `0x00CD`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## SubtractI32

Instruction for SubtractI32

**Opcode**: `0x00CE`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## SubtractI64

Instruction for SubtractI64

**Opcode**: `0x00CF`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## SubtractF32

Instruction for SubtractF32

**Opcode**: `0x00D0`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## SubtractF64

Instruction for SubtractF64

**Opcode**: `0x00D1`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MultiplyU8

Instruction for MultiplyU8

**Opcode**: `0x00E6`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MultiplyU16

Instruction for MultiplyU16

**Opcode**: `0x00E7`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MultiplyU32

Instruction for MultiplyU32

**Opcode**: `0x00E8`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MultiplyU64

Instruction for MultiplyU64

**Opcode**: `0x00E9`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MultiplyI8

Instruction for MultiplyI8

**Opcode**: `0x00EA`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MultiplyI16

Instruction for MultiplyI16

**Opcode**: `0x00EB`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MultiplyI32

Instruction for MultiplyI32

**Opcode**: `0x00EC`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MultiplyI64

Instruction for MultiplyI64

**Opcode**: `0x00ED`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MultiplyF32

Instruction for MultiplyF32

**Opcode**: `0x00EE`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MultiplyF64

Instruction for MultiplyF64

**Opcode**: `0x00EF`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DivideU8

Instruction for DivideU8

**Opcode**: `0x0104`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DivideU16

Instruction for DivideU16

**Opcode**: `0x0105`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DivideU32

Instruction for DivideU32

**Opcode**: `0x0106`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DivideU64

Instruction for DivideU64

**Opcode**: `0x0107`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DivideI8

Instruction for DivideI8

**Opcode**: `0x0108`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DivideI16

Instruction for DivideI16

**Opcode**: `0x0109`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DivideI32

Instruction for DivideI32

**Opcode**: `0x010A`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DivideI64

Instruction for DivideI64

**Opcode**: `0x010B`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DivideF32

Instruction for DivideF32

**Opcode**: `0x010C`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DivideF64

Instruction for DivideF64

**Opcode**: `0x010D`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ModuloU8

Instruction for ModuloU8

**Opcode**: `0x0122`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ModuloU16

Instruction for ModuloU16

**Opcode**: `0x0123`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ModuloU32

Instruction for ModuloU32

**Opcode**: `0x0124`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ModuloU64

Instruction for ModuloU64

**Opcode**: `0x0125`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ModuloI8

Instruction for ModuloI8

**Opcode**: `0x0126`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ModuloI16

Instruction for ModuloI16

**Opcode**: `0x0127`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ModuloI32

Instruction for ModuloI32

**Opcode**: `0x0128`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ModuloI64

Instruction for ModuloI64

**Opcode**: `0x0129`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ModuloF32

Instruction for ModuloF32

**Opcode**: `0x012A`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ModuloF64

Instruction for ModuloF64

**Opcode**: `0x012B`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## EqualU8

Instruction for EqualU8

**Opcode**: `0x015E`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## EqualU16

Instruction for EqualU16

**Opcode**: `0x015F`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## EqualU32

Instruction for EqualU32

**Opcode**: `0x0160`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## EqualU64

Instruction for EqualU64

**Opcode**: `0x0161`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## EqualI8

Instruction for EqualI8

**Opcode**: `0x0162`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## EqualI16

Instruction for EqualI16

**Opcode**: `0x0163`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## EqualI32

Instruction for EqualI32

**Opcode**: `0x0164`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## EqualI64

Instruction for EqualI64

**Opcode**: `0x0165`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## EqualF32

Instruction for EqualF32

**Opcode**: `0x0166`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## EqualF64

Instruction for EqualF64

**Opcode**: `0x0167`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## NotEqualU8

Instruction for NotEqualU8

**Opcode**: `0x017C`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## NotEqualU16

Instruction for NotEqualU16

**Opcode**: `0x017D`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## NotEqualU32

Instruction for NotEqualU32

**Opcode**: `0x017E`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## NotEqualU64

Instruction for NotEqualU64

**Opcode**: `0x017F`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## NotEqualI8

Instruction for NotEqualI8

**Opcode**: `0x0180`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## NotEqualI16

Instruction for NotEqualI16

**Opcode**: `0x0181`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## NotEqualI32

Instruction for NotEqualI32

**Opcode**: `0x0182`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## NotEqualI64

Instruction for NotEqualI64

**Opcode**: `0x0183`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## NotEqualF32

Instruction for NotEqualF32

**Opcode**: `0x0184`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## NotEqualF64

Instruction for NotEqualF64

**Opcode**: `0x0185`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanU8

Instruction for LessThanU8

**Opcode**: `0x019A`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanU16

Instruction for LessThanU16

**Opcode**: `0x019B`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanU32

Instruction for LessThanU32

**Opcode**: `0x019C`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanU64

Instruction for LessThanU64

**Opcode**: `0x019D`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanI8

Instruction for LessThanI8

**Opcode**: `0x019E`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanI16

Instruction for LessThanI16

**Opcode**: `0x019F`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanI32

Instruction for LessThanI32

**Opcode**: `0x01A0`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanI64

Instruction for LessThanI64

**Opcode**: `0x01A1`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanF32

Instruction for LessThanF32

**Opcode**: `0x01A2`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanF64

Instruction for LessThanF64

**Opcode**: `0x01A3`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanOrEqualU8

Instruction for LessThanOrEqualU8

**Opcode**: `0x01B8`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanOrEqualU16

Instruction for LessThanOrEqualU16

**Opcode**: `0x01B9`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanOrEqualU32

Instruction for LessThanOrEqualU32

**Opcode**: `0x01BA`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanOrEqualU64

Instruction for LessThanOrEqualU64

**Opcode**: `0x01BB`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanOrEqualI8

Instruction for LessThanOrEqualI8

**Opcode**: `0x01BC`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanOrEqualI16

Instruction for LessThanOrEqualI16

**Opcode**: `0x01BD`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanOrEqualI32

Instruction for LessThanOrEqualI32

**Opcode**: `0x01BE`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanOrEqualI64

Instruction for LessThanOrEqualI64

**Opcode**: `0x01BF`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanOrEqualF32

Instruction for LessThanOrEqualF32

**Opcode**: `0x01C0`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## LessThanOrEqualF64

Instruction for LessThanOrEqualF64

**Opcode**: `0x01C1`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanU8

Instruction for GreaterThanU8

**Opcode**: `0x01D6`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanU16

Instruction for GreaterThanU16

**Opcode**: `0x01D7`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanU32

Instruction for GreaterThanU32

**Opcode**: `0x01D8`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanU64

Instruction for GreaterThanU64

**Opcode**: `0x01D9`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanI8

Instruction for GreaterThanI8

**Opcode**: `0x01DA`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanI16

Instruction for GreaterThanI16

**Opcode**: `0x01DB`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanI32

Instruction for GreaterThanI32

**Opcode**: `0x01DC`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanI64

Instruction for GreaterThanI64

**Opcode**: `0x01DD`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanF32

Instruction for GreaterThanF32

**Opcode**: `0x01DE`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanF64

Instruction for GreaterThanF64

**Opcode**: `0x01DF`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanOrEqualU8

Instruction for GreaterThanOrEqualU8

**Opcode**: `0x01F4`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanOrEqualU16

Instruction for GreaterThanOrEqualU16

**Opcode**: `0x01F5`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanOrEqualU32

Instruction for GreaterThanOrEqualU32

**Opcode**: `0x01F6`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanOrEqualU64

Instruction for GreaterThanOrEqualU64

**Opcode**: `0x01F7`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanOrEqualI8

Instruction for GreaterThanOrEqualI8

**Opcode**: `0x01F8`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanOrEqualI16

Instruction for GreaterThanOrEqualI16

**Opcode**: `0x01F9`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanOrEqualI32

Instruction for GreaterThanOrEqualI32

**Opcode**: `0x01FA`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanOrEqualI64

Instruction for GreaterThanOrEqualI64

**Opcode**: `0x01FB`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanOrEqualF32

Instruction for GreaterThanOrEqualF32

**Opcode**: `0x01FC`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## GreaterThanOrEqualF64

Instruction for GreaterThanOrEqualF64

**Opcode**: `0x01FD`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## CallFunction

Instruction for CallFunction

**Opcode**: `0x0258`

### Instruction Details

### Arguments

- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## Return

Instruction for Return

**Opcode**: `0x0259`

### Instruction Details

This instruction is not commutative.

## Allocate

Instruction for Allocate

**Opcode**: `0x025A`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## Deallocate

Instruction for Deallocate

**Opcode**: `0x025B`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## Memcpy

Instruction for Memcpy

**Opcode**: `0x025C`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MemSet

Instruction for MemSet

**Opcode**: `0x025D`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## Halt

Instruction for Halt

**Opcode**: `0x025E`

### Instruction Details

This instruction is not commutative.

## MoveU8

Instruction for MoveU8

**Opcode**: `0x0335`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MoveU16

Instruction for MoveU16

**Opcode**: `0x0336`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MoveU32

Instruction for MoveU32

**Opcode**: `0x0337`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MoveU64

Instruction for MoveU64

**Opcode**: `0x0338`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MoveI8

Instruction for MoveI8

**Opcode**: `0x0339`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MoveI16

Instruction for MoveI16

**Opcode**: `0x033A`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MoveI32

Instruction for MoveI32

**Opcode**: `0x033B`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MoveI64

Instruction for MoveI64

**Opcode**: `0x033C`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MoveF32

Instruction for MoveF32

**Opcode**: `0x033D`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## MoveF64

Instruction for MoveF64

**Opcode**: `0x033E`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## IncrementU8

Instruction for IncrementU8

**Opcode**: `0x0349`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u8**: Argument of type u8 (Type: `U8`)

This instruction is not commutative.

## IncrementU16

Instruction for IncrementU16

**Opcode**: `0x034A`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u16**: Argument of type u16 (Type: `U16`)

This instruction is not commutative.

## IncrementU32

Instruction for IncrementU32

**Opcode**: `0x034B`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u32**: Argument of type u32 (Type: `U32`)

This instruction is not commutative.

## IncrementU64

Instruction for IncrementU64

**Opcode**: `0x034C`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## IncrementI8

Instruction for IncrementI8

**Opcode**: `0x034D`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **i8**: Argument of type i8 (Type: `I8`)

This instruction is not commutative.

## IncrementI16

Instruction for IncrementI16

**Opcode**: `0x034E`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **i16**: Argument of type i16 (Type: `I16`)

This instruction is not commutative.

## IncrementI32

Instruction for IncrementI32

**Opcode**: `0x034F`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **i32**: Argument of type i32 (Type: `I32`)

This instruction is not commutative.

## IncrementI64

Instruction for IncrementI64

**Opcode**: `0x0350`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **i64**: Argument of type i64 (Type: `I64`)

This instruction is not commutative.

## IncrementF32

Instruction for IncrementF32

**Opcode**: `0x0351`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **f32**: Argument of type f32 (Type: `F32`)

This instruction is not commutative.

## IncrementF64

Instruction for IncrementF64

**Opcode**: `0x0352`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **f64**: Argument of type f64 (Type: `F64`)

This instruction is not commutative.

## DecrementU8

Instruction for DecrementU8

**Opcode**: `0x035D`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u8**: Argument of type u8 (Type: `U8`)

This instruction is not commutative.

## DecrementU16

Instruction for DecrementU16

**Opcode**: `0x035E`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u16**: Argument of type u16 (Type: `U16`)

This instruction is not commutative.

## DecrementU32

Instruction for DecrementU32

**Opcode**: `0x035F`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u32**: Argument of type u32 (Type: `U32`)

This instruction is not commutative.

## DecrementU64

Instruction for DecrementU64

**Opcode**: `0x0360`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## DecrementI8

Instruction for DecrementI8

**Opcode**: `0x0361`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **i8**: Argument of type i8 (Type: `I8`)

This instruction is not commutative.

## DecrementI16

Instruction for DecrementI16

**Opcode**: `0x0362`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **i16**: Argument of type i16 (Type: `I16`)

This instruction is not commutative.

## DecrementI32

Instruction for DecrementI32

**Opcode**: `0x0363`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **i32**: Argument of type i32 (Type: `I32`)

This instruction is not commutative.

## DecrementI64

Instruction for DecrementI64

**Opcode**: `0x0364`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **i64**: Argument of type i64 (Type: `I64`)

This instruction is not commutative.

## DecrementF32

Instruction for DecrementF32

**Opcode**: `0x0365`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **f32**: Argument of type f32 (Type: `F32`)

This instruction is not commutative.

## DecrementF64

Instruction for DecrementF64

**Opcode**: `0x0366`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **f64**: Argument of type f64 (Type: `F64`)

This instruction is not commutative.

## BitwiseAndU8

Instruction for BitwiseAndU8

**Opcode**: `0x0384`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseAndU16

Instruction for BitwiseAndU16

**Opcode**: `0x0385`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseAndU32

Instruction for BitwiseAndU32

**Opcode**: `0x0386`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseAndU64

Instruction for BitwiseAndU64

**Opcode**: `0x0387`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseAndI8

Instruction for BitwiseAndI8

**Opcode**: `0x0388`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseAndI16

Instruction for BitwiseAndI16

**Opcode**: `0x0389`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseAndI32

Instruction for BitwiseAndI32

**Opcode**: `0x038A`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseAndI64

Instruction for BitwiseAndI64

**Opcode**: `0x038B`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseOrU8

Instruction for BitwiseOrU8

**Opcode**: `0x038E`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseOrU16

Instruction for BitwiseOrU16

**Opcode**: `0x038F`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseOrU32

Instruction for BitwiseOrU32

**Opcode**: `0x0390`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseOrU64

Instruction for BitwiseOrU64

**Opcode**: `0x0391`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseOrI8

Instruction for BitwiseOrI8

**Opcode**: `0x0392`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseOrI16

Instruction for BitwiseOrI16

**Opcode**: `0x0393`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseOrI32

Instruction for BitwiseOrI32

**Opcode**: `0x0394`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseOrI64

Instruction for BitwiseOrI64

**Opcode**: `0x0395`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseXorU8

Instruction for BitwiseXorU8

**Opcode**: `0x0398`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseXorU16

Instruction for BitwiseXorU16

**Opcode**: `0x0399`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseXorU32

Instruction for BitwiseXorU32

**Opcode**: `0x039A`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseXorU64

Instruction for BitwiseXorU64

**Opcode**: `0x039B`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseXorI8

Instruction for BitwiseXorI8

**Opcode**: `0x039C`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseXorI16

Instruction for BitwiseXorI16

**Opcode**: `0x039D`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseXorI32

Instruction for BitwiseXorI32

**Opcode**: `0x039E`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseXorI64

Instruction for BitwiseXorI64

**Opcode**: `0x039F`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseNotU8

Instruction for BitwiseNotU8

**Opcode**: `0x03A2`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseNotU16

Instruction for BitwiseNotU16

**Opcode**: `0x03A3`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseNotU32

Instruction for BitwiseNotU32

**Opcode**: `0x03A4`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseNotU64

Instruction for BitwiseNotU64

**Opcode**: `0x03A5`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseNotI8

Instruction for BitwiseNotI8

**Opcode**: `0x03A6`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseNotI16

Instruction for BitwiseNotI16

**Opcode**: `0x03A7`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseNotI32

Instruction for BitwiseNotI32

**Opcode**: `0x03A8`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## BitwiseNotI64

Instruction for BitwiseNotI64

**Opcode**: `0x03A9`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftLeftU8

Instruction for ShiftLeftU8

**Opcode**: `0x03AC`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftLeftU16

Instruction for ShiftLeftU16

**Opcode**: `0x03AD`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftLeftU32

Instruction for ShiftLeftU32

**Opcode**: `0x03AE`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftLeftU64

Instruction for ShiftLeftU64

**Opcode**: `0x03AF`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftLeftI8

Instruction for ShiftLeftI8

**Opcode**: `0x03B0`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftLeftI16

Instruction for ShiftLeftI16

**Opcode**: `0x03B1`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftLeftI32

Instruction for ShiftLeftI32

**Opcode**: `0x03B2`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftLeftI64

Instruction for ShiftLeftI64

**Opcode**: `0x03B3`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftRightU8

Instruction for ShiftRightU8

**Opcode**: `0x03B6`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftRightU16

Instruction for ShiftRightU16

**Opcode**: `0x03B7`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftRightU32

Instruction for ShiftRightU32

**Opcode**: `0x03B8`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftRightU64

Instruction for ShiftRightU64

**Opcode**: `0x03B9`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftRightI8

Instruction for ShiftRightI8

**Opcode**: `0x03BA`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftRightI16

Instruction for ShiftRightI16

**Opcode**: `0x03BB`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftRightI32

Instruction for ShiftRightI32

**Opcode**: `0x03BC`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## ShiftRightI64

Instruction for ShiftRightI64

**Opcode**: `0x03BD`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## Print

Instruction for Print

**Opcode**: `0x03E8`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## StoreConstantArray

Instruction for StoreConstantArray

**Opcode**: `0x0406`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)
- **u64**: Argument of type u64 (Type: `U64`)

This instruction is not commutative.

## DebugPrintU8

Instruction for DebugPrintU8

**Opcode**: `0x07D0`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DebugPrintU16

Instruction for DebugPrintU16

**Opcode**: `0x07D1`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DebugPrintU32

Instruction for DebugPrintU32

**Opcode**: `0x07D2`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DebugPrintU64

Instruction for DebugPrintU64

**Opcode**: `0x07D3`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DebugPrintI8

Instruction for DebugPrintI8

**Opcode**: `0x4E24`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DebugPrintI16

Instruction for DebugPrintI16

**Opcode**: `0x07D5`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DebugPrintI32

Instruction for DebugPrintI32

**Opcode**: `0x07D6`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DebugPrintI64

Instruction for DebugPrintI64

**Opcode**: `0x07D7`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DebugPrintF32

Instruction for DebugPrintF32

**Opcode**: `0x07D8`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DebugPrintF64

Instruction for DebugPrintF64

**Opcode**: `0x07D9`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

## DebugPrintRaw

Instruction for DebugPrintRaw

**Opcode**: `0x07DA`

### Instruction Details

### Arguments

- **RegisterType**: Argument of type RegisterType (Type: `Register`)

This instruction is not commutative.

