// Lesson 1 exercises.
//
// Run me:   cargo run --bin exercises
//
// Right now this file DOES NOT COMPILE. That is the point.
// Fix each exercise, one at a time, until `cargo run --bin exercises` works.
// Read the compiler error carefully before changing anything — rustc's
// ownership errors are the best teacher in the language.

fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
}

// --- Exercise 1 -------------------------------------------------------------
// Make this compile WITHOUT removing either println!.
// Hint: there is more than one correct answer. Find at least two.
fn ex1() {
    let name = String::from("Sooraj");
    let greeting = name;
    println!("greeting: {greeting}");
    println!("name: {name}"); // <-- error today
}

// --- Exercise 2 -------------------------------------------------------------
// `total_length` should work, and the caller should still be able to print
// both strings afterwards. Change the signature if you need to.
fn ex2() {
    let a = String::from("hello");
    let b = String::from("world!");
    let n = total_length(a, b);
    println!("total = {n}, a = {a}, b = {b}"); // <-- error today
}

fn total_length(x: String, y: String) -> usize {
    x.len() + y.len()
}

// --- Exercise 3 -------------------------------------------------------------
// Why does this one already compile, when ex1 does not?
// Write your answer as a comment here before moving on:
//
// ANSWER:
//
fn ex3() {
    let n = 7u32;
    let m = n;
    println!("n = {n}, m = {m}");
}

// --- Exercise 4 -------------------------------------------------------------
// This is the C++ dangling-pointer bug, written in Rust.
// Make it compile. Note WHAT the compiler refuses to let you do.
fn ex4() {
    let r = dangle();
    println!("r = {r}");
}

fn dangle() -> &String {
    let s = String::from("local value");
    &s // <-- error today: returning a reference to a local
}
