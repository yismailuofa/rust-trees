mod print;
mod rbnode;
pub mod tree_ops_trait;
mod tree_trait;

#[cfg(test)]
mod test;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct RBTree {
    pub root: Tree,
}

impl Default for RBTree {
    fn default() -> Self {
        let tree = RBTree {
            root: Rc::new(RefCell::new(RBNode::Empty)),
        };

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
