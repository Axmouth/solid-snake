# 🐍 **Solid Snake Language Spec Draft (Focused on Syntax)**

### 🛠 **General Principles**

* **Indentation-based**: No `{}`, no `;`. Block structure is defined by whitespace, like Python and Haskell.
* **Colon `:` introduces blocks**: Aligns with Python’s clean syntax model.
* **Whitespace-sensitive**: Indentation must be consistent; promotes visual clarity and structural discipline.
* **Strongly typed**: All types are known at compile time — even with inference. No implicit type coercion.
* **Explicit over implicit**, unless **the behavior is provably unambiguous and safe**.
* **No unexpected behavior**: The compiler should never do things behind the developer’s back. All behavior must be either declared or inferable with no side effects.
* **Fail early and clearly**: Ambiguity, conflict, or unsafe behavior results in a compile-time error with helpful messaging.
* **Readability first**: Syntax should reduce noise, but never at the cost of clarity or correctness.
* **Zero-cost abstractions where possible**: High-level features should compile to minimal, predictable runtime code.
* **Sensible defaults, override when needed**: Prefer defaults that are safe and ergonomic, but make overriding explicit and easy.

---

## ➕ **Recommended Additions**

### 📚 **Type System & Behavior**

* **Structural types by default**, **nominal types opt-in**:
  Structural typing allows ergonomic data modeling; nominal typing enforces semantic boundaries where needed.

* **Behavior (interfaces/traits) must be declared and implemented explicitly**:
  No automatic implementation matching by shape for nominal types.

* **Interfaces describe behavior, not data**:
  Keeps interface semantics clear and distinct from type shape.

* **Method resolution is explicit**:
  No method is magically in scope — if it comes from an interface, it must be visible via implementation.

---

### 💬 **Error Handling & Semantics**

* **No null**: Prefer sum types (`Option`, `Result`) for absence and errors.
* **No exceptions**: Use return-based error types for control flow, following Rust’s lead.
* **Match exhaustiveness required**: All enum matches must be exhaustive unless explicitly wildcarded (`_`).
* **Pattern matching is a first-class citizen**: Encouraged for handling enums, destructuring, and data transformation.
* **Function calls are predictable**: No overloads, no default arguments unless clearly defined.

---

### 🔍 **Clarity in Data and Code**

* **Data shapes and types are transparent**: Compiler errors and documentation should always reveal full type shape where relevant.
* **No hidden control flow**: No implicit async, no magic callbacks — concurrency and effects must be visible.
* **Imports are explicit**: No magic prelude or invisible globals.
  
 ⚠️ **Design Exception — Safe Concurrency**
 The only area where the language may perform work without syntactic indication is **concurrency** — but only when all the following apply:

 * The result is provably deterministic
 * The computation is isolated from shared state
 * Execution order does not affect program meaning

 In these cases, the compiler may run tasks in parallel — not as magic, but as a **verified optimization** that does not alter semantics.

“You don’t *need* to ask for concurrency, unless the compiler can’t guarantee it for you.”

---

### 🧪 **Tooling & Safety**

* **Linter required in standard toolchain**: Enforce formatting, naming, and hygiene rules out of the box.
* **Fast feedback loops**: Compiler and tooling must emphasize short cycle times and clear diagnostics.
* **Support for static analysis and generics must be consistent**: Type inference should work predictably across both.

---

## 🧭 Optional “Aesthetic” Rules

These can guide design without being rigid:

* **Minimal syntax, maximal meaning**: Every construct must pull its weight.
* **Orthogonality over magic**: Few constructs, many combinations. Avoid one-off keywords or constructs.
* **Avoid cleverness**: Favor straightforward constructs that scale with complexity, not against it.

---

Where uncertain:  
👉 **"What would Python do, but stricter and stronger?"**

---

## 🔤 **Lexical Rules**

| Aspect | Rule |
|:---|:---|
| Identifiers | `[a-zA-Z_][a-zA-Z0-9_]*` |
| Keywords | Reserved, no overloading |
| Comments | `# line comment` |
| Literals | `'string'`, `"string"`, `123`, `3.14`, `true`, `false`, `null` |
| Operators | `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `and`, `or`, `not`, `=`, `+=`, `-=` etc. |
| Grouping | `()`, `[]`, `{}` only when needed (data structures, explicit tuples/maps) |

---

## 🗝 **Keywords (Reserved Words)**

| Category | Words |
|:---|:---|
| Control flow | `if`, `elif`, `else`, `for`, `while`, `break`, `continue`, `return`, `match`, `case`, `import`, `fn` |
| Error handling | `?` operator |
| Parallelism | `parallel` (modifier keyword) |
| Ownership | `mut` (mutable variable hint) |
| Boolean literals | `true`, `false` |
| Typing | `as` (for casting) |
| Other | `pass`, `yield` (maybe for generators later?) |

---
  
## 📚 **Built-in Types (or standard lib)**

| Kind | Types |
|:---|:---|
| Primitives | `Int`, `Float`, `Bool`, `String`, `Char` |
| Collections | `List[T]`, `Tuple[T1, T2, ...]`, `HashMap[K, V]`, `Set[T]` |
| Pointers / Ownership | `Rc[T]`, `WeakRef[T]`, `Arc[T]` |
| Result Types | `Result[T, E]` |
| Option Types | `Option[T]` (optional types) |
| Unit Type | `()` (void / no meaningful value) |

---

## 🏛 **Top Level Constructs**

| Construct | Syntax Example | Notes |
|:---|:---|:---|
| Function definition | `fn name(args) -> ReturnType:` | Always need type signature (even if inferred) |
| Import | `import module_name` | Simple, Pythonic |
| Class/Interface | TBD ("interface" for now, traits later) | |
| Constant | `const NAME = value` | |
| Enum | `rust enum vector { v0, v1 { x: i32 }, v2 { x: i32, y: i32 }, v3 { x: i32, y: i32, z i32 } }` | Rich enums similar to rust? |

---

## 📄 **Expressions**

| Expression | Syntax | Notes |
|:---|:---|:---|
| Function call | `name(arg1, arg2)` | |
| Lambda | `(args):` indented block | No `=> {}`, pure Python-style |
| Method call | `obj.method()` | Standard |
| Attribute access | `obj.field` | |
| List / tuple / hashmap literals | `[1, 2, 3]`, `(1, 2)`, `{key: value}` | |
| Casting | `value as Int` | Strong typing enforced |

---

## 🔂 **Control Flow Syntax**

### If / Else
```python
if condition:
    ...
elif condition2:
    ...
else:
    ...
```

### For Loops
```python
for item in iterable:
    ...

for i, item in enumerate(iterable):
    ...
```
- `parallel` modifier allowed later:
  ```python
  for parallel i, item in iterable:
      ...
  ```

### While Loops
```python
while condition:
    ...
```

### Match / Case
```python
match value:
    case Pattern:
        ...
    case OtherPattern:
        ...
```
(**Stick with `case` for now for clarity.**)

---

## ⚡ **Operators**

| Type | Operators |
|:---|:---|
| Arithmetic | `+`, `-`, `*`, `/`, `%` |
| Comparison | `==`, `!=`, `<`, `>`, `<=`, `>=` |
| Logical | `and`, `or`, `not` |
| Assignment | `=`, `+=`, `-=`, `*=`, `/=`, `%=` |
| Membership | `in`, `not in` |

---

## 💾 **Variable Binding and Mutability**

```python
x = 5         # Immutable by default
mut x = 5     # Mutable if explicitly declared
```

✅ **Immutability by default** — prevents subtle bugs, consistent with modern language design (Rust, Swift).

---

## ❓ **Error Propagation**

**Use `?` operator** to propagate errors:

```python
response = network.get(url)?
text = response.read_to_string()?
```

Equivalent to:
```python
match network.get(url):
    case Ok(response):
        ...
    case Err(e):
        return Err(e)
```

---

## 🧬 **Typesystem Notes**

- **Type inference heavily** (e.g., `x = 5` implies `x: Int`).
- **Generics** allowed for collection types: `List[Int]`, `HashMap[String, Int]`.
- **No lifetimes required** (RC model with automatic WeakRef for cycles).

---

# 📜 **Meta Summary**

| Principle | Solid Snake Style |
|:---|:---|
| Syntax visual feel | Very close to Python |
| Typing model | Close to Rust's static safety |
| Error model | Result-based, explicit handling |
| Parallelism | Implicit analysis, opt-in for unordered |
| Memory model | RC by default, WeakRef for cycles |

