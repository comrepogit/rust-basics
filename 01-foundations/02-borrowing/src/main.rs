// Lesson 2 — Borrowing: references without ownership
//
// Run me:   cargo run
// Read NOTES.md alongside this file.

fn main() {
    section("1. Shared borrows: many readers, no writers");
    shared_borrows();

    section("2. Mutable borrows: exactly one writer, no readers");
    mutable_borrows();

    section("3. Aliasing XOR mutation — the rule behind everything");
    aliasing_xor_mutation();

    section("4. Non-lexical lifetimes: a borrow ends at its LAST USE");
    non_lexical_lifetimes();

    section("5. Reborrowing: passing a &mut along without losing it");
    reborrowing();

    section("6. Method receivers: &self, &mut self, self");
    method_receivers();

    section("7. The bug that cannot compile: iterator invalidation");
    iterator_invalidation();
}

// ---------------------------------------------------------------------------

/// `&T` is a shared (immutable) borrow. You may have any number of them at
/// once. The owner stays the owner; nothing is moved.
fn shared_borrows() {
    let s = String::from("borrowed, not moved");

    let r1 = &s;
    let r2 = &s;
    let r3 = &s;

    println!("  r1={r1}");
    println!("  r2 len={}", r2.len());
    println!("  r3 upper={}", r3.to_uppercase());
    println!("  owner still usable: {s}"); // s was never moved
}

/// `&mut T` is an exclusive borrow. While it exists, it is the ONLY way to
/// reach the value — not even the owner may read it.
fn mutable_borrows() {
    let mut s = String::from("hello");

    let r = &mut s;
    r.push_str(", world");
    // println!("{s}");     // <- error: `s` is borrowed mutably by `r`
    println!("  through the &mut: {r}");

    // `r` is dead after its last use, so the owner is usable again.
    println!("  owner again: {s}");
}

/// At any point in a program, for any value, you may have EITHER:
///   - any number of `&T`,   OR
///   - exactly one `&mut T`
/// never both. This single rule is what makes Rust memory-safe AND
/// data-race-free, and it is why the optimiser may assume `&mut` is noalias.
fn aliasing_xor_mutation() {
    let mut v = vec![1, 2, 3];

    {
        let a = &v;
        let b = &v;
        println!("  two shared borrows fine: {a:?} {b:?}");
    }

    {
        let m = &mut v;
        m.push(4);
        println!("  one mutable borrow fine: {m:?}");
    }

    // let a = &v;
    // let m = &mut v;        // <- error[E0502]: cannot borrow `v` as mutable
    // println!("{a:?} {m:?}");  //    because it is also borrowed as immutable
    println!("  mixing the two: compile error");
}

/// A borrow's region is not the enclosing block — it ends after the last
/// place the reference is actually used. This is "NLL" and it is why a lot
/// of code that looks illegal is fine.
fn non_lexical_lifetimes() {
    let mut s = String::from("abc");

    let r = &s;
    println!("  read through r: {r}"); // <- r's borrow ENDS here

    s.push('d'); // legal: no live borrow at this point
    println!("  mutated after r died: {s}");
}

/// Passing a `&mut` to a function does not move it — the compiler inserts an
/// implicit reborrow, and you get it back when the callee returns.
fn reborrowing() {
    let mut count = 0u32;
    let r = &mut count;

    bump(r); // implicit reborrow: &mut *r
    bump(r); // ...so `r` is still usable here
    bump(&mut *r); // the explicit form of the same thing

    println!("  count = {r}");
}

fn bump(n: &mut u32) {
    *n += 1; // `*` to reach through the reference
}

/// The three receivers map exactly onto the three ways to access a value.
struct Counter {
    hits: u32,
}

impl Counter {
    fn get(&self) -> u32 {
        self.hits
    } // borrow shared
    fn bump(&mut self) {
        self.hits += 1;
    } // borrow exclusive
    fn into_total(self) -> u32 {
        self.hits
    } // consume
}

fn method_receivers() {
    let mut c = Counter { hits: 0 };
    c.bump();
    c.bump();
    println!("  get() -> {}", c.get());

    let total = c.into_total(); // `c` is moved here
                                // c.get();            // <- error: use of moved value
    println!("  into_total() -> {total}, and `c` is gone");
}

/// THE classic C++ bug. In Rust it is rejected at compile time.
fn iterator_invalidation() {
    let v = vec![10, 20, 30];

    let first = &v[0]; //  &v borrowed here
                       // v.push(40);        //  <- error[E0502]: cannot borrow `v` as mutable
    println!("  first = {first}");

    // Why it matters: push() may reallocate the buffer. In C++ the old
    // pointer survives and silently points at freed memory.
    println!("  (uncomment the push and read the error)");

    // The correct version: finish with the borrow, then mutate.
    let mut v = v;
    v.push(40);
    println!("  after push: {v:?}");
}

// ---------------------------------------------------------------------------

fn section(title: &str) {
    println!("\n=== {title} ===");
}
