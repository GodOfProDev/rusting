use std::io;
use crate::utils;

pub fn calculator() {
    println!("Enter the first number: ");

    let first_number = utils::get_number_f32();

    let mut operator = String::new();

    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");

    let operator = operator
        .trim()
        .chars()
        .nth(0)
        .expect("There was an issue reading the operator");

    println!("Enter the second number: ");

    let second_number = utils::get_number_f32();

    match operator {
        '*' => println!("Result: {}", first_number * second_number),
        '/' => println!("Result: {}", first_number / second_number),
        '+' => println!("Result: {}", first_number + second_number),
        '-' => println!("Result: {}", first_number - second_number),
        '%' => println!("Result: {}", first_number % second_number),
        '^' => println!("Result: {}", first_number.powf(second_number)),
        _ => println!("Invalid Operator!"),
    }
}


