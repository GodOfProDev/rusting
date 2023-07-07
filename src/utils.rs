use std::io;

pub fn get_number_i32() -> i32 {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            get_number_i32()
        }
    };

    return number;
}

pub fn get_number_f32() -> f32 {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: f32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            get_number_f32()
        }
    };

    return number;
}

pub fn get_string() -> String {
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");

    return string;
}