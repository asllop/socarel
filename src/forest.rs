use std::collections::HashMap as Map;
use std::collections::hash_map::Iter;
use crate::node::*;
use crate::tree::*;

/// Trait to define structs that model a tree ID
pub trait TreeIdentifier: std::cmp::Eq + std::hash::Hash + std::fmt::Display {
    //TODO return a Result with wither Self or an error (impl Error trait).
    /// Constructor.
    /// 
    /// # Arguments
    /// 
    /// * `tree_id` - Tree ID.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the tree ID.
    /// 
    fn new(tree_id: &str) -> Option<Self> where Self: Sized;
    
    /// Get tree ID value.
    /// 
    /// # Return
    /// 
    /// * ID value.
    ///
    fn get_id(&self) -> &str;

    /// Generate tree ID.
    /// 
    /// Use by serializers to create back the string of a tree ID that is parsed by a TreeIdentifier implementer.
    /// 
    /// # Return
    /// 
    /// * Tree ID.
    ///
    fn gen_tree_id(&self) -> String;
}

/// Default [`TreeIdentifier`] struct.
/// 
/// It simply holds the tree ID as is, without parsing or modifying it.
#[derive(Debug)]
pub struct RawTreeId {
    tree_id: String
}

impl TreeIdentifier for RawTreeId {
    fn new(tree_id: &str) -> Option<Self> {
        Some(
            Self {
                tree_id: String::from(tree_id)
            }
        )
    }

    fn get_id(&self) -> &str {
        &self.tree_id
    }

    fn gen_tree_id(&self) -> String {
        String::from(self.get_id())
    }
}

//TODO: create a macro to implement PartialEq, Eq, Hash and Display for all `TreeIdentifier`s, because the implementation is always the same

impl std::cmp::PartialEq for RawTreeId {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl std::cmp::Eq for RawTreeId {}

impl std::hash::Hash for RawTreeId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_id().hash(state);
    }
}

impl std::fmt::Display for RawTreeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_id())
    }
}

//---- Structs ----//

/// A forest is a set of trees.
#[derive(Debug)]
pub struct Forest<I: TreeIdentifier = RawTreeId , T: NodeContent = RawNode> {
    /// Map with all the trees contained in the Forest.
    trees: Map<I, Tree<T>>
}

//---- Implementations ----//

impl<I: TreeIdentifier, T: NodeContent> Forest<I, T> {
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

    //TODO: return Result
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
        if let Some(tid) = I::new(name) {
            self.trees.insert(tid, tree);
        }
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
        if let Some(tid) = I::new(name) {
            self.trees.remove(&tid)
        }
        else {
            None
        }
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
        if let Some(tid) = I::new(name) {
            self.trees.get(&tid)
        }
        else {
            None
        }
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
        if let Some(tid) = I::new(name) {
            self.trees.get_mut(&tid)
        }
        else {
            None
        }
    }

    /// Get forest iterator.
    /// 
    /// # Return
    /// 
    /// * Iterator, provides a tuple with tree_name<[`String`]>, tree_struct<[`Tree`]>.
    /// 
    pub fn iter(&self) -> Iter<I, Tree<T>> {
        self.trees.iter()
    }
}