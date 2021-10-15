use crate::node::*;
use crate::iter::*;
use crate::error::*;

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
    /// * Root node index (always 0).
    ///
    pub fn set_root(&mut self, node_content: &str) -> Result<usize, SocarelError> {
        let node = Node::<T>::new_root(node_content)?;
        if self.nodes.len() == 0 {
            // Create root node
            self.nodes.push(node);
            return Ok(0);
        }
        Err(SocarelError::new("Root node already exist", 1, SocarelErrorType::Tree))
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
    /// * Node index.
    ///
    pub fn link_node(&mut self, node_content: &str, parent_node_index: usize) -> Result<usize, SocarelError> {
        if parent_node_index < self.nodes.len() {
            let new_node_level = self.nodes[parent_node_index].get_level() + 1;
            let mut new_node = Node::<T>::new_node(node_content, new_node_level)?;
            // Update new node, set parent_position and parents_children_pos
            new_node.set_parent_position(parent_node_index);
            let parents_children_pos = self.nodes[parent_node_index].get_num_children();
            new_node.set_parents_children_pos(parents_children_pos);
            // Add new node to nodes array, to parent's children array and to child_map
            let new_node_index = self.nodes.len();
            //TODO: check if a child with the same content already exist, and return Err
            let node_content = String::from(new_node.get_content_ref().get_val());
            self.nodes.push(new_node);
            self.nodes[parent_node_index].add_child(node_content, new_node_index);
            return Ok(new_node_index);
        }
        Err(SocarelError::new("Could not link node", 2, SocarelErrorType::Tree))
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
    /// * Node index.
    ///
    pub fn unlink_node(&mut self, node_index: usize) -> Result<usize, SocarelError> {
        if self.nodes.len() > node_index {
            if let Some(parent) = self.nodes[node_index].get_parent_position() {
                if let Some(parents_children_pos) = self.nodes[node_index].get_parents_children_pos() {
                    if self.nodes[parent].get_num_children() > parents_children_pos {
                        let node_content = String::from(self.nodes[node_index].get_content_ref().get_val());
                        self.nodes[parent].remove_child(&node_content, parents_children_pos);
                        return Ok(node_index);
                    }
                }
            }
        }
        Err(SocarelError::new("Could not unlink node", 3, SocarelErrorType::Tree))
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
    /// * Node index.
    ///
    pub fn update_node(&mut self, node_content: &str, node_index: usize) -> Result<usize, SocarelError> {
        if self.nodes.len() > node_index {
            let new_node = Node::<T>::new_node(node_content, self.nodes[node_index].get_level())?;
            // Update parent's child_map
            if let Some(parent_position) = self.nodes[node_index].get_parent_position() {
                //TODO: check if a child with the same content already exist, and return Err
                let old_node_content = String::from(self.nodes[node_index].get_content_ref().get_val());
                self.nodes[parent_position].update_child(&old_node_content, node_content)?;
            }
            let current_node = self.nodes.get_mut(node_index).unwrap();
            current_node.set_content(new_node.get_content());
            return Ok(node_index);
        }
        Err(SocarelError::new("Could not update node", 4, SocarelErrorType::Tree))
    }

    /// Get reference to node content.
    /// 
    /// # Arguments
    /// 
    /// * `node_index` - Node index.
    /// 
    /// # Return
    /// 
    /// * Node content reference.
    /// 
    pub fn get_node_content(&self, node_index: usize) -> Option<&T> {
        if node_index < self.nodes.len() {
            return Some(self.nodes[node_index].get_content_ref());
        }
        None
    }

    /// Find a node in the tree by its path.
    /// 
    /// The complexity of this operation is O(p), where *p* is the number of elements in the path.
    /// 
    /// # Arguments
    /// 
    /// * `initial_node` - Node index where to start.
    /// * `path` - Path of node contents _(\*)_, not including `initial_node`.
    /// 
    /// _(\*)_: Path contents are compared with [`NodeContent::get_val()`], that is not necessarily equal to the content passed to [`NodeContent::new()`] when the node was created/updated. It is if you use [`RawNode`], but it depends on the specific [`NodeContent`] implementation you are using in your tree.
    /// 
    /// # Return
    /// 
    /// * Node index.
    ///
    pub fn find_path(&self, initial_node: usize, path: &[&str]) -> Option<usize> {
        let mut node_index = initial_node;
        for path_element in path.iter() {
            if self.nodes.len() > node_index {
                if let Some(path_element_index) = self.nodes[node_index].get_child(path_element) {
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
        Some(node_index)
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
    pub fn relink_node(&mut self, _node_index: usize, _parent_node_index: usize) -> Result<usize, SocarelError> {
        Ok(0)
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
    pub fn append_tree(&mut self, _tree: &Tree<T>, _parent_node_index: usize) -> Result<usize, SocarelError> {
        Ok(0)
    }

    //TODO: build a subtree from a tree
    pub fn subtree(&self, _root_node: usize) -> Self {
        Tree::new()
    }
}