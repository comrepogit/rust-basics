// Lesson 4 — Lifetimes
//
// Run me:   cargo run
// Read NOTES.md alongside this file.
//
// A lifetime is a COMPILE-TIME label naming a region of code. It is erased
// before codegen — there is no runtime cost, no runtime representation, and
// no lifetime value stored anywhere.

fn main() {
    section("1. The problem lifetimes solve");
    the_problem();

    section("2. A generic lifetime parameter");
    generic_lifetime();

    section("3. Elision: why most signatures need no annotation");
    elision();

    section("4. Lifetimes are constraints, not durations");
    constraints_not_durations();

    section("5. Structs that hold references");
    structs_with_references();

    section("6. 'static");
    static_lifetime();

    section("7. Escaping a borrow: mem::take / replace / swap");
    mem_tricks();
}

// ---------------------------------------------------------------------------

/// `fn dangle() -> &String` is rejected with E0106: "missing lifetime
/// specifier — this function's return type contains a borrowed value, but
/// there is no value for it to be borrowed from."
///
/// The compiler is not being pedantic. It literally cannot tell what the
/// returned reference points into, so it cannot check that the target
/// outlives the reference.
fn the_problem() {
    // fn dangle() -> &String {
    //     let s = String::from("local");
    //     &s
    // }

    // The fix is either to return ownership...
    fn no_dangle() -> String {
        String::from("owned, moved out")
    }
    println!("  no_dangle() -> {}", no_dangle());

    // ...or to borrow from an input, which is section 2.
}

/// `<'a>` declares a lifetime parameter, exactly like `<T>` declares a type
/// parameter. The signature below says:
///
///   "given two &str that are both valid for some region 'a, I return a &str
///    that is also valid for 'a"
///
/// The caller picks 'a: it becomes the SHORTER of the two inputs' regions.
fn generic_lifetime() {
    let long = String::from("the longer string");

    {
        let short = String::from("short");
        let result = longest(&long, &short);
        println!("  longest = {result}"); // fine: used inside short's scope
    }

    // let result = longest(&long, &short);   // <- `short` does not live long enough
    println!("  (move the call outside the block and read E0597)");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// Three elision rules let the compiler write the annotations for you:
///   1. every elided INPUT lifetime gets its own fresh parameter
///   2. if there is exactly ONE input lifetime, it is assigned to all outputs
///   3. if one input is `&self` / `&mut self`, ITS lifetime goes to all outputs
fn elision() {
    let s = String::from("the quick brown fox");

    // rule 2: one input reference -> the output borrows from it
    println!("  first_word  = {:?}", first_word(&s));

    // rule 3: &self wins, so the returned &str borrows from the struct
    let p = Parser { input: &s, pos: 4 };
    println!("  rest        = {:?}", p.rest());

    // No elision applies to `longest` (two inputs, no self), which is why it
    // must be annotated by hand.
}

// written:   fn first_word(s: &str) -> &str
// means:     fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

/// A lifetime annotation does NOT change how long anything lives. It only
/// states a relationship the compiler must verify. Proof: `always_x` claims
/// both inputs share 'a, but never looks at `y`.
fn constraints_not_durations() {
    let x = String::from("returned");

    let result;
    {
        let y = String::from("ignored");
        result = always_x(&x, &y);
        // `y` dies here. The VALUE is fine — but the signature promised the
        // result borrows from 'a = min(x, y), so this is still rejected:
        println!("  inside block: {result}");
    }
    // println!("{result}");    // <- E0597: `y` does not live long enough

    println!("  the annotation constrained us even though y was unused");
    println!("  fix: give y its own lifetime -> see `always_x_better`");

    let result2;
    {
        let y = String::from("ignored");
        result2 = always_x_better(&x, &y);
    }
    println!("  always_x_better still usable here: {result2}");
}

fn always_x<'a>(x: &'a str, _y: &'a str) -> &'a str {
    x
}

// Two independent lifetimes: the output only ties to `x`.
fn always_x_better<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

/// A struct holding a reference may not outlive what it borrows. The `<'a>`
/// on the struct is how that is expressed.
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    // rule 3: output borrows from &self
    fn rest(&self) -> &str {
        &self.input[self.pos..]
    }

    // explicit 'a: the returned &str borrows from the INPUT TEXT, not from
    // `self`. That is a stronger promise, and it is what lets the caller keep
    // the returned token after `self` is gone. You will need this in Lesson 5.
    fn next_token(&mut self) -> Option<&'a str> {
        let rest = &self.input[self.pos..];
        let rest = rest.trim_start();
        if rest.is_empty() {
            return None;
        }
        let start = self.input.len() - rest.len();
        let end = rest.find(' ').map_or(self.input.len(), |i| start + i);
        self.pos = end;
        Some(&self.input[start..end])
    }
}

fn structs_with_references() {
    let text = String::from("alpha beta gamma");
    let mut p = Parser::new(&text);

    let mut tokens = Vec::new();
    while let Some(t) = p.next_token() {
        tokens.push(t);
    }

    // `p` is finished with, but the tokens still borrow `text` — which is
    // alive. This only type-checks because next_token returns &'a str.
    drop(p);
    println!("  tokens = {tokens:?}");
}

/// `'static` means "valid for the whole program". Two distinct things carry
/// it, and conflating them is a common beginner error.
fn static_lifetime() {
    // 1. data baked into the binary
    let lit: &'static str = "in the executable's rodata";
    println!("  literal: {lit}");

    // 2. a value that contains no borrows at all (T: 'static)
    //    A String is 'static as a TYPE even though it is heap-allocated and
    //    dropped, because it borrows from nothing.
    let owned = String::from("heap, but T: 'static");
    takes_static(owned);

    println!("  '&'static str' != 'T: 'static' — see NOTES.md");
}

fn takes_static<T: 'static>(v: T) -> T {
    v
}

/// Lesson 2 said a `&T` lets you read, not take (E0507). These three
/// functions are how you legally move a value out from behind a `&mut`.
fn mem_tricks() {
    let mut s = String::from("original");

    // take: swap in Default::default(), hand you the old value
    let old = std::mem::take(&mut s);
    println!("  take    -> old={old:?}, left behind={s:?}");

    // replace: same, but you choose the replacement
    let mut v = vec![1, 2, 3];
    let old_v = std::mem::replace(&mut v, vec![9]);
    println!("  replace -> old={old_v:?}, left behind={v:?}");

    // swap: exchange two values behind two &mut
    let mut a = String::from("A");
    let mut b = String::from("B");
    std::mem::swap(&mut a, &mut b);
    println!("  swap    -> a={a:?}, b={b:?}");
}

// ---------------------------------------------------------------------------

fn section(title: &str) {
    println!("\n=== {title} ===");
}
