use std::collections::HashMap as Map;

//---- Structs ----//

/// Trait to define structs that model a node content.
pub trait NodeContent {
    /// Constructor.
    /// 
    /// # Arguments
    /// 
    /// * `content` - Node content.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the node content.
    /// 
    fn new(content: &str) -> Option<Self> where Self: Sized;
    
    /// Get node value.
    /// 
    /// # Return
    /// 
    /// * Node value.
    ///
    fn get_val(&self) -> &str;
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

    fn get_val(&self) -> &str {
        &self.content
    }
}

/// Struct that contains a tree node.
#[derive(Debug)]
pub struct Node<T: NodeContent = RawNode> {
    /// Node content.
    content: T,
    /// Nodel level.
    level: usize,
    /// Level array position.
    level_pos: usize,
    /// Parent node index in the tree array.
    parent_position: Option<usize>,
    // Map of content/node index, to find a child by name.
    child_map: Map<String, usize>,
    /// Index of current node in the parent [`children`][`Node::children`] array.
    parents_children_pos: Option<usize>,
    /// Array that contains indexes of of children nodes.
    children: Vec<usize>
}

//---- Implementations ----//

impl<T: NodeContent> Node<T> {
    /// Create new root node.
    /// 
    /// # Arguments
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
    /// # Arguments
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
                    level_pos: 0,
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

    /// Set content.
    /// 
    /// # Arguments
    /// 
    /// * `content` - Node content.
    /// 
    /// # Return
    /// 
    /// * Nothing.
    ///
    pub fn set_content(&mut self, content: T) {
        self.content = content;
    }

    /// Get content. Move self.
    /// 
    /// # Return
    /// 
    /// * Node content.
    ///
    pub fn get_content(self) -> T {
        self.content
    }

    /// Get content reference.
    /// 
    /// # Return
    /// 
    /// * Node content reference.
    ///
    pub fn get_content_ref(&self) -> &T {
        &self.content
    }

    /// Set level.
    /// 
    /// # Arguments
    /// 
    /// * `level` - Node level.
    /// 
    /// # Return
    /// 
    /// * Nothing.
    ///
    pub fn set_level(&mut self, level: usize) {
        self.level = level;
    }

    /// Get level.
    /// 
    /// # Return
    /// 
    /// * Node level.
    ///
    pub fn get_level(&self) -> usize {
        self.level
    }

    /// Set level array pos.
    /// 
    /// # Arguments
    /// 
    /// * `level_pos` - Position of node in the level array.
    /// 
    /// # Return
    /// 
    /// * Nothing.
    ///
    pub fn set_level_pos(&mut self, level_pos: usize) {
        self.level_pos = level_pos;
    }

    /// Get level array position.
    /// 
    /// # Return
    /// 
    /// * Node level.
    ///
    pub fn get_level_pos(&self) -> usize {
        self.level_pos
    }

    /// Get number of children.
    /// 
    /// # Return
    /// 
    /// * Number of children.
    ///
    pub fn get_num_chuildren(&self) -> usize {
        self.children.len()
    }

    /// Set parent node position.
    /// 
    /// # Arguments
    /// 
    /// * `parent_position` - Parent node position.
    /// 
    /// # Return
    /// 
    /// * Nothing.
    ///
    pub fn set_parent_position(&mut self, parent_position: usize) {
        self.parent_position = Some(parent_position);
    }

    /// Get parent node position.
    /// 
    /// # Return
    /// 
    /// * Parent node position..
    ///
    pub fn get_parent_position(&self) -> Option<usize> {
        self.parent_position
    }

    /// Set parent's children array position.
    /// 
    /// # Arguments
    /// 
    /// * `parents_children_pos` - Position of current node in parent's children array.
    /// 
    /// # Return
    /// 
    /// * Nothing.
    ///
    pub fn set_parents_children_pos(&mut self, parents_children_pos: usize) {
        self.parents_children_pos = Some(parents_children_pos);
    }

    /// Get parent's children array position.
    /// 
    /// * Position of current node in parent's children array.
    ///
    pub fn get_parents_children_pos(&self) -> Option<usize> {
        self.parents_children_pos
    }

    /// Add new child.
    /// 
    /// # Arguments
    /// 
    /// * `node_content` - Node content.
    /// * `node_index` - Node index.
    /// 
    /// # Return
    /// 
    /// * Nothing.
    ///
    pub fn add_child(&mut self, node_content: String, node_index: usize) {
        self.children.push(node_index);
        self.child_map.insert(node_content, node_index);
    }

    /// Remove child.
    /// 
    /// # Arguments
    /// 
    /// * `node_index` - Node index.
    /// 
    /// # Return
    /// 
    /// * Nothing.
    ///
    pub fn remove_child(&mut self, node_content: &str, node_index: usize) {
        self.child_map.remove(node_content);
        self.children.remove(node_index);
    }

    /// Update child map.
    /// 
    /// # Arguments
    /// 
    /// * `node_content` - Current node content.
    /// * `new_node_content` - New node content.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the node index.
    ///
    pub fn update_child(&mut self, node_content: &str, new_node_content: &str) -> Option<usize> {
        if let Some(node_index) = self.child_map.remove(node_content) {
            self.child_map.insert(String::from(new_node_content), node_index);
            return Some(node_index);
        }
        None
    }

    /// Get child index using node content.
    /// 
    /// # Arguments
    /// 
    /// * `node_content` - Current node content.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the node index.
    ///
    pub fn get_child(&self, node_content: &str) -> Option<usize> {
        if let Some(node_index) = self.child_map.get(node_content) {
            Some(*node_index)
        }
        else {
            None
        }
    }
}