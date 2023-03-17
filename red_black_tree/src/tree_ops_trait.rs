use tree::TreeOpsTrait;

use crate::RBNode;

impl TreeOpsTrait for RBNode {
    fn rotate_left(&mut self) {
        // Rotates left
        // Ex:
        //       y                               x
        //      / \                             /  \
        //     x   T3                          T1   y
        //    / \       < - - - - - - -            / \
        //   T1  T2     Left Rotation            T2  T3

        // # Rotation
        // parent = x.parent
        // y = x.right
        // T2 = y.left

        // y.left = x
        // x.right = T2

        // if T2:
        //     T2.parent = x
        // y.parent = parent

        // if parent is None:
        //     self.root = y
        // elif parent.left == x:
        //     parent.left = y
        // elif y.parent.right == x:
        //     parent.right = y

        // match self {
        //     RBNode::Node {
        //         key,
        //         color,
        //         left,
        //         right,
        //         parent,
        //     } => {
        //         let parent = parent.upgrade().unwrap();
        //         let y = right.clone();
        //         let T2 = match &*y.borrow_mut() {
        //             RBNode::Node { left, .. } => left.clone(),
        //             RBNode::Empty => Rc::new(RefCell::new(RBNode::Empty)),
        //         };

        //         match &*y.borrow_mut() {
        //             RBNode::Node { left, .. } => *left = Rc::new(RefCell::new(*self)),
        //             RBNode::Empty => (),
        //         };
        //     }
        //     RBNode::Empty => return,
        // }
    }

    fn rotate_right(&mut self) {
        todo!()
    }

    fn fix_violation(&mut self) {
        todo!()
    }
}
