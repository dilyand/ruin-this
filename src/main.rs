use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Result};

fn main() {
    println!("Give me a string and I'll tell you if it's a word.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if is_word(&input.trim().to_lowercase()) == true {
        println!("{} is a word", input.trim())
    } else {
        println!("{} is not a word", input.trim())
    };
}

fn is_word(w: &str) -> bool {
    let file = File::open("dict/words_alpha.txt").expect("Cannot open file.");
    let reader = BufReader::new(file);

    let result: Vec<_> = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|word| word == w)
        .collect();

    return !result.is_empty();
}
