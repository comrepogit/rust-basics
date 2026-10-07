# Lesson 1 — Ownership and Moves

## The one rule

Every value in Rust has exactly **one** owning variable. When that variable
goes out of scope, the value is dropped (its destructor runs, its memory is
freed). Ownership can be **transferred** (moved) but never duplicated —
unless the type opts in to `Copy`.

That is the entire foundation. Borrowing, lifetimes, `Send`/`Sync`, and every
smart pointer in the standard library are consequences of this one rule.

## What "move" means mechanically

```rust
let a = String::from("hi");
let b = a;
```

A `String` is three words on the stack: `{ ptr, len, capacity }`, pointing at
a heap buffer. `let b = a` copies those three words into `b` — a plain
`memcpy`, same as C++.

The difference is what happens next: **rustc marks `a` as moved-out and
refuses every later use of it, at compile time.** So only `b` is alive to be
dropped, and the buffer is freed exactly once.

## vs C++ — this is the part that matters for you

C++11 gave you move semantics too, but bolted onto a language where copying
was already the default. The consequences:

| | C++ | Rust |
|---|---|---|
| Default for `=` | **copy** (deep, possibly expensive, silently) | **move** |
| State after a move | "valid but unspecified" — you may still call methods on it | statically dead; using it is a compile error |
| Enforcement | none; `std::move` is a cast and a convention | the type system |
| Cost of getting it wrong | use-after-move, double free, UB | won't build |

```cpp
// C++ — compiles clean, passes review, is a bug.
std::string a = "hi";
std::string b = std::move(a);
std::cout << a;        // legal C++. Unspecified value. Silent.
```

```rust
// Rust — same program.
let a = String::from("hi");
let b = a;
println!("{a}");       // error[E0382]: borrow of moved value: `a`
```

Also note what Rust does *not* need, because move is the default and the
compiler owns destruction:

- no copy constructor / move constructor / copy assignment / move assignment
- no destructor to remember to write
- no rule of three, rule of five, or rule of zero
- no `delete`d special members to prevent misuse
- no exception-safety audit of your destructor paths

The "rule of five" is roughly 40 lines of boilerplate per resource type in
C++, and getting any one of them subtly wrong is a memory bug. In Rust the
correct behaviour is the only behaviour you can express.

## `Copy`

A type is `Copy` when a bitwise duplicate is a *complete and correct* second
value: integers, floats, `bool`, `char`, raw pointers, and tuples/arrays of
those. For `Copy` types, `=` duplicates and the original stays usable.

`String`, `Vec<T>`, `Box<T>`, `File` — anything owning a resource — can never
be `Copy`, because `memcpy` would produce two owners of one allocation.

This is C++'s distinction between trivially-copyable types and resource types,
except Rust enforces it instead of documenting it.

## Drop

```rust
struct Noisy(&'static str);
impl Drop for Noisy {
    fn drop(&mut self) { println!("dropping {}", self.0); }
}
```

Deterministic, scope-based, reverse declaration order — identical to C++ RAII.
Two differences:

1. You get it for free. The compiler-generated drop recursively drops fields.
   You implement `Drop` only when you need *extra* behaviour (flush a buffer,
   close a socket, log).
2. Drop runs exactly once, because there is exactly one owner. No double-free
   is expressible in safe Rust.

## The errors you will hit, and what they mean

| Error | Meaning |
|---|---|
| `E0382: borrow of moved value` | you used a variable after its value moved elsewhere |
| `E0505: cannot move out of X because it is borrowed` | something still holds a reference; moving would dangle it |
| `E0106: missing lifetime specifier` | you returned a reference and the compiler cannot tell what it points into |

`E0106` in `dangle()` is the headline result: **returning a pointer to a local
is a compile error in Rust.** In C and C++ it is undefined behaviour that
usually "works" in debug builds and corrupts memory in release builds. This
single class of bug has produced thousands of CVEs.

## Work for this lesson

1. `cargo run` — read `src/main.rs` top to bottom, predict each output first.
2. Uncomment each commented-out `println!` in `main.rs` one at a time. Read
   the error. Put it back. Do not skip this; the errors are the curriculum.
3. `cargo run --bin exercises` — fix all four. Exercise 3 wants a written
   answer in the comment.
4. Answer, in your own words, in one sentence each:
   - Why can `String` never be `Copy`?
   - What does C++ leave behind after `std::move`, and what does Rust leave behind?

Next lesson: **borrowing** — how to use a value without taking ownership, and
why `&mut` being exclusive is what makes data races impossible later.
