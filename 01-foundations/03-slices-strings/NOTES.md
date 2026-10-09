# Lesson 3 — Slices, `str` and `String`

Lesson 2 borrowed a whole value. A **slice** borrows a contiguous *range* of
one: `&v[1..4]`. No copy, no allocation, and it obeys exactly the borrowing
rules you already know.

## Fat pointers

| type | layout |
|---|---|
| `&T` | `ptr` |
| `&[T]` | `ptr`, `len` |
| `&str` | `ptr`, `len` (bytes, guaranteed valid UTF-8) |
| `Vec<T>` | `ptr`, `len`, `capacity` (owns the buffer) |
| `String` | `ptr`, `len`, `capacity` (owns, guaranteed UTF-8) |

The length travels **with** the pointer. There is no separate `size_t n`
parameter to pass wrongly, and no null terminator to scan for.

## The owned/borrowed pairs

| owned | borrowed | note |
|---|---|---|
| `String` | `&str` | heap buffer vs a view into one |
| `Vec<T>` | `&[T]` | |
| `PathBuf` | `&Path` | you will meet these in Phase 2 |
| `OsString` | `&OsStr` | |

String literals are `&'static str` — they live in the binary's read-only
data for the entire run, so they are never freed and never allocated.

## Deref coercion

`&String` automatically becomes `&str`, and `&Vec<T>` becomes `&[T]`, at any
call site that wants the borrowed form. So:

> **Take `&str`, not `String`. Take `&[T]`, not `Vec<T>`.**

A function taking `&str` accepts a `String`, a literal, and a sub-slice. A
function taking `String` accepts only an owned `String`, and forces every
caller holding a literal to allocate. This is one of the highest-value habits
in the language; `clippy` will nag you about it, correctly.

Take the owned form only when you genuinely need to keep or modify the value.

## UTF-8

`str` is *guaranteed* valid UTF-8. That guarantee is exactly why you cannot
write `s[1]`: byte 1 may be the middle of a character.

```rust
let s = "héllo";
s.len()            // 6 — BYTES
s.chars().count()  // 5 — characters
&s[0..2]           // panics: not a char boundary
```

Use `.chars()`, `.char_indices()`, `.bytes()`. Slicing by byte range is
allowed, but panics if either end lands inside a character — which is the
right trade: a loud panic instead of silent mojibake or a buffer overrun.

## `split_at_mut`

The borrow checker reasons about whole variables, so it cannot tell that
`&mut v[..3]` and `&mut v[3..]` are disjoint. `split_at_mut` gives you both,
safely. Inside, it is `unsafe` code that upholds the invariant by hand — the
standard library's job is to package proofs like this behind safe APIs. You
will write your own in Phase 7.

---

## vs C/C++

### The whole category arrived late in C++, and still dangles

| | C++ | Rust |
|---|---|---|
| view of a string | `std::string_view` (C++17) | `&str` (from 1.0) |
| view of a buffer | `std::span` (C++20) | `&[T]` (from 1.0) |
| dangling view | your problem, silently UB | compile error |

```cpp
// C++ — the canonical string_view footgun. Compiles clean.
std::string_view bad() {
    std::string s = "temporary";
    return s;              // s dies at return; view dangles
}
std::string_view also_bad = std::string("x") + "y";   // dangles immediately
```
```rust
// Rust — the same shape.
fn bad() -> &str {
    let s = String::from("temporary");
    &s                     // error[E0106] / E0515: cannot return reference to local
}
```

C++ gave you the *performance* of views without the *safety*. Rust's views
carry lifetimes, so a dangling view is not expressible outside `unsafe`.

### C strings

```c
char buf[64];
strcpy(buf, user_input);        // overflow if input > 63
size_t n = strlen(s);           // O(n) scan, every call
foo(ptr, len);                  // two arguments; pass the wrong len, UB
```

`&str` carries its own length, is bounds-checked on slicing, cannot be null,
and cannot be non-UTF-8. `strlen` is O(1) here (`.len()` reads a field). The
entire `strcpy`/`strncpy`/`strlcpy` family of CVEs does not have a Rust
analogue in safe code.

### `std::string` has no encoding guarantee

A C++ `std::string` is a bag of `char`. Whether it is UTF-8, Latin-1, or
half-decoded garbage is convention. `s.size()` is bytes, `s[i]` is a byte, and
iterating "characters" correctly requires an external library. Rust splits the
question: `str`/`String` are UTF-8 by construction, and if you have arbitrary
bytes you say so with `[u8]`/`Vec<u8>` and convert explicitly with
`str::from_utf8`, which returns a `Result`.

### Bounds checking

`v[10]` on a 3-element slice panics in Rust; in C++ `v[10]` is UB and
`v.at(10)` throws. The Rust check costs a compare-and-branch that LLVM
usually hoists out of loops entirely, and iterators elide it by construction —
which is one reason idiomatic iterator code is often *faster* than indexing.

---

## Work for this lesson

1. `cargo run`. Note the `size_of` output in section 2 — that table is worth
   memorising.
2. Uncomment the three commented-out error lines, read each error.
3. `cargo run --bin exercises`. Ex1–Ex4 fail to compile; **Ex5 compiles and
   panics** — read the panic message, it tells you exactly what is wrong.
4. Answer in one sentence each:
   - Why can't `str` be indexed with an integer?
   - Why should a function take `&str` rather than `String`?
   - What does `&str` carry that `const char*` does not?

Next: **lifetimes** — the annotation that makes `fn first_word(s: &str) -> &str`
legal, and the reason `string_view` can dangle but `&str` cannot.
