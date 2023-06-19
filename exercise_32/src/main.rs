struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freeizing() -> Self {
        Self { degrees_f: 32.0 }
    }

    fn boiling() -> Self {
        Self { degrees_f: 212.0 }
    }

    fn show_temp(&self) {
        println!("{} degrees F", &self.degrees_f);
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.9 };
    hot.show_temp();

    let cold = Temperature::freeizing();
    cold.show_temp();

    let boiling = Temperature::boiling();
    boiling.show_temp();
}
