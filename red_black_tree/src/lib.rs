mod print;
mod rbnode;
mod tree_ops_trait;
mod tree_trait;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use tree::TreeTrait;

pub struct RBTree {
    root: Tree,
}

impl Default for RBTree {
    fn default() -> Self {
        let mut tree = RBTree {
            root: Rc::new(RefCell::new(RBNode::Empty)),
        };

        tree.insert_node(10);
        tree.insert_node(20);
        tree.insert_node(30);
        tree.insert_node(40);
        tree.insert_node(5);
        tree.insert_node(15);
        tree.insert_node(35);
        // tree.delete_node(20);

        tree
    }
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Color {
    Red,
    Black,
}
type Tree = Rc<RefCell<RBNode>>;
type WeakTree = Weak<RefCell<RBNode>>;

#[derive(Clone, Debug)]
pub enum RBNode {
    Node {
        key: u32,
        color: Color,
        left: Tree,
        right: Tree,
        parent: WeakTree,
    },
    Empty,
}
