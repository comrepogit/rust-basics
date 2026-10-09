//! An index-based arena.
//!
//! Why this exists: Lesson 4 noted that a tree of `&'a Node<'a>` (or of
//! `Box<Node>` with parent pointers) fights the borrow checker. The standard
//! answer in real Rust infrastructure — rustc, Cranelift, every ECS, most
//! graph code — is to stop using pointers and use **indices into a `Vec`**.
//!
//! What you get:
//!   * no lifetimes in the node type at all
//!   * `Id` is `Copy`, 4 bytes, and can be stored anywhere
//!   * nodes are contiguous, so walking the tree is cache-friendly
//!   * freeing the whole structure is one `Vec` drop
//!
//! What you give up:
//!   * an `Id` from one arena used on another is a logic bug the compiler
//!     cannot catch (Phase 8 shows how `PhantomData` fixes even that)
//!   * you pass the arena everywhere you want to dereference an `Id`

/// A handle to a value inside an `Arena`.
///
/// This is the newtype pattern: a `u32` wearing a distinct type so it cannot
/// be confused with any other `u32` in the program. It compiles to exactly a
/// `u32` — no wrapper, no indirection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id(u32);

impl Id {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug)]
pub struct Arena<T> {
    items: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Arena { items: Vec::new() }
    }

    pub fn with_capacity(n: usize) -> Self {
        Arena {
            items: Vec::with_capacity(n),
        }
    }

    /// Move `value` into the arena and return a handle to it.
    pub fn alloc(&mut self, value: T) -> Id {
        let id = Id(self.items.len() as u32);
        self.items.push(value);
        id
    }

    /// Panics if `id` came from a different arena and is out of range.
    pub fn get(&self, id: Id) -> &T {
        &self.items[id.index()]
    }

    pub fn get_mut(&mut self, id: Id) -> &mut T {
        &mut self.items[id.index()]
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_and_get() {
        let mut a: Arena<String> = Arena::new();
        let x = a.alloc(String::from("first"));
        let y = a.alloc(String::from("second"));

        assert_eq!(a.get(x), "first");
        assert_eq!(a.get(y), "second");
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn ids_are_stable_across_growth() {
        // The crucial property. With `&T` into a Vec, pushing could
        // reallocate and invalidate the reference — which is exactly the
        // iterator-invalidation error from Lesson 2. An Id is just a number,
        // so it survives any amount of growth.
        let mut a: Arena<u64> = Arena::new();
        let first = a.alloc(7);
        for i in 0..10_000 {
            a.alloc(i);
        }
        assert_eq!(*a.get(first), 7);
    }

    #[test]
    fn id_is_the_size_of_a_u32() {
        assert_eq!(std::mem::size_of::<Id>(), std::mem::size_of::<u32>());
    }

    #[test]
    fn get_mut_mutates_in_place() {
        let mut a: Arena<i32> = Arena::new();
        let id = a.alloc(1);
        *a.get_mut(id) += 41;
        assert_eq!(*a.get(id), 42);
    }
}
