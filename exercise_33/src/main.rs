enum Color {
    Brown,
    Red
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {}", self.width);
        println!("height: {}", self.height);
        println!("depth: {}", self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions { width: 1.0, height: 2.0, depth: 3.0 };
    let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);
    small_box.print();
    println!("");
    let big_dimensions = Dimensions { width: 5.0, height: 6.0, depth: 7.0 };
    let big_box = ShippingBox::new(10.0, Color::Brown, big_dimensions);
    big_box.print();
}
