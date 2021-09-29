//! # SOCAREL
//! 
//! `Socarel` is a library to generate, manipulate and traverse trees.
//! 
//! It provides iterators for **eight** different traversal algorithms.<br>
//! Add and remove nodes in **O(1)** complexity. Find nodes in a path in **O(p)** complexity (*p* being the path lenght).<br>
//! Supports **custom node** models to create complex tree formats.<br>
//! 
//! # Examples
//! 
//! To generate a tree like the following:
//! 
//! ![Tree Example](https://raw.githubusercontent.com/asllop/tref/master/tref_example_1.svg)
//! 
//! We would do:
//! 
//! ```
//! use socarel::*;
//! 
//! let mut tree = <Tree>::new();
//! let _root_node = tree.set_root("root_node").unwrap();
//! let _child_1 = tree.link_node("child_1", _root_node).unwrap();
//! let _child_2 = tree.link_node("child_2", _root_node).unwrap();
//! let _child_3 = tree.link_node("child_3", _root_node).unwrap();
//! let _child_1_1 = tree.link_node("child_1_1", _child_1).unwrap();
//! let _child_1_2 = tree.link_node("child_1_2", _child_1).unwrap();
//! let _child_2_1 = tree.link_node("child_2_1", _child_2).unwrap();
//! ```
//! 
//! Then you can store your tree in a forest:
//! 
//! ```
//! # use socarel::*;
//! # let mut tree = <Tree>::new();
//! let mut forest = <Forest>::new();
//! forest.add_tree("my_tree", tree);
//! ```
//! 
//! And retrieve it later:
//! 
//! ```
//! # use socarel::*;
//! # let mut forest = <Forest>::new();
//! # forest.new_tree("my_tree");
//! let tree = forest.get_tree("my_tree").unwrap();
//! ```
//! 
//! Or iterate the forest to get all the trees:
//! 
//! ```
//! # use socarel::*;
//! # let mut forest = <Forest>::new();
//! for (tree_id, tree) in forest.iter() {
//!     // ...
//! }
//! ```
//! 
//! ## Editing trees
//! 
//! Is possible to modify a tree after being created, of course by linking more nodes as we saw previously, but also unlinking them:
//! 
//! ```
//! # use socarel::*;
//! # let mut tree = <Tree>::new();
//! # let _root_node = tree.set_root("root_node").unwrap();
//! # let _child_1 = tree.link_node("child_1", _root_node).unwrap();
//! # let _child_2 = tree.link_node("child_2", _root_node).unwrap();
//! # let _child_3 = tree.link_node("child_3", _root_node).unwrap();
//! # let mut forest = <Forest>::new();
//! # forest.add_tree("my_tree", tree);
//! let tree = forest.get_mut_tree("my_tree").unwrap();
//! tree.unlink_node(_child_1);
//! tree.unlink_node(_child_2);
//! ```
//! 
//! After unlinking, the node is still in the array of nodes stored inside the [`Tree`], but is not accessible anymore because it's disconnected from the rest of the tree. And any child of the unlinked node will be inaccessible too. So, after the two unlink operations the tree will have only two nodes left: `root_node` and `child_3`.
//! 
//! But why leaving the nodes there? We are wasting memory! Well, yes, but the alternative is recursively removing all the nodes, that can be costly and is actually unpredictable, because we don't know how many children are out there. To keep the unlink operation fast / O(1) we need to do it this way.
//! 
//! We can also change the content of a node without modifying the linking properties:
//! 
//! ```
//! # use socarel::*;
//! # let mut tree = <Tree>::new();
//! # let _root_node = tree.set_root("root_node").unwrap();
//! tree.update_node("new_root_node", _root_node);
//! ```
//! 
//! ## Iterators
//! 
//! There are many ways to traverse a tree, that's why `Socarel` provides multiple iterators, each one implementing one traversal algorithm. Let's see one of the most widely used, [BFS](https://en.wikipedia.org/wiki/Breadth-first_search):
//! 
//! ```
//! # use socarel::*;
//! # let mut tree = <Tree>::new();
//! # let _root_node = tree.set_root("root_node").unwrap();
//! # let _child_1 = tree.link_node("child_1", _root_node).unwrap();
//! # let _child_2 = tree.link_node("child_2", _root_node).unwrap();
//! # let _child_3 = tree.link_node("child_3", _root_node).unwrap();
//! for (node, node_index) in tree.iterators().bfs() {
//!     // ...
//! }
//! ```
//! 
//! Check out [`IterInterface`] for the complete list of iterators.
//! 
//! ## Finding nodes
//! 
//! Sometimes to find a node we don't need an iterator because we know where it is in the tree structure:
//! 
//! ```
//! # use socarel::*;
//! # let mut tree = <Tree>::new();
//! # let _root_node = tree.set_root("root_node").unwrap();
//! # let _child_1 = tree.link_node("child_1", _root_node).unwrap();
//! # let _child_1_1 = tree.link_node("child_1_1", _child_1).unwrap();
//! let _node = tree.find_path(_root_node, &["child_1", "child_1_1"]).unwrap();
//! ```
//! 
//! Using [`Tree::find_path()`] is always better when possible, because the complexity is O(p) (*p* = path length) while the complexity of most traversal algorithms is O(n) (*n* = number of nodes in the tree).
//! 
//! ## Custom Nodes
//! 
//! [`Tree`] contains an array of [`Node`]s, but the node structure as is only gives information about its position in the tree, doesn't provide a way to store the content. The content is stored in another struct that implements the trait [`NodeContent`].
//! 
//! `Socarel` provides a default implementation, [`RawNode`], that is used whenever no other type is supplied. That's the reason of the way we created the tree and forest instances before:
//! 
//! ```
//! # use socarel::*;
//! let tree = <Tree>::new();
//! ```
//! 
//! And the reason why this won't compile:
//! 
//! ```compile_fail
//! # use socarel::*;
//! let tree = Tree::new();
//! ```
//! 
//! Because [`Tree`] is defined with a generic that implements [`NodeContent`], that has a default type [`RawNode`]. So we could also create a tree like this:
//! 
//! ```
//! # use socarel::*;
//! let tree = Tree::<RawNode>::new();
//! ```
//! 
//! But we can also implement our own node content to model whatever we want. Imagine that we want a tree that has weights for node connections. We could define something like:
//! 
//! ```
//! # use socarel::*;
//! struct WeightNode {
//!     content: String,
//!     weight: u32
//! }
//! 
//! impl WeightNode {
//!     fn get_weight(&self) -> u32 {
//!         self.weight
//!     }
//! }
//! 
//! impl NodeContent for WeightNode {
//!     // We parse the node content and return None if not a valid format
//!     fn new(content: &str) -> Option<Self> {
//!         let vec: Vec<&str> = content.split(':').collect();
//!         if vec.len() == 2 {
//!             match vec[0].trim().parse() {
//!                 Ok(num) => Some(Self {
//!                     content: String::from(vec[1]),
//!                     weight: num
//!                 }),
//!                 Err(_) => None
//!             }
//!         }
//!         else {
//!             None
//!         }
//!     }
//! 
//!     fn get_val(&self) -> &str {
//!         &self.content
//!     }
//! 
//!     fn gen_content(&self) -> String {
//!         format!("{}:{}", self.weight, self.content)
//!     }
//! }
//! ```
//! 
//! And now we can use our brand new node content in a tree:
//! 
//! ```
//! # use socarel::*;
//! # struct WeightNode {
//! #     content: String,
//! #     weight: u32
//! # }
//! # impl WeightNode {
//! #     fn get_weight(&self) -> u32 {
//! #         self.weight
//! #     }
//! # }
//! # impl NodeContent for WeightNode {
//! #     fn new(content: &str) -> Option<Self> {
//! #         let vec: Vec<&str> = content.split(':').collect();
//! #         if vec.len() == 2 {
//! #             match vec[0].trim().parse() {
//! #                 Ok(num) => Some(Self {
//! #                     content: String::from(vec[1]),
//! #                     weight: num
//! #                 }),
//! #                 Err(_) => None
//! #             }
//! #         }
//! #         else {
//! #             None
//! #         }
//! #     }
//! #     fn get_val(&self) -> &str {
//! #         &self.content
//! #     }
//! #     fn gen_content(&self) -> String {
//! #         format!("{}:{}", self.weight, self.content)
//! #     }
//! # }
//! let mut tree = Tree::<WeightNode>::new();
//! let _root = tree.set_root("0:my root node").unwrap();
//! let _node_1 = tree.link_node("10:my node 1", _root).unwrap();
//! let _node_1_1 = tree.link_node("100:child of my node 1", _node_1).unwrap();
//! for (node, _) in tree.iterators().bfs() {
//!     let cref = node.get_content_ref();
//!     println!("Node content = `{}` weight = {}", cref.get_val(), cref.get_weight());
//! }
//! ```

mod node;
mod tree;
mod forest;
mod iter;

pub use node::*;
pub use tree::*;
pub use forest::*;
pub use iter::*;

#[cfg(test)]
mod tests;