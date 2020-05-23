use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Result};

fn main() {
    println!("Give me a word and I'll tell you similar words.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let vars = word_variations(&input.to_lowercase().trim());

    println!("Here are some variations: {:?}", vars);
}

fn is_word(w: &str) -> bool {
    let file = File::open("dict/english3.txt").expect("Cannot open file.");
    let reader = BufReader::new(file);

    let result: Vec<_> = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|word| word == w)
        .collect();

    return !result.is_empty();
}

fn word_variations(w: &str) -> Vec<String> {
    const ASCII_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut v: Vec<String> = Vec::new();

    for i in 0..w.len() {
        for l in &ASCII_LOWER {
            let mut test_word_char: Vec<char> = w.chars().collect();
            if test_word_char[i] != *l {
                test_word_char[i] = *l;
                let s_test_word: String = test_word_char.into_iter().collect();
                let test_word: &str = &*s_test_word;
                if is_word(test_word) == true {
                    v.push((*test_word).to_owned())
                }
            }
        }
    }

    return v;
}
