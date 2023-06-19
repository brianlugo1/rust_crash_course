enum Direction {
    Left,
    Right
}
fn main() {
    let _go_left = Direction::Left;
    let _go_right = Direction::Right;
    match _go_left {
        Direction::Left => println!("Left"),
        Direction::Right => println!("Rigth"),
    }
}
