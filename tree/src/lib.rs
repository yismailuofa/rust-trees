pub trait TreeTrait {
    fn insert_node(&mut self, key: u32);
    fn delete_node(&mut self, key: u32);
    fn count_leaves(&self) -> u32;
    fn height(&self) -> u32;
    fn in_order(&self) -> Vec<u32>;
    fn is_empty(&self) -> bool;
    fn print_tree(&self);
}
