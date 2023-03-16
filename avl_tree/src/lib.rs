mod print;
mod tree_ops_trait;
mod tree_trait;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

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
