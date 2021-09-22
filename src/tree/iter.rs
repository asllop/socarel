use crate::tree::*;

/// Interface for tree iterators.
pub struct IterInterface<'a, T: NodeContent> {
    tree: &'a Tree<T>
}

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
    /// * Iterator.
    ///
    pub fn sequential(&self) -> SequentialIter<'a, T> {
        SequentialIter::new(self.tree)
    }

    /// Get inverse sequential iterator.
    /// 
    /// Iterates over the array that contains the nodes in inverted sequential order. Even unlinked nodes.
    /// 
    /// # Return
    /// 
    /// * Iterator.
    ///
    pub fn inv_sequential(&self) -> InvSequentialIter<'a, T> {
        InvSequentialIter::new(self.tree)
    }

    /// Get BFS iterator. It uses the levels structure.
    /// 
    /// # Return
    /// 
    /// * Iterator.
    ///
    pub fn bfs(&self) -> BfsIter<'a, T> {
        BfsIter::new(self.tree)
    }

    /// Get Inverse BFS iterator. It uses the levels structure.
    /// 
    /// # Return
    /// 
    /// * Iterator.
    ///
    pub fn inv_bfs(&self) -> InvBfsIter<'a, T> {
        InvBfsIter::new(self.tree)
    }

    /// Get Pre-Order DFS iterator
    /// 
    /// # Return
    /// 
    /// * Iterator.
    ///
    pub fn pre_dfs(&self) -> PreDfsIter<'a, T> {
        PreDfsIter::new(self.tree)
    }
}

/// Simple Iterator, in sequential order.
pub struct SequentialIter<'a, T: NodeContent> {
    tree: &'a Tree<T>,
    position: usize
}

impl<'a, T: NodeContent> SequentialIter<'a, T> {
    pub fn new(tree: &'a Tree<T>) -> Self {
        Self {
            tree,
            position: 0
        }
    }
}

impl<'a, T: NodeContent> Iterator for SequentialIter<'a, T> {
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

/// Simple Iterator, in inverted sequential order.
pub struct InvSequentialIter<'a, T: NodeContent> {
    tree: &'a Tree<T>,
    position: usize,
    finished: bool
}

impl<'a, T: NodeContent> InvSequentialIter<'a, T> {
    pub fn new(tree: &'a Tree<T>) -> Self {
        if tree.nodes.len() > 0 {
            Self {
                tree,
                position: tree.nodes.len() - 1,
                finished: false
            }
        }
        else {
            Self {
                tree,
                position: 0,
                finished: true
            }
        }
    }
}

impl<'a, 'b, T: NodeContent> Iterator for InvSequentialIter<'a, T> {
    type Item = (&'a Node<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let position = self.position;
        match &self.tree.nodes.get(self.position) {
            Some(node) => {
                if self.position > 0 {
                    self.position -= 1;
                }
                else {
                    self.finished = true;
                }
                Some((node, position))
            },
            None => None
        }
    }
}

/// BFS Iterator, uses levels structure.
pub struct BfsIter<'a, T: NodeContent> {
    tree: &'a Tree<T>,
    position: usize,
    sub_position: usize
}

impl<'a, T: NodeContent> BfsIter<'a, T> {
    pub fn new(tree: &'a Tree<T>) -> Self {
        Self {
            tree,
            position: 0,
            sub_position: 0
        }
    }
}

impl<'a, T: NodeContent> Iterator for BfsIter<'a, T> {
    type Item = (&'a Node<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tree_level) = self.tree.levels.get(self.position) {
            if let Some(node_position) = tree_level.node_positions.get(self.sub_position) {
                self.sub_position += 1;
                let position = *node_position as usize;
                return match self.tree.nodes.get(position) {
                    Some(n) => Some((n, position)),
                    None => None
                };
            }
            else {
                self.position += 1;
                self.sub_position = 0;                    
                return self.next();
            }
        }
        None
    }
}

/// Inverse BFS Iterator, uses levels structure.
pub struct InvBfsIter<'a, T: NodeContent> {
    tree: &'a Tree<T>,
    position: usize,
    sub_position: usize
}

impl<'a, T: NodeContent> InvBfsIter<'a, T> {
    pub fn new(tree: &'a Tree<T>) -> Self {
        Self {
            tree,
            position: tree.levels.len() - 1,
            sub_position: 0
        }
    }
}

impl<'a, T: NodeContent> Iterator for InvBfsIter<'a, T> {
    type Item = (&'a Node<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tree_level) = self.tree.levels.get(self.position) {
            if let Some(node_position) = tree_level.node_positions.get(self.sub_position) {
                self.sub_position += 1;
                let position = *node_position as usize;
                return match self.tree.nodes.get(position) {
                    Some(n) => Some((n, position)),
                    None => None
                };
            }
            else {
                if self.position == 0 {
                    return None;
                }
                self.position -= 1;
                self.sub_position = 0;                    
                return self.next();
            }
        }
        None
    }
}

/// Pre-Order DFS Iterator
pub struct PreDfsIter<'a, T: NodeContent> {
    tree: &'a Tree<T>,
    pila: Vec<usize>,
    next: usize,
    finished: bool
}

impl<'a, T: NodeContent> PreDfsIter<'a, T> {
    pub fn new(tree: &'a Tree<T>) -> Self {
        Self {
            tree,
            pila: vec!(),
            next: 0,
            finished: false
        }
    }
}

impl<'a, T: NodeContent> Iterator for PreDfsIter<'a, T> {
    type Item = (&'a Node<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        // Get current node
        let position = self.next;
        if let Some(node) = self.tree.nodes.get(position) {
            // Put in the stack all children of current node
            for child in node.get_children_ref().iter().rev() {
                self.pila.push(*child);
            }
            // Get next node from stack.
            if let Some(next_node_index) = self.pila.pop() {
                self.next = next_node_index;
            }
            else {
                // If nothing in stack, end
                self.finished = true;
            }
            // Return current node
            Some((node, position))
        }
        else {
            None
        }

    }
}
