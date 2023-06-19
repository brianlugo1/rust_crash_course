fn main() {
    let mut count = 1;
    loop {
        println!("{}", count);
        if count == 4 {
            break;
        }
        count = count + 1;
    }
}
