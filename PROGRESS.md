# Progress

Toolchain: Rust 1.99.0 in **WSL2 Ubuntu** (primary — this is where you run),
plus Rust 1.99.0 on Windows with the `x86_64-pc-windows-gnu` target
(compiles only; see "How to run" below).

| Phase | Topic | Status | Notes |
|---|---|---|---|
| 0 | Toolchain & mental model | done | rustup + cargo installed |
| 1 | Ownership & moves | **exercises open** | `01-foundations/01-ownership` |
| 1 | Borrowing & references | **exercises open** | `01-foundations/02-borrowing` |
| 1 | Slices, `str` vs `String` | **exercises open** | `01-foundations/03-slices-strings` |
| 1 | Lifetimes | **exercises open** | `01-foundations/04-lifetimes` |
| 1 | Project: arena + tokenizer | **extensions open** | `01-foundations/05-project-tokenizer` |
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

## How to run (READ THIS FIRST)

**Build and run everything inside WSL, not in Windows.**

```bash
wsl                                      # drop into Ubuntu
cd /mnt/d/learning/rust/01-foundations/01-ownership
cargo run
cargo run --bin exercises
```

Why: this machine runs Sophos + CrowdStrike Falcon, which block execution of
freshly compiled unsigned binaries. Any `cargo run` on the Windows side fails
with `Access is denied. (os error 5)` even though compilation succeeds. You
are not a local admin, so you cannot add an AV exclusion yourself.

WSL2 Ubuntu 26.04 is already installed and runs compiled binaries fine.
Rust 1.99.0 is installed inside it, and `~/.bashrc` sets
`CARGO_TARGET_DIR=$HOME/.cargo-target` so build artifacts land on ext4
(much faster than `/mnt/d`, and out of the AV scanner's path).

Source files stay on `D:\learning\rust` and are editable from Windows as
normal; only the compiler runs on the Linux side.

If you ever want the Windows toolchain working too, ask IT for a Sophos
exclusion on `D:\learning\rust` — that is a routine developer request.

## Toolchain note

Visual Studio 18 Community is installed on this machine **without the
"Desktop development with C++" workload**, so there is no MSVC `link.exe` and
no Windows SDK. Rust's default `x86_64-pc-windows-msvc` target therefore
cannot link.

Current workaround: the `x86_64-pc-windows-gnu` toolchain, which ships its own
linker. This is fine for everything through Phase 9.

Since the AV block means you run everything in WSL anyway, this matters less
than it looked. The Windows toolchain is still useful for `cargo check` and
`cargo clippy` from the Windows side, which both work fine — only *executing*
binaries is blocked.

Upside of being pushed onto Linux early: Phase 10C (container runtime)
*requires* it — namespaces and cgroups do not exist on Windows — and `perf`,
`miri`, `gdb` and `cargo-flamegraph` are all better there. You are now set up
for Phases 7–10 ahead of schedule.
