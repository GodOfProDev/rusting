use std::fs;

pub fn grep() {
    let contents = fs::read_to_string("").expect("/text.txt");

    println!(contents)
}