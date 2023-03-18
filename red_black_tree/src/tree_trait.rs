use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::tree_ops_trait::fix_violation;
use tree::TreeTrait;

use crate::{Color, RBNode, RBTree};

impl TreeTrait for RBTree {
    fn insert_node(&mut self, _key: u32) {
        match &*self.root.clone().borrow() {
            RBNode::Empty => {
                self.root = Rc::new(RefCell::new(RBNode::Node {
                    key: _key,
                    color: Color::Black,
                    left: Rc::new(RefCell::new(RBNode::Empty)),
                    right: Rc::new(RefCell::new(RBNode::Empty)),
                    parent: Weak::new(),
                }));

                return;
            }
            _ => (),
        }

        let mut curr = self.root.clone();

        while let RBNode::Node {
            key, left, right, ..
        } = &*curr.clone().borrow()
        {
            match _key.cmp(key) {
                std::cmp::Ordering::Less => {
                    let mut left_node = left.borrow_mut();

                    if let RBNode::Empty = *left_node {
                        *left_node = RBNode::Node {
                            key: _key,
                            color: Color::Red,
                            left: Rc::new(RefCell::new(RBNode::Empty)),
                            right: Rc::new(RefCell::new(RBNode::Empty)),
                            parent: Rc::downgrade(&curr.clone()),
                        };
                        curr = left.clone();
                        break;
                    }
                    curr = left.clone();
                }
                std::cmp::Ordering::Greater => {
                    let mut right_node = right.borrow_mut();

                    if let RBNode::Empty = *right_node {
                        *right_node = RBNode::Node {
                            key: _key,
                            color: Color::Red,
                            left: Rc::new(RefCell::new(RBNode::Empty)),
                            right: Rc::new(RefCell::new(RBNode::Empty)),
                            parent: Rc::downgrade(&curr.clone()),
                        };
                        curr = right.clone();
                        break;
                    }
                    curr = right.clone();
                }
                _ => return,
            }
        }
        fix_violation(curr, &mut self.root);
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
        if let RBNode::Empty = &*self.root.borrow() {
            println!("\nTree is empty\n");
            return;
        }
        ptree::print_tree(&*self.root.borrow()).expect("Failed to print tree");
    }
}
