use socarel::*;

fn main() {
    let mut forest = <Forest>::new();
    forest.new_tree("my_tree");

    println!("Forest = {:#?}", forest);

    if let Some(my_tree) = forest.get_mut_tree("my_tree") {
        let _root = my_tree.set_root("my root node").unwrap();
        let _child_1 = my_tree.link_node("child node 1", _root).unwrap();
        let _grandchild = my_tree.link_node("grandchild node", _child_1).unwrap();
        let _child_2 = my_tree.link_node("child node 2", _root).unwrap();

        println!("My Tree = {:#?}", my_tree);
        println!("Root content = {:#?}", my_tree.get_node_content(_root).unwrap().get_val());
        println!("Child 1 content = {:#?}", my_tree.get_node_content(_child_1).unwrap().get_val());
        println!("Child 2 content = {:#?}", my_tree.get_node_content(_child_2).unwrap().get_val());
        println!("Grandchild content = {:#?}", my_tree.get_node_content(_grandchild).unwrap().get_val());

        my_tree.update_node("new child 1 content", _child_1);
        println!("New Child 1 content = {:#?}", my_tree.get_node_content(_child_1).unwrap().get_val());

        my_tree.unlink_node(_child_1);

        println!("My Tree after unlink = {:#?}", my_tree);
    }
}