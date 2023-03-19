use crate::{AVLNode, Tree};
use std::{cell::RefCell, rc::Rc};

/**
Rotates left
   Ex:
         y                               x
        / \                             /  \
       x   T3                          T1   y
      / \       < - - - - - - -            / \
     T1  T2     Left Rotation            T2  T3

   # Rotation
   parent = x.parent
   y = x.right
   T2 = y.left

   y.left = x
   x.right = T2

   if T2:
       T2.parent = x
   y.parent = parent

   if parent is None:
       self.root = y
   elif parent.left == x:
       parent.left = y
   elif y.parent.right == x:
       parent.right = y
*/

pub fn rotate_left(x: &Tree, root: &mut Tree) {
    println!("Rotate left with node: {:#?}", x);
    match &mut *x.borrow_mut() {
        AVLNode::Node { right, parent, .. } => {
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
        }
        _ => (),
    }
}

/**
Rotates right
Ex:

         y                               x
        / \     Right Rotation          /  \
       x   T3   - - - - - - - >        T1   y
      / \                                  / \
     T1  T2                              T2  T3
# Rotation
parent = y.parent

x = y.left
T2 = x.right

x.right = y
y.left = T2

if T2:
    T2.parent = y
x.parent = parent


if parent is None:
    self.root = x
if parent.left == y:
    parent.left = x
elif y.parent.right == y:
    parent.right = x
*/
fn rotate_right(y: &Tree, root: &mut Tree) {
    println!("Rotate right with node: {:#?}", y);
    match &mut *y.borrow_mut() {
        AVLNode::Node { left, parent, .. } => {
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
        }
        _ => (),
    }
}

fn insert_fixup() {
    todo!()
}
