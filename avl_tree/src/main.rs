use avl_tree::*;
use tree::prompt_user;

fn main() {
    let mut tree = AVLNode::new();

    loop {
        prompt_user(&mut tree);
    }
}
