use std::{borrow::Cow, io};

use crate::{Color, RBNode};

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
