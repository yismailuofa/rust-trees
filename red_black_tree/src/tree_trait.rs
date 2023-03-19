use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use tree::TreeTrait;
use tree_ops_trait::insert_fixup;

use crate::{tree_ops_trait::{self, delete_fixup}, Color, RBNode, RBTree};

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
                            parent: Rc::downgrade(&curr),
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
                            parent: Rc::downgrade(&curr),
                        };
                        curr = right.clone();
                        break;
                    }
                    curr = right.clone();
                }
                _ => return,
            }
        }
        insert_fixup(curr, &mut self.root);
    }

    fn delete_node(&mut self, _key: u32) {
        //delete node with key

        //find node with key
        let mut curr = self.root.clone();
        let mut x = Rc::new(RefCell::new(RBNode::Empty));
        let mut old_color = Color::Black;
        
        while let RBNode::Node {
            key,
            left,
            right,
            parent,
            color,
            ..
        } = &mut *curr.clone().borrow_mut()
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
                        Some(rc) => rc,
                        None => Rc::new(RefCell::new(RBNode::Empty)),
                    };
                     old_color = color.clone();
                    //if node has no children, delete it
                    match (&mut *left_node, &mut *right_node) {
                        (RBNode::Empty, RBNode::Empty) => {
                            //delete node
                            match &mut *old_parent.borrow_mut() {
                                RBNode::Node {
                                    left: parent_left,
                                    right: parent_right,
                                    ..
                                } => {
                                    if Rc::ptr_eq(&curr, &parent_left) {
                                        *parent_left = Rc::new(RefCell::new(RBNode::Empty));
                                    } else if Rc::ptr_eq(&curr, &parent_right) {
                                        *parent_right = Rc::new(RefCell::new(RBNode::Empty));
                                    }
                                }
                                RBNode::Empty => {
                                    self.root = Rc::new(RefCell::new(RBNode::Empty));
                                }
                            };
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
                                RBNode::Empty => {
                                    self.root = right.clone();
                                }
                            };
                            *right_parent = Rc::downgrade(&old_parent);
                            curr = right.clone();
                            break;
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
                                RBNode::Empty => {
                                    self.root = left.clone();
                                }
                            };
                            curr = left.clone();
                            *left_parent = Rc::downgrade(&old_parent);

                            break;
                        }
                        (
                            RBNode::Node { .. },
                            RBNode::Node {
                                left: right_left,right:right_right , color:right_color,..
                            },
                        ) => {
                            //if node has two children, find the successor
                            // see if right_left is of type RBNode::Empty
                            // if it is, then right is the successor
                            // if it isn't, then find the leftmost node of right_left
                            // that is the successor

                            let successor = match &*right_left.borrow() {
                                RBNode::Empty => right.clone(),
                                _ => {
                                    let mut successor_candidate = right_left.clone();
                                    loop {
                                        match &*successor_candidate.clone().borrow() {
                                            RBNode::Node { left, .. } => {
                                                let left_node = left.borrow();
                                                if let RBNode::Empty = left_node.clone() {
                                                    break;
                                                } else {
                                                    successor_candidate = left.clone();
                                                }
                                            }
                                            _ => break,
                                        };
                                    }
                                    successor_candidate
                                }
                            };

                            //replace node with successor
                            if Rc::ptr_eq(&successor, right) {
                                if Rc::ptr_eq(&self.root, &curr) {
                                    self.root = successor.clone();
                                }
                                curr = right_right.clone();
                                old_color = right_color.clone();
                                match &mut *old_parent.borrow_mut() {
                                    RBNode::Node {
                                        left: parent_left,
                                        right: parent_right,
                                        ..
                                    } => {
                                        if Rc::ptr_eq(&parent_left, &curr) {
                                            *parent_left = right.clone();
                                        } else if Rc::ptr_eq(&parent_right, &curr) {
                                            *parent_right = right.clone();
                                        }

                                        *right_left = left.clone();
                                    }
                                    RBNode::Empty => {
                                        self.root = right.clone();
                                        //println!("empty parent");
                                        *right_left = left.clone();
                                    }
                                }
                                break;
                            } else {
                                match &mut *successor.borrow_mut() {
                                    RBNode::Node {
                                        left: successor_left,
                                        right: successor_right,
                                        parent: successor_parent,
                                        color: successor_color,
                                        ..
                                    } => {
                                        if Rc::ptr_eq(&self.root, &curr) {
                                            self.root = successor.clone();
                                        }
                                        curr = successor_right.clone();
                                        old_color = successor_color.clone();
                                        match &mut *old_parent.borrow_mut() {
                                            RBNode::Node {
                                                left: parent_left,
                                                right: parent_right,
                                                ..
                                            } => {
                                                if Rc::ptr_eq(&parent_left, &curr) {
                                                    *parent_left = successor.clone();
                                                } else if Rc::ptr_eq(&parent_right, &curr) {
                                                    *parent_right = successor.clone();
                                                }
                                            }
                                            _ => (),
                                        };
                                        *successor_left = left.clone();
                                        let successor_old_right = successor_right.clone();
                                        let successor_parent_strong =
                                            match successor_parent.upgrade() {
                                                Some(_) => successor_parent.upgrade().unwrap(),
                                                None => Rc::new(RefCell::new(RBNode::Empty)),
                                            };

                                        let mut successor_parent_mut =
                                            if Rc::ptr_eq(&successor_parent_strong, &right) {
                                                right_node
                                            } else {
                                                successor_parent_strong.borrow_mut()
                                            };

                                        match &mut *successor_parent_mut {
                                            RBNode::Node {
                                                left: successor_parent_left,
                                                right: successor_parent_right,
                                                ..
                                            } => {
                                                if Rc::ptr_eq(&successor_parent_left, &successor) {
                                                    *successor_parent_left =
                                                        successor_right.clone();
                                                } else if Rc::ptr_eq(
                                                    &successor_parent_right,
                                                    &successor,
                                                ) {
                                                    *successor_parent_right =
                                                        successor_right.clone();
                                                }
                                            }
                                            _ => (),
                                        };
                                        *successor_right = right.clone();
                                        match &mut *successor_old_right.borrow_mut() {
                                            RBNode::Node {
                                                parent: successor_old_right_parent,
                                                ..
                                            } => {
                                                *successor_old_right_parent =
                                                    Rc::downgrade(&successor_parent_strong);
                                            }
                                            _ => (),
                                        };

                                        *successor_parent = Rc::downgrade(&old_parent);
                                    }
                                    _ => (),
                                }
                                //curr = successor.clone();
                                
                                break;
                            };
                        }
                    }
                }
            }
        }
        //println!("old color: {:?}", old_color);
        if old_color == Color::Black {
            delete_fixup(curr, &mut self.root);
        }
        //delete_fixup(curr, &mut self.root);
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
        if let RBNode::Empty = &*self.root.borrow() {
            true
        } else {
            false
        }
    }

    fn print_tree(&self) {
        if let RBNode::Empty = &*self.root.borrow() {
            println!("\nTree is empty\n");
            return;
        }
        ptree::print_tree(&*self.root.borrow()).expect("Failed to print tree");
    }
}
