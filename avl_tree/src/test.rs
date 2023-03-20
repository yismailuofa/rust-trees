use crate::AVLTree;
use rand::seq::SliceRandom;
use rand::thread_rng;
use tree::TreeTrait;

/**
 *
 * Use monkey testing by randomly inserting
 * and deleting nodes to find any bugs
 */
#[test]
fn monkey_test() {
    let mut tree = AVLTree::default();
    let mut rng = thread_rng();
    let mut vec: Vec<u32> = (0..64 as u32).collect();

    vec.shuffle(&mut rng);

    for i in &vec {
        tree.insert_node(*i);
    }

    vec.shuffle(&mut rng);

    // delete n-1 nodes
    let mut counter = 64;
    for i in &vec[0..vec.len() - 1] {
        tree.delete_node(*i);

        counter -= 1;

        // tree.print_tree();
        println!("counter :{}", counter);
        println!("tree.in_order() : {:?}", tree.in_order());
        assert_eq!(tree.in_order().len(), counter);

        println!("{}: {}", counter, *i);
    }
}
