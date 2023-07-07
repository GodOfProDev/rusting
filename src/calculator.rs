use crate::utils;

pub fn calculator() {
    println!("Enter the first number: ");

    let first_number = utils::get_number_f32();

    println!("Enter the operator: ");

    let operator = utils::get_string()
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


