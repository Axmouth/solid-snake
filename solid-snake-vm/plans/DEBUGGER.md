
# 🐞 **Debugger Support**

### 🎯 **Goal**

Enable:

* Breakpoints
* Stepping
* Call stack inspection
* Source-level tracing

---

### ✅ **Required VM Features**

#### 🔹 Instruction Pointer Tracking

```rust
struct VM {
    program_counter: usize, // Current instruction index
}
```

#### 🔹 Call Stack

```rust
struct CallFrame {
    return_ip: usize,
    locals: ...
}
```

#### 🔹 Debug Info Mapping

```rust
struct DebugInfo {
    instr_to_source: HashMap<usize, SourceLocation>,
    source_to_instr: HashMap<SourceLocation, usize>,
}
```

---

### ⛔ **Not Required Yet**

* No need for DWARF, PDB, or heavyweight debugging formats.
* You can defer stepping and breakpoint support.

---

### 🔧 **Hooks to Leave Room For**

* `before_execute(ip)`
* `after_execute(ip)`
* `on_call`, `on_return`
* Optional `VmDebugger` interface for pluggable backends

---

### 🧠 Design Consideration

* Keep `instruction` field in `ExecutableInstruction` for symbolic tracing.
* Use `program_counter` and `DebugInfo` to enable future REPL/debug UIs or trace logs.

---
