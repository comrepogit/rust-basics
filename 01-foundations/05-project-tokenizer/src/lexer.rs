//! A zero-allocation lexer.
//!
//! "Zero-allocation" is a literal claim: between `Lexer::new` and the last
//! `next_token`, the allocator is never called. Every token's text is a
//! borrowed slice of the input. `tests/zero_copy.rs` proves it by pointer
//! arithmetic.

use crate::token::{Token, TokenKind};

pub struct Lexer<'src> {
    src: &'src str,
    /// Byte offset of the next unconsumed character. Always on a UTF-8
    /// character boundary, which is why slicing with it never panics.
    pos: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Lexer { src, pos: 0 }
    }

    /// The unconsumed remainder.
    ///
    /// Note the return type: `&'src str`, not `&str`. Elision rule 3 would
    /// have tied this to `&self`, making the slice die when the borrow of the
    /// lexer ends. Tying it to `'src` instead says the truth — it points into
    /// the source text, which outlives the lexer.
    fn rest(&self) -> &'src str {
        &self.src[self.pos..]
    }

    /// Skip whitespace. (Comments are an exercise.)
    fn skip_trivia(&mut self) {
        let rest = self.rest();
        let trimmed = rest.trim_start();
        // trim_start only removes from the front, so the difference in
        // lengths is exactly how many bytes we skipped.
        self.pos += rest.len() - trimmed.len();
    }

    /// Produce the next token, or `None` at end of input.
    ///
    /// `Option<&'src str>` rather than `Option<&str>` is the whole trick: the
    /// caller may keep the token after dropping the lexer.
    pub fn next_token(&mut self) -> Option<Token<'src>> {
        self.skip_trivia();

        let start = self.pos;
        let rest = self.rest();
        let first = rest.chars().next()?; // None => end of input

        let (kind, len) = if first.is_ascii_digit() {
            // run of digits
            let len = rest
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(rest.len());
            (TokenKind::Number, len)
        } else if first.is_alphabetic() || first == '_' {
            // identifier: letter or _, then letters, digits, _
            let len = rest
                .find(|c: char| !(c.is_alphanumeric() || c == '_'))
                .unwrap_or(rest.len());
            (TokenKind::Ident, len)
        } else {
            let kind = match first {
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Star,
                '/' => TokenKind::Slash,
                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,
                ',' => TokenKind::Comma,
                _ => TokenKind::Unknown,
            };
            // len_utf8, not 1: a stray 'é' or '→' is one character, 2-3 bytes.
            // Using 1 here would split a character and panic on the slice.
            (kind, first.len_utf8())
        };

        self.pos = start + len;

        Some(Token {
            kind,
            text: &self.src[start..self.pos],
            offset: start,
        })
    }

    /// Collect everything. The `Vec` allocates; the tokens inside it do not.
    pub fn tokenize(src: &'src str) -> Vec<Token<'src>> {
        let mut lexer = Lexer::new(src);
        let mut out = Vec::new();
        while let Some(t) = lexer.next_token() {
            out.push(t);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<TokenKind> {
        Lexer::tokenize(src).into_iter().map(|t| t.kind).collect()
    }

    fn texts(src: &str) -> Vec<&str> {
        Lexer::tokenize(src).into_iter().map(|t| t.text).collect()
    }

    #[test]
    fn empty_input_yields_nothing() {
        assert!(Lexer::tokenize("").is_empty());
        assert!(Lexer::tokenize("   \t\n  ").is_empty());
    }

    #[test]
    fn single_tokens() {
        assert_eq!(kinds("abc"), [TokenKind::Ident]);
        assert_eq!(kinds("123"), [TokenKind::Number]);
        assert_eq!(kinds("+"), [TokenKind::Plus]);
        assert_eq!(kinds("?"), [TokenKind::Unknown]);
    }

    #[test]
    fn expression() {
        assert_eq!(
            kinds("x1 + 42 * (y - 7)"),
            [
                TokenKind::Ident,
                TokenKind::Plus,
                TokenKind::Number,
                TokenKind::Star,
                TokenKind::LParen,
                TokenKind::Ident,
                TokenKind::Minus,
                TokenKind::Number,
                TokenKind::RParen,
            ]
        );
        assert_eq!(texts("x1 + 42"), ["x1", "+", "42"]);
    }

    #[test]
    fn identifiers_may_contain_digits_and_underscores() {
        assert_eq!(texts("_a1_b2"), ["_a1_b2"]);
        assert_eq!(kinds("_a1_b2"), [TokenKind::Ident]);
    }

    #[test]
    fn offsets_are_byte_accurate() {
        let src = "ab  cd";
        let toks = Lexer::tokenize(src);
        assert_eq!(toks[0].span(), (0, 2));
        assert_eq!(toks[1].span(), (4, 6));
        assert_eq!(&src[toks[1].offset..toks[1].offset + 2], "cd");
    }

    #[test]
    fn multibyte_input_does_not_panic_or_split_characters() {
        // 'é' is 2 bytes, '→' is 3. A naive `len = 1` would slice mid-character
        // and panic. Identifiers are alphabetic, so 'é' lexes as an Ident and
        // '→' as Unknown.
        let toks = Lexer::tokenize("é → 1");
        assert_eq!(toks.len(), 3);
        assert_eq!(toks[0].text, "é");
        assert_eq!(toks[1].text, "→");
        assert_eq!(toks[1].kind, TokenKind::Unknown);
        assert_eq!(toks[2].text, "1");
    }

    #[test]
    fn tokens_outlive_the_lexer() {
        let src = String::from("keep me");
        let collected: Vec<Token<'_>> = {
            let mut lx = Lexer::new(&src);
            let mut v = Vec::new();
            while let Some(t) = lx.next_token() {
                v.push(t);
            }
            v // `lx` dropped here; tokens survive because they borrow `src`
        };
        assert_eq!(collected[0].text, "keep");
    }
}
