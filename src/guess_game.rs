use std::cmp::Ordering;

use rand::Rng;

use crate::utils;

pub fn guess_game() {
    println!("Guess Game!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess: ");
        let guess = utils::get_number_i32();

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