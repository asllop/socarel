use std::collections::HashMap as Map;
use std::collections::hash_map::Iter;
use crate::node::*;
use crate::tree::*;
use crate::error::*;

/// Generate a default implementation for [`TreeIdentifier`] dependency traits (PartialEq, Eq and Hash).
/// 
/// # Example
/// 
/// ```
/// # use socarel::*;
/// struct MyTreeId(String);
/// 
/// impl TreeIdentifier for MyTreeId {
///     fn new(tree_id: &str) -> Result<Self, SocarelError> {
///         Ok(Self(String::from(tree_id)))
///     }
///     fn get_id(&self) -> &str {
///         &self.0
///     }
/// }
/// 
/// impl_tree_id_traits!(MyTreeId);
/// ```
#[macro_export]
macro_rules! impl_tree_id_traits {
    ( $x:ty ) => {
        impl std::cmp::PartialEq for $x {
            fn eq(&self, other: &Self) -> bool {
                self.get_id() == other.get_id()
            }
        }
        
        impl std::cmp::Eq for $x {}
        
        impl std::hash::Hash for $x {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.get_id().hash(state);
            }
        }
    };
}

/// Trait to define structs that model a tree ID
pub trait TreeIdentifier: std::cmp::Eq + std::hash::Hash {
    /// Constructor.
    /// 
    /// # Arguments
    /// 
    /// * `tree_id` - Tree ID.
    /// 
    /// # Return
    /// 
    /// * Tree ID.
    /// 
    fn new(tree_id: &str) -> Result<Self, SocarelError> where Self: Sized;
    
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
    fn gen_tree_id(&self) -> String {
        String::from(self.get_id())
    }
}

/// Default [`TreeIdentifier`] struct.
/// 
/// It simply holds the tree ID as is, without parsing or modifying it.
#[derive(Debug)]
pub struct RawTreeId(String);

impl TreeIdentifier for RawTreeId {
    fn new(tree_id: &str) -> Result<Self, SocarelError> {
        Ok(Self(String::from(tree_id)))
    }

    fn get_id(&self) -> &str {
        &self.0
    }
}

impl_tree_id_traits!(RawTreeId);

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
    pub fn new_tree(&mut self, name: &str) -> Result<(), SocarelError> {
        self.add_tree(name, Tree::new())
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
    pub fn add_tree(&mut self, name: &str, tree: Tree<T>) -> Result<(), SocarelError> {
        let tid = I::new(name)?;
        if !self.trees.contains_key(&tid) {
            self.trees.insert(tid, tree);
            Ok(())
        }
        else {
            Err(SocarelError::new("Tree ID already exist", 1, SocarelErrorType::Forest))
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
    /// * Removed tree.
    /// 
    pub fn remove_tree(&mut self, name: &str) -> Result<Tree<T>, SocarelError> {
        let tid = I::new(name)?;
        if let Some(t) = self.trees.remove(&tid) {
            Ok(t)
        }
        else {
            Err(SocarelError::new("Could not remove tree", 2, SocarelErrorType::Forest))
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
    /// * Tree reference.
    /// 
    pub fn get_tree(&self, name: &str) -> Result<&Tree<T>, SocarelError> {
        let tid = I::new(name)?;
        if let Some(t) = self.trees.get(&tid) {
            Ok(t)
        }
        else {
            Err(SocarelError::new("Could not get tree", 3, SocarelErrorType::Forest))
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
    /// * Mut tree reference.
    /// 
    pub fn get_mut_tree(&mut self, name: &str) -> Result<&mut Tree<T>, SocarelError> {
        let tid = I::new(name)?;
        if let Some(t) = self.trees.get_mut(&tid) {
            Ok(t)
        }
        else {
            Err(SocarelError::new("Could not get mutable tree", 4, SocarelErrorType::Forest))
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