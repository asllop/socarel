use socarel::{Forest, Tree, Node, NodeContent, TreeIdentifier};

fn main() {
    let mut forest = <Forest>::new();
    forest.new_tree("my_tree");
    forest.new_tree("other_tree");

    println!("Forest = {:#?}", forest);

    if let Some(my_tree) = forest.get_mut_tree("my_tree") {
        let _root = my_tree.set_root("my root node").unwrap();
        let _child_1 = my_tree.link_node("bad child node 1", _root).unwrap();
        let _child_2 = my_tree.link_node("child node 2", _root).unwrap();
        let _grandchild_2 = my_tree.link_node("grandchild 2 node", _child_2).unwrap();
        let _grandchild_1 = my_tree.link_node("grandchild 1 node", _child_1).unwrap();

        println!("My Tree = {:#?}", my_tree);
        println!("Root content = {:#?}", my_tree.get_node_content(_root).unwrap().get_val());
        println!("Child 1 content = {:#?}", my_tree.get_node_content(_child_1).unwrap().get_val());
        println!("Child 2 content = {:#?}", my_tree.get_node_content(_child_2).unwrap().get_val());
        println!("Grandchild content = {:#?}", my_tree.get_node_content(_grandchild_1).unwrap().get_val());

        println!("-------------------------------------------------------");

        my_tree.update_node("child node 1", _child_1);

        println!("New Child 1 content = {:#?}", my_tree.get_node_content(_child_1).unwrap().get_val());

        println!("My Tree after node update = {:#?}", my_tree);

        println!("-------------------------------------------------------");

        my_tree.unlink_node(_child_1);

        println!("My Tree after unlink = {:#?}", my_tree);

        println!("-------------------------------------------------------");

        // Is the unlinked node, will return None
        let _found_node_a = my_tree.find_path(0, &["child node 1", "grandchild 1 node"]);
        println!("find '/my root node/child node 1/grandchild 1 node/' index = {:#?}", _found_node_a);

        let _found_node_b = my_tree.find_path(0, &["child node 2", "grandchild 2 node"]);
        println!("find '/my root node/child node 2/grandchild 2 node/' index = {:#?}", _found_node_b);

        let _found_root = my_tree.find_path(0, &[]);
        println!("find '/my root node/' index = {:#?}", _found_root);
    }

    println!("-------------------------------------------------------");
    println!("All trees:");
    for (k,v) in forest.iter() {
        println!("Tree name `{}` tree = {:#?}", k.get_id(), v);
    }

    println!("-------------------------------------------------------");

    let mut tree = <Tree>::new();
    let _a = tree.set_root("A").unwrap();
    let _b = tree.link_node("B", _a).unwrap();
    let _c = tree.link_node("C", _a).unwrap();
    let _d = tree.link_node("D", _b).unwrap();
    let _e = tree.link_node("E", _b).unwrap();
    let _f = tree.link_node("F", _c).unwrap();
    let _g = tree.link_node("G", _c).unwrap();
    let _h = tree.link_node("H", _e).unwrap();

    println!("--- Sequential Iter:");
    iterate(tree.iterators().sequential());
    println!("--- Inv Sequential Iter:");
    iterate(tree.iterators().inv_sequential());
    println!("--- BFS Iter:");
    iterate(tree.iterators().bfs());
    println!("--- Inv BFS Iter:");
    iterate(tree.iterators().inv_bfs());
    println!("--- Pre DFS Iter:");
    iterate(tree.iterators().pre_dfs());
    println!("--- Inv Pre DFS Iter:");
    iterate(tree.iterators().inv_pre_dfs());
    println!("--- Post DFS Iter:");
    iterate(tree.iterators().post_dfs());
    println!("--- Inv Post DFS Iter:");
    iterate(tree.iterators().inv_post_dfs());
    println!("--- Children Iter:");
    iterate(tree.iterators().children());

    println!("\nIters from node B:\n");

    println!("--- Sequential Iter:");
    iterate(tree.iterators_at(_b).sequential());
    println!("--- Inv Sequential Iter:");
    iterate(tree.iterators_at(_b).inv_sequential());
    println!("--- BFS Iter:");
    iterate(tree.iterators_at(_b).bfs());
    println!("--- Inv BFS Iter:");
    iterate(tree.iterators_at(_b).inv_bfs());
    println!("--- Pre DFS Iter:");
    iterate(tree.iterators_at(_b).pre_dfs());
    println!("--- Inv Pre DFS Iter:");
    iterate(tree.iterators_at(_b).inv_pre_dfs());
    println!("--- Post DFS Iter:");
    iterate(tree.iterators_at(_b).post_dfs());
    println!("--- Inv Post DFS Iter:");
    iterate(tree.iterators_at(_b).inv_post_dfs());
    println!("--- Children Iter:");
    iterate(tree.iterators_at(_b).children());
}

fn iterate<'a>(iter: impl Iterator<Item=(&'a Node, usize)>) {
    for (node, index) in iter {
        println!("At index {} found node `{}` ({})", index, node.get_content_ref().get_val(), node.get_level());
    }
}