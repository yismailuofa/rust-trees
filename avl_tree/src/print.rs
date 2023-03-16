use std::{borrow::Cow, io};

use crate::AVLNode;

impl ptree::TreeItem for AVLNode {
    type Child = Self;
    fn write_self<W: io::Write>(&self, f: &mut W, _: &ptree::Style) -> io::Result<()> {
        if let AVLNode::Node { key, .. } = self {
            write!(f, "{}", key)?;
        }
        Ok(())
    }
    fn children(&self) -> Cow<[Self::Child]> {
        match self {
            AVLNode::Node { left, right, .. } => {
                Cow::from(vec![left.borrow().clone(), right.borrow().clone()])
            }
            AVLNode::Empty => Cow::from(vec![]),
        }
    }
}
