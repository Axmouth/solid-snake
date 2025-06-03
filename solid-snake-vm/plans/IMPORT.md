
---

# 📦 **Imports (and Reload Support)**

### 🔧 **Purpose**

Enable modules to reference and invoke symbols (functions, data) from other modules in a **dynamic**, **link-time-resolved**, and **hot-reloadable** way.

---

### ✅ **Core Concepts**

#### 🔹 `ImportEntry`

Describes a required symbol by name and kind:

```rust
struct ImportEntry {
    name: String, // or SymbolRef { module, name }
    kind: SymbolKind, // Function, Data, etc.
}
```

#### 🔹 `ExportEntry`

Describes a provided symbol:

```rust
struct ExportEntry {
    name: String,
    kind: SymbolKind,
    offset: usize, // instruction offset in the code section
}
```

#### 🔹 Import Usage in Bytecode

* Bytecode uses `CallImport(index)` or similar instruction.
* Compiler maps symbolic names to import table indices during compilation.

---

### 🔄 **Linking and Resolution**

1. During module loading:

   * Imports are matched against available exports from other modules.
   * Each resolved symbol is converted into a closure (or address).
2. `CallImport(idx)` is replaced with an executable closure that performs the call.

---

### ♻️ **Hot Reload Support**

**Goals**: update live references to symbols from reloaded modules.

#### 🔸 Requirements:

* Each module keeps a **jump table** for import calls.
* Resolved imports are stored per-module and used by `CallImport` closures.
* A **dependency graph** tracks which module depends on which.

#### 🔸 On Reload:

1. Recompile/reload the target module.
2. Update all jump table entries in dependent modules.
3. Optional: Invalidate or reprocess pre-fused instructions if affected.

---

### ✅ **Design Advantages**

* No string matching or dynamic lookups at runtime.
* Symbols resolved once at load time.
* Hot reloadable with controlled entry point updates.
* Simple and language-agnostic.

---