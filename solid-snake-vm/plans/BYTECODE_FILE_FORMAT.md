
# 📄 **Bytecode File Format**

### 📦 **Structure**

```rust
struct BytecodeModule {
    code: Vec<u8>, // raw bytecode
    instruction_map: Vec<ExecutableInstruction>, // built at load
    imports: Vec<ImportEntry>,
    exports: Vec<ExportEntry>,
    metadata: Option<ModuleMetadata>,
}
```

---

### 🔍 **Key Components**

#### 🔸 Code Section

* Raw instructions, encoded as bytes.
* Parsed into `ExecutableInstruction`s during loading.

#### 🔸 Import/Export Tables

* Self-describing symbol interfaces.
* Enables linking and introspection.

#### 🔸 Instruction Map

* Generated during preprocessing.
* Includes decoded, pre-executed closures.

#### 🔸 Metadata

Optional:

* Source map (for debugger)
* Bytecode version
* Build info

---

### 🔄 **Linking and Relocation**

* Imports resolved at load time.
* No runtime relocation unless hot-reloading.
* Optional relocation table for address-based patching if needed.

---

### ✅ **Design Principles**

* Modular and self-contained
* No runtime parsing required
* Format extensible (e.g., for debug info)
* Supports cold and hot loading

---
