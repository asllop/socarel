use socarel::*;

fn main() {
    let mut forest = Forest::<RawNode>::new();
    forest.new_tree("my_tree");
    println!("Forest = {:#?}", forest);

    if let Some(my_tree) = forest.get_mut_tree("my_tree") {
        let _root = my_tree.set_root("my root node").unwrap();
        let _child = my_tree.link_node("child node", _root).unwrap();
        println!("Child index = {}", _child);
        println!("My Tree = {:#?}", my_tree);
    }
}