// Lesson 2 exercises — borrowing.
//
// Run me:   cargo run --bin exercises
//
// Does not compile yet. Fix one at a time. Read every error before editing.

fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
    ex5();
}

// --- Exercise 1 -------------------------------------------------------------
// Lesson 1 forced you to hand ownership back and forth. Fix that here:
// change `shout` so the caller keeps its String. Change ONLY the signature
// and the call — the body should still work as written.
fn ex1() {
    let msg = String::from("borrow me");
    let loud = shout(msg);
    println!("1: {loud} / original still here: {msg}");
}

fn shout(s: String) -> String {
    s.to_uppercase()
}

// --- Exercise 2 -------------------------------------------------------------
// Two mutable borrows alive at once. Make it compile without changing what
// the code *does* (both pushes must still happen, in this order).
fn ex2() {
    let mut v = vec![1, 2, 3];
    let a = &mut v;
    let b = &mut v;
    a.push(4);
    b.push(5);
    println!("2: {v:?}");
}

// --- Exercise 3 -------------------------------------------------------------
// Iterator invalidation. Make it compile. The output must still be the
// largest element and then the vector with 100 appended.
// Hint: what is the smallest change to where the borrow ends?
fn ex3() {
    let mut v = vec![3, 9, 4];
    let biggest = v.iter().max().unwrap();
    v.push(100);
    println!("3: biggest was {biggest}, now {v:?}");
}

// --- Exercise 4 -------------------------------------------------------------
// `tally` should add every element into `acc` without consuming the slice
// and without copying it. Fix the signature.
fn ex4() {
    let nums = vec![1, 2, 3, 4];
    let mut acc = 0;
    tally(nums, &mut acc);
    println!("4: sum={acc}, nums still usable: {nums:?}");
}

fn tally(nums: Vec<i32>, acc: i32) {
    for n in nums {
        acc += n;
    }
}

// --- Exercise 5 -------------------------------------------------------------
// This one compiles already. Explain in the comment WHY it is legal, given
// that `v` is borrowed by `first` before `len()` is called.
//
// ANSWER:
//
fn ex5() {
    let v = vec![1, 2, 3];
    let first = &v[0];
    println!("5: first={first}");
    let n = v.len();
    println!("5: len={n}");
}
