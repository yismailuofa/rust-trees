use std::{
    borrow::Cow,
    cell::RefCell,
    io,
    rc::{Rc, Weak},
};

use tree::TreeTrait;

#[derive(PartialEq, Clone, Debug)]
pub enum Color {
    Red,
    Black,
}

pub type Tree = Rc<RefCell<RBNode>>;
pub type WeakTree = Weak<RefCell<RBNode>>;

#[derive(Clone, Debug)]
pub enum RBNode {
    Node {
        key: u32,
        color: Color,
        left: Tree,
        right: Tree,
        parent: WeakTree,
    },
    Empty,
}

impl TreeTrait for RBNode {
    fn insert_node(&mut self, _key: u32) {
        if let RBNode::Empty = self {
            *self = RBNode::Node {
                key: _key,
                color: Color::Black,
                left: Rc::new(RefCell::new(RBNode::Empty)),
                right: Rc::new(RefCell::new(RBNode::Empty)),
                parent: Weak::new(),
            };
            return;
        }

        let mut curr = self.clone();

        while let RBNode::Node {
            key, left, right, ..
        } = &curr.clone()
        {
            if _key < *key {
                let mut left_node = left.borrow_mut();

                if let RBNode::Empty = left_node.clone() {
                    *left_node = RBNode::Node {
                        key: _key,
                        color: Color::Red,
                        left: Rc::new(RefCell::new(RBNode::Empty)),
                        right: Rc::new(RefCell::new(RBNode::Empty)),
                        parent: Rc::downgrade(&Rc::new(RefCell::new(curr.clone()))),
                    };
                    return;
                } else {
                    curr = left_node.clone();
                }
            } else if _key > *key {
                let mut right_node = right.borrow_mut();

                if let RBNode::Empty = right_node.clone() {
                    *right_node = RBNode::Node {
                        key: _key,
                        color: Color::Red,
                        left: Rc::new(RefCell::new(RBNode::Empty)),
                        right: Rc::new(RefCell::new(RBNode::Empty)),
                        parent: Rc::downgrade(&Rc::new(RefCell::new(curr.clone()))),
                    };
                    return;
                } else {
                    curr = right_node.clone();
                }
            }
        }
    }

    fn delete_node(&mut self, _key: u32) {
        todo!()
    }

    fn count_leaves(&self) -> u32 {
        todo!()
    }

    fn height(&self) -> u32 {
        todo!()
    }

    fn in_order(&self) -> Vec<u32> {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn print_tree(&self) {
        ptree::print_tree(self).expect("Failed to print tree");
    }

    fn new() -> Self {
        RBNode::Empty
    }
}

impl ptree::TreeItem for RBNode {
    type Child = Self;
    fn write_self<W: io::Write>(&self, f: &mut W, _: &ptree::Style) -> io::Result<()> {
        if let RBNode::Node { key, color, .. } = self {
            write!(f, "{}", key)?;
            if *color == Color::Red {
                write!(f, " (R)")?;
            } else {
                write!(f, " (B)")?;
            }
        }
        Ok(())
    }
    fn children(&self) -> Cow<[Self::Child]> {
        match self {
            RBNode::Node { left, right, .. } => {
                Cow::from(vec![left.borrow().clone(), right.borrow().clone()])
            }
            RBNode::Empty => Cow::from(vec![]),
        }
    }
}
