
---

# 🧩 **Shared Memory Pool Blueprint**

## 🔷 1. **Purpose**

The shared pool exists to:

* **Enable safe inter-thread communication and synchronization.**
* Provide **atomic operations and data transfer mechanisms** without relying on locks or runtime checks.
* Be **strictly separated from thread-local memory**, accessed only via **dedicated VM instructions**.

---

## 🔷 2. **Shared Pool Structure**

### A. **Atomic `u64` Pool**

* A contiguous `Vec<AtomicU64>` or equivalent low-level memory.
* Each element is 8-byte aligned and accessed atomically.
* Treated as untyped; semantics are defined by the instruction.

#### Uses:

* Message passing (channels, mailboxes).
* Synchronization (flags, locks, counters).
* Custom inter-thread coordination.

---

### B. **Swappable Segments**

* Read-only data regions intended for efficient data sharing between threads.
* Each segment is written privately, then atomically **swapped into visibility**.
* Could be implemented via:

  * An atomic pointer or index into a segment table.
  * A versioned handle or slot system.

#### Uses:

* Bulk data transfer (e.g., message payloads).
* Immutable state handoff (e.g., double buffering).

---

## 🔷 3. **Access Rules and Instruction Semantics**

### 🔒 **Hard Isolation**

* Shared memory is **only accessed by special instructions**.
* Normal instructions **cannot** read/write shared memory—enforced structurally.

---

### ✅ **Atomic Instruction Semantics**

| Instruction                   | Allowed Ops | Suggested Ordering                  | Purpose / Notes                         |
| ----------------------------- | ----------- | ----------------------------------- | --------------------------------------- |
| `shared_load(addr)`           | Load `u64`  | `Acquire`                           | Observe value written by another thread |
| `shared_store(addr, v)`       | Store `u64` | `Release`                           | Publish value visible to other threads  |
| `shared_cas(addr, old, new)`  | CAS         | `AcqRel` or `SeqCst`                | Synchronization or conditional update   |
| `shared_fetch_add(addr, x)`   | RMW         | `AcqRel`                            | Counters, tokens, etc.                  |
| `swap_segment(slot, new_ptr)` | Atomic swap | `Release` (write), `Acquire` (read) | Swap in a buffer segment for readers    |
| `shared_barrier()`            | Fence       | `SeqCst`                            | Optional, enforce ordering globally     |

---

## 🔷 4. **Memory Model Guarantees**

* The pool provides **data-race freedom by construction**, assuming:

  * All access is via atomic instructions.
  * All instructions use correct ordering (e.g., Acquire/Release pairs).
* No shared memory location is accessed in a non-atomic way.

### Uncertain:

* **Formal memory model** (e.g., should you define a “happens-before” graph?).

  * **Option A**: Adopt Rust-style data-race freedom model.
  * **Option B**: Define a weaker minimal model (e.g., per-location coherence + release/acquire).
  * **Option C**: Leave unspecified and document expected usage patterns only.

---

## 🔷 5. **Access and Allocation Semantics**

* Shared pool memory is globally addressable by threads.
* **Unspecified:**

  * Allocation mechanism for shared slots or segment buffers.

    * **Option A**: Static, fixed-size pool.
    * **Option B**: Allocator API built on top of the shared pool.
    * **Option C**: User responsibility to manage allocation via protocols.

---

## 🔷 6. **Swappable Segment Semantics**

### ✅ Confirmed

* Written privately by the producer thread.
* Made visible to readers via atomic pointer/index update.
* Read-only for all readers after swap.

### Uncertain:

* **Segment lifetime and reuse policy**:

  * **Option A**: Manual recycling by sender/receiver after agreement.
  * **Option B**: Reference counting in user space via atomics.
  * **Option C**: Generational ring buffer (swap replaces oldest).

---

## 🔷 7. **Instruction-Level Isolation**

* Shared access instructions are a separate opcode group.
* They only operate on the shared pool—no overlap with local memory.
* Makes bugs (like races or aliasing) structurally impossible.

✅ Fully confirmed.

---

## 🔷 8. **Example Use Cases**

### 🔁 Lock-Free Channel

* Ring buffer backed by atomic index + shared slots.
* Uses `fetch_add`, `compare_exchange` for coordination.

### 📬 One-shot Message

* Producer allocates segment.
* Fills it, swaps pointer into shared slot.
* Consumer reads segment via atomic pointer.

### ⛓ Lock / Semaphore

* Atomic counter or flag at known shared memory location.
* Uses `fetch_sub`, `compare_exchange` to acquire/release.

SqgQueue? Concurrent stack/queue? Plus increasing atomic. Get index from queue or use that one.

---

## ✅ Summary

| Component             | Status      | Notes                                      |
| --------------------- | ----------- | ------------------------------------------ |
| Atomic `u64` pool     | ✅ Finalized | Used for low-level sync primitives         |
| Segment swapping      | ✅ Specified | Finalized conceptually, impl details open  |
| Access isolation      | ✅ Finalized | Normal vs. shared enforced structurally    |
| Instruction semantics | ✅ Stable    | Orderings chosen, extensible               |
| Memory model          | ⚠️ TBD      | Depends on how formal you want to be       |
| Allocation/reuse      | ⚠️ TBD      | Static vs. dynamic; segment reuse protocol |

---

