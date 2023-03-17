mod print;
mod tree_ops_trait;
mod tree_trait;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use tree::TreeTrait;
use tree_ops_trait::rotate_left;

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

        rotate_left(&tree.root.clone(), &mut tree.root);

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

impl PartialEq for RBNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RBNode::Node { key, .. }, RBNode::Node { key: other_key, .. }) => key == other_key,
            (RBNode::Empty, RBNode::Empty) => true,
            _ => false,
        }
    }
}
