use std::cmp::max;

use crate::AVLNode;

impl AVLNode {
    pub fn count_leaves(&self) -> u32 {
        match self {
            AVLNode::Node { left, right, .. } => {
                left.borrow().count_leaves() + right.borrow().count_leaves()
            }
            AVLNode::Empty => 1,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            AVLNode::Node { left, right, .. } => {
                1 + max(left.borrow().height(), right.borrow().height())
            }
            AVLNode::Empty => 0,
        }
    }

    pub fn in_order(&self) -> Vec<u32> {
        match self {
            AVLNode::Node {
                key, left, right, ..
            } => {
                let mut vec = left.borrow().in_order();
                vec.push(*key);
                vec.extend(right.borrow().in_order());
                vec
            }
            AVLNode::Empty => Vec::new(),
        }
    }
}
