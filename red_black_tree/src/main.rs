use red_black_tree::RBNode;
use tree::{prompt_user, TreeTrait};

fn main() {
    loop {
        // Create a tree with 3 values
        let mut tree = RBNode::new();
        tree.insert_node(10);
        tree.insert_node(20);
        // tree.insert_node(30);

        println!("Tree: {:?}", tree);

        prompt_user(&mut tree);
    }
}
