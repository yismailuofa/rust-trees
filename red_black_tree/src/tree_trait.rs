use std::{
    borrow::Borrow,
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
        //delete node with key

        //find node with key
        let root = self.root.borrow_mut();
        let mut curr = root;

        while let RBNode::Node {
            key, left, right, parent, ..
        } = &curr.clone()
        {
            match _key.cmp(key) {
                std::cmp::Ordering::Less => {
                    let mut left_node = left.borrow_mut();

                    if let RBNode::Empty = left_node.clone() {
                        println!("Node not found");
                        return;
                    } else {
                        curr = left_node;
                    }
                }
                std::cmp::Ordering::Greater => {
                    let mut right_node = right.borrow_mut();

                    if let RBNode::Empty = right_node.clone() {
                        println!("Node not found");
                        return;
                    } else {
                        curr = right_node;
                    }
                }
                std::cmp::Ordering::Equal => {
                    //if node has no children, delete it
                    let mut left_node = left.borrow_mut();
                    let mut right_node = right.borrow_mut();
                    let old_parent = match parent.upgrade() {
                        Some(_) => parent.upgrade().unwrap(),
                        None => Rc::new(RefCell::new(RBNode::Empty)),
                    };

                    match (&mut *left_node, &mut *right_node) {
                        (RBNode::Empty, RBNode::Empty) => {
                            //delete node
                            *curr = RBNode::Empty;
                            return;
                        }
                        // if a node has one child, replace it with the child
                        (RBNode::Empty, RBNode::Node { parent:right_parent, .. }) => {
                            
                            match &mut *old_parent.borrow_mut() {
                                RBNode::Node { left:parent_left, right:parent_right, .. } => {
                                    if (*parent_left.borrow()== curr) {
                                        *parent_left = right.clone();
                                    } else if (*parent_right.borrow()== curr) {
                                        *parent_right = right.clone();
                                    }
                                }
                                _ => (), // this is the case where the old rotation point was the root, this is wht indicates to update root
                            };
                            *right_parent = Rc::downgrade(&old_parent);
                            *curr = RBNode::Empty;
                            
                            return;
                        }
                        (RBNode::Node { parent:left_parent, .. }, RBNode::Empty) => {
                            match &mut *old_parent.borrow_mut() {
                                RBNode::Node { left:parent_left, right:parent_right, .. } => {
                                    if (*parent_left.borrow()== curr) {
                                        *parent_left = left.clone();
                                    } else if (*parent_right.borrow()== curr) {
                                        *parent_right = left.clone();
                                    }
                                }
                                _ => (), // this is the case where the old rotation point was the root, this is wht indicates to update root
                            };
                            
                            *left_parent = Rc::downgrade(&old_parent);
                            *curr = RBNode::Empty;
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
                            let mut successor = right;

                            while let RBNode::Node {
                                key: _,
                                left: left,
                                right: _,
                                ..
                            } = &*successor.borrow_mut()
                            {
                                let mut left_node = left.borrow_mut();
                                if let RBNode::Empty = left_node.clone() {
                                    break;
                                } else {
                                    successor = left;
                                }
                            }

                            //replace node with successor
                            if let RBNode::Node {
                                key: successor_key,
                                color: successor_color,
                                left: successor_left,
                                right: successor_right,
                                parent: successor_parent,
                            } = &mut *successor.borrow_mut() {
                                match &mut *old_parent.borrow_mut() {
                                    RBNode::Node { left:parent_left, right:parent_right, .. } => {
                                        if Rc::ptr_eq(&parent_left, &right) {
                                            *parent_left = successor.clone();
                                        } else if Rc::ptr_eq(&parent_right, &right) {
                                            *parent_right = successor.clone();
                                        }
                                    }
                                    _ => (), // this is the case where the old rotation point was the root, this is wht indicates to update root
                                };
                                *successor_left = left.clone();
                                *successor_parent = Rc::downgrade(&old_parent);
                                *curr = RBNode::Empty;
                            }

                            //delete successor
                            if let RBNode::Node {
                                key: _,
                                color: _,
                                left: _,
                                right: _,
                                parent: parent,
                            } = successor.borrow().clone()
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
        ptree::print_tree(&*self.root.borrow()).expect("Failed to print tree");
    }
}
