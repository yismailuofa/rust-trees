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
    fn left_rotate(&mut self);
    fn right_rotate(&mut self);
    fn fix_tree(&mut self);
    fn clone(&self) -> Self;
}

impl RedBlackTreeOps for RedBlackTree {
    fn left_rotate(&mut self) {
        // assuming it is left
        if let Some(node) = &self.0 {
            let mut node_ref = node.borrow_mut();
            let mut node_parent = &node_ref.parent;
            let mut node_right = &node_ref.right;

            if let Some(node_right_ref) = &node_right.0 {
                let mut node_right = node_right_ref.borrow_mut();
                node.borrow_mut().right = node_right.left.clone();
                node_right.parent = node_ref.parent.clone();
                node_right.left = self.clone();

            }
            node.borrow_mut().parent = node_right.clone();
            if let Some(parent_ref) = &node_parent.0 {
                let mut parent = parent_ref.borrow_mut();
                if parent.key < node_ref.key {
                    parent.right = node_right.clone();
                } else {
                    parent.left = node_right.clone();
                }
            }
        }

    }
    fn right_rotate(&mut self) {
        // assuming it is right
        if let Some(node) = &self.0 {
            let mut node_ref = node.borrow_mut();
            let mut node_parent = &node_ref.parent;
            let mut node_left = &node_ref.left;

            if let Some(node_left_ref) = &node_left.0 {
                let mut node_left = node_left_ref.borrow_mut();
                node.borrow_mut().left = node_left.right.clone();
                node_left.parent = node_ref.parent.clone();
                node_left.right = self.clone();

            }
            node.borrow_mut().parent = node_left.clone();
            if let Some(parent_ref) = &node_parent.0 {
                let mut parent = parent_ref.borrow_mut();
                if parent.key < node_ref.key {
                    parent.right = node_left.clone();
                } else {
                    parent.left = node_left.clone();
                }
            }
        }

    }
    fn fix_tree(&mut self) {
        todo!()
    }
    fn clone(&self) -> Self {
        if let Some(node) = &self.0 {
            let new_node = RedBlackTree(Some(Rc::clone(node)));
            new_node
        } else {
            RedBlackTree(None)
        }
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
