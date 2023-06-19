enum Color {
    Blue,
    Red,
    Green
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Blue => println!("Blue"),
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
    }
}

fn main() {
    let _color_blue = Color::Blue;
    let _color_red = Color::Red;
    let _color_green = Color::Green;
    print_color(_color_blue);
}
