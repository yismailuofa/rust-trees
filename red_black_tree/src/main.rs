use std::{cell::RefCell, rc::Rc};

use red_black_tree::RedBlackTree;

fn main() {
    println!("Hello, world!");

    let t = RedBlackTree::Some(Rc::new(RefCell::new(red_black_tree::TreeNode {
        color: red_black_tree::NodeColor::Red,
        key: 1u32,
        parent: RedBlackTree::None,
        left: RedBlackTree::None,
        right: RedBlackTree::None,
    })));
}
