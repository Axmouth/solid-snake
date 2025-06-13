
---

# 🧩 Embedding Blueprint for the VM

This blueprint ensures the VM can be used as a **scripting engine inside a host application** (e.g., game engine, CLI tool, simulation framework), with clear separation of roles and extensibility.

---

## 🧱 Core Embedding Architecture

| Component      | Responsibility                                           |
| -------------- | -------------------------------------------------------- |
| **VM**         | Executes bytecode, manages memory, registers, async ops  |
| **Host**       | Owns system APIs, I/O, engine features, state            |
| **Builtins**   | Bridge: VM → host calls (e.g., print, spawn, load asset) |
| **Extensions** | Optional FFI plugins with native functionality           |

---

## 🧠 Essential Embedding Interfaces

### ✅ `Vm::run()`

Main execution entry point. Should accept:

* Bytecode
* Optional preloaded memory segments
* Function to call (`main`, `on_event`, etc.)

### ✅ `Vm::call_function(name, args)`

Host → VM call. Used to:

* Trigger event handlers
* Execute callbacks

### ✅ `register_builtin(info: BuiltinInfo)`

Registers a builtin available to the VM.

```rust
pub struct BuiltinInfo {
    pub name: &'static str,
    pub doc: &'static str,
    pub args: &'static [(&'static str, &'static str)],
    pub arity: usize,
    pub handler: fn(&mut VmState, &[Value]) -> Value,
}
```

---

## 🧩 Builtin Extension Model

### 🔸 Macro-Driven Builtin Definitions

```rust
define_builtin! {
    Print {
        doc: "Prints to console",
        args: [("value", "Value to print")],
        handler: print_handler,
    }
}
```

* Generates enum `Builtin::Print`, handler, and doc block.
* You may allow `compiler:` for IR lowering if compiling.

### 🔸 Namespacing & Conflict Handling

* Recommended naming: `namespace::function` (e.g., `core::print`)
* Default: **fail on conflict**
* Optional: `override_builtin(name, new_handler)` if needed

---

## 🔌 Plugin Builtin Registration

Allow users to write:

```rust
pub fn register_plugin_builtins() {
    register_builtin(BuiltinInfo {
        name: "game::spawn_entity",
        ...
    });
}
```

Expose:

```rust
pub fn init_vm_with_plugins(vm: &mut Vm) {
    game_plugin::register_plugin_builtins();
    audio_plugin::register_plugin_builtins();
}
```

---

## 🧬 Memory Interop Model

| VM Side                          | Host Side                                |
| -------------------------------- | ---------------------------------------- |
| Segments with index & offset     | Viewed as "buffers"                      |
| Register values (ints, ptrs)     | Passed via `Value` type                  |
| Host allocates or fills segments | Before calling into VM or reading result |

### ✅ Recommended:

* Heap segment is primary data transfer method.
* VM owns memory; host only uses segments via public API.
* Use `segment_id + offset` like pointers.

---

## ⏳ Async Op Model (Host Integration)

| Feature                       | Description                           |
| ----------------------------- | ------------------------------------- |
| `op_id`                       | VM receives handle to async operation |
| `poll_op(op_id)`              | VM polls until complete               |
| `dispose_op(op_id)`           | Host cleans up if needed              |
| **VM controls when it polls** | Host only implements logic            |

Perfect fit for:

* Game loops
* Resource loading
* Network requests

---

## 📤 VM Exposure API (for Host)

* `Vm::load_script(name, bytecode)`
* `Vm::set_var(name, Value)`
* `Vm::get_var(name) -> Value`
* `Vm::register_builtin(...)`
* `Vm::poll_async_ops()`
* `Vm::call(name, args)`

Expose this via:

* C FFI
* Rust API
* Python/Ruby bindings via wrapper

---

## 🧠 Optional Enhancements

| Feature            | Value                                        |
| ------------------ | -------------------------------------------- |
| **Reflection API** | List builtins, args, docs at runtime         |
| **JSON export**    | Builtin registry and ABI as doc format       |
| **Debugger hooks** | Step, break, inspect stack                   |
| **Sandbox mode**   | Disable certain builtins (I/O, unsafe ops)   |
| **Script reload**  | Replace bytecode at runtime for live updates |

---

## ✅ TL;DR – Embedding Checklist

* [x] Exposed `run()`, `call()`, and memory access APIs
* [x] Builtin registration with safe conflict handling
* [x] Optional plugin system for builtins
* [x] VM-managed segmented memory model
* [x] Async ops with polling handle model
* [x] Metadata-rich builtin system for reflection and tooling
* [x] Clean interop layer for host app (Rust/C/etc.)

---

