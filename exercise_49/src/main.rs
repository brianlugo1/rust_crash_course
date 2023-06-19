struct GroceryItem {
    name: String,
    qty: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem { name: "bananas".to_owned(), qty: 4, },
        GroceryItem { name: "eggs".to_owned(), qty: 12, },
        GroceryItem { name: "bread".to_owned(), qty: 1, },
    ];
    for item in groceries {
        if item.name == name {
            return Some(item.qty);
        }
    }
    None
}

fn main() {
    // Inorder to retrieve the value stores in a Some(), one must call .unwrap()!
                                             // find_quantity("bananas") returns Some(4)
    println!("The quantity of bananas is {:?}", find_quantity("bananas"));
                                             // find_quantity("bananas").unwrap() returns 4
    println!("The quantity of bananas is {:?}", find_quantity("bananas").unwrap());
}
