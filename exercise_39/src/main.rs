// Because a string is automatically borrowed,
// the struct cannot delete the variable and
// the compiler will give back an error.
// The fix is to set the type to String.
// Then we need to create a new string from
// the string to be able to create owned data.
struct Employee {
    // name: &str
    name: String
}

fn main() {
    // let emp_name = "Brian";
    // let emp_name = "Brian".to_owned();
    let emp_name = String::from("Brian");
    let _emp = Employee {
        name: emp_name
    };
    println!("{}", _emp.name);
}
