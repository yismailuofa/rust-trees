use avl_tree::*;
use tree::prompt_user;

/**
Your library must
allow the user to perform the following operation:
1- Insert a node to the AVL tree.
2- Delete a node from the AVL tree.
3- Count the number of leaves in a tree.
4- Return the height of a tree.
5- Print In-order traversal of the tree.
6- Check if the tree is empty.
7- Print the tree showing its structure. (Using println!(“{:#?}”,tree); is NOT sufficient)
*/

fn main() {
    let mut tree = AVLTree(None);

    loop {
        prompt_user(&mut tree);
    }
}
