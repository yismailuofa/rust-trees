use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use tree::TreeTrait;
use tree_ops_trait::{insert_fixup, rotate_right};

use crate::{
    tree_ops_trait::{self, delete_fixup, find_node, rotate_left},
    Color, RBNode, RBTree,
};

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

    // def delete(self, k):
    //     z = self.search(k)

    //     if z == self.NIL:
    //         return "Key not found!"

    //     y = z
    //     y_orig_color = y.color

    //     # case 1
    //     if z.left == self.NIL:
    //         x = z.right
    //         self.transplant(z, z.right)
    //     # case 2
    //     elif z.right == self.NIL:
    //         x = z.left
    //         self.transplant(z, z.left)
    //     # case 3
    //     else:
    //         y = self.minimum(z.right)
    //         y_orig_color = y.color
    //         x = y.right

    //         if y.p == z:
    //             x.p = y
    //         else:
    //             self.transplant(y, y.right)
    //             y.right = z.right
    //             y.right.p = y

    //         self.transplant(z, y)
    //         y.left = z.left
    //         y.left.p = y
    //         y.color = z.color

    //      if x is nil and no kids: do the fixup manually

    //     if y_orig_color == BLACK:
    //         self.delete_fixup(x)
    fn delete_node(&mut self, _key: u32) {
        //delete node with key

        //find node with key
        let z = match find_node(&self.root, _key) {
            Some(node) => node,
            None => {
                println!("Node with key {} not found", _key);
                return;
            }
        };

        let y = z.clone();
        let mut y_orig_color = match &*y.borrow() {
            RBNode::Node { color, .. } => color.clone(),
            RBNode::Empty => panic!(), //should never happen
        };
        let x;
        // case 1
        match &mut *z.borrow_mut() {
            RBNode::Node {
                left: z_left,
                right: z_right,
                parent: z_parent,
                ..
            } => {
                match *z_left.borrow() {
                    RBNode::Empty => {
                        x = z_right.clone();
                        //transplant(z, z.right)
                        let z_parent = match z_parent.upgrade() {
                            Some(_) => z_parent.upgrade().unwrap(),
                            None => Rc::new(RefCell::new(RBNode::Empty)),
                        };
                        let mut z_parent_node = z_parent.borrow_mut();
                        match &mut *z_parent_node {
                            RBNode::Node {
                                left: z_parent_left,
                                right: z_parent_right,
                                ..
                            } => {
                                if Rc::ptr_eq(&z_parent_left, &z) {
                                    *z_parent_left = z_right.clone();
                                } else {
                                    *z_parent_right = z_right.clone();
                                }
                            }
                            RBNode::Empty => self.root = z_right.clone(),
                        }
                        // z.left.parent = z.parent
                        match &mut *z_right.borrow_mut() {
                            RBNode::Node { parent, .. } => {
                                *parent = Rc::downgrade(&z_parent);
                            }
                            RBNode::Empty => (),
                        }

                        if y_orig_color == Color::Black {
                            delete_fixup(x, &mut self.root);
                        }
                        return;
                    }
                    _ => (),
                }
                match *z_right.borrow() {
                    RBNode::Empty => {
                        x = z_left.clone();
                        //transplant(z, z.left)
                        let z_parent = match z_parent.upgrade() {
                            Some(_) => z_parent.upgrade().unwrap(),
                            None => Rc::new(RefCell::new(RBNode::Empty)),
                        };
                        let mut z_parent_node = z_parent.borrow_mut();
                        match &mut *z_parent_node {
                            RBNode::Node {
                                left: z_parent_left,
                                right: z_parent_right,
                                ..
                            } => {
                                if Rc::ptr_eq(&z_parent_left, &z) {
                                    *z_parent_left = z_left.clone();
                                } else {
                                    *z_parent_right = z_left.clone();
                                }
                            }
                            RBNode::Empty => self.root = z_left.clone(),
                        }
                        // z.left.parent = z.parent
                        match &mut *z_left.borrow_mut() {
                            RBNode::Node { parent, .. } => {
                                *parent = Rc::downgrade(&z_parent);
                            }
                            RBNode::Empty => (),
                        }

                        if y_orig_color == Color::Black {
                            delete_fixup(x, &mut self.root);
                        }
                        return;
                    }
                    _ => (),
                }
            }

            RBNode::Empty => panic!(), //should never happen
        };

        // case 3
        // y = self.minimum(z.right)
        let y = match &*z.borrow() {
            RBNode::Node { right: z_right, .. } => {
                let mut y = z_right.clone();
                loop {
                    match &*y.clone().borrow() {
                        RBNode::Node { left, .. } => {
                            let left_node = left.borrow();
                            if let RBNode::Empty = left_node.clone() {
                                break;
                            } else {
                                y = left.clone();
                            }
                        }
                        _ => break,
                    };
                }
                y
            }
            RBNode::Empty => panic!(), //should never happen
        };
        // y_orig_color = y.color
        y_orig_color = match &*y.borrow() {
            RBNode::Node { color, .. } => color.clone(),
            RBNode::Empty => panic!(), //should never happen
        };
        // x = y.right
        x = match &*y.borrow() {
            RBNode::Node { right, .. } => right.clone(),
            RBNode::Empty => panic!(), //should never happen
        };
        // if y.p == z:
        //     x.p = y
        let mut flag = false;
        let mut case_3_flag = false;
        match &mut *y.borrow_mut() {
            RBNode::Node {
                parent: y_parent,
                right: y_right,
                ..
            } => {
                let y_parent = match y_parent.upgrade() {
                    Some(_) => y_parent.upgrade().unwrap(),
                    None => Rc::new(RefCell::new(RBNode::Empty)),
                };
                if Rc::ptr_eq(&y_parent, &z) {
                    match &mut *x.borrow_mut() {
                        RBNode::Node { parent, .. } => {
                            *parent = Rc::downgrade(&y);
                        }
                        RBNode::Empty => (),
                    }
                } else {
                    // transplant(y, y.right)
                    let mut y_parent_node = y_parent.borrow_mut();
                    match &mut *y_parent_node {
                        RBNode::Node {
                            left: y_parent_left,
                            right: y_parent_right,
                            ..
                        } => {
                            if Rc::ptr_eq(&y_parent_left, &y) {
                                *y_parent_left = y_right.clone();
                            } else {
                                *y_parent_right = y_right.clone();
                            }
                        }
                        RBNode::Empty => self.root = y_right.clone(),
                    }
                    // y.right.parent = y.parent
                    match &mut *y_right.borrow_mut() {
                        RBNode::Node { parent, .. } => {
                            *parent = Rc::downgrade(&y_parent);
                        }
                        RBNode::Empty => (),
                    }
                    // y.right = z.right
                    match &*z.borrow() {
                        RBNode::Node { right, .. } => {
                            *y_right = right.clone();
                        }
                        RBNode::Empty => panic!(), //should never happen
                    }

                    flag = true;
                }
                //transplant(z, y)
            }
            RBNode::Empty => panic!(), //should never happen
        };
        if flag {
            match &*y.borrow_mut() {
                RBNode::Node { right: y_right, .. } => match &mut *y_right.borrow_mut() {
                    RBNode::Node { parent, .. } => {
                        *parent = Rc::downgrade(&y); //look here
                    }
                    RBNode::Empty => (),
                },
                RBNode::Empty => panic!(), //should never happen
            };
        }
        //transplant(z, y)
        match &mut *z.borrow_mut() {
            RBNode::Node {
                parent: z_parent,
                left: z_left,
                color: z_color,
                ..
            } => {
                let z_parent = match z_parent.upgrade() {
                    Some(_) => z_parent.upgrade().unwrap(),
                    None => Rc::new(RefCell::new(RBNode::Empty)),
                };
                let mut z_parent_node = z_parent.borrow_mut();
                match &mut *z_parent_node {
                    RBNode::Node {
                        left: z_parent_left,
                        right: z_parent_right,
                        ..
                    } => {
                        if Rc::ptr_eq(&z_parent_left, &z) {
                            *z_parent_left = y.clone();
                        } else {
                            *z_parent_right = y.clone();
                        }
                    }
                    RBNode::Empty => self.root = y.clone(),
                }
                // y.parent = z.parent
                match &mut *y.borrow_mut() {
                    RBNode::Node { parent, .. } => {
                        *parent = Rc::downgrade(&z_parent);
                    }
                    RBNode::Empty => (),
                }
                // y.left = z.left
                match &mut *y.borrow_mut() {
                    RBNode::Node { left, .. } => {
                        *left = z_left.clone();
                    }
                    RBNode::Empty => (),
                }
                // y.left.parent = y
                match &mut *z_left.borrow_mut() {
                    RBNode::Node { parent, .. } => {
                        *parent = Rc::downgrade(&y);
                    }
                    RBNode::Empty => (),
                }
                // y.color = z.color
                match &mut *y.borrow_mut() {
                    RBNode::Node { color, .. } => {
                        *color = z_color.clone();
                    }
                    RBNode::Empty => (),
                }

                // We need to to handle the case where x is empty
                match &*x.borrow() {
                    RBNode::Empty => case_3_flag = true,
                    _ => (),
                }
            }

            RBNode::Empty => panic!(), //should never happen
        };

        if case_3_flag {
            // rotate left on y.left
            // match &*y.borrow() {
            //     RBNode::Node { left, .. } => rotate_left(&left, &mut self.root),
            //     _ => (),
            // };
            let left = match &*y.borrow() {
                RBNode::Node { left, .. } => left.clone(),
                _ => Rc::new(RefCell::new(RBNode::Empty)),
            };

            rotate_left(&left, &mut self.root);

            rotate_right(&y, &mut self.root);

            // set root color to black
            match &mut *self.root.borrow_mut() {
                RBNode::Node { color, .. } => {
                    *color = Color::Black;
                }
                RBNode::Empty => (),
            }

            return;
        }

        if y_orig_color == Color::Black {
            delete_fixup(x, &mut self.root);
        }
    }

    //let mut old_color = Color::Black;

    //println!("old color: {:?}", old_color);

    // let mut left_node = left.borrow_mut();
    // let mut right_node = right.borrow_mut();
    // let old_parent = match parent.upgrade() {
    //     Some(rc) => rc,
    //     None => Rc::new(RefCell::new(RBNode::Empty)),
    // };
    //     old_color = color.clone();
    // //if node has no children, delete it
    // match (&mut *left_node, &mut *right_node) {
    //     (RBNode::Empty, RBNode::Empty) => {
    //         //delete node
    //         match &mut *old_parent.borrow_mut() {
    //             RBNode::Node {
    //                 left: parent_left,
    //                 right: parent_right,
    //                 ..
    //             } => {
    //                 if Rc::ptr_eq(&curr, &parent_left) {
    //                     *parent_left = Rc::new(RefCell::new(RBNode::Empty));
    //                 } else if Rc::ptr_eq(&curr, &parent_right) {
    //                     *parent_right = Rc::new(RefCell::new(RBNode::Empty));
    //                 }
    //             }
    //             RBNode::Empty => {
    //                 self.root = Rc::new(RefCell::new(RBNode::Empty));
    //             }
    //         };
    //         return;
    //     }
    //     // if a node has one child, replace it with the child
    //     (
    //         RBNode::Empty,
    //         RBNode::Node {
    //             parent: right_parent,
    //             ..
    //         },
    //     ) => {
    //         match &mut *old_parent.borrow_mut() {
    //             RBNode::Node {
    //                 left: parent_left,
    //                 right: parent_right,
    //                 ..
    //             } => {
    //                 if Rc::ptr_eq(&curr, &parent_left) {
    //                     *parent_left = right.clone();
    //                 } else if Rc::ptr_eq(&curr, &parent_right) {
    //                     *parent_right = right.clone();
    //                 }
    //             }
    //             RBNode::Empty => {
    //                 self.root = right.clone();
    //             }
    //         };
    //         *right_parent = Rc::downgrade(&old_parent);
    //         curr = right.clone();
    //         break;
    //     }
    //     (
    //         RBNode::Node {
    //             parent: left_parent,
    //             ..
    //         },
    //         RBNode::Empty,
    //     ) => {
    //         match &mut *old_parent.borrow_mut() {
    //             RBNode::Node {
    //                 left: parent_left,
    //                 right: parent_right,
    //                 ..
    //             } => {
    //                 if Rc::ptr_eq(&curr, &parent_left) {
    //                     *parent_left = left.clone();
    //                 } else if Rc::ptr_eq(&curr, &parent_right) {
    //                     *parent_right = left.clone();
    //                 }
    //             }
    //             RBNode::Empty => {
    //                 self.root = left.clone();
    //             }
    //         };
    //         curr = left.clone();
    //         *left_parent = Rc::downgrade(&old_parent);

    //         break;
    //     }
    //     (
    //         RBNode::Node { .. },
    //         RBNode::Node {
    //             left: right_left,right:right_right , color:right_color,..
    //         },
    //     ) => {
    //         //if node has two children, find the successor
    //         // see if right_left is of type RBNode::Empty
    //         // if it is, then right is the successor
    //         // if it isn't, then find the leftmost node of right_left
    //         // that is the successor

    // let successor = match &*right_left.borrow() {
    //     RBNode::Empty => right.clone(),
    //     _ => {
    //         let mut successor_candidate = right_left.clone();
    //         loop {
    //             match &*successor_candidate.clone().borrow() {
    //                 RBNode::Node { left, .. } => {
    //                     let left_node = left.borrow();
    //                     if let RBNode::Empty = left_node.clone() {
    //                         break;
    //                     } else {
    //                         successor_candidate = left.clone();
    //                     }
    //                 }
    //                 _ => break,
    //             };
    //         }
    //         successor_candidate
    //     }
    // };

    //         //replace node with successor
    //         if Rc::ptr_eq(&successor, right) {
    //             if Rc::ptr_eq(&self.root, &curr) {
    //                 self.root = successor.clone();
    //             }
    //             curr = right_right.clone();
    //             old_color = right_color.clone();
    //             match &mut *old_parent.borrow_mut() {
    //                 RBNode::Node {
    //                     left: parent_left,
    //                     right: parent_right,
    //                     ..
    //                 } => {
    //                     if Rc::ptr_eq(&parent_left, &curr) {
    //                         *parent_left = right.clone();
    //                     } else if Rc::ptr_eq(&parent_right, &curr) {
    //                         *parent_right = right.clone();
    //                     }

    //                     *right_left = left.clone();
    //                 }
    //                 RBNode::Empty => {
    //                     self.root = right.clone();
    //                     //println!("empty parent");
    //                     *right_left = left.clone();
    //                 }
    //             }
    //             break;
    //         } else {
    //             match &mut *successor.borrow_mut() {
    //                 RBNode::Node {
    //                     left: successor_left,
    //                     right: successor_right,
    //                     parent: successor_parent,
    //                     color: successor_color,
    //                     ..
    //                 } => {
    //                     if Rc::ptr_eq(&self.root, &curr) {
    //                         self.root = successor.clone();
    //                     }
    //                     curr = successor_right.clone();
    //                     old_color = successor_color.clone();
    //                     match &mut *old_parent.borrow_mut() {
    //                         RBNode::Node {
    //                             left: parent_left,
    //                             right: parent_right,
    //                             ..
    //                         } => {
    //                             if Rc::ptr_eq(&parent_left, &curr) {
    //                                 *parent_left = successor.clone();
    //                             } else if Rc::ptr_eq(&parent_right, &curr) {
    //                                 *parent_right = successor.clone();
    //                             }
    //                         }
    //                         _ => (),
    //                     };
    //                     *successor_left = left.clone();
    //                     let successor_old_right = successor_right.clone();
    //                     let successor_parent_strong =
    //                         match successor_parent.upgrade() {
    //                             Some(_) => successor_parent.upgrade().unwrap(),
    //                             None => Rc::new(RefCell::new(RBNode::Empty)),
    //                         };

    //                     let mut successor_parent_mut =
    //                         if Rc::ptr_eq(&successor_parent_strong, &right) {
    //                             right_node
    //                         } else {
    //                             successor_parent_strong.borrow_mut()
    //                         };

    //                     match &mut *successor_parent_mut {
    //                         RBNode::Node {
    //                             left: successor_parent_left,
    //                             right: successor_parent_right,
    //                             ..
    //                         } => {
    //                             if Rc::ptr_eq(&successor_parent_left, &successor) {
    //                                 *successor_parent_left =
    //                                     successor_right.clone();
    //                             } else if Rc::ptr_eq(
    //                                 &successor_parent_right,
    //                                 &successor,
    //                             ) {
    //                                 *successor_parent_right =
    //                                     successor_right.clone();
    //                             }
    //                         }
    //                         _ => (),
    //                     };
    //                     *successor_right = right.clone();
    //                     match &mut *successor_old_right.borrow_mut() {
    //                         RBNode::Node {
    //                             parent: successor_old_right_parent,
    //                             ..
    //                         } => {
    //                             *successor_old_right_parent =
    //                                 Rc::downgrade(&successor_parent_strong);
    //                         }
    //                         _ => (),
    //                     };

    //                     *successor_parent = Rc::downgrade(&old_parent);
    //                 }
    //                 _ => (),
    //             }
    //             //curr = successor.clone();

    //             break;
    //         };
    //     }
    // }

    /*     if old_color == Color::Black {
        delete_fixup(curr, &mut self.root);
    }*/
    //delete_fixup(curr, &mut self.root);

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
