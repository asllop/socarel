use crate::tree::*;

//---- Structs ----//

/// Interface for tree iterators.
pub struct IterInterface<'a, T: NodeContent> {
    tree: &'a Tree<T>
}

/// Simple Iterator, in sequential order.
pub struct TreeIterSequential<'a, T: NodeContent> {
    tree: &'a Tree<T>,
    position: usize
}

//---- Implementations ----//

impl<'a, T: NodeContent> IterInterface<'a, T> {
    /// Create iterator interface.
    /// 
    /// # Arguments
    /// 
    /// * `tree` - Reference to tree.
    /// 
    /// # Return
    /// 
    /// * Iterator interface.
    ///
    pub fn new(tree: &'a Tree<T>) -> Self {
        IterInterface { tree }
    }

    /// Get sequential iterator.
    /// 
    /// Iterates over the array that contains the nodes in sequential order. Even unlinked nodes.
    /// 
    /// # Return
    /// 
    /// * Sequential iterator.
    ///
    pub fn sequential(&self) -> TreeIterSequential<'a, T> {
        TreeIterSequential::new(self.tree)
    }
}

impl<'a, T: NodeContent> TreeIterSequential<'a, T> {
    pub fn new(tree: &'a Tree<T>) -> Self {
        Self {
            tree,
            position: 0
        }
    }
}

impl<'a, T: NodeContent> Iterator for TreeIterSequential<'a, T> {
    type Item = (&'a Node<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position;
        match &self.tree.nodes.get(self.position) {
            Some(node) => {
                self.position += 1;
                Some((node, position))
            },
            None => None
        }
    }
}