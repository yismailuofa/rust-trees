use std::cell::RefCell;
use std::rc::Rc;

type Tree = Rc<RefCell<TreeNode>>;

pub type AVLTree = Option<Tree>;

pub struct TreeNode {
    pub key: u32,
    pub parent: AVLTree,
    pub left: AVLTree,  // Maybe make these private later
    pub right: AVLTree, // Maybe make these private late
}

pub trait AVLTreeTrait {
    fn insert_node(&mut self, key: u32);
    fn delete_node(&mut self, key: u32);
    fn count_leaves(&self) -> u32;
    fn height(&self) -> u32;
    fn in_order(&self) -> Vec<u32>;
    fn is_empty(&self) -> bool;
    fn print_tree(&self);
}

impl AVLTreeTrait for AVLTree {
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
