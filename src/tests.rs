use crate::forest::*;
use crate::tree::*;
use crate::node::*;

fn forest_sample() -> Forest {
    let mut forest = <Forest>::new();
    let mut tree = <Tree>::new();
    let _root = tree.set_root("root_node").unwrap();
    let _child_1 = tree.link_node("child_1", _root).unwrap();
    let _child_2 = tree.link_node("child_2", _root).unwrap();
    let _child_2_1 = tree.link_node("child_2_1", _child_2).unwrap();
    let _child_2_1_1 = tree.link_node("child_2_1_1", _child_2_1).unwrap();
    let _child_2_2 = tree.link_node("child_2_2", _child_2).unwrap();
    let _child_2 = tree.link_node("child_3", _root).unwrap();
    forest.add_tree("test_tree", tree);
    forest
}

#[test]
fn check_tree_integrity() {
    let forest = forest_sample();
    if let Some(tree) = forest.get_tree("test_tree") {
        for (i, (n, _)) in tree.iterators().sequential().enumerate() {
            match i {
                0 => {
                    if !n.get_content_ref().get_val().eq("root_node") { panic!("Wrong root_node content") }
                    if let Some(_) = n.get_parent_position() { panic!("root_node has a parent") }
                    if n.get_num_chuildren() != 3 { panic!("root_node hasn't 3 children") }
                    if n.get_children_ref()[0] != 1 || n.get_children_ref()[1] != 2 || n.get_children_ref()[2] != 6 { panic!("root_node children are incorrect") }
                },
                1 => {
                    if !n.get_content_ref().get_val().eq("child_1") { panic!("Wrong child_1 content"); }
                    if let None = n.get_parent_position() { panic!("child_1 has a no parent"); }
                    if let Some(parent_n) = n.get_parent_position() {
                        if parent_n != 0 {
                            panic!("child_1 has wrong parent");
                        }
                    }
                    if n.get_num_chuildren() != 0 { panic!("child_1 hasn't 0 children"); }
                },
                2 => {
                    if !n.get_content_ref().get_val().eq("child_2") { panic!("Wrong child_2 content"); }
                    if let None = n.get_parent_position() { panic!("child_2 has a no parent"); }
                    if let Some(parent_n) = n.get_parent_position() {
                        if parent_n != 0 {
                            panic!("child_2 has wrong parent");
                        }
                    }
                    if n.get_num_chuildren() != 2 { panic!("child_2 hasn't 2 children"); }
                    if n.get_children_ref()[0] != 3 || n.get_children_ref()[1] != 5 { panic!("child_2 children are incorrect"); }
                },
                3 => {
                    if !n.get_content_ref().get_val().eq("child_2_1") { panic!("Wrong child_2_1 content"); }
                    if let None = n.get_parent_position() { panic!("child_2_1 has a no parent"); }
                    if let Some(parent_n) = n.get_parent_position() {
                        if parent_n != 2 {
                            panic!("child_2_1 has wrong parent");
                        }
                    }
                    if n.get_num_chuildren() != 1 { panic!("child_2_1 hasn't 1 child"); }
                    if n.get_children_ref()[0] != 4 { panic!("child_2_1 children are incorrect"); }
                },
                4 => {
                    if !n.get_content_ref().get_val().eq("child_2_1_1") { panic!("Wrong child_2_1_1 content"); }
                    if let None = n.get_parent_position() { panic!("child_2_1_1 has a no parent"); }
                    if let Some(parent_n) = n.get_parent_position() {
                        if parent_n != 3 {
                            panic!("child_2_1_1 has wrong parent");
                        }
                    }
                    if n.get_num_chuildren() != 0 { panic!("child_2_1_1 hasn't 0 children"); }
                },
                5 => {
                    if !n.get_content_ref().get_val().eq("child_2_2") { panic!("Wrong child_2_2 content"); }
                    if let None = n.get_parent_position() { panic!("child_2_2 has a no parent"); }
                    if let Some(parent_n) = n.get_parent_position() {
                        if parent_n != 2 {
                            panic!("child_2_2 has wrong parent");
                        }
                    }
                    if n.get_num_chuildren() != 0 { panic!("child_2_2 hasn't 0 children"); }
                },
                6 => {                          
                    if !n.get_content_ref().get_val().eq("child_3") { panic!("Wrong child_3 content"); }
                    if let None = n.get_parent_position() { panic!("child_3 has a no parent"); }
                    if let Some(parent_n) = n.get_parent_position() {
                        if parent_n != 0 {
                            panic!("child_3 has wrong parent");
                        }
                    }
                    if n.get_num_chuildren() != 0 { panic!("child_3 hasn't 0 children"); }
                }
                _ => {}
            }
        }
    }
    else {
        panic!("Tree ID not found");
    }
}

#[test]
fn mutate_and_check_integrity() {
    let mut forest = forest_sample();
    let tree = forest.get_mut_tree("test_tree").expect("Could not find tree ID");
    let child_2_1 = tree.find_node(&["root_node", "child_2", "child_2_1"]).expect("Could nod find node");
    tree.update_node("remove_me", child_2_1).expect("Could not update node");
    let remove_me = tree.find_node(&["root_node", "child_2", "remove_me"]).expect("Could nod find modified node");
    assert_eq!(child_2_1, remove_me);
    tree.unlink_node(remove_me).expect("Could unlink node");
    if let Some(_) = tree.find_node(&["root_node", "child_2", "remove_me"]) {
        panic!("Found unlinked node");
    }
    for (i, (n, _)) in tree.iterators().bfs().enumerate() {
        match i {
            0 => {
                if !n.get_content_ref().get_val().eq("root_node") { panic!("Wrong root_node content") }
            },
            1 => {
                if !n.get_content_ref().get_val().eq("child_1") { panic!("Wrong child_1 content") }
            },
            2 => {
                if !n.get_content_ref().get_val().eq("child_2") { panic!("Wrong child_2 content") }
            },
            3 => {
                if !n.get_content_ref().get_val().eq("child_3") { panic!("Wrong child_3 content") }
            },
            _ => {
                panic!("Invalid number of nodes");
            }
        }
    }
}

#[test]
fn check_custom_node_content() {
    #[derive(Debug)]
    struct WeightNode {
        content: String,
        weight: u32
    }

    impl WeightNode {
        fn get_weight(&self) -> u32 {
            self.weight
        }
    }

    impl NodeContent for WeightNode {
        fn new(content: &str) -> Option<Self> {
            let vec: Vec<&str> = content.split(':').collect();
            if vec.len() == 2 {
                match vec[0].trim().parse() {
                    Ok(num) => Some(Self {
                        content: String::from(vec[1]),
                        weight: num
                    }),
                    Err(_) => None
                }
            }
            else {
                None
            }
        }

        fn get_val(&self) -> &str {
            &self.content
        }

        fn gen_content(&self) -> String {
            format!("{}:{}", self.weight, self.content)
        }
    }

    let mut tree = Tree::<WeightNode>::new();
    let _root = tree.set_root("0:root_node").unwrap();
    let _child_1 = tree.link_node("10:child_1", _root).unwrap();
    let _child_1_1 = tree.link_node("5:child_1_1", _child_1).unwrap();
    let _child_1_1_1 = tree.link_node("12:child_1_1_1", _child_1_1).unwrap();

    for (i, (n, _)) in tree.iterators().sequential().enumerate() {
        match i {
            0 => {
                if !n.get_content_ref().get_val().eq("root_node") { panic!("Wrong {} node content!", n.get_content_ref().get_val()); }
                if n.get_content_ref().get_weight() != 0 { panic!("Wrong {} node weight!", n.get_content_ref().get_weight()); }
            },
            1 => {
                if !n.get_content_ref().get_val().eq("child_1") { panic!("Wrong {} node content!", n.get_content_ref().get_val()); }
                if n.get_content_ref().get_weight() != 10 { panic!("Wrong {} node weight!", n.get_content_ref().get_weight()); }
            },
            2 => {
                if !n.get_content_ref().get_val().eq("child_1_1") { panic!("Wrong {} node content!", n.get_content_ref().get_val()); }
                if n.get_content_ref().get_weight() != 5 { panic!("Wrong {} node weight!", n.get_content_ref().get_weight()); }
            },
            3 => {
                if !n.get_content_ref().get_val().eq("child_1_1_1") { panic!("Wrong {} node content!", n.get_content_ref().get_val()); }
                if n.get_content_ref().get_weight() != 12 { panic!("Wrong {} node weight!", n.get_content_ref().get_weight()); }
            },
            _ => {}
        }
    }
}

//TODO: add check for all iterators

// Tree like https://en.wikipedia.org/wiki/Breadth-first_search#/media/File:Animated_BFS.gif
fn tree_sample() -> Tree {
    let mut tree = <Tree>::new();
    let _a = tree.set_root("A").unwrap();
    let _b = tree.link_node("B", _a).unwrap();
    let _c = tree.link_node("C", _a).unwrap();
    let _d = tree.link_node("D", _b).unwrap();
    let _e = tree.link_node("E", _b).unwrap();
    let _f = tree.link_node("F", _c).unwrap();
    let _g = tree.link_node("G", _c).unwrap();
    let _h = tree.link_node("H", _e).unwrap();
    tree
}

#[test]
fn test_bfs_iter() {
    let tree = tree_sample();
    let nodes_in_order = ["A", "B", "C", "D", "E", "F", "G", "H"];
    for (i, (n, _)) in tree.iterators().bfs().enumerate() {
        if nodes_in_order.len() > i {
            assert_eq!(n.get_content_ref().get_val(), nodes_in_order[i]);
        }
        else {
            panic!("Wrong node index");
        }
    }
}

#[test]
fn test_inv_bfs_iter() {
    let tree = tree_sample();
    let nodes_in_order = ["A", "C", "B", "G", "F", "E", "D", "H"];
    for (i, (n, _)) in tree.iterators().inv_bfs().enumerate() {
        if nodes_in_order.len() > i {
            assert_eq!(n.get_content_ref().get_val(), nodes_in_order[i]);
        }
        else {
            panic!("Wrong node index");
        }
    }
}