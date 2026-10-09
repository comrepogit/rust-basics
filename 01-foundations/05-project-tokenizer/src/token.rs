//! Tokens.
//!
//! The important line in this file is the lifetime on `Token<'src>`.

/// What kind of thing a token is.
///
/// This is a plain C-style enum — every variant is a bare tag with no data,
/// exactly like `enum TokenKind { IDENT, NUMBER, ... };` in C. Phase 2 shows
/// what Rust enums can really do (variants carrying different payloads, which
/// is what makes `Option` and `Result` work).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Ident,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Comma,
    /// Any character we do not recognise. A real lexer would report an error
    /// here; `Result` is Phase 2.
    Unknown,
}

/// A single token.
///
/// `text` does **not** own a string. It borrows a run of bytes out of the
/// source text, which is why the struct carries a lifetime parameter: a
/// `Token<'src>` may not outlive the `&'src str` it was lexed from.
///
/// Because every field is `Copy` (a `&str` is just ptr+len), `Token` itself
/// is `Copy`. Passing one around costs two moves of a register pair and
/// never touches the allocator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'src> {
    pub kind: TokenKind,
    /// The exact source text of this token, borrowed from the input.
    pub text: &'src str,
    /// Byte offset of `text` within the original source.
    pub offset: usize,
}

impl<'src> Token<'src> {
    /// Byte range this token occupies in the source.
    pub fn span(&self) -> (usize, usize) {
        (self.offset, self.offset + self.text.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_is_copy_and_small() {
        // If this ever fails, someone put an owned String in Token.
        fn assert_copy<T: Copy>() {}
        assert_copy::<Token<'_>>();
    }

    #[test]
    fn span_is_offset_plus_len() {
        let t = Token {
            kind: TokenKind::Ident,
            text: "abc",
            offset: 10,
        };
        assert_eq!(t.span(), (10, 13));
    }
}
