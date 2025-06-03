

# ⚡ **Fused Instructions**

### 🔧 **Purpose**

Optimize sequences of instructions by **combining them into single closures** to reduce:

* Dispatch overhead
* Redundant register/memory traffic

---

### ✅ **How It Works**

#### 🔹 Representation

```rust
struct ExecutableInstruction {
    instruction: Instruction, // for symbolic/debugging/logging
    exec: Box<dyn FnMut(&mut VmState)>,
}
```

* `instruction` holds metadata (`Fused { id }` or equivalent)
* `exec` holds actual behavior, including fused logic

---

### 🔁 **Fusion Process**

1. **Detection Pass**:

   * Scan for patterns (e.g., `Load`, `Add`, `Store`)
   * Check for non-contiguous but interference-free sequences

2. **Transform Plan**:

   ```rust
   enum Transform {
       Replace { index, new_instr },
       Remove { index },
   }
   ```

3. **Application Pass**:

   * Apply all transformations in a single pass
   * Compact code if needed (remove `NoOp`s)

4. **Optional**: Maintain a debug ID (`id: u32`) in `instruction` for tracing

---

### 🔍 **Fusion Examples**

* `Load + Add + Store` → `exec = |vm| { ... }`
* `Cmp + BranchIf` → fused conditional jump
* Loop unrolling or tail-recursion fusion (future)

---

### 🧠 **Design Benefits**

* No need for new opcodes or enum variants
* Fully safe in Rust
* Fusion logic isolated in preprocessing; VM stays minimal
* Allows performance tuning without altering base instruction set

---
