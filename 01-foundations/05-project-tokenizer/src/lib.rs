//! # Phase 1 project — zero-allocation tokenizer + index arena
//!
//! Everything in Phase 1 shows up here at once:
//!
//! | Lesson | Where it appears |
//! |---|---|
//! | 1 — ownership | `Arena::alloc` takes `T` by value and owns it |
//! | 2 — borrowing | `tokenize(&self)` vs `next_token(&mut self)` |
//! | 3 — slices | every `Token.text` is a `&str` into the source |
//! | 4 — lifetimes | `Token<'src>`, and `-> Option<Token<'src>>` not `Option<Token<'_>>` |
//!
//! ## Quick start
//!
//! ```
//! use tokenizer::lexer::Lexer;
//! use tokenizer::token::TokenKind;
//!
//! let src = "x1 + 42";
//! let toks = Lexer::tokenize(src);
//!
//! assert_eq!(toks.len(), 3);
//! assert_eq!(toks[0].text, "x1");
//! assert_eq!(toks[0].kind, TokenKind::Ident);
//!
//! // The token text is not a copy — it points INTO `src`.
//! assert!(std::ptr::eq(toks[0].text.as_ptr(), src.as_ptr()));
//! ```

pub mod arena;
pub mod lexer;
pub mod token;
pub mod tree;

pub use arena::{Arena, Id};
pub use lexer::Lexer;
pub use token::{Token, TokenKind};
