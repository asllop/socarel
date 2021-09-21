use std::collections::HashMap as Map;
use crate::node::*;

//---- Structs ----//

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
pub struct Tree<T: NodeContent = RawNode> {
    /// Tree nodes.
    nodes: Vec<Node<T>>,
    /// Tree levels.
    levels: Vec<TreeLevel>
}

/// A forest is a set of trees.
#[derive(Debug)]
pub struct Forest<T: NodeContent = RawNode> {
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
                current_root.set_content(node.get_content());
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
            let new_node_level = self.nodes[parent_node_index].get_level() + 1;
            if let Some(mut new_node) = Node::<T>::new_node(node_content, new_node_level) {
                // Update new node, set parent_position and parents_children_pos
                new_node.set_parent_position(parent_node_index);
                let parents_children_pos = self.nodes[parent_node_index].get_num_chuildren();
                new_node.set_parents_children_pos(parents_children_pos);
                // Add new node to nodes array, to parent's children array and to child_map
                let new_node_index = self.nodes.len();
                self.nodes.push(new_node);
                self.nodes[parent_node_index].add_child(node_content, new_node_index);
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

impl<T: NodeContent> Forest<T> {
    /// Create an empty forest.
    pub fn new() -> Self {
        Self {
            trees: Map::new()
        }
    }

    /// Create new empty tree.
    /// 
    /// # Aeguments
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

    /// Remove tree from the forest.
    /// 
    /// # Aeguments
    /// 
    /// * `name` - Tree name.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the removed tree.
    /// 
    pub fn remove_tree(&mut self, name: &str) -> Option<Tree<T>> {
        return self.trees.remove(name);
    }

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