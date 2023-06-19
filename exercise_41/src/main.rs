struct Person {
    name: String,
    favorite_color: String,
    age: i32,
}

impl Person {
    // fn print_name(&self) {
    //     println!("{}", self.name);
    // }
    // fn print_favorite_color(&self) {
    //     println!("{}", self.favorite_color);
    // }
    fn print(&self) {
        println!("Hello my name is {}, my favorite color is: {}!", self.name, self.favorite_color);
    }
}

// fn print(data: &str) {
//     println!("{}", data);
// }

fn main() {
    let people = vec![
        Person {
            name: String::from("George"),
            favorite_color: String::from("green"),
            age: 7,
        },
        Person {
            name: String::from("Ann"),
            favorite_color: String::from("purple"),
            age: 9,
        },
        Person {
            name: String::from("Katie"),
            favorite_color: String::from("blue"),
            age: 14,
        },
    ];
    for person in people {
        if person.age <= 10 {
            person.print();
            // person.print_name();
            // person.print_favorite_color();
            // print(&person.name);
            // print(&person.favorite_color);
        }
    }
}
