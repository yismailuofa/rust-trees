mod avlnode;
mod print;
mod tree_ops_trait;
mod tree_trait;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use tree::TreeTrait;

pub struct AVLTree {
    root: Tree,
}

impl Default for AVLTree {
    fn default() -> Self {
        let mut tree = AVLTree {
            root: Rc::new(RefCell::new(AVLNode::Empty)),
        };

        tree.insert_node(10);
        tree.insert_node(20);
        tree.insert_node(30);
        tree.insert_node(40);
        tree.insert_node(5);
        tree.insert_node(15);
        tree.insert_node(35);

        tree
    }
}

pub type Tree = Rc<RefCell<AVLNode>>;
pub type WeakTree = Weak<RefCell<AVLNode>>;

#[derive(Clone, Debug)]
pub enum AVLNode {
    Node {
        key: u32,
        left: Tree,
        right: Tree,
        parent: WeakTree,
        height: u32,
        balance: i32,
    },
    Empty,
}
