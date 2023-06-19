struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let mary = Student {
        name: "Mary".to_owned(),
        locker: Some(3),
    };
    // When using the debugging formatter, prints strings with quotes
    // When using the empty formatter, prints the characters in a string only
    // println!("student: {:?}", mary.name);
    println!("student: {}", mary.name);
    match mary.locker {
        Some(number) => println!("locker number: {:?}", number),
        None => println!("no locker assigned"),
    }
}
