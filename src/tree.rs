use crate::node::*;
use crate::iter::*;

//---- Structs ----//

/// Struct that contains a tree.
#[derive(Debug)]
pub struct Tree<T: NodeContent = RawNode> {
    /// Tree nodes.
    nodes: Vec<Node<T>>
}

//---- Implementations ----//

impl<T: NodeContent> Tree<T> {
    /// Create new empty tree.
    pub fn new() -> Self {
        Self {
            nodes: vec!(),
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
        if let Some(node) = Node::<T>::new_root(node_content) {
            if self.nodes.len() == 0 {
                // Create root node
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
                let node_content = String::from(new_node.get_content_ref().get_val());
                self.nodes.push(new_node);
                self.nodes[parent_node_index].add_child(node_content, new_node_index);
                return Some(new_node_index);
            }
        }
        None
    }

    /// Unlink node. It doesn't remove node from the tree, it just disconnects it from parent.
    /// 
    /// This process is O(1) complexity.
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
                        return Some(node_index);
                    }
                }
            }
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

    //TODO: deprecate find_node, create find_path, starts by any node and the path doesn't include the root.
    pub fn find_path(&self, initial_node: usize, path: &[&str]) -> Option<usize> {
        Some(0)
    }

    #[deprecated(since="0.5.0", note="find_node will be removed in the next major release, please use `find_path` instead")]
    /// Find node in the tree by content.
    /// 
    /// The complexity of this operation is O(p), where `p` is the number of elements in the path.
    /// 
    /// # Deprecated:
    /// 
    /// Please use [`find_path`][`Tree::find_path()`] instead. To update, please look at the following example that demonstrates an equivalent usage:
    /// 
    /// ```
    /// # use socarel::*;
    /// # let tree = <Tree>::new();
    /// tree.find_node(&["root", "child_1", "child_1_1"]);
    /// // Is equivalent to:
    /// tree.find_path(0, &["child_1", "child_1_1"]);
    /// // Note that, with find_path, we specify the node where to start finding and the first postion in the path is not the root.
    /// ```
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
    pub fn iterators(&self) -> IterInterface<T> {
        IterInterface::new(self)
    }

    /// Get iterators interface defining initial node to start traversing.
    /// 
    /// If `initial_node` contains an invalid index, it just ignores it and behaves like [`iterators()`][`Tree::iterators()`].
    /// 
    /// # Arguments
    /// 
    /// * `initial_node` - Initial node index.
    /// 
    /// # Return
    /// 
    /// * Iterators interface.
    ///
    pub fn iterators_at(&self, initial_node: usize) -> IterInterface<T> {
        IterInterface::new_at(self, initial_node)
    }

    /// Get reference to nodes array.
    /// 
    /// # Return
    /// 
    /// * Array reference.
    ///
    pub fn get_nodes_ref(&self) -> &[Node<T>] {
        &self.nodes
    }

    /// Get size of nodes array.
    /// 
    /// # Return
    /// 
    /// * Size.
    ///
    pub fn get_nodes_len(&self) -> usize {
        self.nodes.len()
    }

    //TODO: link an existing node to a different parent (it can be an unlinked node -> we need a flag in the node to know it is already unlinked).
    pub fn relink_node(&mut self, node_index: usize, parent_node_index: usize) -> Option<usize> {
        Some(0)
    }

    // SLOW OPERATIONS: usually O(n) complexity.

    // TODO
    /// Obtain a copy of the current tree without unlinked nodes and updating node indexes.
    /// 
    /// Node indexes of the old tree may be no longer valid in the new tree returned by this function.
    /// 
    /// # Return
    /// 
    /// * Regenerated tree.
    ///
    pub fn regenerate(&self) -> Self {
        Tree::new()
    }

    //TODO: append one tree to another. Works like link_node, but links a whole tree instead of a single node.
    pub fn append_tree(&mut self, tree: &Tree<T>, parent_node_index: usize) -> Option<usize> {
        Some(0)
    }
}