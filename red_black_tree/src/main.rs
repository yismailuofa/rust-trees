use red_black_tree::RBTree;
use tree::{prompt_user, TreeTrait};

fn main() {
    let mut tree = RBTree::default();
    tree.insert_node(10);
    tree.insert_node(20);
    tree.insert_node(30);

    tree.print_tree();

    loop {
        prompt_user(&mut tree);
    }
}
