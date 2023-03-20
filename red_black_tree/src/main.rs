use red_black_tree::RBTree;
use tree::{prompt_user, TreeTrait};

fn main() {
    let mut tree = RBTree::default();

    tree.print_tree();

    loop {
        prompt_user(&mut tree);
    }
}
// i love rust 😎
