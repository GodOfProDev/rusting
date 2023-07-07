use std::cmp::Ordering;
use std::io;

use rand::Rng;

pub fn guess_game() {
    println!("Guess Game!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess: ");
        let guess = get_number();

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }
}

fn get_number() -> i32 {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            get_number()
        }
    };

    return number;
}
