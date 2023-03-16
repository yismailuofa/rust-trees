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

#[derive(Clone, Debug)]
pub enum RBNode {
    Node {
        key: u32,
        color: Color,
        left: Rc<RefCell<RBNode>>,
        right: Rc<RefCell<RBNode>>,
        parent: Weak<RefCell<RBNode>>,
    },
    Empty,
}

impl TreeTrait for RBNode {
    fn insert_node(&mut self, _key: u32) {
        let parent = Rc::new(RefCell::new(RBNode::Empty));

        if let RBNode::Empty = self.clone() {
            *self = RBNode::Node {
                key: _key,
                color: Color::Red,
                left: Rc::new(RefCell::new(RBNode::Empty)),
                right: Rc::new(RefCell::new(RBNode::Empty)),
                parent: Rc::downgrade(&parent),
            };
            return;
        }

        while let RBNode::Node {
            key, left, right, ..
        } = self
        {
            if _key > key {
                if let RBNode::Empty = right.borrow().clone() {
                    right.replace(RBNode::Node {
                        key: _key,
                        color: Color::Red,
                        left: Rc::new(RefCell::new(RBNode::Empty)),
                        right: Rc::new(RefCell::new(RBNode::Empty)),
                        parent: Rc::downgrade(&parent),
                    });
                } else {
                    *self = right.borrow().clone();
                }
            } else {
                if let RBNode::Empty = left.borrow().clone() {
                    left.replace(RBNode::Node {
                        key: _key,
                        color: Color::Red,
                        left: Rc::new(RefCell::new(RBNode::Empty)),
                        right: Rc::new(RefCell::new(RBNode::Empty)),
                        parent: Rc::downgrade(&parent),
                    });
                    break;
                } else {
                    *self = left.borrow().clone();
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
