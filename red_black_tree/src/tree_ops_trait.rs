use std::{cell::RefCell, rc::Rc};

use crate::{print, RBNode, Tree};

pub fn rotate_left(x: &Tree) -> Option<Tree> {
    // Rotates left
    // Ex:
    //       y                               x
    //      / \                             /  \
    //     x   T3                          T1   y
    //    / \       < - - - - - - -            / \
    //   T1  T2     Left Rotation            T2  T3

    // # Rotation
    // parent = x.parent
    // y = x.right
    // T2 = y.left

    // y.left = x
    // x.right = T2

    // if T2:
    //     T2.parent = x
    // y.parent = parent

    // if parent is None:
    //     self.root = y
    // elif parent.left == x:
    //     parent.left = y
    // elif y.parent.right == x:
    //     parent.right = y

    match &mut *x.borrow_mut() {
        RBNode::Node { right, parent, .. } => {
            let old_parent = match parent.upgrade() {
                Some(_) => parent.upgrade().unwrap(),
                None => Rc::new(RefCell::new(RBNode::Empty)),
            }; // parent = x.parent
            let y = right.clone(); // y = x.right

            // T2 = y.left
            let t2 = match &*y.borrow_mut() {
                RBNode::Node { left, .. } => left.clone(),
                RBNode::Empty => Rc::new(RefCell::new(RBNode::Empty)),
            };

            // y.left = x
            match &mut *y.borrow_mut() {
                RBNode::Node { left, .. } => *left = x.clone(),
                _ => (),
            };

            //x.right = t2
            *right = t2.clone();

            // if t2:
            //     t2.parent = x
            match &*t2.borrow() {
                RBNode::Node { .. } => match &mut *t2.borrow_mut() {
                    RBNode::Node { parent, .. } => *parent = Rc::downgrade(&x),
                    _ => (),
                },
                _ => (),
            };

            // y.parent = parent
            match &mut *y.borrow_mut() {
                RBNode::Node { parent, .. } => *parent = Rc::downgrade(&old_parent),
                _ => (),
            };

            // if parent is None:
            //     self.root = y
            // elif parent.left == x:
            //     parent.left = y
            // elif y.parent.right == x:
            //     parent.right = y
            match &*old_parent.borrow() {
                RBNode::Node { left, right, .. } => {
                    if Rc::ptr_eq(&left, &x) {
                        match &mut *old_parent.borrow_mut() {
                            RBNode::Node { left, .. } => *left = y.clone(),
                            _ => (),
                        };
                    } else if Rc::ptr_eq(&right, &x) {
                        match &mut *old_parent.borrow_mut() {
                            RBNode::Node { right, .. } => *right = y.clone(),
                            _ => (),
                        };
                    }
                }
                _ => return Some(y), // this is the case where the old rotation point was the root, this is wht indicates to update root
            };
        }
        _ => (),
        
    }
    
    None
    // im sying
}

pub fn rotate_right(x: &Tree) -> Option<Tree> {
    match &mut *x.borrow_mut() {
        RBNode::Node { left, parent, .. } => {
            let old_parent = match parent.upgrade() {
                Some(_) => parent.upgrade().unwrap(),
                None => Rc::new(RefCell::new(RBNode::Empty)),
            }; // parent = x.parent
            let y = left.clone(); // y = x.right

            // T2 = y.left
            let t2 = match &*y.borrow_mut() {
                RBNode::Node { right, .. } => right.clone(),
                RBNode::Empty => Rc::new(RefCell::new(RBNode::Empty)),
            };

            // y.left = x
            match &mut *y.borrow_mut() {
                RBNode::Node { right, .. } => *right = x.clone(),
                _ => (),
            };

            //x.right = t2
            *left = t2.clone();

            // if t2:
            //     t2.parent = x
            match &*t2.borrow() {
                RBNode::Node { .. } => match &mut *t2.borrow_mut() {
                    RBNode::Node { parent, .. } => *parent = Rc::downgrade(&x),
                    _ => (),
                },
                _ => (),
            };

            // y.parent = parent
            match &mut *y.borrow_mut() {
                RBNode::Node { parent, .. } => *parent = Rc::downgrade(&old_parent),
                _ => (),
            };

            // if parent is None:
            //     self.root = y
            // elif parent.left == x:
            //     parent.left = y
            // elif y.parent.right == x:
            //     parent.right = y
            match &*old_parent.borrow() {
                RBNode::Node { left, right, .. } => {
                    if Rc::ptr_eq(&left, &x) {
                        match &mut *old_parent.borrow_mut() {
                            RBNode::Node { left, .. } => *left = y.clone(),
                            _ => (),
                        };
                    } else if Rc::ptr_eq(&right, &x) {
                        match &mut *old_parent.borrow_mut() {
                            RBNode::Node { right, .. } => *right = y.clone(),
                            _ => (),
                        };
                    }
                }
                _ => return Some(y), // this is the case where the old rotation point was the root, this is wht indicates to update root
            };
        }
        _ => (),
        
    }
    
    None
}

fn fix_violation(x: Tree) {
    todo!()
}
