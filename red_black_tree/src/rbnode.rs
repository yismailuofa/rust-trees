use std::cmp::max;

use crate::RBNode;

impl RBNode {
    pub fn count_leaves(&self) -> u32 {
        match self {
            RBNode::Node { left, right, .. } => match (&*left.borrow(), &*right.borrow()) {
                (RBNode::Empty, RBNode::Empty) => 1,
                _ => left.borrow().count_leaves() + right.borrow().count_leaves(),
            },
            RBNode::Empty => 0,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            RBNode::Node { left, right, .. } => {
                1 + max(left.borrow().height(), right.borrow().height())
            }
            RBNode::Empty => 0,
        }
    }

    pub fn in_order(&self) -> Vec<u32> {
        match self {
            RBNode::Node {
                key, left, right, ..
            } => {
                let mut vec = left.borrow().in_order();
                vec.push(*key);
                vec.extend(right.borrow().in_order());
                vec
            }
            RBNode::Empty => Vec::new(),
        }
    }
}
