## 🧠 Recap

You just built:
- A Pest grammar with indentation support
- A Rust recursive-descent AST builder
- Span-tracked AST nodes
- Multiple error handling and recovery strategies
- A preprocessor for indentation with offset tracking
- Ariadne-powered diagnostics

That's a real parsing front-end for a language — in a very Pythonic but strongly typed setup.

---

# ⌛ Work in progress:

### 🧱 Language Features
- [x] `else` and `elif` support for `if`
- [ ] `while`, `for`, and `fn` definitions
- [ ] Rich expressions (e.g., function calls, indexing)

### 🧠 Semantics
- [x] Type tracking (per expression, not just AST)
- [x] Symbol table or scope-aware analysis
- [x] Early type inference

### 🧰 Infrastructure
- [ ] Pretty-printer / formatter
- [ ] Macro-based AST walkers (to simplify passes)
- [ ] IR generation or bytecode emission
