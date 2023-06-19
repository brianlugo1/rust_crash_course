fn print_many(msg: &str, count: i32) {
    println!("message: {}, count: {}", msg, count);
}

enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
}

fn main() {
    let _num: i32 = 15;
    let _a: char = 'a';
    let _left_click: Mouse = Mouse::LeftClick;
    let _right_click: Mouse = Mouse::RightClick;
    let _middle_click: Mouse = Mouse::MiddleClick;
    print_many("Please help!!!", _num);
}
