// Lesson 4 exercises — lifetimes.
//
// Run me:   cargo run --bin exercises

fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
    ex5();
}

// --- Exercise 1 -------------------------------------------------------------
// Annotate `pick` so this compiles. Do not change the body or the call.
fn ex1() {
    let a = String::from("apple");
    let b = String::from("banana");
    println!("1: {}", pick(&a, &b, true));
}

fn pick(x: &str, y: &str, first: bool) -> &str {
    if first {
        x
    } else {
        y
    }
}

// --- Exercise 2 -------------------------------------------------------------
// `Excerpt` holds a borrow. Make it compile.
// Then answer: what stops an `Excerpt` from outliving `novel`?
//
// ANSWER:
//
fn ex2() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let e = Excerpt {
        part: first_sentence,
    };
    println!("2: {}", e.announce());
}

struct Excerpt {
    part: &str,
}

impl Excerpt {
    fn announce(&self) -> &str {
        self.part
    }
}

// --- Exercise 3 -------------------------------------------------------------
// This is rejected even though it looks harmless. Read E0597 carefully,
// then fix it WITHOUT cloning and WITHOUT changing `shortest`'s signature.
fn ex3() {
    let outer = String::from("a long-lived string");
    let result;
    {
        let inner = String::from("short");
        result = shortest(&outer, &inner);
    }
    println!("3: {result}");
}

fn shortest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}

// --- Exercise 4 -------------------------------------------------------------
// `prefix_of` returns a borrow of `text`, never of `marker`.
// Give the two parameters SEPARATE lifetimes so that the call below compiles
// even though `marker` dies before the result is printed.
fn ex4() {
    let text = String::from("hello world");
    let found;
    {
        let marker = String::from("world");
        found = prefix_of(&text, &marker);
    }
    println!("4: {found:?}");
}

fn prefix_of<'a>(text: &'a str, marker: &'a str) -> Option<&'a str> {
    let i = text.find(marker)?;
    Some(&text[..i])
}

// --- Exercise 5 -------------------------------------------------------------
// E0507: cannot move out of borrowed content. Fix it using one of
// std::mem::take / replace / swap. `buf.lines` must end up empty.
fn ex5() {
    let mut buf = Buffer {
        lines: vec![String::from("a"), String::from("b")],
    };
    let drained = buf.drain();
    println!("5: drained={drained:?}, left={:?}", buf.lines);
}

struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    fn drain(&mut self) -> Vec<String> {
        self.lines
    }
}
