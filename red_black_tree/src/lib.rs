extern crate tree;
use tree::TreeTrait;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    Red,
    Black,
}

type Tree = Rc<RefCell<RedBlackTreeNode>>;

pub struct RedBlackTree(pub Option<Tree>);

pub struct RedBlackTreeNode {
    pub color: NodeColor,
    pub key: u32,
    pub parent: RedBlackTree,
    pub left: RedBlackTree,  // Maybe make these private later
    pub right: RedBlackTree, // Maybe make these private later
}
trait RedBlackTreeOps {
    fn rotate(&mut self, is_left: bool);
    fn fix_tree(&mut self);
}

impl RedBlackTreeOps for RedBlackTree {
    fn rotate(&mut self, is_left: bool) {}
    fn fix_tree(&mut self) {
        todo!()
    }
}

impl TreeTrait for RedBlackTree {
    fn insert_node(&mut self, key: u32) {
        if let Some(node) = &self.0 {
            let mut node = node.borrow_mut();

            if key < node.key {
                node.left.insert_node(key);
            } else if key > node.key {
                node.right.insert_node(key);
            }
        } else {
            self.0 = Some(Rc::new(RefCell::new(RedBlackTreeNode {
                color: NodeColor::Red,
                key,
                parent: RedBlackTree(None),
                left: RedBlackTree(None),
                right: RedBlackTree(None),
            })));

            self.fix_tree();
        }
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
