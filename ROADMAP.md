# Rust: Zero → Systems Engineer

Goal: write real infrastructure in Rust — a database, a compiler, a kernel, a
Redis/Kafka/Docker-class tool — and understand *why* every language decision
was made, in contrast to C and C++.

Background assumed: some C and C++. Every lesson carries a **"vs C/C++"**
section, because the fastest way to learn Rust with your background is to see
which specific C/C++ failure mode each feature was built to kill.

---

## How this works

- Each lesson = concept → runnable code → the C/C++ bug it prevents → exercise.
- You type the code. Reading Rust teaches nothing; fighting the borrow checker
  teaches everything.
- Every phase ends in a project that cannot be faked.
- Track yourself in `PROGRESS.md`.

Directory layout: one cargo project per topic, grouped by phase.

---

## Phase 0 — Toolchain & mental model  (1 session)

| Topic | vs C/C++ |
|---|---|
| rustup, cargo, crates.io, `cargo new/run/test/clippy/fmt` | no CMake, no vcpkg/conan, no header/source split, no include guards, no ODR violations |
| compilation model: crates, not translation units | no preprocessor text pasting; no fragile `#include` ordering |
| editions, MSRV, debug vs release | |

**Project:** hello world + inspect the generated assembly for debug vs release.

---

## Phase 1 — Ownership: the whole language  (the hard part; take your time)

1. Move semantics by default, `Copy` types
2. Borrowing: `&T`, `&mut T`, aliasing XOR mutation
3. Slices, and the `str` / `String` split
4. Lifetimes: what they actually are (they are *not* "how long a value lives")
5. Lifetime elision, structs holding references
6. Shadowing, scopes, `Drop` order

| vs C/C++ |
|---|
| C++ move leaves a "valid but unspecified" husk you can still use → Rust makes use-after-move a **compile error** |
| Dangling reference from returning `&local` → compile error, not UB |
| Iterator invalidation (`push` while holding a reference into the vector) → compile error |
| `string_view` / `span` dangling → lifetimes make it impossible |
| No RAII discipline to remember — you cannot forget; no double-free, no leak on an exception path |

**Project:** a `Vec`-backed arena allocator + a text tokenizer returning
borrowed `&str` slices with zero allocation.

---

## Phase 2 — Types that make illegal states unrepresentable

1. `enum` as a real sum type; `match` exhaustiveness
2. `Option<T>` — the end of `NULL`
3. `Result<T, E>`, `?`, error types, `thiserror` / `anyhow`
4. Structs, tuple structs, the newtype pattern
5. Pattern matching in depth: guards, bindings, destructuring, `if let` / `let else`
6. Generics & monomorphization
7. Traits: `impl`, default methods, associated types, blanket impls, coherence
8. Operator overloading, `Deref`, `Drop`, `From`/`Into`, `Display`/`Debug`
9. Collections: `Vec`, `HashMap`, `BTreeMap`, `VecDeque`, `HashSet`
10. Modules, visibility, workspaces, unit + integration tests, doctests

| vs C/C++ |
|---|
| C tagged unions / C++ `std::variant` → no `std::get` throwing, no forgotten case; `match` is exhaustive or it does not compile |
| `nullptr` deref is the top C++ CVE class → `Option` forces the check |
| errno / return-code checking is optional and silently skipped → `Result` is `#[must_use]` |
| C++ exceptions: invisible control flow, unwind cost, `noexcept` audits → `?` is explicit and visible in the source |
| Templates fail at *instantiation* with 400-line errors → traits are checked at *definition*; no SFINAE, no concepts bolted on twenty years late |
| ODR violations, `inline` rules, link-time surprises → crate-level coherence |

**Project:** a JSON parser + serializer, hand-written, zero dependencies.

---

## Phase 3 — Closures, iterators, and zero-cost abstraction

1. `Fn` / `FnMut` / `FnOnce`, capture rules, `move` closures
2. The `Iterator` trait, adapters, laziness, `collect`, `IntoIterator`
3. Writing your own iterators
4. `impl Trait` vs `dyn Trait`, trait objects, vtables, object safety
5. Reading generated assembly to prove "zero-cost" is literal

| vs C/C++ |
|---|
| C++ lambdas capture by reference and dangle silently → Rust closures carry lifetimes |
| `std::function` always type-erases: heap allocation + indirect call → Rust closures are unboxed by default; you opt in to `Box<dyn Fn>` |
| Ranges landed in C++20 and is still awkward → iterators are idiomatic here and compile to the same loop as hand-written C |

**Project:** a streaming log-processing pipeline; benchmark against the
hand-rolled `for` loop and show identical codegen.

---

## Phase 4 — Smart pointers & interior mutability

1. `Box`, `Rc`, `Arc`, `Weak`, reference cycles
2. `Cell`, `RefCell`, `UnsafeCell` — the real foundation
3. `Cow`
4. Shared-mutable patterns: `Rc<RefCell<T>>`, and why it is often a smell
5. Graphs, trees, intrusive structures — the arena/index approach

| vs C/C++ |
|---|
| `shared_ptr`'s refcount is atomic *always* → Rust splits `Rc` (cheap) from `Arc` (atomic); you pay only when you share across threads |
| `shared_ptr` cycles leak silently → `Weak` is in your face |
| `const` in C++ is a lie (`mutable`, `const_cast`) → `UnsafeCell` is the single sanctioned hole, and the compiler knows about it |

**Project:** an LRU cache (intrusive doubly linked list) — first with
`Rc<RefCell<_>>`, then rewritten index-based over an arena. Compare both.

---

## Phase 5 — Fearless concurrency

1. `thread::spawn`, `JoinHandle`, scoped threads
2. `Send` and `Sync`: the two traits that eliminate data races
3. Channels: `mpsc`, `crossbeam`
4. `Mutex`, `RwLock`, poisoning, `parking_lot`
5. Atomics and memory ordering (`Relaxed` → `SeqCst`), the C++11 memory model
6. Lock-free structures, ABA, hazard pointers / epoch GC (`crossbeam-epoch`)
7. Thread pools, work stealing, `rayon`

| vs C/C++ |
|---|
| **The headline feature.** A data race in C++ is UB the compiler never mentions. In Rust it does not compile. |
| `std::mutex` plus a forgotten lock → `Mutex<T>` *contains* the data; there is no way to reach it unlocked |
| TSan catches races at runtime, sometimes, on the paths you exercised → Rust catches them at compile time, always |
| The memory model underneath is the same one — you learn the same `acquire`/`release`, but Rust makes the unsafe parts explicit |

**Project:** a work-stealing thread pool + a lock-free MPMC queue.

---

## Phase 6 — Async Rust

1. `Future`, `poll`, wakers, the state-machine transform
2. `async` / `.await` desugaring
3. `Pin`, `Unpin`, self-referential structs — why `Pin` has to exist
4. Executors: write a toy one from scratch
5. Tokio: runtime, tasks, `select!`, cancellation, `spawn_blocking`
6. Async traits, `Stream`, backpressure
7. What is underneath: epoll / io_uring / IOCP

| vs C/C++ |
|---|
| C: hand-rolled epoll state machines, callback spaghetti, manual lifetime tracking across callbacks |
| C++20 coroutines: frame allocation is implicit, no standard executor, easy dangling capture across a suspension point |
| Rust: the state machine is generated, stack-allocated where possible, and the borrow checker follows you across `.await` |

**Project:** async TCP echo → then a **Redis clone** (RESP protocol,
pipelining, expiry, pub/sub) on tokio.

---

## Phase 7 — Unsafe Rust & the machine

1. Raw pointers, `unsafe` blocks; what `unsafe` does and does *not* switch off
2. The rules: aliasing, validity, alignment, provenance, Stacked/Tree Borrows
3. The UB catalogue, and `miri`
4. `MaybeUninit`, `ptr::read`/`write`, `NonNull`
5. Implement `Vec`, `Rc`, and a doubly linked list from scratch
6. `PhantomData`, variance, dropck
7. FFI: calling C, being called from C, `bindgen`, `cbindgen`, `#[repr(C)]`
8. Custom allocators, `GlobalAlloc`
9. `no_std`; `core` vs `alloc` vs `std`; panics without an OS

| vs C/C++ |
|---|
| In C/C++ **all** code is unsafe; an audit means reading the whole program. In Rust the audit surface is the `unsafe` blocks, and they are greppable. |
| UB in C/C++ is largely undiscoverable by tooling → `miri` interprets your unsafe code and names the exact violation |
| Strict aliasing / `restrict` is opt-in and in practice ignored in C → Rust's `&mut` *is* `noalias` everywhere, which is why safe Rust sometimes beats C here |

**Project:** your own `Vec<T>` that passes `miri`, then a slab allocator.

---

## Phase 8 — Advanced type system & metaprogramming

1. Higher-ranked trait bounds (`for<'a>`)
2. Generic associated types (GATs), lending iterators
3. Const generics, `const fn`, compile-time evaluation
4. Typestate pattern, sealed traits, phantom types
5. Declarative macros (`macro_rules!`) and hygiene
6. Procedural macros: derive, attribute, function-like; `syn` / `quote`
7. Build scripts, codegen, conditional compilation

| vs C/C++ |
|---|
| The C preprocessor is textual and unhygienic → `macro_rules!` operates on token trees and cannot capture your locals |
| C++ template metaprogramming is a Turing tarpit → proc macros are ordinary Rust programs that transform syntax |
| `static_assert` + concepts vs typestate: Rust can make "call `start()` before `send()`" a compile error at zero runtime cost |

**Project:** a `#[derive(Serialize)]` proc macro, and a typestate state machine.

---

## Phase 9 — Performance engineering

1. Profiling: `perf`, `cargo-flamegraph`, `criterion`, `hyperfine`
2. Cache behaviour, struct layout, `#[repr]`, false sharing
3. SIMD: `std::simd`, intrinsics, autovectorisation
4. Allocation strategy: arenas, pooling, `smallvec`
5. Compile time: LTO, codegen-units, PGO
6. Binary size, panic strategies

---

## Phase 10 — Capstone: pick your infrastructure

Each is a multi-week project. Do **at least two**.

### A. Storage engine / database
WAL → page cache → B+Tree or LSM (memtable, SSTable, compaction, bloom
filters) → MVCC transactions → crash recovery → KV or SQL front-end.
*Reference points: sled, RocksDB, TiKV.*

### B. Compiler / language
Lexer → Pratt parser → AST → resolver → type checker → IR → optimiser →
bytecode VM, then a Cranelift or LLVM backend.
*Reference points: rustc itself, Cranelift.*

### C. Container runtime (Docker-class)
Linux namespaces, cgroups v2, pivot_root, overlayfs, seccomp, the OCI image
spec, registry pull. (Needs Linux — WSL2 is fine.)
*Reference points: youki, runc.*

### D. Distributed log (Kafka-class)
Segmented append-only log, offset index, consumer groups, replication via
Raft, wire protocol.
*Reference points: Fluvio, openraft.*

### E. OS kernel
`no_std`, bootloader, VGA/serial, GDT/IDT, paging, heap allocator, preemptive
multitasking, an async executor in kernel space.
*Reference: Philipp Oppermann, "Writing an OS in Rust".*

---

## The honest summary: why Rust over C++

Rust is not better because the syntax is nicer. It is better because the
compiler statically proves three properties that C and C++ cannot:

1. **Memory safety without a GC** — no use-after-free, double-free, buffer
   overflow, or dangling pointer in safe code.
2. **Data-race freedom** — enforced by `Send` / `Sync` at compile time.
3. **No undefined behaviour in safe code** — UB is confined to `unsafe`
   blocks, which you can grep for.

It pays for this with a steep ownership learning curve, longer compile times,
a smaller (though excellent) ecosystem, and real friction around cyclic data
structures and some self-referential async patterns. Those costs are genuine.
The trade is worth it for infrastructure, where one use-after-free is a CVE.

Where C++ still wins: existing codebases, GPU/CUDA, parts of the scientific
and game-engine ecosystems, and the maturity of template metaprogramming.
