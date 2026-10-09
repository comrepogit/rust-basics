//! YOUR WORK.
//!
//! Every test here is `#[ignore]`d and currently fails. For each one:
//!   1. remove the `#[ignore]` line
//!   2. run `cargo test --test extensions`
//!   3. make it pass by extending `src/lexer.rs` (and `src/token.rs`)
//!
//! Run only the un-ignored ones:  cargo test --test extensions
//! Run everything including ignored:  cargo test --test extensions -- --include-ignored
//!
//! Constraint for all of them: **stay zero-allocation**. No `String` in
//! `Token`, no `.to_string()`, no `format!`. `tests/zero_copy.rs` must keep
//! passing when you are done — run it.

use tokenizer::{Lexer, TokenKind};

fn kinds(src: &str) -> Vec<TokenKind> {
    Lexer::tokenize(src).into_iter().map(|t| t.kind).collect()
}

fn texts(src: &str) -> Vec<&str> {
    Lexer::tokenize(src).into_iter().map(|t| t.text).collect()
}

// --- 1. Line comments -------------------------------------------------------
// `# ...` to end of line is trivia: skipped entirely, producing no token.
// Hint: extend `skip_trivia`, and loop — trivia can alternate with whitespace.
#[test]
#[ignore = "extension 1: implement line comments"]
fn line_comments_are_skipped() {
    assert_eq!(texts("a # this is ignored\nb"), ["a", "b"]);
    assert_eq!(texts("# only a comment"), Vec::<&str>::new());
    assert_eq!(texts("a  # c1\n  # c2\n  b"), ["a", "b"]);
}

// --- 2. Floating-point numbers ----------------------------------------------
// `12`, `12.5`, `0.5` lex as one Number. A trailing dot is NOT part of it:
// `12.` is Number("12") then Unknown("."). Keep it to one token kind.
#[test]
#[ignore = "extension 2: implement float literals"]
fn numbers_may_have_a_fractional_part() {
    assert_eq!(texts("12.5"), ["12.5"]);
    assert_eq!(kinds("12.5"), [TokenKind::Number]);
    assert_eq!(texts("0.5 + 2"), ["0.5", "+", "2"]);
    assert_eq!(texts("12."), ["12", "."]);
    assert_eq!(kinds("12."), [TokenKind::Number, TokenKind::Unknown]);
}

// --- 3. String literals -----------------------------------------------------
// Double-quoted, no escapes yet. `Token.text` must include the quotes, and
// you must add a `TokenKind::Str` variant.
// An unterminated literal runs to end of input and lexes as Unknown.
#[test]
#[ignore = "extension 3: implement string literals"]
fn string_literals() {
    // After adding the variant, change these to TokenKind::Str.
    let toks = Lexer::tokenize(r#"a "hello world" b"#);
    assert_eq!(toks.len(), 3);
    assert_eq!(toks[1].text, "\"hello world\"");
    assert_eq!(format!("{:?}", toks[1].kind), "Str");

    let bad = Lexer::tokenize("\"unterminated");
    assert_eq!(bad.len(), 1);
    assert_eq!(bad[0].kind, TokenKind::Unknown);
}

// --- 4. Keywords ------------------------------------------------------------
// `let`, `if`, `else`, `fn` lex as TokenKind::Keyword, not Ident.
// `lettuce` and `iff` are still Idents.
#[test]
#[ignore = "extension 4: implement keywords"]
fn keywords_are_distinguished_from_identifiers() {
    let toks = Lexer::tokenize("let lettuce if iff");
    assert_eq!(format!("{:?}", toks[0].kind), "Keyword");
    assert_eq!(toks[1].kind, TokenKind::Ident);
    assert_eq!(format!("{:?}", toks[2].kind), "Keyword");
    assert_eq!(toks[3].kind, TokenKind::Ident);
}

// --- 5. Two-character operators ---------------------------------------------
// `==`, `!=`, `<=`, `>=`, `->` are single tokens. `=` and `<` stay single.
// This is where "longest match wins" starts to matter.
#[test]
#[ignore = "extension 5: implement multi-character operators"]
fn longest_match_wins() {
    assert_eq!(texts("a == b"), ["a", "==", "b"]);
    assert_eq!(texts("a = b"), ["a", "=", "b"]);
    assert_eq!(texts("x -> y"), ["x", "->", "y"]);
    assert_eq!(texts("x - > y"), ["x", "-", ">", "y"]);
    assert_eq!(texts("a <= b != c"), ["a", "<=", "b", "!=", "c"]);
}

// --- 6. Line/column reporting -----------------------------------------------
// Add `Lexer::line_col(&self, offset: usize) -> (usize, usize)`, 1-based.
// This is what every compiler needs for error messages.
// Hint: you can compute it from `self.src` and the offset — no extra state.
#[test]
#[ignore = "extension 6: implement line_col"]
fn line_and_column_from_offset() {
    let src = "ab\ncd\n  ef";
    let lx = Lexer::new(src);
    // Uncomment once implemented:
    // assert_eq!(lx.line_col(0), (1, 1));   // 'a'
    // assert_eq!(lx.line_col(3), (2, 1));   // 'c'
    // assert_eq!(lx.line_col(8), (3, 3));   // 'e'
    let _ = lx;
    panic!("remove this line once line_col is implemented and asserted");
}

// --- 7. Keep the invariant --------------------------------------------------
// After ALL of the above, this must still hold.
#[test]
#[ignore = "extension 7: run last, after 1-6"]
fn still_zero_copy_after_every_extension() {
    let src = String::from("let x = 12.5 # comment\nif x >= \"s\" -> y");
    let base = src.as_ptr() as usize;
    let end = base + src.len();

    for t in Lexer::tokenize(&src) {
        let p = t.text.as_ptr() as usize;
        assert!(
            p >= base && p + t.text.len() <= end,
            "token {:?} escaped the source buffer — you allocated",
            t.text
        );
    }
}
