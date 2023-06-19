fn main() {
    let my_name = "Brian";
    match my_name {
        "Brian" => println!("that is my name"),
        "Bob" => println!("not my name"),
        "Alice" => println!("hello alice"),
        _ => println!("nice to meet you {}", my_name),
    }
}
