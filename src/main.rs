use socarel::*;

fn main() {
    let mut tree = Tree::<RawNode>::new();
    let _root = tree.set_root("my root node").unwrap();
    println!("Tree = {:#?}", tree);
    let mut forest = Forest::<RawNode>::new();
    forest.add_tree("my_tree", tree);
    println!("Forest = {:#?}", forest);
}