use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    Red,
    Black,
}
type Tree = Rc<RefCell<TreeNode<u32>>>;

pub type RedBlackTree = Option<Tree>;

pub struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree,
    pub left: RedBlackTree,  // Maybe make these private later
    pub right: RedBlackTree, // Maybe make these private later
}
