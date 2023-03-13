use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)]

pub enum NodeColor {
    Red,
    Black,
}
type Tree = Rc<RefCell<TreeNode>>;

pub type RedBlackTree = Option<Tree>;

pub struct TreeNode {
    pub color: NodeColor,
    pub key: u32,
    pub parent: RedBlackTree,
    pub left: RedBlackTree,  // Maybe make these private later
    pub right: RedBlackTree, // Maybe make these private later
}

pub trait RedBlackTreeTrait {
    fn insert_node(&mut self, key: u32);
    fn delete_node(&mut self, key: u32);
    fn count_leaves(&self) -> u32;
    fn height(&self) -> u32;
    fn in_order(&self) -> Vec<u32>;
    fn is_empty(&self) -> bool;
    fn print_tree(&self);
}

impl RedBlackTreeTrait for RedBlackTree {
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
