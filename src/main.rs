use std::io;
use std::process::Command;

mod calculator;
mod guess_game;
mod utils;
mod temperature_converter;

enum Projects {
    Calculator = 1,
    GuessGame = 2,
    TemperatureConverter = 3
}

impl Projects {
    fn from_number(value: u32) -> Option<Projects> {
        match value {
            1 => Some(Projects::Calculator),
            2 => Some(Projects::GuessGame),
            3 => Some(Projects::TemperatureConverter),
            _ => {
                None
            }
        }
    }
}

fn main() {
    loop {
        println!("Which project you would like to use?");

        println!("\t1.Calculator");
        println!("\t2.Guess Game");
        println!("\t2.Temperature Converter");

        println!("Enter the corresponding number.");

        let number = Projects::from_number(utils::get_number_i32() as u32);

        match number {
            Some(project) => {
                match project {
                    Projects::Calculator => {
                        clear_screen();
                        calculator::calculator();
                        break;
                    }
                    Projects::GuessGame => {
                        clear_screen();
                        guess_game::guess_game();
                        break;
                    }
                    Projects::TemperatureConverter => {
                        clear_screen();
                        temperature_converter::temperature_convertor();
                        break;
                    }
                }
            }
            None => {
                println!("Enter a valid project!");
                println!();
            }
        };
    }

    println!();

    println!("Press enter to exit!");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}
