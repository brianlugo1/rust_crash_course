struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32
}

fn main () {
    let my_box = ShippingBox{
        depth: 3,
        width: 2,
        height: 5,
    };
    let _depth = my_box.depth;
    let _width = my_box.width;
    let _height = my_box.height;
    println!("the box is {} units tall", _height);
}
