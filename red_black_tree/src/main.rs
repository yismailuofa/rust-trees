use red_black_tree::RBNode;
use tree::{prompt_user, TreeTrait};

fn main() {
    let mut tree = RBNode::new();
    tree.insert_node(10);
    tree.insert_node(20);
    tree.insert_node(30);

    tree.print_tree();

    loop {
        prompt_user(&mut tree);
    }
}
