enum Light {
    Bright,
    Dull,
}

// fn display_light(light: Light) {
fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}
fn main() {
    let _bright = Light::Bright;
    let dull = Light::Dull;
    // Because Rust uses the idea of Ownership to manage memory,
    // variables are passed to other scopes who then have the
    // responsibility to delete the memory.
    // In the example below, the variable dull is declared in main().
    // Therefore, main() is responsible for deleting dull.
    // However, the variable dull is passed to display_light().
    // Now the first function call of display_light() gets
    // handed the responsibility to delete the variable dull.
    // After display_light() finishes doing its thing, it deletes
    // the variable dull. Now comes the second function call of
    // display_light(), however, the variable dull has been deleted.
    // Therefore, the value stored in the variable has been lost. 
    // The solution is to borrow (reference) the variable with &s.
    // display_light(dull);
    // display_light(dull);
    display_light(&dull);
    display_light(&dull);
}
