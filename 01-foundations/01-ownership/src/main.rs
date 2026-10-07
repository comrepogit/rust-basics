// Lesson 1 — Ownership and Moves
//
// Run me:   cargo run
// Read NOTES.md alongside this file.
//
// Every section is a function. Read the code, predict the output, THEN run it.

fn main() {
    section("1. Every value has exactly one owner");
    one_owner();

    section("2. Assignment MOVES, it does not copy");
    moves();

    section("3. Passing to a function moves too");
    moves_into_functions();

    section("4. ...unless the type is Copy");
    copy_types();

    section("5. Giving ownership back");
    returning_ownership();

    section("6. Drop runs deterministically, in reverse order");
    drop_order();
}

// ---------------------------------------------------------------------------

/// A `String` owns a heap buffer. When the owner goes out of scope, the buffer
/// is freed. No GC, no free() call, no leak.
fn one_owner() {
    {
        let s = String::from("heap-allocated");
        println!("  inside scope: {s}");
        // `s` goes out of scope here. Rust inserts the equivalent of free().
    }
    // println!("{s}");   // <- would not compile: `s` does not exist here.
    println!("  scope ended, buffer already freed");
}

/// The single most important difference from C++.
fn moves() {
    let a = String::from("owned by a");
    let b = a; // the heap buffer is now owned by `b`. `a` is DEAD.

    // println!("{a}");   // <- compile error: borrow of moved value: `a`
    println!("  b = {b}");
    println!("  `a` is statically unusable — not empty, not null: unusable");
}

/// Same rule: a function parameter takes ownership.
fn moves_into_functions() {
    let s = String::from("I will be consumed");
    consume(s);
    // println!("{s}");   // <- compile error, `s` was moved into consume()
    println!("  s was moved into consume() and dropped there");
}

fn consume(s: String) {
    println!("  consume() now owns: {s}");
} // `s` dropped here

/// Types that are a fixed-size bit pattern with no heap resource implement
/// `Copy`. For them, `=` duplicates instead of moving.
fn copy_types() {
    let x = 42i64; // i64: Copy
    let y = x; // bitwise duplicate
    println!("  x = {x}, y = {y}   (both still valid)");

    let p = (1u8, true, 'z'); // tuples of Copy types are Copy
    let q = p;
    println!("  p = {p:?}, q = {q:?}");

    // Rule of thumb: Copy == "memcpy is a complete, correct duplicate".
    // String is not Copy because memcpy would duplicate the *pointer*,
    // giving two owners of one buffer -> double free. Rust forbids it
    // at the type level instead of trusting you to write a copy constructor.
}

/// You can hand ownership out of a function, and hand it back.
fn returning_ownership() {
    let s = String::from("round trip");
    let (s, len) = with_length(s); // take it back
    println!("  '{s}' has length {len}");
    println!("  (this is clumsy on purpose — Lesson 2 fixes it with borrowing)");
}

fn with_length(s: String) -> (String, usize) {
    let n = s.len();
    (s, n) // give ownership back to the caller
}

/// Destruction is deterministic and in reverse declaration order,
/// exactly like C++ RAII — but you cannot forget to implement it.
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  dropping {}", self.0);
    }
}

fn drop_order() {
    let _first = Noisy("first");
    let _second = Noisy("second");
    let _third = Noisy("third");
    println!("  all three constructed; watch the order on scope exit:");
}

// ---------------------------------------------------------------------------

fn section(title: &str) {
    println!("\n=== {title} ===");
}
