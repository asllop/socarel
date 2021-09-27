use std::collections::HashMap as Map;
use std::collections::hash_map::Iter;
use crate::node::*;
use crate::tree::*;

//---- Structs ----//

/// A forest is a set of trees.
#[derive(Debug)]
pub struct Forest<T: NodeContent = RawNode> {
    /// Map with all the trees contained in the Forest.
    trees: Map<String, Tree<T>>
}

//---- Implementations ----//

impl<T: NodeContent> Forest<T> {
    /// Create an empty forest.
    pub fn new() -> Self {
        Self {
            trees: Map::new()
        }
    }

    /// Create new empty tree.
    /// 
    /// # Arguments
    /// 
    /// * `name` - Tree name.
    /// 
    /// # Return
    /// 
    /// * Nothing.
    /// 
    pub fn new_tree(&mut self, name: &str) {
        self.add_tree(name, Tree::new());
    }

    /// Add a tree to forest.
    /// 
    /// # Arguments
    /// 
    /// * `name` - Tree name.
    /// * `forest` - Tree struct.
    /// 
    /// # Return
    /// 
    /// * Nothing.
    /// 
    pub fn add_tree(&mut self, name: &str, tree: Tree<T>) {
        self.trees.insert(String::from(name), tree);
    }

    /// Remove tree from the forest.
    /// 
    /// # Arguments
    /// 
    /// * `name` - Tree name.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the removed tree.
    /// 
    pub fn remove_tree(&mut self, name: &str) -> Option<Tree<T>> {
        self.trees.remove(name)
    }

    /// Get tree reference.
    /// 
    /// # Arguments
    /// 
    /// * `name` - Tree name.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the tree reference.
    /// 
    pub fn get_tree(&self, name: &str) -> Option<&Tree<T>> {
        self.trees.get(name)
    }

    /// Get mutable tree reference.
    /// 
    /// # Arguments
    /// 
    /// * `name` - Tree name.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the mut tree reference.
    /// 
    pub fn get_mut_tree(&mut self, name: &str) -> Option<&mut Tree<T>> {
        self.trees.get_mut(name)
    }

    /// Get forest iterator.
    /// 
    /// # Return
    /// 
    /// * Iterator, provides a tuple with tree_name<[`String`]>, tree_struct<[`Tree`]>.
    /// 
    pub fn iter(&self) -> Iter<String, Tree<T>> {
        self.trees.iter()
    }
}