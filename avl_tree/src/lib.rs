extern crate tree;
use tree::TreeTrait;

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Rc<RefCell<AVLTreeNode>>;

pub struct AVLTree(pub Option<Tree>);

pub struct AVLTreeNode {
    pub key: u32,
    pub parent: AVLTree,
    pub left: AVLTree,  // Maybe make these private later
    pub right: AVLTree, // Maybe make these private late
    pub height: u32,
    pub balance_factor: i32,
}

impl TreeTrait for AVLTree {
    fn insert_node(&mut self, key: u32) {
        todo!()
    }

    fn delete_node(&mut self, key: u32) {
        todo!()
    }

    fn count_leaves(&self) -> u32 {
        match &self.0 {
            Some(tree) => {
                if tree.borrow().left.is_empty() && tree.borrow().right.is_empty() {
                    return 1;
                }
                else {
                    return tree.borrow().left.count_leaves() + tree.borrow().right.count_leaves();
                }
            }, 
            None => 0
        }
    }

    fn height(&self) -> u32 {
        let leaves: u32 = self.count_leaves();

        if leaves < 3 {
            return leaves;
        } else {
            return ((leaves + 1) as f64).log2().ceil() as u32 - 1;
        }
    }

    fn in_order(&self) -> Vec<u32> {

        // initialize empty vector
        let mut vec: Vec<u32> = Vec::new();

        match &self.0 {
            Some(tree) => {
                
                // add left subtree
                vec.append(&mut tree.borrow().left.in_order());

                // add root
                vec.push(tree.borrow().key);

                // add right subtree
                vec.append(&mut tree.borrow().right.in_order());

                return vec;
            },
            None => {
                return vec;
            }
        };

    }

    fn is_empty(&self) -> bool {
        match &self.0 {
            Some(_) => false,
            None => true,
        }
    }

    fn print_tree(&self) {
        todo!()
    }
}
