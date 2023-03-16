use red_black_tree::{RedBlackTree, RedBlackTreeOps};
use tree::{prompt_user, TreeTrait};

fn main() {
    let mut tree = RedBlackTree(None);
    tree.insert_node(None, 1);
    // tree.fix_tree();
    tree.insert_node(None, 2);
    // tree.fix_tree();
    tree.insert_node(None, 3);
    // tree.fix_tree();
    tree.insert_node(None, 4);
    tree.fix_tree();

    loop {
        prompt_user(&mut tree);
    }
}
