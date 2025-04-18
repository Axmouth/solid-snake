## 🐍 Solid Snake VM

A **low-level bytecode virtual machine** with strong abstraction, designed to support future development of a **Python-like language** with **Rust-inspired guarantees**.

---

### 📌 Project Goals

- ✅ **Current Phase:**  
  Building a clean, extensible **low-level bytecode VM**, with support for:
  - Basic arithmetic
  - Memory and register manipulation
  - Function calls and a sliding stack
  - Heap allocation and memory access

- 🔭 **Longer-Term Vision:**  
  A high-level, Python-like language that:
  - Compiles to this VM's bytecode
  - Supports strong typing and safety constraints
  - Is ideal for scripting with a low-level backend

---

### 🧱 Design Philosophy

- **Low-level but ergonomic:**  
  Bytecode is explicit, but VM design uses macros and abstractions to minimize boilerplate and encourage reuse.

- **Macro-driven instruction system:**  
  Instructions are defined using declarative Rust macros that auto-generate:
  - Opcode bindings
  - Argument parsing
  - Byte encoding/decoding
  - Handler dispatch
  
- **Typed Register Interface:**  
  All values are stored as raw `u64`, but safely interpreted via trait-based access per instruction.

- **Sliding Stack Model:**  
  On each `CallFunction`:
  - The **stack top** is incremented
  - Registers `R1`–`R2` (function arguments) are copied to the new frame
  - Return value `R0` is copied back on `Return`
  - `R3` is **reserved** for future scratch/metadata use and not preserved

- **Memory Model:**
  - Heap allocation uses index-based "pointers" (section IDs)
  - All memory accesses are **bounds-checked**
  - Memory instructions support:
    - Load/store via immediate address
    - Indirect with offset
    - Section de/allocation
    - Memory copy / fill operations (`Memcpy`, `MemSet`)

- **Encodable Instructions:**
  - All instructions implement `InstructionArgs`, which supports `encode()`/`parse_args()` methods
  - Macro-based assembly (e.g. `asm!(AddI64, R1, R2, R3)`) makes testing and generation ergonomic

---

### ✨ Highlights

- Strong separation between instruction *definition*, *encoding*, and *execution*
- Extensible opcode system — new instructions require minimal boilerplate
- Dynamic instruction encoding/decoding for testing and tooling
- Bytecode-level debugging logs via `log` crate
- Safe Rust with room for future performance optimizations

---

### 🧪 Planned / In Progress

- [ ] Arithmetic (int + float)
- [ ] Memory alloc/store/load/copy
- [ ] Function call and return mechanics
- [ ] Branching and loops
- [ ] Error handling (overflow, invalid op, etc.)
- [ ] Macro test DSL
- [ ] Label support and assembler for symbolic jumps
- [ ] Language frontend!

---

### 📎 Example (Assembly DSL)

```rust
    let bytecode = asm! {
        LoadImmediateI64, R1, (-123);
        LoadImmediateI64, R2, 456;
        AddI64, R3, R1, R2;
        DebugPrintI64, R3;
        Halt;
    };

    vm.execute_bytecode(&bytecode).unwrap();
```

```
Load: Directly i64 R1 <= -123
Load: Directly i64 R2 <= 456
Add: R3 <= i64 R1 (-123) + R2 (456)
Debug: R3
R3 : 333
```


### 🧠 Future Considerations

- SSA-like register flow or SSA hints?
- Symbolic call/return stack traces
- Direct memory aliasing?
- Calling convention formalization (`R0` = return, `R1`–`Rn` = args, scratch vs preserved?)
