use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use tree::TreeTrait;
use tree_ops_trait::{rotate_left, rotate_right};

use crate::{tree_ops_trait, AVLNode, AVLTree};

impl TreeTrait for AVLTree {
    fn insert_node(&mut self, _key: u32) {
        match &*self.root.clone().borrow() {
            AVLNode::Empty => {
                self.root = Rc::new(RefCell::new(AVLNode::Node {
                    key: _key,
                    height: 1,
                    left: Rc::new(RefCell::new(AVLNode::Empty)),
                    right: Rc::new(RefCell::new(AVLNode::Empty)),
                    parent: Weak::new(),
                }));
            }
            _ => (),
        }

        let mut curr = self.root.clone();

        while let AVLNode::Node {
            key,
            height,
            left,
            right,
            ..
        } = &mut *curr.clone().borrow_mut()
        {
            match _key.cmp(key) {
                std::cmp::Ordering::Less => {
                    let mut left_node = left.borrow_mut();

                    if let AVLNode::Empty = *left_node {
                        *left_node = AVLNode::Node {
                            key: _key,
                            height: 1,
                            left: Rc::new(RefCell::new(AVLNode::Empty)),
                            right: Rc::new(RefCell::new(AVLNode::Empty)),
                            parent: Rc::downgrade(&curr),
                        };
                        *height = std::cmp::max(left_node.height(), right.borrow().height()) + 1;

                        curr = left.clone();

                        break;
                    }
                    curr = left.clone();
                }
                std::cmp::Ordering::Greater => {
                    let mut right_node = right.borrow_mut();

                    if let AVLNode::Empty = *right_node {
                        *right_node = AVLNode::Node {
                            key: _key,
                            height: 1,
                            left: Rc::new(RefCell::new(AVLNode::Empty)),
                            right: Rc::new(RefCell::new(AVLNode::Empty)),
                            parent: Rc::downgrade(&curr),
                        };

                        *height = std::cmp::max(left.borrow().height(), right_node.height()) + 1;

                        curr = right.clone();

                        break;
                    }
                    curr = right.clone();
                }
                _ => return,
            }
        }

        loop {
            match &*curr.clone().borrow() {
                AVLNode::Empty => break,
                _ => (),
            }

            let balance = curr.borrow().balance();

            match &mut *curr.clone().borrow_mut() {
                AVLNode::Node {
                    height,
                    left,
                    right,
                    ..
                } => {
                    *height = std::cmp::max(left.borrow().height(), right.borrow().height()) + 1;
                }

                _ => (),
            }

            if balance > 1 {
                let case = match &*curr.clone().borrow() {
                    AVLNode::Node { left, .. } => match &*left.borrow() {
                        AVLNode::Node { key: left_key, .. } => {
                            if _key < *left_key {
                                1
                            } else if _key > *left_key {
                                2
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    },
                    _ => 0,
                };

                // Avoids passing a borrow_mut() to rotate_right() and rotate_left()
                // which would cause a double borrow_mut() error
                match case {
                    1 => curr = rotate_right(&curr, &mut self.root),
                    2 => {
                        let left = match &*curr.borrow() {
                            AVLNode::Node { left, .. } => left.clone(),
                            _ => Rc::new(RefCell::new(AVLNode::Empty)),
                        };

                        let left_node = rotate_left(&left.clone(), &mut self.root);

                        match &mut *curr.clone().borrow_mut() {
                            AVLNode::Node { left, .. } => *left = left_node,
                            _ => (),
                        }

                        curr = rotate_right(&curr, &mut self.root);
                    }
                    _ => (),
                }
            } else if balance < -1 {
                let case = match &*curr.clone().borrow() {
                    AVLNode::Node { right, .. } => match &*right.borrow() {
                        AVLNode::Node { key: right_key, .. } => {
                            if _key < *right_key {
                                1
                            } else if _key > *right_key {
                                2
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    },
                    _ => 0,
                };

                // Avoids passing a borrow_mut() to rotate_right() and rotate_left()
                // which would cause a double borrow_mut() error
                match case {
                    1 => {
                        let right = match &*curr.borrow() {
                            AVLNode::Node { right, .. } => right.clone(),
                            _ => Rc::new(RefCell::new(AVLNode::Empty)),
                        };

                        let right_node = rotate_right(&right.clone(), &mut self.root);

                        match &mut *curr.clone().borrow_mut() {
                            AVLNode::Node { right, .. } => *right = right_node,
                            _ => (),
                        }

                        curr = rotate_left(&curr, &mut self.root);
                    }
                    2 => curr = rotate_left(&curr, &mut self.root),
                    _ => (),
                }
            }

            match &*curr.clone().borrow() {
                AVLNode::Node { parent, .. } => {
                    if let Some(parent) = parent.upgrade() {
                        curr = parent;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn delete_node(&mut self, _key: u32) {
        todo!()
    }

    fn count_leaves(&self) -> u32 {
        self.root.borrow().count_leaves()
    }

    fn height(&self) -> u32 {
        self.root.borrow().height()
    }

    fn in_order(&self) -> Vec<u32> {
        self.root.borrow().in_order()
    }

    fn is_empty(&self) -> bool {
        if let AVLNode::Empty = *self.root.borrow() {
            true
        } else {
            false
        }
    }

    fn print_tree(&self) {
        if let AVLNode::Empty = &*self.root.borrow() {
            println!("\nTree is empty\n");
            return;
        }
        ptree::print_tree(&*self.root.borrow()).expect("Failed to print tree");
    }
}
