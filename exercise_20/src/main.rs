struct GroceryItem {
    stock: i32,
    price: f64
}

fn main() {
    let _cereal = GroceryItem {
        stock: 10,
        price: 2.99
    };
    println!("stock: {}", _cereal.stock);
    println!("price: {}", _cereal.price);
}
