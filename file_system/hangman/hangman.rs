use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::thread_rng;
use rand::seq::SliceRandom;
use text_io::read;

fn main() {
    let words = read_words_from_file("words.txt");
    let word = words.choose(&mut thread_rng()).unwrap();
    let mut guessed_letters = Vec::new();
    let mut guessed_word = vec!['_'; word.len()];
    let mut remaining_attempts = 6;

    loop {
        println!("Guessed word: {}", guessed_word.iter().collect::<String>());
        println!("Remaining attempts: {}", remaining_attempts);

        if guessed_word.iter().eq(word.chars()) {
            println!("Congratulations! You won!");
            break;
        }

        if remaining_attempts == 0 {
            println!("You lost! The word was {}", word);
            break;
        }

        println!("Please enter a letter: ");
        let letter: char = read!();

        if guessed_letters.contains(&letter) {
            println!("You already guessed that letter!");
            continue;
        }

        guessed_letters.push(letter);

        for (i, c) in word.chars().enumerate() {
            if c == letter {
                guessed_word[i] = letter;
            }
        }

        if !word.contains(letter) {
            remaining_attempts -= 1;
        }
    }
}

fn read_words_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Unable to open file!");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| line.expect("Unable to read line"))
        .collect()
}
