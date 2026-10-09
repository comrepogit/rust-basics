//! Run me:  cargo run --bin demo
//!
//! Or with your own input:  cargo run --bin demo -- "1 + foo * (2 - bar)"

use tokenizer::arena::Arena;
use tokenizer::lexer::Lexer;
use tokenizer::tree::{self, Node};

fn main() {
    let src: String = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("total = price * qty + shipping_2"));

    println!("source: {src:?}\n");

    // --- part 1: the lexer ---------------------------------------------------
    println!("{:<4} {:<8} {:>6}  {}", "#", "KIND", "SPAN", "TEXT");
    println!("{}", "-".repeat(40));

    let toks = Lexer::tokenize(&src);
    for (i, t) in toks.iter().enumerate() {
        let (a, b) = t.span();
        println!("{:<4} {:<8?} {:>2}..{:<2}  {:?}", i, t.kind, a, b, t.text);
    }

    // --- part 2: the zero-copy proof ----------------------------------------
    println!("\nevery token's text points into the source buffer:");
    let base = src.as_ptr() as usize;
    for t in toks.iter().take(4) {
        let off = t.text.as_ptr() as usize - base;
        println!("  {:>8?}  src_ptr + {}", t.text, off);
    }
    println!("  ...no heap allocation was performed per token.");

    // --- part 3: the arena ---------------------------------------------------
    // Build a flat tree: a synthetic root with every token as a child.
    let mut tree: Arena<Node<'_>> = Arena::with_capacity(toks.len() + 1);
    let kids: Vec<_> = toks.iter().map(|&t| tree::leaf(&mut tree, t)).collect();

    if let Some(&first) = toks.first() {
        let root = tree::branch(&mut tree, first, kids);
        println!("\narena: {} nodes", tree.len());
        println!("depth: {}", tree::depth(&tree, root));
        println!("sexpr: {}", tree::to_sexpr(&tree, root));
    }
}
