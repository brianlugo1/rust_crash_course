fn coordinate() -> (i32, i32) {
    (1, 7)
}

fn main() {
    let (_x, y) = coordinate();
    if y > 5 {
        println!("{} > 5", y);
    } else if y < 5 {
        println!("{} < 5", y);
    } else {
        println!("{} = 5", y);
    }
}
