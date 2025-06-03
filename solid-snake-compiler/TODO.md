## 🧠 Recap

Built:
- A Pest grammar with indentation support
- A Rust recursive-descent AST builder
- Span-tracked AST nodes
- Multiple error handling and recovery strategies
- A preprocessor for indentation with offset tracking
- Ariadne-powered diagnostics

---

# ⌛ Work in progress:

### 🧱 Language Features
- [x] `else` and `elif` support for `if`
- [x] `while`
- [ ] `for`
- [ ] `fn` definitions
- [ ] function calls
- [ ] indexing

### 🧠 Semantics
- [x] Type tracking (per expression, not just AST)
- [x] Symbol table or scope-aware analysis
- [x] Early type inference
- [ ] Ref types are ref counted
- [ ] weak keyword to denote weak refs and to create them too
- [ ] concept of owned types, more performant alternative to the ref counted types, more strict linting using move semantics(no ref counting, cycles are error)

### 🧰 Infrastructure
- [ ] Pretty-printer / formatter
- [ ] Macro-based AST walkers (to simplify passes)
- [x] IR generation or bytecode emission
- [ ] IR generation or bytecode emission gen 2
- [ ] Nested type flattening

scratch:
heap structure enum => type | pointer | object<String, Self> | bytes
auto gen ir/bytecode rucursively. cache offsets per field. offset getter => static/indexed undearneath (enum? closure?)

scopes, variable, function registry during parsing ast?