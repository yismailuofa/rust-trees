use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use tree::TreeTrait;

use crate::{Color, RBNode, RBTree};

impl TreeTrait for RBTree {
    fn insert_node(&mut self, _key: u32) {
        let root = self.root.borrow_mut().clone();

        if let RBNode::Empty = root {
            *self.root.borrow_mut() = RBNode::Node {
                key: _key,
                color: Color::Black,
                left: Rc::new(RefCell::new(RBNode::Empty)),
                right: Rc::new(RefCell::new(RBNode::Empty)),
                parent: Weak::new(),
            };

            return;
        }

        let mut curr = root;

        while let RBNode::Node {
            key, left, right, ..
        } = &curr.clone()
        {
            match _key.cmp(key) {
                std::cmp::Ordering::Less => {
                    let mut left_node = left.borrow_mut();

                    if let RBNode::Empty = left_node.clone() {
                        *left_node = RBNode::Node {
                            key: _key,
                            color: Color::Red,
                            left: Rc::new(RefCell::new(RBNode::Empty)),
                            right: Rc::new(RefCell::new(RBNode::Empty)),
                            parent: Rc::downgrade(&Rc::new(RefCell::new(curr))),
                        };
                        return;
                    } else {
                        curr = left_node.clone();
                    }
                }
                std::cmp::Ordering::Greater => {
                    let mut right_node = right.borrow_mut();

                    if let RBNode::Empty = right_node.clone() {
                        *right_node = RBNode::Node {
                            key: _key,
                            color: Color::Red,
                            left: Rc::new(RefCell::new(RBNode::Empty)),
                            right: Rc::new(RefCell::new(RBNode::Empty)),
                            parent: Rc::downgrade(&Rc::new(RefCell::new(curr))),
                        };
                        return;
                    } else {
                        curr = right_node.clone();
                    }
                }
                _ => (),
            }
        }
    }

    fn delete_node(&mut self, _key: u32) {
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
        ptree::print_tree(&*self.root.borrow()).expect("Failed to print tree");
    }
}
