use crate::RBTree;
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
    let mut tree = RBTree::default();
    let mut rng = thread_rng();
    let mut vec: Vec<u32> = (0..10000).collect();

    vec.shuffle(&mut rng);
    let mut count = 0;
    for i in &vec {
        tree.insert_node(*i);
        count += 1;
        if count > 1 {
            assert!(tree.height() <= 2 * (count as f64).log2().ceil() as u32);
        }
        //assert!(tree.height() <= 2 * (count as f64).log2().ceil() as u32);
    }
    
    vec.shuffle(&mut rng);
    
    // delete n-1 nodes
    let mut counter = 10000;
    for i in &vec[0..vec.len() - 1] {
        tree.delete_node(*i);
        
        counter -= 1;
        //assert!(tree.height() <= 2 * (counter as f64).log2().ceil() as u32);
        if counter > 1 {
            assert!(tree.height() <= 2 * (counter as f64).log2().ceil() as u32);
        }
        assert_eq!(tree.in_order().len(), counter);
    }
}
