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
    println!("Rotate left with node: {:#?}", x);
    match &mut *x.borrow_mut() {
        RBNode::Node { right, parent, .. } => {
            let old_parent = match parent.upgrade() {
                Some(_) => parent.upgrade().unwrap(),
                None => Rc::new(RefCell::new(RBNode::Empty)),
            }; // parent = x.parent
            let y = right.clone(); // y = x.right
            *parent = Rc::downgrade(&y.clone());
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
    println!("Rotate right with node: {:#?}", y);
    match &mut *y.borrow_mut() {
        RBNode::Node { left, parent, .. } => {
            let old_parent = match parent.upgrade() {
                Some(_) => parent.upgrade().unwrap(),
                None => Rc::new(RefCell::new(RBNode::Empty)),
            }; // parent = y.parent
            let x = left.clone(); // x = y.left
            *parent = Rc::downgrade(&x.clone());

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

                    let parent = match &*curr.borrow() {
                        RBNode::Node { parent, .. } => match parent.upgrade() {
                            Some(n) => n,
                            None => break,
                        },
                        RBNode::Empty => break,
                    };

                    match &mut *parent.borrow_mut() {
                        RBNode::Node { color, .. } => *color = Color::Black,
                        _ => (),
                    };

                    let grandparent = match &*parent.borrow() {
                        RBNode::Node { parent, .. } => match parent.upgrade() {
                            Some(n) => n,
                            None => break,
                        },
                        RBNode::Empty => break,
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
                    let parent = match &*curr.borrow() {
                        RBNode::Node { parent, .. } => match parent.upgrade() {
                            Some(n) => n,
                            None => break,
                        },
                        RBNode::Empty => break,
                    };

                    match &mut *parent.borrow_mut() {
                        RBNode::Node { color, .. } => *color = Color::Black,
                        _ => (),
                    };

                    let grandparent = match &*parent.borrow() {
                        RBNode::Node { parent, .. } => match parent.upgrade() {
                            Some(n) => n,
                            None => break,
                        },
                        RBNode::Empty => break,
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

// def delete_fixup(self, x): x -> curr5
// while x != self.root and x.color == BLACK:
//     if x == x.p.left:
//         w = x.p.right
//         # type 1
//         if w.color == RED:
//             w.color = BLACK
//             x.p.color = RED
//             self.left_rotate(x.p)
//             w = x.p.right
//         # type 2
//         if w.left.color == BLACK and w.right.color == BLACK:
//             w.color = RED 
//             x = x.p 
//         else:
//             # type 3
//             if w.right.color == BLACK:
//                 w.left.color = BLACK
//                 w.color = RED
//                 self.right_rotate(w)
//                 w = x.p.right
//             # type 4
//             w.color = x.p.color 
//             x.p.color = BLACK 
//             w.right.color = BLACK 
//             self.left_rotate(x.p)
//             x = self.root
//     else:
//         w = x.p.left
//         # type 1
//         if w.color == RED:
//             w.color = BLACK
//             x.p.color = RED
//             self.right_rotate(x.p)
//             w = x.p.left
//         # type 2
//         if w.right.color == BLACK and w.left.color == BLACK:
//             w.color = RED 
//             x = x.p 
//         else:
//             # type 3
//             if w.left.color == BLACK:
//                 w.right.color = BLACK
//                 w.color = RED
//                 self.left_rotate(w)
//                 w = x.p.left
//             # type 4
//             w.color = x.p.color 
//             x.p.color = BLACK 
//             w.left.color = BLACK 
//             self.right_rotate(x.p)
//             x = self.root
// x.color = BLACK

// delete fixup based on and above python code
pub fn delete_fixup(x: Tree, root: &mut Tree) {
    let mut curr = x;

    loop {
        if Rc::ptr_eq(&curr, root) {
            break;
        }
        let curr_parent = match &*curr.borrow() {
            RBNode::Node {
                color: node_color,
                parent: node_parent,
                ..
            } => {
                if node_color == &Color::Red {
                    break;
                }
                node_parent.clone()
            },
            _ => break,
        };

        let curr_parent = match curr_parent.upgrade() {
            Some(n) => n,
            None => break,
        };

        let curr_parent_left = match &*curr_parent.borrow() {
            RBNode::Node { left, .. } => left.clone(),
            _ => break,
        };
            
        ;
        if Rc::ptr_eq(&curr, &curr_parent_left) {
            let mut w = match &*curr_parent.borrow() {
                RBNode::Node { right, .. } => right.clone(),
                _ => break,
            };
            // type 1
            let mut flag = false;
            match &mut *w.borrow_mut() {
                RBNode::Node { color, .. } if color == &Color::Red => {
                    *color = Color::Black;
                    match &mut *curr_parent.borrow_mut() {
                        RBNode::Node { color, .. } => *color = Color::Red,
                        _ => (),
                    };
                    rotate_left(&curr_parent, root);
                    flag=true;
                }
                _ => (),
            };
            if flag {
                w = match &*curr_parent.borrow() {
                    RBNode::Node { right, .. } => right.clone(),
                    _ => w,
                };
            }
            
            
            // type 2
            flag = false;
            match &mut *w.borrow_mut() {
                RBNode::Node {
                    color: w_color,
                    left: w_left,
                    right: w_right,
                    ..
                } => {
                    match (&*w_left.borrow(), &*w_right.borrow()) {
                        (RBNode::Node{color: w_left_color, ..}, RBNode::Node{color:w_right_color, ..}) if w_left_color==&Color::Black && w_right_color==&Color::Black => {
                            *w_color = Color::Red;
                            curr = curr_parent.clone();
                            continue;
                        }
                        _ => (),
                        //type 3
                        
                    };
                    match  (&mut *w_left.borrow_mut(), &*w_right.borrow_mut()){
                        (RBNode::Node{color: w_left_color, ..}, RBNode::Node{color:w_right_color, ..}) => {
                            if w_right_color == &Color::Black {
                                *w_left_color = Color::Black;
                                /*match &mut *w_left.borrow_mut() {
                                    RBNode::Node { color, .. } => ,
                                    _ => (),
                                };*/
                                *w_color = Color::Red;
                                rotate_right(&w, root);
                                flag = true;
                            }
                            
                        },
                        _ => (),
                    }
                }
                _ => (),
            };
            if flag {w = match &*curr_parent.borrow() {
                RBNode::Node { right, .. } => right.clone(),
                _ => break,
            };}
            //type 4
            match &mut *w.borrow_mut() {
                RBNode::Node { color, .. } => *color = match &*curr_parent.borrow() {
                    RBNode::Node { color, .. } => color.clone(),
                    _ => Color::Black,
                },
                _ => (),
            };
            match &mut *curr_parent.borrow_mut() {
                RBNode::Node { color, .. } => *color = Color::Black,
                _ => (),
            };
            match &*w.borrow() {
                RBNode::Node { right, .. } => match &mut *right.borrow_mut() {
                    RBNode::Node { color, .. } => *color = Color::Black,
                    _ => (),
                },
                _ => (),
            };
            rotate_left(&curr_parent, root);
            curr = root.clone();
            break;
        }
        else {
            let mut w = match &*curr_parent.borrow() {
                RBNode::Node { left, .. } => left.clone(),
                _ => break,
            };
            // type 1
            let mut flag = false;
            match &mut *w.borrow_mut() {
                RBNode::Node { color, .. } if color == &Color::Red => {
                    *color = Color::Black;
                    match &mut *curr_parent.borrow_mut() {
                        RBNode::Node { color, .. } => *color = Color::Red,
                        _ => (),
                    };
                    rotate_right(&curr_parent, root);
                    flag=true;
                }
                _ => (),
            };
            if flag {w = match &*curr_parent.borrow() {
                RBNode::Node { left, .. } => left.clone(),
                _ => w,
            };}
            flag = false;
            // type 2
            match &mut *w.borrow_mut() {
                RBNode::Node {
                    color: w_color,
                    left: w_left,
                    right: w_right,
                    ..
                } => {
                    match (&*w_left.borrow(), &*w_right.borrow()) {
                        (RBNode::Node{color: w_left_color, ..}, RBNode::Node{color:w_right_color, ..}) if w_left_color==&Color::Black && w_right_color==&Color::Black => {
                            *w_color = Color::Red;
                            curr = curr_parent.clone();
                            continue;
                        }
                        _ => (),
                        //type 3
                    }
                    match (&mut *w_left.borrow_mut(), &mut *w_right.borrow_mut()) {
                        (RBNode::Node{color: w_left_color, ..}, RBNode::Node{color:w_right_color, ..}) => {
                            if w_left_color == &Color::Black {
                                *w_right_color = Color::Black;
                                *w_color = Color::Red;
                                rotate_left(&w, root);
                                flag = true;
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            };
            if flag {w = match &*curr_parent.borrow() {
                RBNode::Node { left, .. } => left.clone(),
                _ => break,
            };}
            match &mut *w.borrow_mut() {
                RBNode::Node { color:w_color, left:w_left, .. } => {
                    //type 4
                    *w_color = match &*curr_parent.borrow() {
                        RBNode::Node { color, .. } => color.clone(),                             
                        _ => Color::Black,
                    };
                    match &mut *curr_parent.borrow_mut() {
                        RBNode::Node { color, .. } => *color = Color::Black,
                        _ => (),
                    };
                    match &mut *w_left.borrow_mut() {
                            RBNode::Node { color, .. } => *color = Color::Black,
                            _ => (),
                    }
                    rotate_right(&curr_parent, root);
                    curr = root.clone();
                },
                _ => (),
            };
        }
    }
                
    match &mut *curr.borrow_mut() {
        RBNode::Node { color, .. } => *color = Color::Black,
        _ => (),
    };
}