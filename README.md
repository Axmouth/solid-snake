## 🐍 Solid Snake VM

A **low-level bytecode virtual machine** with strong abstraction, designed to support future development of a **Python-like language** with **Rust-inspired guarantees**.

> _Trademarked by Konami? Perhaps. But we’re not building a stealth action game — just a hardened VM that doesn’t mess around._

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
  - Embraces Python-style ergonomics with Rust-like semantics
  - Can serve as a robust scripting language or backend DSL

---

### 🧱 Design Philosophy

- **Low-level but ergonomic:**  
  Bytecode is explicit, but the VM is built with Rust macros and abstractions to reduce boilerplate and increase clarity.

- **Macro-driven instruction system:**  
  Instructions are defined via declarative Rust macros that auto-generate:
  - Opcode bindings
  - Argument parsing
  - Byte encoding/decoding
  - Execution dispatch

- **Register Semantics:**  
  All values are stored as raw `u64`, interpreted per instruction. No global typing—each opcode defines how to decode inputs.

- **Sliding Stack Model:**  
  - On `CallFunction`, the stack pointer is incremented
  - Arguments in `R1`–`R2` are copied to the new frame
  - Return value is placed in `R0`
  - `R3` is reserved for future or scratch use, not preserved between calls

- **Memory Model:**
  - Heap memory uses segment-indexed allocations
  - All accesses are **bounds-checked**
  - Instructions support:
    - Direct and indirect load/store
    - Memory section creation/destruction
    - Block operations (`Memcpy`, `MemSet`)
  - Memory is **untyped** at runtime; type interpretation is deferred to compilers and high-level tools

---

### ✨ Highlights

- ✅ Modular instruction definition with low boilerplate
- ✅ Fully dynamic instruction encoding/decoding
- ✅ Strong separation between definition, binary format, and execution
- ✅ Log-driven bytecode debugging (via `log` crate)
- ✅ Memory-safe execution model
- ✅ Safe Rust, with deliberate `unsafe` boundaries only where necessary (FFI)

---

### 🧪 Development Status

- [x] Arithmetic (int + float)
- [x] Memory alloc/store/load/copy
- [x] Function call and return
- [x] Branching and loops
- [x] Error handling (overflow, invalid op, etc.)
- [ ] Documentation per instruction, flags (HeapAccess, Pure, Commutative, etc.), shorter notation support, generate InstructionDoc per instruction, write generator for .md, .json, or both cargo doc maybe
- [ ] Ability to fork, join threads
- [ ] Shared memory pool and related instructions for syncronization and data passing
- [ ] Builtin functions
- [ ] File format (header, data, etc)
- [ ] Fused instructions
- [ ] Importing code files
- [ ] Importing bytecode (or do we even?)
- [ ] Native extensions
- [ ] Symbolic assembler with label resolution

---

### 🔌 Language-Agnostic Compilation Target

While originally designed for a custom language, Solid Snake VM is **language-agnostic** and structured to support external compilers.

Any language that:
- Supports static typing
- Has analyzable control flow
- Manages memory through segment-based allocation

...can target this VM directly, including potential Rust backend support or embedded DSLs.

---

### 🔐 Safety Considerations (Planned)

- ✅ All memory is bounds-checked
- 🧪 Optional segment isolation for threads (in progress)
- 🧠 Atomics-only shared memory segment for message passing
- 🧪 ARC-style handle system for controlled shared memory access
- ⚙️ Future borrow-checker-inspired ownership hints
- 💡 Optional read-only memory views for safe sharing

---

### 🧠 Future Considerations

- SSA-style register tracking
- Debug-friendly symbolic stack traces
- Memory aliasing support
- Formal calling convention (`R0` = return, `R1`–`Rn` = args, reserved scratch registers)
- Declarative FFI system with auto-generated bindings
- Extension-safe APIs with type registries

---

### 🧪 Example Bytecode Execution (TBD)

Sample test cases and full bytecode listings to come.  
For now, expect something like:

```rust
let bytecode = vec![
    Instruction::LoadImmediateI64((RegisterType(1), -123)),
    Instruction::LoadImmediateI64((RegisterType(2), 456)),
    Instruction::AddI64((RegisterType(3), RegisterType(1), RegisterType(2))),
    Instruction::DebugPrintI64((RegisterType(3))),
    Instruction::Halt,
];

execute_bytecode(&bytecode).unwrap();
````

Expected output:

```
Load: R1 <= -123
Load: R2 <= 456
Add: R3 = R1 + R2 => 333
Debug: R3 = 333
```

---

### 🤝 Contributing

Currently in early development. Core structure and instruction system stable.
Contributors welcome, especially around:

* Instruction set design
* Testing macros and debug tooling
* Language frontend and parser
* Extensions or FFI ideas

---

### 🏁 Final Thoughts

Solid Snake VM is a platform for experimentation with **strong guarantees**, **minimalist design**, and **future-facing features**. It's an evolving foundation for a new kind of scripting runtime—**Python-flavored**, but **battle-hardened**.

---

> 🔧 “A VM should be small, sharp, and clear—like a knife. Not a suitcase full of opinions.”




