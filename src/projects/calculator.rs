use crate::utils;

pub fn calculator() {
    println!("Enter the first number: ");

    let first_number: f32 = utils::get_input();

    println!("Enter the operator: ");

    let operator: char = utils::get_input::<String>()
        .chars()
        .next()
        .expect("There was an issue reading the operator");

    println!("Enter the second number: ");

    let second_number: f32 = utils::get_input();

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


