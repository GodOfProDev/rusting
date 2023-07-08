use std::io;
use std::str::FromStr;

pub fn get_input<T: FromStr>() -> T {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: T = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            get_input()
        }
    };

    return input;
}