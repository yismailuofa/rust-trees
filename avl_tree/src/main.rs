use avl_tree::AVLTree;
use tree::prompt_user;

fn main() {
    let mut tree = AVLTree::default();

    loop {
        prompt_user(&mut tree);
    }
}
