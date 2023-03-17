use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use tree::TreeTrait;

use crate::{Color, RBNode};

impl TreeTrait for RBNode {
    fn insert_node(&mut self, _key: u32) {
        if let RBNode::Empty = self {
            *self = RBNode::Node {
                key: _key,
                color: Color::Black,
                left: Rc::new(RefCell::new(RBNode::Empty)),
                right: Rc::new(RefCell::new(RBNode::Empty)),
                parent: Weak::new(),
            };
            return;
        }

        let mut curr = self.clone();

        while let RBNode::Node {
            key, left, right, ..
        } = &curr.clone()
        {
            if _key < *key {
                let mut left_node = left.borrow_mut();

                if let RBNode::Empty = left_node.clone() {
                    *left_node = RBNode::Node {
                        key: _key,
                        color: Color::Red,
                        left: Rc::new(RefCell::new(RBNode::Empty)),
                        right: Rc::new(RefCell::new(RBNode::Empty)),
                        parent: Rc::downgrade(&Rc::new(RefCell::new(curr.clone()))),
                    };
                    return;
                } else {
                    curr = left_node.clone();
                }
            } else if _key > *key {
                let mut right_node = right.borrow_mut();

                if let RBNode::Empty = right_node.clone() {
                    *right_node = RBNode::Node {
                        key: _key,
                        color: Color::Red,
                        left: Rc::new(RefCell::new(RBNode::Empty)),
                        right: Rc::new(RefCell::new(RBNode::Empty)),
                        parent: Rc::downgrade(&Rc::new(RefCell::new(curr.clone()))),
                    };
                    return;
                } else {
                    curr = right_node.clone();
                }
            }
        }
    }

    fn delete_node(&mut self, _key: u32) {
        if *self == RBNode::Empty {
            return;
        }

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
        ptree::print_tree(self).expect("Failed to print tree");
    }

    fn new() -> Self {
        RBNode::Empty
    }
}
