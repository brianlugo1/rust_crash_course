enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32, i32),
}

enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}

fn main() {
    let _left = Mouse::LeftClick;
    let _right = Mouse::RightClick;
    let _middle = Mouse::MiddleClick;
    let _scroll = Mouse::Scroll(10);
    let _move = Mouse::Move(-10, 5);
    let _new_user = PromoDiscount::NewUser;
    let _holiday = PromoDiscount::Holiday("Special Holiday".to_owned());
    let _discount = Discount::Percent(40.0);
    let _flat = Discount::Flat(10);
    let _promo = Discount::Promo(_holiday);
    let _custom = Discount::Custom("This is a custom discount!".to_owned());
}
