// Lesson 3 — Slices, str and String
//
// Run me:   cargo run
// Read NOTES.md alongside this file.

fn main() {
    section("1. A slice is a borrow of a RANGE");
    slices();

    section("2. Slices are fat pointers: (ptr, len)");
    fat_pointers();

    section("3. String vs &str");
    string_vs_str();

    section("4. Deref coercion: why &String works where &str is wanted");
    deref_coercion();

    section("5. UTF-8 is not an array of characters");
    utf8();

    section("6. The good signature: take &str and &[T], not String and Vec");
    good_signatures();

    section("7. split_at_mut: two disjoint &mut from one slice");
    disjoint_mutable_slices();
}

// ---------------------------------------------------------------------------

/// `&v[a..b]` borrows a contiguous run of elements. No copy, no allocation.
fn slices() {
    let v = vec![10, 20, 30, 40, 50];

    let all = &v[..];
    let middle = &v[1..4]; // 20, 30, 40   (end is exclusive)
    let head = &v[..2];
    let tail = &v[3..];

    println!("  all    = {all:?}");
    println!("  [1..4] = {middle:?}");
    println!("  [..2]  = {head:?}");
    println!("  [3..]  = {tail:?}");

    // A slice borrows the vector, so the usual rule applies:
    // v.push(60);          // <- error[E0502]: `v` is borrowed by `middle`
    println!("  owner still alive: len {}", v.len());
}

/// `&[T]` is two words: a pointer and a length. `&T` is one word.
/// That is why `&[T]` is called a "fat pointer" — it carries metadata.
fn fat_pointers() {
    use std::mem::size_of;
    println!("  size_of::<&i32>()   = {}", size_of::<&i32>());
    println!("  size_of::<&[i32]>() = {}  (ptr + len)", size_of::<&[i32]>());
    println!("  size_of::<&str>()   = {}  (ptr + len)", size_of::<&str>());
    println!(
        "  size_of::<String>() = {}  (ptr + len + capacity)",
        size_of::<String>()
    );

    // The length travels WITH the pointer. There is no separate `n` argument
    // to get wrong, and no null terminator to scan for.
}

/// `String` owns a growable heap buffer. `&str` borrows a UTF-8 run of bytes
/// that somebody else owns.
fn string_vs_str() {
    let owned: String = String::from("owned");
    let literal: &'static str = "baked into the binary";
    let borrowed: &str = &owned[..];

    println!("  String  : {owned}");
    println!("  &'static: {literal}");
    println!("  &str    : {borrowed}");

    // Literals live in the executable's read-only data for the whole program,
    // which is what 'static means. They are not heap-allocated.

    let built: String = format!("{literal} + {owned}"); // allocates
    println!("  built   : {built}");
}

/// You almost never write `&s[..]` by hand, because `&String` coerces to
/// `&str` automatically (and `&Vec<T>` to `&[T]`). This is Deref coercion.
fn deref_coercion() {
    let s = String::from("hello");
    let v = vec![1, 2, 3];

    takes_str(&s); //  &String -> &str
    takes_str("literal"); //  already &str
    takes_slice(&v); //  &Vec<i32> -> &[i32]
    takes_slice(&[9, 8]); //  &[i32; 2] -> &[i32]

    println!("  all four calls hit the same two functions");
}

fn takes_str(s: &str) {
    println!("    takes_str({s})");
}
fn takes_slice(xs: &[i32]) {
    println!("    takes_slice({xs:?})");
}

/// `str` is guaranteed-valid UTF-8. That guarantee is why you cannot index it
/// by integer: byte 3 may be the middle of a character.
fn utf8() {
    let s = "héllo";

    println!("  s            = {s}");
    println!("  s.len()      = {}  <- BYTES, not characters", s.len());
    println!("  s.chars()    = {}  characters", s.chars().count());

    // let c = s[1];                 // <- error: `str` cannot be indexed by usize
    // let bad = &s[0..2];           // <- panics: not a char boundary

    print!("  chars        :");
    for c in s.chars() {
        print!(" {c}");
    }
    println!();

    print!("  char_indices :");
    for (i, c) in s.char_indices() {
        print!(" {i}:{c}");
    }
    println!();

    print!("  bytes        :");
    for b in s.bytes() {
        print!(" {b}");
    }
    println!();
}

/// Take the borrowed form. It accepts strictly more callers and never forces
/// an allocation on them.
fn good_signatures() {
    let owned = String::from("the quick brown fox");

    //                      works with String, &str, literals, and slices of each
    println!("  first_word(&owned)      = {:?}", first_word(&owned));
    println!("  first_word(\"ab cd\")     = {:?}", first_word("ab cd"));
    println!("  first_word(&owned[4..])  = {:?}", first_word(&owned[4..]));

    // Note the return type: a &str borrowed FROM the input. Zero allocation.
    // Lesson 4 explains the lifetime rule that makes this signature legal.
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

/// The borrow checker works on whole variables, so it cannot see that
/// `&mut v[..2]` and `&mut v[2..]` are disjoint. `split_at_mut` is the
/// std-provided, `unsafe`-inside, safe-outside escape hatch.
fn disjoint_mutable_slices() {
    let mut v = [1, 2, 3, 4, 5, 6];

    let (left, right) = v.split_at_mut(3);
    left[0] = 100;
    right[0] = 200;

    println!("  left  = {left:?}");
    println!("  right = {right:?}");
    println!("  whole = {v:?}");

    // You will write your own version of this in Phase 7.
}

// ---------------------------------------------------------------------------

fn section(title: &str) {
    println!("\n=== {title} ===");
}
