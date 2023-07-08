use std::io;
use std::str::FromStr;

pub trait ValidInputTrait: FromStr {}

impl ValidInputTrait for u16 {}
impl ValidInputTrait for u32 {}
impl ValidInputTrait for u64 {}

impl ValidInputTrait for i16 {}
impl ValidInputTrait for i32 {}
impl ValidInputTrait for i64 {}

impl ValidInputTrait for f32 {}
impl ValidInputTrait for f64 {}

impl ValidInputTrait for String {}

pub fn get_input<T: ValidInputTrait>() -> T {
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