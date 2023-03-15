extern crate tree;
use ptree::*;
use std::borrow::Cow;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use tree::TreeTrait;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    Red,
    Black,
}

type Tree = Rc<RefCell<RedBlackTreeNode>>;

#[derive(Debug)]
pub struct RedBlackTree(pub Option<Tree>);

#[derive(Debug)]
pub struct RedBlackTreeNode {
    pub color: NodeColor,
    pub key: u32,
    pub parent: RedBlackTree,
    pub left: RedBlackTree,  // Maybe make these private later
    pub right: RedBlackTree, // Maybe make these private later
}
trait RedBlackTreeOps {
    fn left_rotate(&mut self) -> RedBlackTree;
    fn right_rotate(&mut self) -> RedBlackTree;
    fn fix_tree(&mut self);
}

impl Clone for RedBlackTree {
    fn clone(&self) -> Self {
        if let Some(node) = &self.0 {
            RedBlackTree(Some(Rc::clone(node)))
        } else {
            RedBlackTree(None)
        }
    }
}

impl RedBlackTreeOps for RedBlackTree {
    fn left_rotate(&mut self) -> RedBlackTree {
        // assuming it is left
        if let Some(node) = &self.0 {
            let mut node_ref = node.borrow_mut();
            let node_parent = node_ref.parent.clone();
            let node_right = node_ref.right.clone();

            if let Some(node_right_ref) = &node_right.0 {
                let mut node_right_ref = node_right_ref.borrow_mut();
                node_ref.right = node_right_ref.left.clone();
                node_right_ref.parent = node_ref.parent.clone();
                node_right_ref.left = self.clone();
            }
            node_ref.parent = node_right.clone();
            if let Some(parent_ref) = &node_parent.0 {
                let mut parent = parent_ref.borrow_mut();
                if parent.key < node_ref.key {
                    parent.right = node_right.clone();
                } else {
                    parent.left = node_right.clone();
                }
            }
            return node_right.clone();
        }
        self.clone()
    }
    fn right_rotate(&mut self) -> RedBlackTree {
        // assuming it is right
        if let Some(node) = &self.0 {
            let node_ref = node.borrow_mut();
            let node_parent = &node_ref.parent;
            let node_left = &node_ref.left;

            if let Some(node_left_ref) = &node_left.0 {
                let mut node_left = node_left_ref.borrow_mut();
                node.borrow_mut().left = node_left.right.clone();
                node_left.parent = node_ref.parent.clone();
                node_left.right = self.clone();
            }
            node.borrow_mut().parent = node_left.clone();
            if let Some(parent_ref) = &node_parent.0 {
                let mut parent = parent_ref.borrow_mut();
                if parent.key < node_ref.key {
                    parent.right = node_left.clone();
                } else {
                    parent.left = node_left.clone();
                }
            }
            return node_left.clone();
        }
        self.clone()
    }
    fn fix_tree(&mut self) {
        if let Some(node_ref) = &self.0 {
            let mut node = node_ref.borrow_mut();
            if let Some(parent_node) = &node.parent.0 {
                let parent = parent_node.borrow_mut();

                if parent.color == NodeColor::Red {
                    if let Some(grandparent_node) = &parent.parent.0 {
                        let grandparent = grandparent_node.borrow_mut();

                        let uncle = if parent.key < grandparent.key {
                            grandparent.right.clone()
                        } else {
                            grandparent.left.clone()
                        };

                        match uncle.0 {
                            Some(uncle_node) if uncle_node.borrow_mut().color == NodeColor::Red => {
                                parent_node.borrow_mut().color = NodeColor::Black;
                                uncle_node.borrow_mut().color = NodeColor::Black;
                                grandparent_node.borrow_mut().color = NodeColor::Red;
                                parent.parent.clone().fix_tree();
                            }
                            _ => {
                                if node.key < parent.key && parent.key < grandparent.key {
                                    parent.parent.clone().right_rotate();
                                    parent_node.borrow_mut().color = NodeColor::Black;
                                    grandparent_node.borrow_mut().color = NodeColor::Red;
                                } else if node.key > parent.key && parent.key > grandparent.key {
                                    parent.parent.clone().left_rotate();
                                    parent_node.borrow_mut().color = NodeColor::Black;
                                    grandparent_node.borrow_mut().color = NodeColor::Red;
                                } else if node.key > parent.key && parent.key < grandparent.key {
                                    node.parent.clone().left_rotate();
                                    parent.parent.clone().right_rotate();
                                    node_ref.clone().borrow_mut().color = NodeColor::Black;
                                    grandparent_node.borrow_mut().color = NodeColor::Red;
                                } else {
                                    node.parent.clone().right_rotate();
                                    parent.parent.clone().left_rotate();
                                    node_ref.borrow_mut().color = NodeColor::Black;
                                    grandparent_node.borrow_mut().color = NodeColor::Red;
                                }
                            }
                        }
                    }
                }
            } else {
                println!("{:?}", &node.parent);
                node.color = NodeColor::Black;
            }
        }
    }
}

impl TreeItem for RedBlackTree {
    type Child = Self;
    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        if let Some(node) = &self.0 {
            let node = node.borrow_mut();
            match node.color {
                NodeColor::Black => {
                    write!(f, "{}", style.paint(node.key.to_string() + "(B)"))
                        .expect("Error printing tree");
                }
                NodeColor::Red => {
                    write!(f, "{}", style.paint(node.key.to_string() + "(R)"))
                        .expect("Error printing tree");
                }
            }
        }
        Ok(())
    }
    fn children(&self) -> Cow<[Self::Child]> {
        let mut childs = Vec::new();
        if let Some(node) = &self.0 {
            let node = node.borrow_mut();
            childs.push(node.left.clone());
            childs.push(node.right.clone());
        }
        Cow::from(childs)
    }
}

impl TreeTrait for RedBlackTree {
    fn insert_node(&mut self,parent: Option<&RedBlackTree>, key: u32) {
        if let Some(node) = &self.0 {
            let mut node = node.borrow_mut();

            if key < node.key {
                node.left.insert_node(Some(self), key);
            } else if key > node.key {
                node.right.insert_node(Some(self),key);
            }
        } else {
            self.0 = Some(Rc::new(RefCell::new(RedBlackTreeNode {
                color: NodeColor::Red,
                key,
                parent: parent.unwrap().clone(),
                left: RedBlackTree(None),
                right: RedBlackTree(None),
            })));

            self.fix_tree();
        }
    }

    fn delete_node(&mut self, _: u32) {
        *self = self.left_rotate();
    }

    fn count_leaves(&self) -> u32 {
        print_tree(self).expect("Error printing tree");
        1
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
        print_tree(self).expect("Error printing tree")
    }
}
