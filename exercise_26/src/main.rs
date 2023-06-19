enum Menu {
    Burger,
    Fries,
    Drink
}

fn main() {
    let _paid = true;
    let item = Menu::Drink;
    let _fries = Menu::Fries;
    let _burgers = Menu::Burger;
    let drink_type = "water";
    let _order_placed = match item {
        Menu::Drink => {
            if drink_type == "water" {
                true
            } else {
                false
            }
        }
        _ => true,
    };
}
