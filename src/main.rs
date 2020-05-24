use std::io;
use std::io::{BufRead, BufReader, Result};

fn main() {
    println!("Hello! What can I ruin for you today?");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let vars: Vec<String> = phrase_variations(&input.to_lowercase().trim());

    println!("Ah, did you mean: {:?}?", vars);
}

fn is_word(w: &str) -> bool {
    const BYTES: &[u8] = include_bytes!("../dict/english3.txt");

    let reader = BufReader::new(BYTES);

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

fn phrase_variations(s: &str) -> Vec<String> {
    let words: Vec<String> = s.split(" ").map(|w| w.to_owned()).collect();

    let mut results: Vec<String> = Vec::new();

    for word in &words {
        let variations = word_variations(word);
        for var in variations {
            let result: String = (&words.join(" ").replace(word, &var)).to_owned();
            results.push(result);
        }
    }

    return results
        .iter()
        .map(|phrase| {
            phrase
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|w| uppercase_first_letter(w))
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect();
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}
