use tree::TreeTrait;

use crate::{AVLNode, AVLTree};

impl TreeTrait for AVLTree {
    fn insert_node(&mut self, _key: u32) {
        todo!()
    }

    fn delete_node(&mut self, _key: u32) {
        todo!()
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
