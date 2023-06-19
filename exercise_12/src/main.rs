fn main() {
    let mut count_down = 3;
    loop {
        println!("{}", count_down);
        count_down = count_down - 1;
        if count_down == 0 {
            break;
        }
    }
    println!("done!");
}
