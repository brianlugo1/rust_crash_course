// Exercise 57
#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string()
            })
        } else {
            Err("Age must be at least 21")
        }
    }
}

fn match_adult(adult: Result<Adult, &str>) {
    match adult {
        Ok(a) => println!("{} is {} years old", a.name, a.age),
        Err(e) => println!("{e}"),
    }
}

fn main() {
    let child = Adult::new(15, "Anita");
    let adult = Adult::new(21, "Sanjay");
    match_adult(child);
    match_adult(adult);
}
