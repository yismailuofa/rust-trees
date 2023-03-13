use red_black_tree::{RedBlackTree, RedBlackTreeTrait};

/**
Your library
must allow the user to perform the following operation:
1- Insert a node to the red-black tree.
2- Delete a node from the red-black tree.
3- Count the number of leaves in a tree.
4- Return the height of a tree.
5- Print In-order traversal of the tree.
6- Check if the tree is empty.
7- Print the tree showing its colors and structure. (Using println!(“{:#?}”,tree); is NOT
sufficient)
*/

fn main() {
    let options = vec![
        "Insert a node",
        "Delete a node",
        "Count the number of leaves",
        "Height of the tree",
        "Print In-order traversal of the tree",
        "Check if the tree is empty",
        "Print the tree showing its colors and structure",
    ];

    let mut tree: RedBlackTree = None;

    loop {
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
                println!("Value inserted successfully");
            }
            "2" => {
                println!("Enter the value of the node");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let value = input.trim().parse::<u32>().unwrap();
                tree.delete_node(value);
                println!("Value deleted successfully");
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
            _ => {
                println!("Invalid option");
            }
        }
    }
}
