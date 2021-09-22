use crate::node::*;
mod iter;

//---- Structs ----//

/// Struct that contains tree levels information. Used to speed-up some iterators.
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
    /// # Arguments
    /// 
    /// * `content` - Node content.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the root node index (always 0).
    ///
    pub fn set_root(&mut self, node_content: &str) -> Option<usize> {
        if let Some(mut node) = Node::<T>::new_root(node_content) {
            if self.nodes.len() == 0 {
                // Create root node
                let level_pos = self.add_to_level(1, 0).expect("Could not create level for root node");
                node.set_level_pos(level_pos);
                self.nodes.push(node);
                return Some(0);
            }
        }
        None
    }

    /// Create new node and link it to its parent.
    /// 
    /// # Arguments
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
                let level_pos = self.add_to_level(new_node_level, new_node_index).expect("Could not create level for node");
                new_node.set_level_pos(level_pos);
                let node_content = String::from(new_node.get_content_ref().get_val());
                self.nodes.push(new_node);
                self.nodes[parent_node_index].add_child(node_content, new_node_index);
                return Some(new_node_index);
            }
        }
        None
    }

    /// Get reference to node content.
    /// 
    /// # Arguments
    /// 
    /// * `node_index` - Node index.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the node content reference.
    /// 
    pub fn get_node_content(&self, node_index: usize) -> Option<&T> {
        if node_index < self.nodes.len() {
            return Some(self.nodes[node_index].get_content_ref());
        }
        None
    }
    
    /// Overwrite node content. It must exist.
    /// 
    /// # Arguments
    /// 
    /// * `node_content` - Node content.
    /// * `node_index` - Node index.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the node index.
    ///
    pub fn update_node(&mut self, node_content: &str, node_index: usize) -> Option<usize> {
        if self.nodes.len() > node_index {
            if let Some(new_node) = Node::<T>::new_node(node_content, self.nodes[node_index].get_level()) {
                // Update parent's child_map
                if let Some(parent_position) = self.nodes[node_index].get_parent_position() {
                    let old_node_content = String::from(self.nodes[node_index].get_content_ref().get_val());
                    self.nodes[parent_position].update_child(&old_node_content, node_content);
                }
                let current_node = self.nodes.get_mut(node_index).unwrap();
                current_node.set_content(new_node.get_content());
                return Some(node_index);
            }
        }
        None
    }

    /// Unlink node. It doesn't remove node from the tree, it just disconnects it from parent.
    /// 
    /// This process is O(l) complexity, where `l` is the number of nodes of the same level of `node_index`.
    /// 
    /// # Arguments
    /// 
    /// * `node_index` - Node index.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the node index.
    ///
    pub fn unlink_node(&mut self, node_index: usize) -> Option<usize> {
        if self.nodes.len() > node_index {
            if let Some(parent) = self.nodes[node_index].get_parent_position() {
                if let Some(parents_children_pos) = self.nodes[node_index].get_parents_children_pos() {
                    if self.nodes[parent].get_num_chuildren() > parents_children_pos {
                        let node_content = String::from(self.nodes[node_index].get_content_ref().get_val());
                        self.nodes[parent].remove_child(&node_content, parents_children_pos);
                        // Remove node from levels
                        if self.nodes[node_index].get_level() <= self.levels.len() {
                            self.levels[self.nodes[node_index].get_level() - 1].node_positions.remove(self.nodes[node_index].get_level_pos());
                        }
                        return Some(node_index);
                    }
                }
            }
        }
        None
    }

    /// Find node in the try by content.
    /// 
    /// The complexity of this operation is O(p), where `p` is the number of elements in the path.
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path of nodes, starting from root.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the node index.
    ///
    pub fn find_node(&self, path: &[&str]) -> Option<usize> {
        let mut last_node_index = None;
        // Check root node
        if self.nodes.len() > 0 && path.len() > 0 {
            if self.nodes[0].get_content_ref().get_val() == path[0] {
                last_node_index = Some(0);
            }
            else {
                return None;
            }
        }
        // Check following nodes
        let mut node_index = 0;
        for path_element in path[1..].iter() {
            if self.nodes.len() > node_index {
                if let Some(path_element_index) = self.nodes[node_index].get_child(path_element) {
                    last_node_index = Some(path_element_index);
                    node_index = path_element_index;
                }
                else {
                    return None;
                }
            }
            else {
                return None;
            }
        }
        last_node_index
    }

    /// Get iterators interface.
    /// 
    /// # Return
    /// 
    /// * Iterators interface.
    ///
    pub fn iterators(&self) -> iter::IterInterface<T> {
        iter::IterInterface::new(self)
    }

    /// Add node to levels array.
    /// 
    /// # Arguments
    /// 
    /// * `level` - Node level.
    /// * `node_index` - Node index.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the node position in the level's `node_position` array.
    ///
    fn add_to_level(&mut self, level: usize, node_index: usize) -> Option<usize> {
        if level <= self.levels.len() {
            // There is a pos for this level, add node_index
            self.levels[level - 1].node_positions.push(node_index);
            Some(self.levels[level - 1].node_positions.len() - 1)
        }
        else if level - 1 == self.levels.len() {
            // No pos for this level but we can create it
            self.levels.push(TreeLevel {
                level,
                node_positions: vec!(node_index)
            });
            Some(0)
        }
        else {
            // Error
            None
        }
    }
}