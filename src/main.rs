use socarel::*;

fn main() {
    let mut tree = Tree::<RawNode>::new();
    let _root = tree.set_root("my root node").unwrap();
    println!("Tree = {:#?}", tree);
    let mut forest = Forest::<RawNode>::new();
    forest.add_tree("my_tree", tree);
    println!("Forest = {:#?}", forest);

    if let Some(my_tree) = forest.get_mut_tree("my_tree") {
        my_tree.link_node("child node", _root);
        println!("My Tree = {:#?}", my_tree);
    }
}