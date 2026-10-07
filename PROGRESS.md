# Progress

Toolchain: Rust 1.99.0, default target `x86_64-pc-windows-gnu`
(see "Toolchain note" at the bottom).

| Phase | Topic | Status | Notes |
|---|---|---|---|
| 0 | Toolchain & mental model | done | rustup + cargo installed |
| 1 | Ownership & moves | **in progress** | `01-foundations/01-ownership` |
| 1 | Borrowing & references | next | |
| 1 | Slices, `str` vs `String` | | |
| 1 | Lifetimes | | |
| 1 | Project: arena + tokenizer | | |
| 2 | Enums, Option, Result | | |
| 2 | Traits & generics | | |
| 2 | Project: JSON parser | | |
| 3 | Closures & iterators | | |
| 4 | Smart pointers | | |
| 5 | Concurrency | | |
| 6 | Async + Redis clone | | |
| 7 | Unsafe, FFI, own `Vec` | | |
| 8 | Macros & advanced types | | |
| 9 | Performance | | |
| 10 | Capstone | | |

## Open questions / things to revisit

- (add yours here as you hit them)

## Toolchain note

Visual Studio 18 Community is installed on this machine **without the
"Desktop development with C++" workload**, so there is no MSVC `link.exe` and
no Windows SDK. Rust's default `x86_64-pc-windows-msvc` target therefore
cannot link.

Current workaround: the `x86_64-pc-windows-gnu` toolchain, which ships its own
linker. This is fine for everything through Phase 9.

Two things to consider later:

1. **Install the VS C++ workload** (needs admin, a few GB) and switch back
   with `rustup default stable-x86_64-pc-windows-msvc`. Better Windows
   debugging, and what most Windows crates assume.
2. **Install WSL2.** Phase 10C (container runtime) *requires* Linux —
   namespaces and cgroups do not exist on Windows. `perf` and `miri` are also
   nicer there. Recommended before Phase 7.
