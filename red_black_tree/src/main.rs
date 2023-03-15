use red_black_tree::RedBlackTree;
use tree::{prompt_user, TreeTrait};

fn main() {
    let mut tree = RedBlackTree(None);
    //tree.insert_node(None,1);
    //tree.insert_node(None, 2);
    //tree.insert_node(None, 3);

    loop {
        prompt_user(&mut tree);
    }
}
