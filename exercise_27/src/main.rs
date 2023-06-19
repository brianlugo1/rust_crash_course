enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {
    // secret rule: admins only
    let access_level = Access::Guest;
    let _access_level2 = Access::User;
    let _access_level3 = Access::Manager;
    let _access_level4 = Access::Admin;
    let _can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
    println!("can access: {}", _can_access_file);
}
