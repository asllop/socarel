use crate::forest::*;
use crate::node::*;

fn forest_sample() -> Forest {
    let mut forest = <Forest>::new();
    forest.new_tree("test_tree");
    let tree = forest.get_mut_tree("test_tree").unwrap();
    let _root = tree.set_root("root_node").unwrap();
    let _child_1 = tree.link_node("child_1", _root).unwrap();
    let _child_2 = tree.link_node("child_2", _root).unwrap();
    let _child_2_1 = tree.link_node("child_2_1", _child_2).unwrap();
    let _child_2_1_1 = tree.link_node("child_2_1_1", _child_2_1).unwrap();
    let _child_2_2 = tree.link_node("child_2_2", _child_2).unwrap();
    let _child_2 = tree.link_node("child_3", _root).unwrap();
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
}