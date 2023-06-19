struct Customer {
    age: std::option::Option<i32>,
    email: String,
}

fn main() {
    let mark = Customer {
        age: Some(22), email: "mark@example.com".to_owned(),
    };
    match mark.age {
        Some(age) => println!("Mark is {:?} years old.", age),
        None => println!("Mark did not provide the age!"),
    }
    match mark.email {
        email => println!("Mark's email is {:?}!", email),
    }
    let becky = Customer {
        age: None, email: "becky@example.com".to_owned(),
    };
    match becky.age {
        Some(age) => println!("Becky is {:?} years old.", age),
        None => println!("Becky's age not provided."),
    }
}
