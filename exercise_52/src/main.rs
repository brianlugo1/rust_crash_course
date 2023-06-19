// Learned how to add custom sentence as part of Documentation
/// A favorite color.
enum Color {
    Red,
    Blue,
}

/// A piece of mail.
struct Mail {
    /// The destination address.
    address: String,
}

/// Adds two numbers together.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let _red = Color::Red;
    let _blue = Color::Blue;
    let _mail = Mail{ address: "123 NewLine Drive, City, 12345".to_owned()};
    println!("{:?}", _mail.address);
    println!("{:?}", add(1, 2));
}
