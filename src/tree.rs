use std::collections::HashMap as Map;

//---- Structs ----//

/// Trait to define structs that model a node content.
pub trait NodeContent {
    /// Constructor.
    /// 
    /// # Aeguments
    /// 
    /// * `content` - Node content.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the node content.
    /// 
    fn new(content: &str) -> Option<Self> where Self: Sized;
    /// Get node content.
    /// 
    /// # Return
    /// 
    /// * Node content.
    ///
    fn get_content(&self) -> &str;
}

/// Default [`NodeContent`] struct.
/// 
/// It simply holds the content as is, without parsing or modifying it.
#[derive(Debug)]
pub struct RawNode {
    /// Node content.
    content: String
}

impl NodeContent for RawNode {
    fn new(content: &str) -> Option<Self> {
        Some(
            Self {
                content: String::from(content)
            }
        )
    }

    fn get_content(&self) -> &str {
        &self.content
    }
}

/// Struct that contains a tree node.
#[derive(Debug)]
pub struct Node<T: NodeContent> {
    /// Node content.
    pub content: T,
    /// Nodel level.
    pub level: usize,
    /// Parent node index in the tree array.
    pub parent_position: Option<usize>,
    // Map of content/node index, to find a child by name.
    pub child_map: Map<String, usize>,
    /// Index of current node in the parent [`children`][`Node::children`] array.
    pub parents_children_pos: Option<usize>,
    /// Array that contains indexes of of children nodes.
    pub children: Vec<usize>
}

/// Struct that contains tree levels information.
#[derive(Debug)]
pub struct TreeLevel {
    /// Tree level.
    pub level: usize,
    /// Nodes of the tree level. Positions within the [`nodes`][`Tree::nodes`] array.
    pub node_positions: Vec<usize>
}

/// Struct that contains a tree.
#[derive(Debug)]
pub struct Tree<T: NodeContent> {
    /// Tree nodes.
    pub nodes: Vec<Node<T>>,
    /// Tree levels.
    pub levels: Vec<TreeLevel>
}

/// A forest is a set of trees.
#[derive(Debug)]
pub struct Forest<T: NodeContent> {
    /// Map with all the trees contained in the Forest.
    pub trees: Map<String, Tree<T>>
}

//---- Implementations ----//

impl<T: NodeContent> Tree<T> {
    pub fn new() -> Self {
        Self {
            nodes: vec!(),
            levels: vec!()
        }
    }

    pub fn set_root(&mut self, node_content: &str) -> Option<usize> {
        if let Some(n) = Node::<T>::new_root(node_content) {
            if self.nodes.len() == 0 {
                self.nodes.push(n);
                self.levels.push(TreeLevel {
                    level: 1,
                    node_positions: vec!(0)
                });
            }
            else {
                let current_root = self.nodes.get_mut(0).unwrap();
                current_root.content = n.content;
            }
            Some(0)
        }
        else {
            None
        }
    }

    //TODO: link_node
    //TODO: set_node (overwrite content, it must exist)
    //TODO: unlink_node (careful with levels!)
    //TODO: find_node (use `Node::child_map`)
}

impl<T: NodeContent> Node<T> {
    pub fn new_root(content: &str) -> Option<Self> {
        if let Some(content_node) = NodeContent::new(content) {
            Some(
                Node {
                    content: content_node,
                    level: 1,
                    parent_position: None,
                    child_map: Map::new(),
                    parents_children_pos: None,
                    children: vec!()
                }
            )
        }
        else {
            None
        }
    }

    //TODO: new_node
}

impl<T: NodeContent> Forest<T> {
    /// Create an empty forest.
    pub fn new() -> Self {
        Self {
            trees: Map::new()
        }
    }

    /// Add a tree to forest.
    /// 
    /// # Aeguments
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

    //TODO: remove_tree

    /// Get tree reference.
    /// 
    /// # Aeguments
    /// 
    /// * `name` - Tree name.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the tree reference.
    /// 
    pub fn get_tree(&self, name: &str) -> Option<&Tree<T>> {
        if let Some(t) = self.trees.get(name) {
            Some(t)
        }
        else {
            None
        }
    }

    /// Get mutable tree reference.
    /// 
    /// # Aeguments
    /// 
    /// * `name` - Tree name.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the mut tree reference.
    /// 
    pub fn get_mut_tree(&mut self, name: &str) -> Option<&mut Tree<T>> {
        if let Some(t) = self.trees.get_mut(name) {
            Some(t)
        }
        else {
            None
        }
    }
}