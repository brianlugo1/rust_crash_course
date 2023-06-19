enum Flavor {
    Sparkling,
    Sweet,
    Fruity
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("sparkling"),
        Flavor::Sweet => println!("sweet"),
        Flavor::Fruity => println!("fruity"),
    }
    println!("oz: {}", drink.fluid_oz);
}

fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 6.0,
    };
    print_drink(sweet);

    let sparkling = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 10.0,
    };
    print_drink(sparkling);

    let fruity = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 3.0,
    };
    print_drink(fruity);
}
