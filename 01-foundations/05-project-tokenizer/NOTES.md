# Phase 1 Project — Zero-allocation tokenizer + index arena

This is the exam for Phase 1. Everything from the four lessons shows up at
once, and the two halves teach opposite lessons:

- **The lexer** shows lifetimes doing their job: borrowed `&'src str` tokens,
  no allocation, and a compiler guarantee that no token can outlive its source.
- **The arena** shows what to do when lifetimes *don't* fit: stop using
  references, use indices.

Knowing which of those two to reach for is most of what separates fluent Rust
from fighting the borrow checker.

## Run it

```
cargo run --bin demo
cargo run --bin demo -- "1 + foo * (2 - bar_9)"
cargo test
cargo test --test zero_copy -- --nocapture
```

## Layout

| file | what it teaches |
|---|---|
| `src/token.rs` | `Token<'src>` — a struct that borrows, and is `Copy` |
| `src/lexer.rs` | `-> Option<Token<'src>>` vs `-> Option<Token<'_>>` |
| `src/arena.rs` | the index-handle pattern, `Id` as a newtype |
| `src/tree.rs` | a tree with no self-referential lifetimes |
| `tests/zero_copy.rs` | pointer-level proof that nothing is copied |
| `tests/extensions.rs` | **your work** — seven `#[ignore]`d tests to make pass |

## The one line that matters

```rust
pub fn next_token(&mut self) -> Option<Token<'src>>
```

Elision rule 3 would have written `Option<Token<'_>>` for you, tying the token
to the borrow of `self`. The token would then die the moment you stopped
borrowing the lexer, and `tokenize()` could not exist. Writing `'src`
explicitly says what is actually true: the text points into the **source**,
not into the lexer.

Change it to `Option<Token<'_>>` and read the error. That single experiment is
worth more than the rest of this file.

## Why the arena

A tree of references is the shape that does not work:

```rust
struct Node<'a> { token: Token<'a>, kids: Vec<&'a Node<'a>> }  // pain
struct Node<'a> { token: Token<'a>, kids: Vec<Id> }            // fine
```

The first is self-referential — nodes borrow from the same collection that
owns them. Safe Rust cannot express that, and `Box` with parent pointers runs
into the same wall from the other side.

`Id` is a plain `u32` in a newtype. It is `Copy`, storable anywhere, stable
across arena growth (see `ids_are_stable_across_growth` — that test is the
whole argument), and the entire structure frees in one `Vec` drop.

This is not a beginner's workaround. rustc's own AST and HIR, Cranelift's IR,
and essentially every ECS and graph library in Rust are built this way.

### vs C/C++

The arena pattern is not unique to Rust — it is the same trick as a slab or
pool allocator, and for the same cache-locality reasons. The difference:

| | C/C++ | Rust |
|---|---|---|
| `Node*` after the pool reallocates | dangling, silent | — you hold an `Id`, nothing to dangle |
| mixing handles from two pools | silent corruption | logic bug; Phase 8 makes it a type error |
| freeing | remember to, and only once | one `Vec` drop |
| iterating while inserting | UB if it reallocates | compile error |

And the lexer half has no C++ analogue at all. A `std::string_view`-based
lexer is just as fast and just as zero-copy — and the moment someone passes it
a temporary, you get silent use-after-free. `Token<'src>` makes that a
compile error, at zero runtime cost.

## Your work

`tests/extensions.rs` has seven ignored tests. Remove the `#[ignore]` from one
at a time and make it pass:

1. **Line comments** — `# ...` to end of line is trivia.
2. **Float literals** — `12.5` is one Number; `12.` is `12` then `.`.
3. **String literals** — needs a new `TokenKind` variant; unterminated → Unknown.
4. **Keywords** — `let`/`if`/`else`/`fn`, but `lettuce` stays an Ident.
5. **Two-character operators** — `==`, `!=`, `<=`, `>=`, `->`; longest match wins.
6. **`line_col(offset)`** — 1-based line and column, for error messages.
7. **The invariant** — after all six, still zero-copy.

Hard rule: **no allocation**. No `String` field on `Token`, no `.to_string()`,
no `format!` in the lexer. Test 7 and `tests/zero_copy.rs` enforce it by
checking that every token's pointer still lands inside the source buffer.

### Then, to be sure you have it

Answer without looking anything up:

1. Why does `next_token` return `Option<Token<'src>>` instead of
   `Option<Token<'_>>`? What breaks if you change it?
2. Why is `Token<'src>` `Copy` when `String` is not?
3. `rest()` returns `&'src str`, not `&str`. Which elision rule would have
   applied, and what would it have given you?
4. Why can't `Node` hold `Vec<&Node>`?
5. What does `ids_are_stable_across_growth` prove, and what is the
   corresponding C++ bug?

## Phase 1 is done when

- all 15 unit tests and 5 `zero_copy` tests pass
- all 7 extension tests pass with no allocation added
- you can answer the five questions above cold

Then Phase 2: **enums, `Option`, `Result`, traits and generics** — where this
lexer gets real error handling, and the tokens become a proper sum type
carrying their parsed values.
