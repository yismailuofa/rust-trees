mod avlnode;
mod print;
pub mod tree_ops_trait;
mod tree_trait;

#[cfg(test)]
mod test;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct AVLTree {
    pub root: Tree,
}

impl Default for AVLTree {
    fn default() -> Self {
        let tree = AVLTree {
            root: Rc::new(RefCell::new(AVLNode::Empty)),
        };

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
    },
    Empty,
}
