use crate::{AVLNode, Tree};
use std::{cell::RefCell, rc::Rc};

// Rotates left
//    Ex:
//          y                               x
//         / \                             /  \
//        x   T3                          T1   y
//       / \       < - - - - - - -            / \
//      T1  T2     Left Rotation            T2  T3

pub fn rotate_left(x: &Tree, root: &mut Tree) -> Tree {
    match &mut *x.borrow_mut() {
        AVLNode::Node {
            left,
            right,
            parent,
            height,
            ..
        } => {
            let old_parent = match parent.upgrade() {
                Some(_) => parent.upgrade().unwrap(),
                None => Rc::new(RefCell::new(AVLNode::Empty)),
            }; // parent = x.parent
            let y = right.clone(); // y = x.right
            *parent = Rc::downgrade(&y.clone());
            // T2 = y.left
            let t2 = match &*y.borrow() {
                AVLNode::Node { left, .. } => left.clone(),
                AVLNode::Empty => Rc::new(RefCell::new(AVLNode::Empty)),
            };

            // y.left = x
            match &mut *y.borrow_mut() {
                AVLNode::Node { left, .. } => *left = x.clone(),
                _ => (),
            };

            //x.right = t2
            *right = t2.clone();

            // if t2:
            //     t2.parent = x
            match &mut *t2.borrow_mut() {
                AVLNode::Node { parent, .. } => *parent = Rc::downgrade(x),
                _ => (),
            };

            // y.parent = parent
            match &mut *y.borrow_mut() {
                AVLNode::Node { parent, .. } => *parent = Rc::downgrade(&old_parent),
                _ => (),
            };

            // if parent is None:
            //     self.root = y
            // elif parent.left == x:
            //     parent.left = y
            // elif y.parent.right == x:
            //     parent.right = y
            match &mut *old_parent.borrow_mut() {
                AVLNode::Node { left, right, .. } => {
                    if Rc::ptr_eq(&left, &x) {
                        *left = y.clone();
                    } else if Rc::ptr_eq(&right, &x) {
                        *right = y.clone();
                    }
                }
                _ => *root = y.clone(),
            };

            // update height
            *height = 1 + std::cmp::max(left.borrow().height(false), right.borrow().height(false));

            match &mut *y.borrow_mut() {
                AVLNode::Node {
                    height: y_height,
                    right,
                    ..
                } => *y_height = 1 + std::cmp::max(*height, right.borrow().height(false)),
                _ => (),
            };

            y
        }
        _ => Rc::new(RefCell::new(AVLNode::Empty)),
    }
}

// Rotates right
// Ex:

//          y                               x
//         / \     Right Rotation          /  \
//        x   T3   - - - - - - - >        T1   y
//       / \                                  / \
//      T1  T2                              T2  T3

pub fn rotate_right(y: &Tree, root: &mut Tree) -> Tree {
    match &mut *y.borrow_mut() {
        AVLNode::Node {
            left,
            right,
            parent,
            height,
            ..
        } => {
            let old_parent = match parent.upgrade() {
                Some(_) => parent.upgrade().unwrap(),
                None => Rc::new(RefCell::new(AVLNode::Empty)),
            }; // parent = y.parent
            let x = left.clone(); // x = y.left
            *parent = Rc::downgrade(&x.clone());

            // T2 = x.right
            let t2 = match &*x.borrow() {
                AVLNode::Node { right, .. } => right.clone(),
                AVLNode::Empty => Rc::new(RefCell::new(AVLNode::Empty)),
            };

            // x.right = y
            match &mut *x.borrow_mut() {
                AVLNode::Node { right, .. } => *right = y.clone(),
                _ => (),
            };

            // y.left = t2
            *left = t2.clone();

            // if t2:
            //     t2.parent = y
            match &mut *t2.borrow_mut() {
                AVLNode::Node { parent, .. } => *parent = Rc::downgrade(y),
                _ => (),
            };

            // x.parent = parent
            match &mut *x.borrow_mut() {
                AVLNode::Node { parent, .. } => *parent = Rc::downgrade(&old_parent),
                _ => (),
            };

            // if parent is None:
            //     self.root = x
            // elif parent.left == y:
            //     parent.left = x
            // elif y.parent.right == y:
            //     parent.right = x
            match &mut *old_parent.borrow_mut() {
                AVLNode::Node { left, right, .. } => {
                    if Rc::ptr_eq(&left, &y) {
                        *left = x.clone();
                    } else if Rc::ptr_eq(&right, &y) {
                        *right = x.clone();
                    }
                }
                _ => *root = x.clone(),
            };

            *height = 1 + std::cmp::max(left.borrow().height(false), right.borrow().height(false));

            match &mut *x.borrow_mut() {
                AVLNode::Node {
                    height: x_height,
                    left,
                    ..
                } => *x_height = 1 + std::cmp::max(left.borrow().height(false), *height),
                _ => (),
            };

            x
        }
        _ => Rc::new(RefCell::new(AVLNode::Empty)),
    }
}

pub fn find_node(root: &Tree, _key: u32) -> Option<Tree> {
    let mut curr = root.clone();

    while let AVLNode::Node {
        key, left, right, ..
    } = &*curr.clone().borrow()
    {
        match _key.cmp(key) {
            std::cmp::Ordering::Less => {
                let left_node = left.borrow_mut();

                if let AVLNode::Empty = *left_node {
                    println!("Node not found");
                    return None;
                } else {
                    curr = left.clone();
                }
            }
            std::cmp::Ordering::Greater => {
                let right_node = right.borrow_mut();

                if let AVLNode::Empty = *right_node {
                    println!("Node not found");
                    return None;
                } else {
                    curr = right.clone();
                }
            }
            std::cmp::Ordering::Equal => {
                return Some(curr);
            }
        }
    }
    None
}
