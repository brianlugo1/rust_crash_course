use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table",2);
    stock.insert("Couch", 0);
    let mut total_stock = 0;
    for (item, quantity) in stock.iter() {
        total_stock = total_stock + quantity;
        // let stock_count = if quantity == &0 {
        let stock_count = if quantity.eq(&0) {
            "out of stock".to_owned()
        } else {
            format!("{}", quantity)
        };
        println!("item = {:?}, stock = {}", item, stock_count)
    }
    println!("total stock = {:?}", total_stock);
}
