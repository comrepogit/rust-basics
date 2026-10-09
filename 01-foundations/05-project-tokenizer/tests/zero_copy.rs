//! Proof, not assertion: every token's text lives *inside* the source buffer.
//!
//! Run with:  cargo test --test zero_copy -- --nocapture

use tokenizer::{Lexer, TokenKind};

/// Is `slice` a view into `owner`'s buffer?
fn points_into(slice: &str, owner: &str) -> bool {
    let base = owner.as_ptr() as usize;
    let end = base + owner.len();
    let p = slice.as_ptr() as usize;
    p >= base && p + slice.len() <= end
}

#[test]
fn every_token_borrows_from_the_source() {
    let src = String::from("alpha + 123 * (beta_2 - 7), gamma");
    let toks = Lexer::tokenize(&src);

    assert!(!toks.is_empty());
    for t in &toks {
        assert!(
            points_into(t.text, &src),
            "token {:?} at offset {} is NOT a view into the source",
            t.text,
            t.offset
        );
    }
}

#[test]
fn offset_and_pointer_agree() {
    let src = "ab  cd ef";
    for t in Lexer::tokenize(src) {
        let from_offset = &src[t.offset..t.offset + t.text.len()];
        assert_eq!(from_offset, t.text);
        assert!(std::ptr::eq(from_offset.as_ptr(), t.text.as_ptr()));
    }
}

#[test]
fn lexing_allocates_nothing_per_token() {
    // Indirect but convincing: tokenize a large input and confirm the total
    // bytes "held" by the tokens exceeds what we could have allocated, while
    // every pointer still lands inside the one source buffer.
    let src = "foo + bar * 12345 - baz_9 , ".repeat(5_000);
    let toks = Lexer::tokenize(&src);

    assert_eq!(toks.len(), 5_000 * 8);
    // 20 bytes of token text per repetition, 5000 repetitions.
    let held: usize = toks.iter().map(|t| t.text.len()).sum();
    assert_eq!(held, 100_000);

    for t in &toks {
        assert!(points_into(t.text, &src));
    }
}

#[test]
fn token_struct_stays_small() {
    // kind (1 byte, padded) + &str (ptr + len) + offset.
    // If this grows, something started owning data.
    use std::mem::size_of;
    assert_eq!(size_of::<tokenizer::Token<'_>>(), 4 * size_of::<usize>());
}

#[test]
fn unknown_characters_do_not_derail_the_lexer() {
    let toks = Lexer::tokenize("a ? b");
    assert_eq!(toks.len(), 3);
    assert_eq!(toks[1].kind, TokenKind::Unknown);
    assert_eq!(toks[1].text, "?");
}
