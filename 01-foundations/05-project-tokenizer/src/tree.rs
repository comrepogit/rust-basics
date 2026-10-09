//! A tree built out of arena handles.
//!
//! Compare the two shapes:
//!
//! ```text
//! struct Node<'a> { token: Token<'a>, kids: Vec<&'a Node<'a>> }   // pain
//! struct Node<'a> { token: Token<'a>, kids: Vec<Id> }             // fine
//! ```
//!
//! The second has no self-reference, so no lifetime relates a node to another
//! node. The only lifetime left is `'src`, from the borrowed token text.

use crate::arena::{Arena, Id};
use crate::token::Token;

#[derive(Debug)]
pub struct Node<'src> {
    pub token: Token<'src>,
    pub kids: Vec<Id>,
}

pub type Tree<'src> = Arena<Node<'src>>;

/// Add a leaf node.
pub fn leaf<'src>(tree: &mut Tree<'src>, token: Token<'src>) -> Id {
    tree.alloc(Node {
        token,
        kids: Vec::new(),
    })
}

/// Add an internal node with the given children.
pub fn branch<'src>(tree: &mut Tree<'src>, token: Token<'src>, kids: Vec<Id>) -> Id {
    tree.alloc(Node { token, kids })
}

/// Render the tree as an S-expression: `(+ 1 (* 2 3))`.
///
/// Note how the recursion takes `&Tree` and an `Id` rather than a `&Node`.
/// That is the arena trade-off made concrete: you carry the arena with you.
pub fn to_sexpr(tree: &Tree<'_>, root: Id) -> String {
    let node = tree.get(root);
    if node.kids.is_empty() {
        return node.token.text.to_string();
    }

    let mut out = String::from("(");
    out.push_str(node.token.text);
    for &kid in &node.kids {
        out.push(' ');
        out.push_str(&to_sexpr(tree, kid));
    }
    out.push(')');
    out
}

/// Number of nodes in the subtree rooted at `root`.
pub fn count(tree: &Tree<'_>, root: Id) -> usize {
    1 + tree
        .get(root)
        .kids
        .iter()
        .map(|&k| count(tree, k))
        .sum::<usize>()
}

/// Maximum depth of the subtree rooted at `root` (a leaf has depth 1).
pub fn depth(tree: &Tree<'_>, root: Id) -> usize {
    1 + tree
        .get(root)
        .kids
        .iter()
        .map(|&k| depth(tree, k))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    /// Build `1 + 2 * 3` by hand. (Writing the parser that does this for you
    /// is the Phase 2 project.)
    fn build<'src>(toks: &[Token<'src>]) -> (Tree<'src>, Id) {
        // toks: 1 + 2 * 3
        let mut tree = Tree::new();
        let one = leaf(&mut tree, toks[0]);
        let two = leaf(&mut tree, toks[2]);
        let three = leaf(&mut tree, toks[4]);
        let mul = branch(&mut tree, toks[3], vec![two, three]);
        let add = branch(&mut tree, toks[1], vec![one, mul]);
        (tree, add)
    }

    #[test]
    fn sexpr_round_trip() {
        let src = "1 + 2 * 3";
        let toks = Lexer::tokenize(src);
        let (tree, root) = build(&toks);

        assert_eq!(to_sexpr(&tree, root), "(+ 1 (* 2 3))");
        assert_eq!(count(&tree, root), 5);
        assert_eq!(depth(&tree, root), 3);
    }

    #[test]
    fn tree_outlives_the_lexer_but_not_the_source() {
        let src = String::from("1 + 2 * 3");
        let toks = Lexer::tokenize(&src);
        let (tree, root) = build(&toks);
        drop(toks); // tokens gone
        assert_eq!(to_sexpr(&tree, root), "(+ 1 (* 2 3))");
        // `src` must still be alive here — and it is, by the type system.
    }
}
