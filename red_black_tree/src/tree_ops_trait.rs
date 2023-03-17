use std::{cell::RefCell, rc::Rc};

use crate::{RBNode, Tree};

fn rotate_left(x: Tree) {
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

    match &*x.borrow() {
        RBNode::Node {
            key,
            color,
            left,
            right,
            parent,
        } => {
            let parent = parent.upgrade().unwrap();
            let y = right.clone();
            let T2 = match &*y.borrow_mut() {
                RBNode::Node { left, .. } => left.clone(),
                RBNode::Empty => Rc::new(RefCell::new(RBNode::Empty)),
            };

            match &*y.borrow_mut() {
                RBNode::Node { left, .. } => todo!(), // idk if this is right, cuz we swapping not assigning
                RBNode::Empty => (),
            };
        }
        RBNode::Empty => return,
    }
}

fn rotate_right(x: Tree) {
    todo!()
}

fn fix_violation(x: Tree) {
    todo!()
}
