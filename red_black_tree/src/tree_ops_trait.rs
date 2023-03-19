use std::{cell::RefCell, rc::Rc};

use crate::{Color, RBNode, Tree};

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
    match &mut *x.borrow_mut() {
        RBNode::Node { right, parent, .. } => {
            let old_parent = match parent.upgrade() {
                Some(_) => parent.upgrade().unwrap(),
                None => Rc::new(RefCell::new(RBNode::Empty)),
            }; // parent = x.parent
            let y = right.clone(); // y = x.right

            // T2 = y.left
            let t2 = match &*y.borrow() {
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
            match &mut *t2.borrow_mut() {
                RBNode::Node { parent, .. } => *parent = Rc::downgrade(x),
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
            match &mut *old_parent.borrow_mut() {
                RBNode::Node { left, right, .. } => {
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
    match &mut *y.borrow_mut() {
        RBNode::Node { left, parent, .. } => {
            let old_parent = match parent.upgrade() {
                Some(_) => parent.upgrade().unwrap(),
                None => Rc::new(RefCell::new(RBNode::Empty)),
            }; // parent = y.parent
            let x = left.clone(); // x = y.left

            // T2 = x.right
            let t2 = match &*x.borrow() {
                RBNode::Node { right, .. } => right.clone(),
                RBNode::Empty => Rc::new(RefCell::new(RBNode::Empty)),
            };

            // x.right = y
            match &mut *x.borrow_mut() {
                RBNode::Node { right, .. } => *right = y.clone(),
                _ => (),
            };

            // y.left = t2
            *left = t2.clone();

            // if t2:
            //     t2.parent = y
            match &mut *t2.borrow_mut() {
                RBNode::Node { parent, .. } => *parent = Rc::downgrade(y),
                _ => (),
            };

            // x.parent = parent
            match &mut *x.borrow_mut() {
                RBNode::Node { parent, .. } => *parent = Rc::downgrade(&old_parent),
                _ => (),
            };

            // if parent is None:
            //     self.root = x
            // elif parent.left == y:
            //     parent.left = x
            // elif y.parent.right == y:
            //     parent.right = x
            match &mut *old_parent.borrow_mut() {
                RBNode::Node { left, right, .. } => {
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
// def insert_fixup(self, z):
//     while z.p and z.p.color == RED:
//         if z.p == z.p.p.left:
//             y = z.p.p.right
//             if y.color == RED:
//                 z.p.color = BLACK
//                 y.color = BLACK
//                 z.p.p.color = RED
//                 z = z.p.p
//             else:
//                 if z == z.p.right:
//                     z = z.p
//                     self.left_rotate(z)
//                 z.p.color = BLACK
//                 z.p.p.color = RED
//                 self.right_rotate(z.p.p)
//         else:
//             y = z.p.p.left
//             if y.color == RED:
//                 z.p.color = BLACK
//                 y.color = BLACK
//                 z.p.p.color = RED
//                 z = z.p.p
//             else:
//                 if z == z.p.left:
//                     z = z.p
//                     self.right_rotate(z)
//                 z.p.color = BLACK
//                 z.p.p.color = RED
//                 self.left_rotate(z.p.p)
//         if z == self.root:
//             break
//     self.root.color = BLACK
pub fn insert_fixup(x: Tree, root: &mut Tree) {
    let mut curr = x;

    loop {
        let node_parent = match &*curr.borrow() {
            RBNode::Node {
                parent: node_parent,
                ..
            } => node_parent.clone(),
            _ => break,
        };

        println!("{:?} curr", curr);

        let parent = match node_parent.upgrade() {
            Some(n) => {
                match &*n.borrow() {
                    RBNode::Node { color, .. } => {
                        if color == &Color::Black {
                            break;
                        }
                    }
                    _ => break,
                };
                n
            }
            None => break,
        };

        let grandparent = match &*parent.borrow() {
            RBNode::Node { parent, .. } => match parent.upgrade() {
                Some(n) => n,
                None => break,
            },
            RBNode::Empty => break,
        };

        let uncle = match &*grandparent.borrow() {
            RBNode::Node { left, right, .. } => {
                if Rc::ptr_eq(&left, &parent) {
                    right.clone()
                } else {
                    left.clone()
                }
            }
            RBNode::Empty => break,
        };

        match &mut *uncle.borrow_mut() {
            RBNode::Node { color, .. } if color == &Color::Red => {
                match &mut *parent.borrow_mut() {
                    RBNode::Node { color, .. } => *color = Color::Black,
                    _ => (),
                };

                // Uncle
                *color = Color::Black;

                match &mut *grandparent.borrow_mut() {
                    RBNode::Node { color, .. } => *color = Color::Red,
                    _ => (),
                };

                curr = grandparent.clone();
            }
            _ => {
                let parent_left = match &*parent.borrow() {
                    RBNode::Node { left, .. } => left.clone(),
                    RBNode::Empty => break,
                };

                let parent_right = match &*parent.borrow() {
                    RBNode::Node { right, .. } => right.clone(),
                    RBNode::Empty => break,
                };

                let grandparent_left = match &*grandparent.borrow() {
                    RBNode::Node { left, .. } => left.clone(),
                    RBNode::Empty => break,
                };

                if Rc::ptr_eq(&parent, &grandparent_left) {
                    if Rc::ptr_eq(&curr, &parent_right) {
                        curr = parent.clone();
                        rotate_left(&parent, root);
                    }
                    match &mut *parent.borrow_mut() {
                        RBNode::Node { color, .. } => *color = Color::Black,
                        _ => (),
                    };
                    match &mut *grandparent.borrow_mut() {
                        RBNode::Node { color, .. } => *color = Color::Red,
                        _ => (),
                    };
                    rotate_right(&grandparent, root);
                } else {
                    if Rc::ptr_eq(&curr, &parent_left) {
                        curr = parent.clone();
                        rotate_right(&parent, root);
                    }
                    match &mut *parent.borrow_mut() {
                        RBNode::Node { color, .. } => *color = Color::Black,
                        _ => (),
                    };
                    match &mut *grandparent.borrow_mut() {
                        RBNode::Node { color, .. } => *color = Color::Red,
                        _ => (),
                    };
                    rotate_left(&grandparent, root);
                }
            }
        };

        if Rc::ptr_eq(&curr, root) {
            break;
        }
    }

    match &mut *root.borrow_mut() {
        RBNode::Node { color, .. } => *color = Color::Black,
        _ => (),
    };
}
