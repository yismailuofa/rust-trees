pub trait TreeTrait {
    fn insert_node(&mut self, key: u32);
    fn delete_node(&mut self, key: u32);
    fn count_leaves(&self) -> u32;
    fn height(&self) -> u32;
    fn in_order(&self) -> Vec<u32>;
    fn is_empty(&self) -> bool;
    fn print_tree(&self);
}

pub fn prompt_user(tree: &mut impl TreeTrait) {
    let options = vec![
        "Insert a node",
        "Delete a node",
        "Count the number of leaves",
        "Height of the tree",
        "Print In-order traversal of the tree",
        "Check if the tree is empty",
        "Print the tree",
        "Quit",
    ];

    for (i, option) in options.iter().enumerate() {
        println!("{}: {}", i + 1, option);
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => {
            println!("Enter the value of the node");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let value = input.trim().parse::<u32>().unwrap();

            tree.insert_node(value);

            tree.print_tree();
        }
        "2" => {
            println!("Enter the value of the node");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let value = input.trim().parse::<u32>().unwrap();
            tree.delete_node(value);

            tree.print_tree();
        }
        "3" => {
            println!("Number of leaves: {}", tree.count_leaves());
        }
        "4" => {
            println!("Height of the tree: {}", tree.height());
        }
        "5" => {
            println!("In-order traversal of the tree: {:?}", tree.in_order());
        }
        "6" => {
            println!("Is the tree empty: {}", tree.is_empty());
        }
        "7" => {
            tree.print_tree();
        }
        "8" => {
            std::process::exit(0);
        }
        _ => {
            println!("Invalid option");
        }
    }
}
