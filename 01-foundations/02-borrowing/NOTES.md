# Lesson 2 — Borrowing

Lesson 1 left you with a problem: if passing a value to a function moves it,
every function has to hand its arguments back. That is unusable. Borrowing is
the fix — temporary access without transfer of ownership.

## The two reference types

| | meaning | how many |
|---|---|---|
| `&T` | shared / read-only borrow | any number at once |
| `&mut T` | exclusive / read-write borrow | exactly one, and nothing else |

## The one rule: aliasing XOR mutation

At any point in the program, for any value, you may have **either** any number
of `&T`, **or** exactly one `&mut T`. Never both.

That is it. Everything else in Rust's safety story — memory safety, data-race
freedom, the `noalias` optimisation, `Send`/`Sync` in Phase 5 — is this rule
applied in a different setting.

Note what `&mut` really means. It is usually read as "mutable reference", but
the accurate name is **exclusive reference**: while it is alive, it is the only
path to that value in the entire program. Not even the owner may read through
its own variable.

## Non-lexical lifetimes

A borrow does not last until the end of the block. It ends after the
reference's **last use**:

```rust
let mut s = String::from("abc");
let r = &s;
println!("{r}");   // r's borrow ends here
s.push('d');       // legal
```

Most "but that should be fine!" moments resolve once you think in terms of
last-use rather than scope.

## Reborrowing

```rust
let r = &mut count;
bump(r);     // compiler inserts &mut *r
bump(r);     // so r is still yours afterwards
```

`&mut T` is not `Copy` (two copies would be two exclusive references), but
passing one to a function *reborrows* rather than moves, so you get it back.

## The three method receivers

```rust
fn get(&self)            // shared borrow     — read
fn bump(&mut self)       // exclusive borrow  — mutate
fn into_total(self)      // move              — consume, caller loses the value
```

The `into_*` naming convention signals the third case.

---

## vs C/C++ — what this rule buys you

### 1. Iterator / reference invalidation

```cpp
// C++ — compiles, passes review, is UB.
std::vector<int> v{10, 20, 30};
int& first = v[0];
v.push_back(40);          // may reallocate; `first` now dangles
std::cout << first;       // undefined behaviour
```
```rust
// Rust — same program.
let mut v = vec![10, 20, 30];
let first = &v[0];
v.push(40);               // error[E0502]: cannot borrow `v` as mutable
println!("{first}");      //   because it is also borrowed as immutable
```

Every container in the C++ standard library carries a paragraph of
invalidation rules you are expected to memorise. In Rust `push` needs
`&mut self`, you are holding a `&`, and the compiler stops you. The rules are
not documentation — they are the type signatures.

### 2. `const&` is not a guarantee in C++

```cpp
void f(const std::string& a, std::string& b);
f(s, s);   // legal. `a` is const, yet changes under you when b is written.
```

A C++ `const&` promises only that *you* will not write through it. Rust's `&T`
promises that **nobody** will write through any path while it lives. That is a
much stronger claim, and the compiler enforces it at the call site:

```rust
fn f(a: &String, b: &mut String);
f(&s, &mut s);   // error[E0502]: cannot borrow `s` as mutable
```

### 3. Aliasing and the optimiser

C has `restrict`; almost nobody uses it correctly, and C++ does not have it at
all in the standard. So the compiler must assume two pointers may alias and
reload values it could otherwise keep in registers.

In Rust, `&mut T` *is* `noalias`, always, for free, because the type system
proved it. This is the reason safe Rust sometimes outperforms equivalent C:
the optimiser is working with better information, not worse.

### 4. No null references

`&T` and `&mut T` are never null, never dangling, always aligned, always point
at an initialised value. C++ references are "supposed to" satisfy that too,
but `*(T*)nullptr` into a reference is a one-line mistake. Here it is
unrepresentable without `unsafe`.

---

## Errors you will meet

| Error | Meaning |
|---|---|
| `E0502` | mixing a shared and a mutable borrow |
| `E0499` | two mutable borrows at once |
| `E0506` | assigning to a value that is currently borrowed |
| `E0596` | borrowing as mutable through a non-`mut` binding |
| `E0507` | trying to move a value out from behind a reference |

`E0507` is worth internalising now: a `&T` lets you *read*, not *take*. To get
an owned value out you need `.clone()`, or `Copy`, or one of the `std::mem`
functions (`take`, `replace`, `swap`) you will meet in Lesson 4.

---

## Work for this lesson

1. `cargo run`, predict outputs first.
2. Uncomment every commented-out line in `src/main.rs`, one at a time, read the
   error, restore it. Especially section 7.
3. `cargo run --bin exercises` — five exercises. Ex5 wants a written answer.
4. Answer in one sentence each:
   - Why is `&mut T` better described as "exclusive" than as "mutable"?
   - What does a C++ `const&` promise that a Rust `&T` does not, and vice versa?

Next: **slices and strings** — the borrow of a *range*, and why Rust splits
`String` from `&str` when C++ took until C++17 to add `string_view`.
