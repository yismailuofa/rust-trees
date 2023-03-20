use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use tree::TreeTrait;
use tree_ops_trait::{rotate_left, rotate_right};

use crate::{tree_ops_trait, AVLNode, AVLTree};

impl TreeTrait for AVLTree {
    fn insert_node(&mut self, _key: u32) {
        // BASE CASE
        match &*self.root.clone().borrow() {
            AVLNode::Empty => {
                self.root = Rc::new(RefCell::new(AVLNode::Node {
                    key: _key,
                    height: 1,
                    left: Rc::new(RefCell::new(AVLNode::Empty)),
                    right: Rc::new(RefCell::new(AVLNode::Empty)),
                    parent: Weak::new(),
                }));
            }
            _ => (),
        }

        // INSERT AT CORRECT POSITION

        let mut curr = self.root.clone();

        while let AVLNode::Node {
            key,
            height,
            left,
            right,
            ..
        } = &mut *curr.clone().borrow_mut()
        {
            match _key.cmp(key) {
                std::cmp::Ordering::Less => {
                    let mut left_node = left.borrow_mut();

                    if let AVLNode::Empty = *left_node {
                        *left_node = AVLNode::Node {
                            key: _key,
                            height: 1,
                            left: Rc::new(RefCell::new(AVLNode::Empty)),
                            right: Rc::new(RefCell::new(AVLNode::Empty)),
                            parent: Rc::downgrade(&curr),
                        };
                        *height = std::cmp::max(left_node.height(), right.borrow().height()) + 1;

                        curr = left.clone();

                        break;
                    }
                    curr = left.clone();
                }
                std::cmp::Ordering::Greater => {
                    let mut right_node = right.borrow_mut();

                    if let AVLNode::Empty = *right_node {
                        *right_node = AVLNode::Node {
                            key: _key,
                            height: 1,
                            left: Rc::new(RefCell::new(AVLNode::Empty)),
                            right: Rc::new(RefCell::new(AVLNode::Empty)),
                            parent: Rc::downgrade(&curr),
                        };

                        *height = std::cmp::max(left.borrow().height(), right_node.height()) + 1;

                        curr = right.clone();

                        break;
                    }
                    curr = right.clone();
                }
                _ => return,
            }
        }

        // BALANCE TREE

        loop {
            match &*curr.clone().borrow() {
                AVLNode::Empty => break,
                _ => (),
            }

            let balance = curr.borrow().balance();

            match &mut *curr.clone().borrow_mut() {
                AVLNode::Node {
                    height,
                    left,
                    right,
                    ..
                } => {
                    *height = std::cmp::max(left.borrow().height(), right.borrow().height()) + 1;
                }

                _ => (),
            }

            if balance > 1 {
                let case = match &*curr.clone().borrow() {
                    AVLNode::Node { left, .. } => match &*left.borrow() {
                        AVLNode::Node { key: left_key, .. } => {
                            if _key < *left_key {
                                1
                            } else if _key > *left_key {
                                2
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    },
                    _ => 0,
                };

                // Avoids passing a borrow_mut() to rotate_right() and rotate_left()
                // which would cause a double borrow_mut() error
                match case {
                    1 => curr = rotate_right(&curr, &mut self.root),
                    2 => {
                        let left = match &*curr.borrow() {
                            AVLNode::Node { left, .. } => left.clone(),
                            _ => Rc::new(RefCell::new(AVLNode::Empty)),
                        };

                        let left_node = rotate_left(&left.clone(), &mut self.root);

                        match &mut *curr.clone().borrow_mut() {
                            AVLNode::Node { left, .. } => *left = left_node,
                            _ => (),
                        }

                        curr = rotate_right(&curr, &mut self.root);
                    }
                    _ => (),
                }
            } else if balance < -1 {
                let case = match &*curr.clone().borrow() {
                    AVLNode::Node { right, .. } => match &*right.borrow() {
                        AVLNode::Node { key: right_key, .. } => {
                            if _key < *right_key {
                                1
                            } else if _key > *right_key {
                                2
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    },
                    _ => 0,
                };

                // Avoids passing a borrow_mut() to rotate_right() and rotate_left()
                // which would cause a double borrow_mut() error
                match case {
                    1 => {
                        let right = match &*curr.borrow() {
                            AVLNode::Node { right, .. } => right.clone(),
                            _ => Rc::new(RefCell::new(AVLNode::Empty)),
                        };

                        let right_node = rotate_right(&right.clone(), &mut self.root);

                        match &mut *curr.clone().borrow_mut() {
                            AVLNode::Node { right, .. } => *right = right_node,
                            _ => (),
                        }

                        curr = rotate_left(&curr, &mut self.root);
                    }
                    2 => curr = rotate_left(&curr, &mut self.root),
                    _ => (),
                }
            }

            match &*curr.clone().borrow() {
                AVLNode::Node { parent, .. } => {
                    if let Some(parent) = parent.upgrade() {
                        curr = parent;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
    // def delete(self, root, key):
    //     if not root:
    //         return root

    //     parent = None
    //     curr = root
    //     while curr and curr.val != key:
    //         parent = curr
    //         if key < curr.val:
    //             curr = curr.left
    //         else:
    //             curr = curr.right

    //     if not curr:
    //         return root

    //     if not curr.left or not curr.right:
    //         child = curr.left or curr.right
    //         if not parent:
    //             root = child
    //         elif parent.left == curr:
    //             parent.left = child
    //         else:
    //             parent.right = child
    //     else:
    //         successor = curr.right
    //         successor_parent = curr
    //         while successor.left:
    //             successor_parent = successor
    //             successor = successor.left

    //         curr.val = successor.val
    //         if successor_parent.left == successor:
    //             successor_parent.left = successor.right
    //         else:
    //             successor_parent.right = successor.right

    //     curr = parent
    //     while curr:
    //         curr.height = 1 + max(self.getHeight(curr.left), self.getHeight(curr.right))
    //         balance = self.getBalance(curr)

    //         if balance > 1 and self.getBalance(curr.left) >= 0:
    //             curr = self.rightRotate(curr)
    //             break
    //         elif balance < -1 and self.getBalance(curr.right) <= 0:
    //             curr = self.leftRotate(curr)
    //             break
    //         elif balance > 1 and self.getBalance(curr.left) < 0:
    //             curr.left = self.leftRotate(curr.left)
    //             curr = self.rightRotate(curr)
    //             break
    //         elif balance < -1 and self.getBalance(curr.right) > 0:
    //             curr.right = self.rightRotate(curr.right)
    //             curr = self.leftRotate(curr)
    //             break

    //         parent = curr
    //         curr = curr.parent

    //     return root
    fn delete_node(&mut self, _key: u32) {
        if let AVLNode::Empty = &*self.root.borrow() {
            return;
        }

        let mut curr = self.root.clone();

        while let AVLNode::Node { key, .. } = &*curr.clone().borrow_mut() {
            match _key.cmp(key) {
                std::cmp::Ordering::Less => {
                    let left = match &*curr.borrow() {
                        AVLNode::Node { left, .. } => left.clone(),
                        _ => return,
                    };

                    curr = left;
                }
                std::cmp::Ordering::Greater => {
                    let right = match &*curr.borrow() {
                        AVLNode::Node { right, .. } => right.clone(),
                        _ => return,
                    };

                    curr = right;
                }
                _ => break,
            }
        }

        if let AVLNode::Empty = &*curr.borrow() {
            return;
        };

        let mut parent = match &*curr.borrow() {
            AVLNode::Node { parent, .. } => match parent.upgrade() {
                Some(parent) => parent,
                _ => Rc::new(RefCell::new(AVLNode::Empty)),
            },
            _ => Rc::new(RefCell::new(AVLNode::Empty)),
        };

        let mut flag = false;
        let mut child = Rc::new(RefCell::new(AVLNode::Empty));
        if let AVLNode::Node {
            left, right, key, ..
        } = &mut *curr.borrow_mut()
        {
            let left_empty = match &*left.borrow() {
                AVLNode::Empty => true,
                _ => false,
            };

            let right_empty = match &*right.borrow() {
                AVLNode::Empty => true,
                _ => false,
            };

            if left_empty || right_empty {
                child = if left_empty {
                    right.clone()
                } else if right_empty {
                    left.clone()
                } else {
                    Rc::new(RefCell::new(AVLNode::Empty))
                };

                match &mut *parent.borrow_mut() {
                    AVLNode::Node { left, right, .. } => {
                        if Rc::ptr_eq(&curr, left) {
                            *left = child.clone();
                        } else if Rc::ptr_eq(&curr, right) {
                            *right = child.clone();
                        }
                    }
                    AVLNode::Empty => flag = true, // May fuck up hard
                }
            } else {
                let mut successor = right.clone();
                let mut successor_parent = curr.clone();

                while let AVLNode::Node { left, .. } = &*successor.clone().borrow() {
                    if let AVLNode::Empty = &*left.borrow() {
                        break;
                    }

                    successor_parent = successor.clone();
                    successor = left.clone();
                }

                println!("Successor: {:#?}", successor.borrow());

                if let AVLNode::Node { key: suc_key, .. } = &*successor.borrow() {
                    *key = *suc_key;
                }

                if let AVLNode::Node { left, .. } = &mut *successor_parent.borrow_mut() {
                    if Rc::ptr_eq(&successor, left) {
                        *left = right.clone()
                    } else {
                        *right = right.clone()
                    }
                };
            }
        };

        if flag {
            self.root = child;
        }

        let mut curr = parent.clone();

        loop {
            match &mut *curr.borrow_mut() {
                AVLNode::Node {
                    height,
                    left,
                    right,
                    ..
                } => *height = 1 + std::cmp::max(left.borrow().height(), right.borrow().height()),
                _ => break,
            }

            let balance = curr.borrow().balance();

            if balance > 1 {
                let case = match &*curr.clone().borrow() {
                    AVLNode::Node { left, .. } => {
                        if left.borrow().balance() >= 0 {
                            1
                        } else if left.borrow().balance() < 0 {
                            2
                        } else {
                            0
                        }
                    }
                    _ => 0,
                };

                match case {
                    1 => curr = rotate_right(&curr, &mut self.root),
                    2 => {
                        let left = match &*curr.borrow() {
                            AVLNode::Node { left, .. } => left.clone(),
                            _ => Rc::new(RefCell::new(AVLNode::Empty)),
                        };

                        let left_node = rotate_left(&left.clone(), &mut self.root);

                        match &mut *curr.clone().borrow_mut() {
                            AVLNode::Node { left, .. } => *left = left_node,
                            _ => (),
                        }

                        curr = rotate_right(&curr, &mut self.root);
                    }
                    _ => (),
                }
            } else if balance < -1 {
                let case = match &*curr.clone().borrow() {
                    AVLNode::Node { right, .. } => {
                        if right.borrow().balance() <= 0 {
                            2
                        } else if right.borrow().balance() > 0 {
                            1
                        } else {
                            0
                        }
                    }
                    _ => 0,
                };

                // Avoids passing a borrow_mut() to rotate_right() and rotate_left()
                // which would cause a double borrow_mut() error
                match case {
                    1 => {
                        let right = match &*curr.borrow() {
                            AVLNode::Node { right, .. } => right.clone(),
                            _ => Rc::new(RefCell::new(AVLNode::Empty)),
                        };

                        let right_node = rotate_right(&right.clone(), &mut self.root);

                        match &mut *curr.clone().borrow_mut() {
                            AVLNode::Node { right, .. } => *right = right_node,
                            _ => (),
                        }

                        curr = rotate_left(&curr, &mut self.root);
                    }
                    2 => curr = rotate_left(&curr, &mut self.root),
                    _ => (),
                }
            }

            parent = curr.clone();
            let _ = parent; // Just to satisfy Ferris 🦀
            curr = match &*curr.clone().borrow() {
                AVLNode::Node { parent, .. } => match parent.upgrade() {
                    Some(parent) => parent,
                    None => return,
                },
                _ => return,
            };
        }
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
        if let AVLNode::Empty = *self.root.borrow() {
            true
        } else {
            false
        }
    }

    fn print_tree(&self) {
        if let AVLNode::Empty = &*self.root.borrow() {
            println!("\nTree is empty\n");
            return;
        }
        ptree::print_tree(&*self.root.borrow()).expect("Failed to print tree");
    }
}
