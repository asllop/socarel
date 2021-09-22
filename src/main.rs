use socarel::*;

fn main() {
    let mut forest = <Forest>::new();
    forest.new_tree("my_tree");

    println!("Forest = {:#?}", forest);

    if let Some(my_tree) = forest.get_mut_tree("my_tree") {
        let _root = my_tree.set_root("my root node").unwrap();
        let _child_1 = my_tree.link_node("child node 1", _root).unwrap();
        let _grandchild_1 = my_tree.link_node("grandchild 1 node", _child_1).unwrap();
        let _child_2 = my_tree.link_node("child node 2", _root).unwrap();
        let _grandchild_2 = my_tree.link_node("grandchild 2 node", _child_2).unwrap();

        println!("My Tree = {:#?}", my_tree);
        println!("Root content = {:#?}", my_tree.get_node_content(_root).unwrap().get_val());
        println!("Child 1 content = {:#?}", my_tree.get_node_content(_child_1).unwrap().get_val());
        println!("Child 2 content = {:#?}", my_tree.get_node_content(_child_2).unwrap().get_val());
        println!("Grandchild content = {:#?}", my_tree.get_node_content(_grandchild_1).unwrap().get_val());

        println!("-------------------------------------------------------");

        my_tree.update_node("new child 1 content", _child_1);
        println!("New Child 1 content = {:#?}", my_tree.get_node_content(_child_1).unwrap().get_val());

        println!("My Tree after node update = {:#?}", my_tree);

        println!("-------------------------------------------------------");

        my_tree.unlink_node(_child_1);

        println!("My Tree after unlink = {:#?}", my_tree);

        println!("-------------------------------------------------------");

        let _found_node_1 = my_tree.find_node(&["my root node", "new child 1 content", "grandchild node"]);
        println!("_found_node_1 index = {:#?}", _found_node_1);

        let _found_node_2 = my_tree.find_node(&["my root node", "child node 2", "grandchild 2 node"]);
        println!("_found_node_2 index = {:#?}", _found_node_2);

        let _found_root = my_tree.find_node(&["my root node"]);
        println!("_found_root index = {:#?}", _found_root);
    }
}