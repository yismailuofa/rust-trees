extern crate tree;
use tree::TreeTrait;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

type Tree = Rc<RefCell<AVLTreeNode>>;

pub struct AVLTree(pub Option<Tree>);

pub struct AVLTreeNode {
    pub key: u32,
    pub parent: AVLTree,
    pub left: AVLTree,  // Maybe make these private later
    pub right: AVLTree, // Maybe make these private late
    pub height: u32,
    pub balance_factor: i32,
}

// Allows us to avoid using self.0 everywhere
impl Deref for AVLTree {
    type Target = Option<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TreeTrait for AVLTree {
    fn insert_node(&mut self, key: u32) {
        todo!()
    }

    fn delete_node(&mut self, key: u32) {
        todo!()
    }

    fn count_leaves(&self) -> u32 {
        todo!()
    }

    fn height(&self) -> u32 {
        todo!()
    }

    fn in_order(&self) -> Vec<u32> {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn print_tree(&self) {
        todo!()
    }
}
