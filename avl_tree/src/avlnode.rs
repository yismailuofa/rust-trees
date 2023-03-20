use std::cmp::max;

use crate::AVLNode;

impl AVLNode {
    pub fn count_leaves(&self) -> u32 {
        match self {
            AVLNode::Node { left, right, .. } => match (&*left.borrow(), &*right.borrow()) {
                (AVLNode::Empty, AVLNode::Empty) => 1,
                _ => left.borrow().count_leaves() + right.borrow().count_leaves(),
            },
            AVLNode::Empty => 0,
        }
    }

    pub fn height(&self, recalculate: bool) -> u32 {
        if !recalculate {
            match self {
                AVLNode::Node { height, .. } => return *height,
                _ => (),
            }
        };

        match self {
            AVLNode::Node { left, right, .. } => {
                1 + max(left.borrow().height(true), right.borrow().height(true))
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

    pub fn balance(&self) -> i32 {
        match self {
            AVLNode::Node { left, right, .. } => {
                left.borrow().height(false) as i32 - right.borrow().height(false) as i32
            }
            AVLNode::Empty => 0,
        }
    }
}
