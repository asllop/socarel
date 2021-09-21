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
    content: T,
    /// Nodel level.
    level: usize,
    /// Parent node index in the tree array.
    parent_position: Option<usize>,
    // Map of content/node index, to find a child by name.
    child_map: Map<String, usize>,
    /// Index of current node in the parent [`children`][`Node::children`] array.
    parents_children_pos: Option<usize>,
    /// Array that contains indexes of of children nodes.
    children: Vec<usize>
}

/// Struct that contains tree levels information.
#[derive(Debug)]
pub struct TreeLevel {
    /// Tree level.
    level: usize,
    /// Nodes of the tree level. Positions within the [`nodes`][`Tree::nodes`] array.
    node_positions: Vec<usize>
}

/// Struct that contains a tree.
#[derive(Debug)]
pub struct Tree<T: NodeContent> {
    /// Tree nodes.
    nodes: Vec<Node<T>>,
    /// Tree levels.
    levels: Vec<TreeLevel>
}

/// A forest is a set of trees.
#[derive(Debug)]
pub struct Forest<T: NodeContent> {
    /// Map with all the trees contained in the Forest.
    trees: Map<String, Tree<T>>
}

//---- Implementations ----//

impl<T: NodeContent> Tree<T> {
    /// Create new empty tree.
    pub fn new() -> Self {
        Self {
            nodes: vec!(),
            levels: vec!()
        }
    }

    /// Set root node.
    /// 
    /// # Aeguments
    /// 
    /// * `content` - Node content.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the root node index (always 0).
    ///
    pub fn set_root(&mut self, node_content: &str) -> Option<usize> {
        if let Some(node) = Node::<T>::new_root(node_content) {
            if self.nodes.len() == 0 {
                self.nodes.push(node);
                self.add_to_level(1, 0);
            }
            else {
                let current_root = self.nodes.get_mut(0).unwrap();
                current_root.content = node.content;
            }
            Some(0)
        }
        else {
            None
        }
    }

    /// Create new node and link it to its parent.
    /// 
    /// # Aeguments
    /// 
    /// * `node_content` - Node content.
    /// * `parent_node_index` - Parent node index.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the new node index.
    ///
    pub fn link_node(&mut self, node_content: &str, parent_node_index: usize) -> Option<usize> {
        if parent_node_index < self.nodes.len() {
            let new_node_level = self.nodes[parent_node_index].level + 1;
            if let Some(mut new_node) = Node::<T>::new_node(node_content, new_node_level) {
                // Update new node, set parent_position and parents_children_pos
                new_node.parent_position = Some(parent_node_index);
                let parents_children_pos = self.nodes[parent_node_index].children.len();
                new_node.parents_children_pos = Some(parents_children_pos);
                // Add new node to nodes array, to parent's children array and to child_map
                let new_node_index = self.nodes.len();
                self.nodes.push(new_node);
                self.nodes[parent_node_index].children.push(new_node_index);
                self.nodes[parent_node_index].child_map.insert(String::from(node_content), new_node_index);
                self.add_to_level(new_node_level, new_node_index);
                return Some(new_node_index);
            }
        }
        None
    }
    
    //TODO: set_node (overwrite content, it must exist)
    //TODO: unlink_node (careful with levels!)
    //TODO: find_node (use `Node::child_map`)

    fn add_to_level(&mut self, level: usize, node_index: usize) -> Option<usize> {
        if level <= self.levels.len() {
            // There is a pos for this level, add node_index
            self.levels[level - 1].node_positions.push(node_index);
            Some(self.levels.len())
        }
        else if level - 1 == self.levels.len() {
            // No pos for this level but we can create it
            self.levels.push(TreeLevel {
                level,
                node_positions: vec!(node_index)
            });
            Some(self.levels.len())
        }
        else {
            // Error
            None
        }
    }
}

impl<T: NodeContent> Node<T> {
    /// Create new root node.
    /// 
    /// # Aeguments
    /// 
    /// * `content` - Node content.
    /// 
    /// # Return
    /// 
    /// * Node struct or None if content parsing fails.
    /// 
    pub fn new_root(content: &str) -> Option<Self> {
        Self::new_node(content, 1)
    }

    /// Create new node.
    /// 
    /// # Aeguments
    /// 
    /// * `content` - Node content.
    /// * `level` - Node level.
    /// 
    /// # Return
    /// 
    /// * Node struct or None if content parsing fails.
    /// 
    pub fn new_node(content: &str, level: usize) -> Option<Self> {
        if let Some(content_node) = NodeContent::new(content) {
            Some(
                Node {
                    content: content_node,
                    level,
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