# 🐍 **Solid Snake Language Spec Draft (Focused on Syntax)**

## 🛠 **General Principles**

- **Indentation-based** (no `{}`, no `;`).
- **Colon `:`** to open blocks (just like Python).
- **Whitespace-sensitive**.
- **Strongly typed** (types known at compile time, even with inference).
- **Explicit is better than implicit**, unless it’s *obviously safe*.

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

