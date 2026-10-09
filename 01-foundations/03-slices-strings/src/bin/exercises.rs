// Lesson 3 exercises — slices and strings.
//
// Run me:   cargo run --bin exercises
//
// Ex1-Ex4 do not compile. Ex5 compiles but PANICS — fix that too.

fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
    ex5();
}

// --- Exercise 1 -------------------------------------------------------------
// Widen the signature so all three calls work, without changing the calls
// and without allocating.
fn ex1() {
    let owned = String::from("alpha beta");
    println!("1: {}", char_count(&owned));
    println!("1: {}", char_count("gamma"));
    println!("1: {}", char_count(&owned[6..]));
}

fn char_count(s: String) -> usize {
    s.chars().count()
}

// --- Exercise 2 -------------------------------------------------------------
// Same idea for slices: make `sum` work for a Vec, an array, and a sub-slice.
fn ex2() {
    let v = vec![1, 2, 3, 4];
    let a = [10, 20, 30];
    println!("2: {} {} {}", sum(&v), sum(&a), sum(&v[1..3]));
}

fn sum(xs: Vec<i32>) -> i32 {
    xs.iter().sum()
}

// --- Exercise 3 -------------------------------------------------------------
// Make it compile. Explain to yourself why `clear()` is rejected here —
// which borrow is still alive, and until when?
fn ex3() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("3: {word}");
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

// --- Exercise 4 -------------------------------------------------------------
// `last_word` should return the final whitespace-separated word, borrowed
// from the input with zero allocation. Make it compile and be correct.
fn ex4() {
    println!("4: {:?}", last_word("one two three"));
    println!("4: {:?}", last_word("single"));
}

fn last_word(s: &str) -> &str {
    let i = s.rfind(' ');
    &s[i..]
}

// --- Exercise 5 -------------------------------------------------------------
// This compiles but panics at runtime. Find out why, then fix it so it prints
// the first three CHARACTERS of each input.
// Hint: `char_indices`.
fn ex5() {
    println!("5: {}", first_three("abcdef"));
    println!("5: {}", first_three("héllo")); // <- panics today
}

fn first_three(s: &str) -> &str {
    &s[..3]
}
