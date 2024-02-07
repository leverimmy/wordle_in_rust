use clap::Parser;
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, Write};
use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::collections::HashSet;
use std::fs::read_to_string;

mod cli;
use cli::Cli;
mod status;
use status::Status;
include!("builtin_words.rs");

const TOTAL_CHANCES: i32 = 6;
const WORD_LENGTH: usize = 5;
const ALPHABET_SIZE: usize = 26;
const TOP_N: usize = 5;

enum Outcome {
    SUCCESS,
    FAILED,
}

/// Checks the validity of guessed word
fn is_valid(round: i32, word: &str, difficult: bool, last_guessed_strings: &Option<&String>, last_word_state: &Option<&[Status; WORD_LENGTH]>, acceptable_set: &Vec<String>) -> bool {
    if !difficult || round == 1 {
        ACCEPTABLE.contains(&(word.to_lowercase().as_str()))
    } else {
        let last_guessed_strings = last_guessed_strings.unwrap();
        let last_word_state = last_word_state.unwrap();
        let len = word.len();
        // 标准黄色字母个数
        let mut std_count = [0; ALPHABET_SIZE];
        // 已有黄色字母个数
        let mut counted = [0; ALPHABET_SIZE];

        // 检查绿色字母
        for i in 0..len {
            let letter = word.chars().nth(i).unwrap();
            let std_letter = last_guessed_strings.chars().nth(i).unwrap();
            let index = (letter as u8 - b'A') as usize;
            if last_word_state[i] == Status::GREEN && letter != std_letter {
                return false
            } else {
                if last_word_state[i] == Status::YELLOW {
                    let std_index = (std_letter as u8 - b'A') as usize;
                    std_count[std_index] += 1;
                }
                counted[index] += 1;
            }
        }
        // 检查黄色字母
        for i in 0..len {
            let std_letter = last_guessed_strings.chars().nth(i).unwrap();
            let std_index = (std_letter as u8 - b'A') as usize;
            if last_word_state[i] == Status::YELLOW {
                if counted[std_index] < std_count[std_index] {
                    return false
                }
            }
        }
        true
    }
}

/// Updates the state of the alphabet
fn update_state(guess: &str, word_state: &mut[Status; WORD_LENGTH], alphabet_state: &mut[Status; ALPHABET_SIZE], answer: &str) {
    assert_eq!(guess.len(), answer.len());
    let len = guess.len();

    let mut counted = [0; ALPHABET_SIZE];
    let std_count: [i32; 26] = answer.chars()
    .fold([0; 26], |mut acc, c| {
        let index = (c as u8 - b'A') as usize;
        acc[index] += 1;
        acc
    })
    .into();
    // Match all the greens
    for i in 0usize..len {
        let guess_letter = guess.chars().nth(i).unwrap();
        let std_letter = answer.chars().nth(i).unwrap();
        let index = (guess_letter as u8 - b'A') as usize;

        if guess_letter == std_letter {
            counted[index] += 1;
            word_state[i] = Status::GREEN;
        }
    }
    // Match the others
    for i in 0usize..len {
        let guess_letter = guess.chars().nth(i).unwrap();
        let std_letter = answer.chars().nth(i).unwrap();
        let index = (guess_letter as u8 - b'A') as usize;

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
    for letter in 'A'..='Z' {
        let index = (letter as u8 - b'A') as usize;
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
fn print_state_tty(saved_guessed_strings: &Vec<String>, saved_word_state: &Vec<[Status; WORD_LENGTH]>, saved_alphabet_state: &Vec<[Status; ALPHABET_SIZE]>) {
    assert_eq!(saved_word_state.len(), saved_alphabet_state.len());
    let len = saved_word_state.len();
    for i in 0..len {
        let guess = saved_guessed_strings[i].to_string();
        let word_state = saved_word_state[i];
        for j in 0..word_state.len() {
            let letter = guess.chars().nth(j).unwrap();
            match word_state[j] {
                Status::RED => print!("{}", letter.to_string().bold().red()),
                Status::YELLOW => print!("{}", letter.to_string().bold().yellow()),
                Status::GREEN => print!("{}", letter.to_string().bold().green()),
                Status::UNKNOWN => print!("{}", letter.to_string().bold()),
            }
        }
        print!(" ");
        let alphabet_state = saved_alphabet_state[i];
        for letter in 'A'..='Z' {
            let index = (letter as u8 - b'A') as usize;
            match alphabet_state[index] {
                Status::RED => print!("{}", letter.to_string().bold().red()),
                Status::YELLOW => print!("{}", letter.to_string().bold().yellow()),
                Status::GREEN => print!("{}", letter.to_string().bold().green()),
                Status::UNKNOWN => print!("{}", letter.to_string().bold()),
            }
        }
        println!("");
        io::stdout().flush().unwrap();
    }
}

/// Returns the top n frequent strings
fn find_most_frequent_strings(strings: &Vec::<String>, n: usize) -> Vec<(String, u32)> {
    let mut frequency_map: HashMap<String, u32> = HashMap::new();

    // 统计出现次数
    for s in strings {
        *frequency_map.entry(s.to_string()).or_insert(0) += 1;
    }

    // 排序并返回前 n 个出现次数最多的 String
    let mut frequency_vec: Vec<(String, u32)> = frequency_map.into_iter().collect();
    frequency_vec.sort_by(|(s1, c1), (s2, c2)| {
        c2.cmp(&c1).then_with(|| s1.cmp(&s2))
    });

    if frequency_vec.len() < n {
        frequency_vec
    } else {
        frequency_vec.into_iter().take(n).collect()
    }
}

/// Load word lists from files
fn load_word_list(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = read_to_string(file_path)?;
    let word_list: Vec<String> = content
        .lines()
        .map(|line| line.trim().to_uppercase())
        .collect();
    let unique_words: HashSet<String> = word_list.iter().cloned().collect();

    if unique_words.len() != word_list.len() {
        return Err("Duplicate words found in the word list".into());
    }

    Ok(word_list)
}

/// Check subset
fn check_subset(subset: &[String], superset: &[String]) -> bool {
    let subset_set: HashSet<&String> = subset.iter().collect();
    let superset_set: HashSet<&String> = superset.iter().collect();
    subset_set.is_subset(&superset_set)
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);
    // let is_tty = false;
    let config = Cli::parse();

    let mut bias = 0;
    let mut win_rounds = 0;
    let mut total_rounds = 0;
    let mut win_guesses = 0;
    let mut all_guesses_strings: Vec<String> = Vec::new();

    let final_word_list = load_word_list(&config.final_set)?;
    let acceptable_word_list = load_word_list(&config.acceptable_set)?;
    if !check_subset(&final_word_list, &acceptable_word_list) {
        return Err("The final word list is not a strict subset of the acceptable word list".into());
    }
    
    // 排序候选词库和可用词库
    let mut final_word_list_sorted = final_word_list.clone();
    final_word_list_sorted.sort();
    let mut acceptable_word_list_sorted = acceptable_word_list.clone();
    acceptable_word_list_sorted.sort();

    loop {
        let mut saved_guessed_strings: Vec<String> = Vec::new();
        let mut saved_alphabet_state: Vec<[Status; ALPHABET_SIZE]> = Vec::new();
        let mut saved_word_state: Vec<[Status; WORD_LENGTH]> = Vec::new();
        let mut answer = String::new();
        if config.random {
            let mut rng = StdRng::seed_from_u64(config.seed);
            let mut final_set_vec = final_word_list_sorted.clone();
            final_set_vec.shuffle(&mut rng);
            // Get a random string as the final answer
            answer = final_set_vec[(config.day + bias - 1) as usize].to_ascii_uppercase();
        } else {
            if config.word != "" {
                // Get the string in config as the final answer
                answer = config.word.to_ascii_uppercase();
            } else {
                // Get user's input string as the final answer
                io::stdin().read_line(&mut answer)?;
            }
        }
        let answer = answer.to_ascii_uppercase();
        let answer = answer.trim();
        // let answer = answer.trim();
        let mut chances_left = TOTAL_CHANCES;
        let mut alphabet_state = [Status::UNKNOWN; ALPHABET_SIZE];

        let status = loop {
            chances_left -= 1;

            if chances_left < 0 {
                break Outcome::FAILED;
            }

            let mut guess = String::new();
            io::stdin().read_line(&mut guess)?;
            let guess = guess.to_ascii_uppercase();
            let guess = guess.trim();

            if is_valid(TOTAL_CHANCES - chances_left, guess, config.difficult, &saved_guessed_strings.last(), &saved_word_state.last(), &acceptable_word_list_sorted) {
                saved_guessed_strings.push(guess.to_string().clone());
                all_guesses_strings.push(guess.to_string().clone());
                let mut word_state = [Status::UNKNOWN; WORD_LENGTH];
                update_state(guess, &mut word_state, &mut alphabet_state, answer);
                saved_word_state.push(word_state);
                saved_alphabet_state.push(alphabet_state);
                match is_tty {
                    true => print_state_tty(&saved_guessed_strings, &saved_word_state, &saved_alphabet_state),
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
            Outcome::SUCCESS => {
                println!("CORRECT {}", TOTAL_CHANCES - chances_left);
                win_rounds += 1;
                win_guesses += TOTAL_CHANCES - chances_left;
            },
            Outcome::FAILED => {
                println!("FAILED {}", answer);
            },
        }
        total_rounds += 1;

        if config.stats {
            println!("{} {} {:.2}", win_rounds, total_rounds - win_rounds,
                match win_rounds {
                    0 => 0f64,
                    _ => win_guesses as f64 / win_rounds as f64,
                });
            let top5 = find_most_frequent_strings(&all_guesses_strings, TOP_N);

            for (i, (word, total)) in top5.iter().enumerate() {
                print!("{} {}", word, total);
                if i < top5.len() - 1 {
                    print!(" ");
                } else {
                    println!();
                }
            }
        }

        if config.word != "" {
            break;
        }
        let mut option = String::new();
        io::stdin().read_line(&mut option)?;
        let option = option.trim();

        match option {
            "Y" => bias += 1,
            "N" => break,
            _ => panic!("Invalid input!")
        }
    }

    Ok(())
}
