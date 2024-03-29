use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{Color, RBNode, Tree};

/**
Rotates left
   Ex:
         y                               x
        / \                             /  \
       x   T3                          T1   y
      / \       < - - - - - - -            / \
     T1  T2     Left Rotation            T2  T3

*/

pub fn rotate_left(x: &Tree, root: &mut Tree) {
    //println!("Rotate left with node: {:#?}", x);
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

// Rotates right
// Ex:

//          y                               x
//         / \     Right Rotation          /  \
//        x   T3   - - - - - - - >        T1   y
//       / \                                  / \
//      T1  T2                              T2  T3

pub fn rotate_right(y: &Tree, root: &mut Tree) {
    //println!("Rotate right with node: {:#?}", y);
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

        //println!("{:?} curr", curr);

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
            RBNode::Empty => break, // Should be a case 4
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
                        rotate_left(&curr, root);
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
                        rotate_right(&curr, root);
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

// delete fixup based on and above python code
pub fn find_node(root: &Tree, _key: u32) -> Option<Tree> {
    let mut curr = root.clone();

    while let RBNode::Node {
        key, left, right, ..
    } = &*curr.clone().borrow()
    {
        match _key.cmp(key) {
            std::cmp::Ordering::Less => {
                let left_node = left.borrow_mut();

                if let RBNode::Empty = *left_node {
                    println!("Node not found");
                    return None;
                } else {
                    curr = left.clone();
                }
            }
            std::cmp::Ordering::Greater => {
                let right_node = right.borrow_mut();

                if let RBNode::Empty = *right_node {
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

pub fn delete_fixup(x: Tree, root: &mut Tree, x_parent: &Tree) {
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
            }
            _ => Weak::new(),
        };

        let curr_parent = match curr_parent.upgrade() {
            Some(n) => n,
            None => x_parent.clone(), //maybe set to panic
        };

        let curr_parent_left = match &*curr_parent.borrow() {
            RBNode::Node { left, .. } => left.clone(),
            _ => Rc::new(RefCell::new(RBNode::Empty)),
        };

        if Rc::ptr_eq(&curr, &curr_parent_left) {
            let mut w = match &*curr_parent.borrow() {
                RBNode::Node { right, .. } => right.clone(),
                _ => Rc::new(RefCell::new(RBNode::Empty)),
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
                    flag = true;
                }
                _ => (),
            };
            if flag {
                rotate_left(&curr_parent, root);
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
                        (
                            RBNode::Node {
                                color: w_left_color,
                                ..
                            },
                            RBNode::Node {
                                color: w_right_color,
                                ..
                            },
                        ) if w_left_color == &Color::Black && w_right_color == &Color::Black => {
                            *w_color = Color::Red;
                            curr = curr_parent.clone();
                            continue;
                        }
                        (RBNode::Empty, RBNode::Empty) => {
                            *w_color = Color::Red;
                            curr = curr_parent.clone();
                            continue;
                        }
                        _ => (),
                        //type 3
                    };
                    match (&mut *w_left.borrow_mut(), &*w_right.borrow()) {
                        (
                            RBNode::Node {
                                color: w_left_color,
                                ..
                            },
                            RBNode::Node {
                                color: w_right_color,
                                ..
                            },
                        ) => {
                            if w_right_color == &Color::Black {
                                *w_left_color = Color::Black;
                                /*match &mut *w_left.borrow_mut() {
                                    RBNode::Node { color, .. } => ,
                                    _ => (),
                                };*/
                                *w_color = Color::Red;
                                flag = true;
                            }
                        }
                        (
                            RBNode::Node {
                                color: w_left_color,
                                ..
                            },
                            RBNode::Empty,
                        ) => {
                            *w_left_color = Color::Black;
                            *w_color = Color::Red;
                            flag = true;
                        }
                        (RBNode::Empty, RBNode::Empty) => {
                            *w_color = Color::Red;
                            flag = true;
                        }
                        _ => (),
                    }
                }
                _ => (),
            };
            if flag {
                rotate_right(&w, root);
                w = match &*curr_parent.borrow() {
                    RBNode::Node { right, .. } => right.clone(),
                    _ => Rc::new(RefCell::new(RBNode::Empty)),
                };
            }
            //type 4
            match &mut *w.borrow_mut() {
                RBNode::Node { color: w_color, .. } => match &*curr_parent.borrow() {
                    RBNode::Node {
                        color: parent_color,
                        ..
                    } => *w_color = *parent_color,
                    _ => *w_color = Color::Black,
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
        } else {
            let mut w = match &*curr_parent.borrow() {
                RBNode::Node { left, .. } => left.clone(),
                _ => Rc::new(RefCell::new(RBNode::Empty)),
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
                    flag = true;
                }
                _ => (),
            };
            if flag {
                rotate_right(&curr_parent, root);
                w = match &*curr_parent.borrow() {
                    RBNode::Node { left, .. } => left.clone(),
                    _ => w,
                };
            }
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
                        (
                            RBNode::Node {
                                color: w_left_color,
                                ..
                            },
                            RBNode::Node {
                                color: w_right_color,
                                ..
                            },
                        ) if w_left_color == &Color::Black && w_right_color == &Color::Black => {
                            *w_color = Color::Red;
                            curr = curr_parent.clone();
                            continue;
                        }
                        (RBNode::Empty, RBNode::Empty) => {
                            *w_color = Color::Red;
                            curr = curr_parent.clone();
                            continue;
                        }
                        _ => (),
                        //type 3
                    }
                    match (&mut *w_left.borrow_mut(), &mut *w_right.borrow_mut()) {
                        (
                            RBNode::Node {
                                color: w_left_color,
                                ..
                            },
                            RBNode::Node {
                                color: w_right_color,
                                ..
                            },
                        ) => {
                            if w_left_color == &Color::Black {
                                *w_right_color = Color::Black;
                                *w_color = Color::Red;
                                //rotate_left(&w, root);
                                flag = true;
                            }
                        }
                        (
                            RBNode::Empty,
                            RBNode::Node {
                                color: w_right_color,
                                ..
                            },
                        ) => {
                            *w_right_color = Color::Black;
                            *w_color = Color::Red;
                            flag = true;
                        }
                        (RBNode::Empty, RBNode::Empty) => {
                            *w_color = Color::Red;
                            flag = true;
                        }
                        _ => (),
                    }
                }
                _ => (),
            };
            if flag {
                rotate_left(&w, root);
                w = match &*curr_parent.borrow() {
                    RBNode::Node { left, .. } => left.clone(),
                    _ => Rc::new(RefCell::new(RBNode::Empty)),
                };
            }
            flag = false;
            match &mut *w.borrow_mut() {
                RBNode::Node {
                    color: w_color,
                    left: w_left,
                    ..
                } => {
                    //type 4

                    match &*curr_parent.borrow() {
                        RBNode::Node { color, .. } => *w_color = *color,
                        _ => (),
                    };
                    match &mut *curr_parent.borrow_mut() {
                        RBNode::Node { color, .. } => *color = Color::Black,
                        _ => (),
                    };
                    match &mut *w_left.borrow_mut() {
                        RBNode::Node { color, .. } => *color = Color::Black,
                        _ => (),
                    }
                    flag = true;
                }
                _ => (),
            };
            if flag {
                rotate_right(&curr_parent, root);
            }
            curr = root.clone();
        }
    }

    match &mut *curr.borrow_mut() {
        RBNode::Node { color, .. } => *color = Color::Black,
        _ => (),
    };
}
