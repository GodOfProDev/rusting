use std::io;

pub fn calculator() {
    println!("Enter the first number: ");

    let first_number = get_number();

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

    let second_number = get_number();

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

fn get_number() -> f32 {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: f32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            get_number()
        }
    };

    return number;
}
