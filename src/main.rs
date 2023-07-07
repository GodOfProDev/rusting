mod calculator;
mod guess_game;

use std::io;

fn main() {
    loop {
        println!("Which project you would like to use?");

        println!("\t1.Calculator");
        println!("\t2.Guess Game");

        println!("Enter the corresponding number.");

        let number = get_number();

        match number {
            1 => {
                clear_screen();
                calculator::calculator();
                break;
            }

            2 => {
                clear_screen();
                guess_game::guess_game();
                break;
            }

            _ => {}
        };
    }

    println!();

    println!("Press enter to exit!");
    io::stdin().read_line(&mut String::new()).unwrap();
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

fn clear_screen() {
    for _i in 1..100 {
        println!()
    }
}
