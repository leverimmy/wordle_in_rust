use clap::Parser;
use colored::Colorize;
use std::io::{self, Write};
use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;

mod cli;
use cli::Cli;
mod status;
use status::Status;
include!("builtin_words.rs");

const TOTAL_CHANCES: i32 = 6;
const WORD_LENGTH: usize = 5;
const ALPHABET_SIZE: usize = 26;

enum Outcome {
    SUCCESS,
    FAILED,
}

/// Checks the invalidity of guessed word
fn is_valid(word: &str) -> bool {
    ACCEPTABLE.contains(&word)
}

/// Updates the state of the alphabet
fn update_state(guess: &str, word_state: &mut [Status; WORD_LENGTH], alphabet_state: &mut [Status; ALPHABET_SIZE], answer: &str) {
    assert_eq!(guess.len(), answer.len());
    let len = guess.len();

    let mut counted = [0; ALPHABET_SIZE];
    let std_count: [i32; 26] = answer.chars()
    .fold([0; 26], |mut acc, c| {
        let index = (c as u8 - b'a') as usize;
        acc[index] += 1;
        acc
    })
    .into();
    // Match all the greens
    for i in 0usize..len {
        let guess_letter = guess.chars().nth(i).unwrap();
        let std_letter = answer.chars().nth(i).unwrap();
        let index = (guess_letter as u8 - b'a') as usize;

        if guess_letter == std_letter {
            counted[index] += 1;
            word_state[i] = Status::GREEN;
        }
    }
    // Match the others
    for i in 0usize..len {
        let guess_letter = guess.chars().nth(i).unwrap();
        let std_letter = answer.chars().nth(i).unwrap();
        let index = (guess_letter as u8 - b'a') as usize;

        if guess_letter != std_letter {
            counted[index] += 1;

            if counted[index] <= std_count[index] {
                word_state[i] = std::cmp::max(word_state[i], Status::YELLOW);
            } else {
                word_state[i] = std::cmp::max(word_state[i], Status::RED);
            }
        }
        alphabet_state[index] = std::cmp::max(alphabet_state[index], word_state[i]);
    }
}

/// Print the state of the word and the alphabet(not in tty)
fn print_state_not_tty(word_state: &[Status; WORD_LENGTH], &alphabet_state: &[Status; ALPHABET_SIZE]) {
    for i in 0..word_state.len() {
        match word_state[i] {
            Status::RED => print!("R"),
            Status::YELLOW => print!("Y"),
            Status::GREEN => print!("G"),
            Status::UNKNOWN => print!("X"),
        }
    }
    print!(" ");
    for letter in 'a'..='z' {
        let index = (letter as u8 - b'a') as usize;
        match alphabet_state[index] {
            Status::RED => print!("R"),
            Status::YELLOW => print!("Y"),
            Status::GREEN => print!("G"),
            Status::UNKNOWN => print!("X"),
        }
    }
    println!("");
    io::stdout().flush().unwrap();
}

/// Print the state of the word and the alphabet(in tty)
fn print_state_tty(guess: &str, word_state: &[Status; WORD_LENGTH], &alphabet_state: &[Status; ALPHABET_SIZE]) {
    for i in 0..word_state.len() {
        let letter = guess.chars().nth(i).unwrap();
        match word_state[i] {
            Status::RED => print!("{}", letter.to_ascii_uppercase().to_string().bold().red()),
            Status::YELLOW => print!("{}", letter.to_ascii_uppercase().to_string().bold().yellow()),
            Status::GREEN => print!("{}", letter.to_ascii_uppercase().to_string().bold().green()),
            Status::UNKNOWN => print!("{}", letter.to_ascii_uppercase().to_string().bold()),
        }
    }
    print!(" ");
    for letter in 'a'..='z' {
        let index = (letter as u8 - b'a') as usize;
        match alphabet_state[index] {
            Status::RED => print!("{}", letter.to_ascii_uppercase().to_string().bold().red()),
            Status::YELLOW => print!("{}", letter.to_ascii_uppercase().to_string().bold().yellow()),
            Status::GREEN => print!("{}", letter.to_ascii_uppercase().to_string().bold().green()),
            Status::UNKNOWN => print!("{}", letter.to_ascii_uppercase().to_string().bold()),
        }
    }
    println!("");
    io::stdout().flush().unwrap();
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let is_tty = atty::is(atty::Stream::Stdout);
    let is_tty = false;
    let config = Cli::parse().get();

    let mut answer = String::new();

    if config.random {
        let mut rng = StdRng::seed_from_u64(config.seed);
        let mut final_set_vec = FINAL.to_vec();
        final_set_vec.shuffle(&mut rng);
        // Get a random string as the final answer
        answer = final_set_vec[(config.day - 1) as usize].to_string();
    } else {
        if config.word != "" {
            // Get the string in config as the final answer
            answer = config.word.to_string();
        } else {
            // Get user's input string as the final answer
            io::stdin().read_line(&mut answer)?;
        }
    }

    let answer = answer.trim();
    let mut chances_left = TOTAL_CHANCES;
    let mut alphabet_state = [Status::UNKNOWN; ALPHABET_SIZE];

    let status = loop {
        chances_left -= 1;

        if chances_left < 0 {
            break Outcome::FAILED;
        }

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;
        let guess = guess.trim();

        if is_valid(guess) {
            let mut word_state = [Status::UNKNOWN; WORD_LENGTH];
            update_state(guess, &mut word_state, &mut alphabet_state, answer);
            match is_tty {
                true => print_state_tty(guess, &word_state, &alphabet_state),
                false => print_state_not_tty(&word_state, &alphabet_state),
            }
            if guess == answer {
                break Outcome::SUCCESS;
            }
        } else {
            chances_left += 1;
            println!("INVALID");
            continue;
        }
    };

    match status {
        Outcome::SUCCESS => println!("CORRECT {}", TOTAL_CHANCES - chances_left),
        Outcome::FAILED => println!("FAILED {}", answer.to_ascii_uppercase()),
    }

    Ok(())
}
