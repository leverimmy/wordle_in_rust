use clap::Parser;
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, Write};
use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::collections::HashSet;
use std::fs::{self, read_to_string};

mod cli;
use cli::Cli;
mod status;
use status::Status;
mod state;
use state::{State, Game};

const TOTAL_CHANCES: usize = 6;
const WORD_LENGTH: usize = 5;
const ALPHABET_SIZE: usize = 26;
const TOP_N: usize = 5;

enum Outcome {
    SUCCESS,
    FAILED,
}

/// Checks the validity of guessed word
fn is_valid(word: &String, difficult: bool,
            last_guessed_string: &Option<&String>,
            last_word_state: &Option<&[Status; WORD_LENGTH]>,
            acceptable_set: &Vec<String>) -> bool {
    if !difficult || *last_guessed_string == None {
        acceptable_set.contains(&word.to_string())
    } else {
        let last_guessed_string = last_guessed_string.unwrap();
        let last_word_state = last_word_state.unwrap();
        let len = word.len();
        // 标准黄色字母个数
        let mut std_count = [0; ALPHABET_SIZE];
        // 已有黄色字母个数
        let mut counted = [0; ALPHABET_SIZE];

        // 检查绿色字母
        for i in 0..len {
            let letter = word.chars().nth(i).unwrap();
            let std_letter = last_guessed_string.chars().nth(i).unwrap();
            if last_word_state[i] == Status::GREEN && letter != std_letter {
                return false
            } else {
                if last_word_state[i] == Status::YELLOW {
                    let std_index = (std_letter as u8 - b'A') as usize;
                    std_count[std_index] += 1;
                }
                let index = (letter as u8 - b'A') as usize;
                counted[index] += 1;
            }
        }
        // 检查黄色字母
        for i in 0..len {
            let std_letter = last_guessed_string.chars().nth(i).unwrap();
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
fn update_state(guess: &String, answer: &String,
                word_state: &mut[Status; WORD_LENGTH],
                alphabet_state: &mut[Status; ALPHABET_SIZE]) {
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
fn print_state_not_tty(word_state: &[Status; WORD_LENGTH],
                        alphabet_state: &[Status; ALPHABET_SIZE]) {
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
fn print_state_tty(saved_guessed_strings: &Vec<String>,
                    saved_word_state: &Vec<[Status; WORD_LENGTH]>,
                    saved_alphabet_state: &Vec<[Status; ALPHABET_SIZE]>) {
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
    }
    io::stdout().flush().unwrap();
}

/// Returns the top n frequent strings
fn find_most_frequent_strings(strings: &Vec::<String>, n: usize) -> Vec<(String, usize)> {
    let mut frequency_map: HashMap<String, usize> = HashMap::new();

    // 统计出现次数
    for s in strings {
        *frequency_map.entry(s.to_string()).or_insert(0) += 1;
    }

    // 排序并返回前 n 个出现次数最多的 String
    let mut frequency_vec: Vec<(String, usize)> = frequency_map.into_iter().collect();
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
fn load_word_list(file_path: &String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
    let mut config = Cli::parse();

    // 如果指定了 config.json
    if config.config != None {
        if let Ok(file) = fs::read(config.config.clone().unwrap()) {
            // 如果当前文件存在，则取出来
            let config_file: Cli = serde_json::from_slice(&file).expect("JSON 反序列化失败");
            config.merge(config_file);
        }
    }

    let mut bias = 0;
    let mut win_rounds = 0;
    let mut total_rounds = 0;
    let mut win_guesses = 0;
    let mut all_guesses_strings: Vec<String> = Vec::new();

    let mut final_word_list = load_word_list(&config.final_set.unwrap_or("./final_set.txt".to_string()))?;
    let mut acceptable_word_list = load_word_list(&config.acceptable_set.unwrap_or("./acceptable_set.txt".to_string()))?;
    if !check_subset(&final_word_list, &acceptable_word_list) {
        return Err("The final word list is not a strict subset of the acceptable word list".into());
    }
    
    // 排序候选词库和可用词库
    final_word_list.sort();
    acceptable_word_list.sort();

    let mut data: State = State::default();

    // 如果指定了 state.json
    if config.state != None {
        if let Ok(file) = fs::read(config.state.clone().unwrap()) {
            // 如果当前文件存在，则取出来
            data = serde_json::from_slice(&file).expect("JSON 反序列化失败");
            for game in data.games.iter() {
                let answer = &game.answer;
                let last_guess = game.guesses.last().unwrap();
                if last_guess == answer {
                    win_rounds += 1;
                    win_guesses += game.guesses.len();
                }
                total_rounds += 1;
                game.guesses.iter().for_each(|g| all_guesses_strings.push(g.to_string()));
            }
        }
    }

    loop {
        let mut saved_guessed_strings: Vec<String> = Vec::new();
        let mut saved_alphabet_state: Vec<[Status; ALPHABET_SIZE]> = Vec::new();
        let mut saved_word_state: Vec<[Status; WORD_LENGTH]> = Vec::new();
        let mut answer = String::new();

        
        if config.random {
            // 如果为随机模式
            let mut rng = StdRng::seed_from_u64(config.seed.unwrap_or(19260817));
            let mut final_set_vec = final_word_list.clone();
            final_set_vec.shuffle(&mut rng);
            // Get a random string as the final answer
            answer = final_set_vec[(config.day.unwrap_or(1) + bias - 1) as usize].to_ascii_uppercase();
        } else {
            if config.word != None {
                // 如果指定单词
                answer = config.word.clone().unwrap().to_ascii_uppercase();
            } else {
                // 从标准输入取出单词
                io::stdin().read_line(&mut answer)?;
            }
        }
        let answer = answer.trim().to_uppercase();
        
        let mut chances_used = 0usize;
        let mut alphabet_state = [Status::UNKNOWN; ALPHABET_SIZE];

        // 进行一轮猜测
        let status = loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess)?;
            let guess = guess.trim().to_uppercase();

            // 判断是否为合法输入
            // 1) 在单词库中
            // 2) 如果为 hard mode，则需要满足条件
            if is_valid(&guess, config.difficult, &saved_guessed_strings.last(), &saved_word_state.last(), &acceptable_word_list) {
                chances_used += 1;
                
                saved_guessed_strings.push(guess.to_string());
                all_guesses_strings.push(guess.to_string());
                let mut word_state = [Status::UNKNOWN; WORD_LENGTH];

                // 更新单词状态和字母表状态
                update_state(&guess, &answer, &mut word_state, &mut alphabet_state);
                saved_word_state.push(word_state);
                saved_alphabet_state.push(alphabet_state);

                // 判断是否为交互模式
                match is_tty {
                    true => print_state_tty(&saved_guessed_strings, &saved_word_state, &saved_alphabet_state),
                    false => print_state_not_tty(&word_state, &alphabet_state),
                }
                // 判断是否猜对
                if guess == answer {
                    break Outcome::SUCCESS;
                } else if chances_used == TOTAL_CHANCES {
                    break Outcome::FAILED;
                }
            } else {
                println!("INVALID");
                continue;
            }
        };

        // 完成一轮游戏，输出结果
        total_rounds += 1;
        match status {
            Outcome::SUCCESS => {
                println!("CORRECT {}", chances_used);
                win_rounds += 1;
                win_guesses += chances_used;
            },
            Outcome::FAILED => {
                println!("FAILED {}", answer);
            },
        }

        // 输出统计数据
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

        // 更新存档
        if config.state != None {
            data.total_rounds = total_rounds;
            data.games.push(Game {
                answer: answer.to_string(),
                guesses: saved_guessed_strings.clone(),
            });
        }

        // 是否继续游戏
        if config.word != None {
            break;
        }
        let mut option = String::new();
        io::stdin().read_line(&mut option)?;
        let option = option.trim().to_uppercase();

        match option.as_str() {
            "Y" => bias += 1,
            "N" => break,
            _ => panic!("Invalid input!")
        }
    }

    // 进行存档
    if config.state != None {
        fs::write(config.state.unwrap(), serde_json::to_string_pretty(&data).unwrap())?;
    }

    Ok(())
}
