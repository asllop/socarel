use socarel::*;

fn main() {
    let mut tree = Tree::<RawNode>::new();
    let _root = tree.set_root("my root node").unwrap();
    println!("Tree = {:#?}", tree);
}