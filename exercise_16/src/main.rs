enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn which_way(go: Direction) {
    match go {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
    }
}

fn main() {
    let _going_up = Direction::Up;
    let _going_down = Direction::Down;
    let _going_left = Direction::Left;
    let _going_right = Direction::Right;
    which_way(_going_up);
    // which_way(_going_down);
    // which_way(_going_left);
    // which_way(_going_right);
}
