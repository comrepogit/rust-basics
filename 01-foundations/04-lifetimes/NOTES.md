# Lesson 4 — Lifetimes

## What a lifetime actually is

A lifetime is a **compile-time label naming a region of code**. It is not a
duration, not a runtime value, not stored anywhere, and it is erased before
code generation. Annotating a reference changes *nothing* about when anything
is allocated or freed.

The single most common beginner misreading:

> ❌ "`<'a>` makes the value live as long as `'a`."
> ✅ "`<'a>` *requires* that the value already lives at least as long as `'a`,
>    and the compiler will reject the program if it cannot prove that."

Lifetimes are **constraints you state and the compiler checks**, exactly like
types. `fn f(x: i32)` does not make `x` an integer; it demands one.

## Why they exist

```rust
fn dangle() -> &String {       // error[E0106]: missing lifetime specifier
    let s = String::from("x");
    &s
}
```

> *this function's return type contains a borrowed value, but there is no
> value for it to be borrowed from*

The compiler cannot verify a reference outlives its referent unless it knows
what the referent *is*. A returned reference must therefore be traceable to an
input. That is all a lifetime parameter does — it names the connection.

## Generic lifetime parameters

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

Read it as: *for any region `'a` such that both inputs are valid throughout
`'a`, the result is valid throughout `'a`.* The caller picks `'a`, and the
compiler infers it as the **shorter** of the two inputs' regions.

## The three elision rules

You rarely write annotations, because of these:

1. Every elided *input* lifetime gets its own fresh parameter.
2. If there is exactly **one** input lifetime, it is assigned to every output.
3. If one input is `&self` or `&mut self`, **its** lifetime is assigned to
   every output (this rule wins over rule 2).

```rust
fn first_word(s: &str) -> &str              // rule 2
fn first_word<'a>(s: &'a str) -> &'a str    // what it means

fn rest(&self) -> &str                      // rule 3 — borrows from self
```

`longest` has two input lifetimes and no `self`, so no rule applies and you
must annotate by hand. That is the whole reason it is the standard example.

## Separate lifetimes matter

```rust
fn always_x<'a>(x: &'a str, _y: &'a str) -> &'a str { x }
```

This over-constrains: the result is tied to `min(x, y)` even though `y` is
never read. Giving each input its own parameter states the truth:

```rust
fn always_x<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str { x }
```

Rule of thumb: **one lifetime parameter per independent input**, and tie the
output only to what it actually borrows from.

## Structs holding references

```rust
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}
```

The parameter means: *a `Parser<'a>` may not outlive the `'a` it borrows.*
Any struct containing a reference needs one.

Note `next_token` in `main.rs`:

```rust
fn next_token(&mut self) -> Option<&'a str>   // not -> Option<&str>
```

Elision rule 3 would tie the token to `&self`, meaning the token dies when you
stop borrowing the parser. Writing `&'a str` explicitly ties it to the
*input text* instead — so you can collect tokens, drop the parser, and keep
using them. That distinction is the core of the Lesson 5 project.

## `'static` — two different things

| form | meaning |
|---|---|
| `&'static T` | the *reference* is valid for the whole program (e.g. a string literal in rodata) |
| `T: 'static` | the *type* contains no borrows at all |

`String` satisfies `T: 'static` despite being heap-allocated and dropped,
because it borrows from nothing. Confusing the two produces a lot of
misdirected `'static` bounds. You will meet `T: 'static` again when spawning
threads in Phase 5.

## Escaping `E0507`

A `&mut T` lets you mutate, not move out. These three move a value out while
leaving something valid behind:

```rust
std::mem::take(&mut s)              // leaves Default::default()
std::mem::replace(&mut v, vec![9])  // leaves what you supply
std::mem::swap(&mut a, &mut b)      // exchanges two
```

---

## vs C/C++

### C++ has no way to say this at all

```cpp
std::string_view longest(std::string_view x, std::string_view y);
```

Nothing in that signature says whether the result borrows from `x`, from `y`,
from a global, or from a temporary that already died. The information lives in
a comment, if you are lucky. Every caller must reason about it manually, and
every refactor can silently invalidate that reasoning.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

Here the relationship is **in the type**, checked at every call site, forever.

### The bugs this eliminates outright

```cpp
std::string_view sv = std::string("hi") + "there";   // dangles immediately
const int& r = std::vector<int>{1,2,3}[0];           // dangles
auto& x = *std::make_unique<T>();                    // dangles
[&]{ return local; }                                 // lambda captures by ref, escapes
```

All four are compile errors in Rust. Lifetime-annotation bugs are the single
largest remaining source of memory-unsafety in modern "safe-style" C++, which
is why the C++ committee has spent years on lifetime profiles and static
analysers to retrofit a weaker version of what Rust checks by construction.

### What it costs you

Honesty: this is the part of Rust that is genuinely harder than C++.

- Self-referential structs (a struct holding both a `String` and a `&str` into
  it) are not expressible safely. You use indices or an arena instead — which
  is exactly the Lesson 5 project.
- Doubly linked lists, graphs with back-edges, and some observer patterns
  need `Rc`/`Weak` (Phase 4), indices, or `unsafe` (Phase 7).
- Lifetime errors in generic + async code get verbose.

The trade: those patterns are *also* where C++ memory bugs cluster. Rust makes
the difficulty visible at compile time instead of at 3am in production.

---

## Work for this lesson

1. `cargo run`. Section 4 is the important one — read it twice.
2. In `main.rs`, uncomment the `dangle` function and the two commented
   `println!`s, and read E0106 / E0597.
3. `cargo run --bin exercises` — five exercises. Ex2 wants a written answer.
   Ex4 is the "separate lifetimes" lesson; ex5 is `mem::take`.
4. Answer in one sentence each:
   - Does `<'a>` make anything live longer? What does it do?
   - When does elision fail, and why is `longest` the classic example?
   - What is the difference between `&'static str` and `T: 'static`?

Next: the **Phase 1 project** — an arena allocator and a zero-allocation
tokenizer that returns `&'a str` slices borrowed from the source text.
