mod print;
mod tree_ops_trait;
mod tree_trait;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(PartialEq, Clone, Debug)]
pub enum Color {
    Red,
    Black,
}

pub type Tree = Rc<RefCell<RBNode>>;
pub type WeakTree = Weak<RefCell<RBNode>>;

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
