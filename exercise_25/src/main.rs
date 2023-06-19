fn main() {
    let my_num = 3;
    let _is_lt_5 = if my_num < 5 {
        true
    } else {
        false
    };

    let _is_lt_5 = my_num < 5;

    let my_num = 3;
    let _message = match my_num {
        1 => "hello",
        _ => "goodbye",
    };
}
