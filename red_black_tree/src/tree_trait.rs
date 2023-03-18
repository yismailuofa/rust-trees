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
        //delete node with key

        //find node with key
        let mut curr = self.root.clone();

        while let RBNode::Node {
            key,
            left,
            right,
            parent,
            ..
        } = &*curr.clone().borrow_mut()
        {
            match _key.cmp(key) {
                std::cmp::Ordering::Less => {
                    let left_node = left.borrow_mut();

                    if let RBNode::Empty = left_node.clone() {
                        println!("Node not found");
                        return;
                    } else {
                        curr = left.clone();
                    }
                }
                std::cmp::Ordering::Greater => {
                    let right_node = right.borrow_mut();

                    if let RBNode::Empty = right_node.clone() {
                        println!("Node not found");
                        return;
                    } else {
                        curr = right.clone();
                    }
                }
                std::cmp::Ordering::Equal => {
                    let mut left_node = left.borrow_mut();
                    let mut right_node = right.borrow_mut();
                    let old_parent = match parent.upgrade() {
                        Some(_) => parent.upgrade().unwrap(),
                        None => Rc::new(RefCell::new(RBNode::Empty)),
                    };

                    //if node has no children, delete it
                    match (&mut *left_node, &mut *right_node) {
                        (RBNode::Empty, RBNode::Empty) => {
                            //delete node
                            curr.replace(RBNode::Empty);
                            return;
                        }
                        // if a node has one child, replace it with the child
                        (
                            RBNode::Empty,
                            RBNode::Node {
                                parent: right_parent,
                                ..
                            },
                        ) => {
                            match &mut *old_parent.borrow_mut() {
                                RBNode::Node {
                                    left: parent_left,
                                    right: parent_right,
                                    ..
                                } => {
                                    if Rc::ptr_eq(&curr, &parent_left) {
                                        *parent_left = right.clone();
                                    } else if Rc::ptr_eq(&curr, &parent_right) {
                                        *parent_right = right.clone();
                                    }
                                }
                                _ => (),
                            };
                            *right_parent = Rc::downgrade(&old_parent);
                            curr.replace(RBNode::Empty);

                            return;
                        }
                        (
                            RBNode::Node {
                                parent: left_parent,
                                ..
                            },
                            RBNode::Empty,
                        ) => {
                            match &mut *old_parent.borrow_mut() {
                                RBNode::Node {
                                    left: parent_left,
                                    right: parent_right,
                                    ..
                                } => {
                                    if Rc::ptr_eq(&curr, &parent_left) {
                                        *parent_left = left.clone();
                                    } else if Rc::ptr_eq(&curr, &parent_right) {
                                        *parent_right = left.clone();
                                    }
                                }
                                _ => (),
                            };

                            *left_parent = Rc::downgrade(&old_parent);
                            curr.replace(RBNode::Empty);
                            return;
                        }
                        (
                            RBNode::Node {
                                left: left_left,
                                right: left_right,
                                parent: left_parent,
                                ..
                            },
                            RBNode::Node {
                                left: right_left,
                                right: right_right,
                                parent: right_parent,
                                ..
                            },
                        ) => {
                            //if node has two children, find the successor
                            let mut successor = right.clone();

                            while let RBNode::Node { left, .. } = &*successor.clone().borrow() {
                                let left_node = left.borrow_mut();
                                if let RBNode::Empty = left_node.clone() {
                                    break;
                                } else {
                                    successor = left.clone();
                                }
                            }

                            // If this breaks use this LOL 🐤
                            // loop {
                            //     match  {

                            //     } let
                            // }

                            //replace node with successor
                            if let RBNode::Node {
                                key: successor_key,
                                color: successor_color,
                                left: successor_left,
                                right: successor_right,
                                parent: successor_parent,
                            } = &mut *successor.borrow_mut()
                            {
                                match &mut *old_parent.borrow_mut() {
                                    RBNode::Node {
                                        left: parent_left,
                                        right: parent_right,
                                        ..
                                    } => {
                                        if Rc::ptr_eq(&parent_left, &right) {
                                            *parent_left = successor.clone();
                                        } else if Rc::ptr_eq(&parent_right, &right) {
                                            *parent_right = successor.clone();
                                        }
                                    }
                                    _ => (),
                                };
                                *successor_left = left.clone();
                                *successor_parent = Rc::downgrade(&old_parent);
                                curr.replace(RBNode::Empty);
                            }

                            //delete successor
                            if let RBNode::Node {
                                key: _,
                                color: _,
                                left: _,
                                right: _,
                                parent: parent,
                            } = &*successor.clone().borrow()
                            {
                                if let Some(parent) = parent.upgrade() {
                                    if let RBNode::Node {
                                        key: _,
                                        color: _,
                                        left: left,
                                        right: _,
                                        parent: _,
                                    } = parent.borrow().clone()
                                    {
                                        if let RBNode::Node {
                                            key: _,
                                            color: _,
                                            left: _,
                                            right: _,
                                            parent: _,
                                        } = left.borrow().clone()
                                        {
                                            *left.borrow_mut() = RBNode::Empty;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        //if node has no children, delete it

        //if node has one child, replace it with the child
        //if node has two children, replace it with the smallest node in the right subtree
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
