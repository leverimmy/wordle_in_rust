use std::collections::HashSet;

use super::builtin_words::ACCEPTABLE;
use super::builtin_words::FINAL;
use super::common::Word;
use super::common::WORD_LENGTH;
use super::WordsPath;

#[derive(Debug)]
pub struct WordSet {
    pub final_words: Vec<Word>,
    acceptable_words: Vec<Word>,
}
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl WordSet {
    pub fn is_valid_guess(&self, w: &Word) -> bool {
        self.acceptable_words.contains(w)
    }
    pub fn new(option: WordsPath) -> Self {
        match option {
            WordsPath::Default => {
                let mut fin: Vec<Word> = Vec::new();
                let mut acc: Vec<Word> = Vec::new();
                fin.extend(
                    FINAL
                        .iter()
                        .map(|x| x.to_ascii_uppercase())
                        .collect::<Vec<Word>>(),
                );
                acc.extend(
                    ACCEPTABLE
                        .iter()
                        .map(|x| x.to_ascii_uppercase())
                        .collect::<Vec<Word>>(),
                );
                Self {
                    final_words: fin,
                    acceptable_words: acc,
                }
            }

            WordsPath::Customized(final_file, acceptable_file) => {
                let mut builtin_fin: HashSet<String> = HashSet::new();
                let mut builtin_acc: HashSet<String> = HashSet::new();

                builtin_fin.extend(
                    FINAL
                        .iter()
                        .map(|&x| String::from(x).to_ascii_uppercase())
                        .collect::<HashSet<String>>(),
                );
                builtin_acc.extend(
                    ACCEPTABLE
                        .iter()
                        .map(|&x| String::from(x).to_ascii_uppercase())
                        .collect::<HashSet<String>>(),
                );
                let mut acc_hash: HashSet<String> = HashSet::new();
                let mut fin_hash: HashSet<String> = HashSet::new();
                if let Ok(lines) = read_lines(acceptable_file.as_str()) {
                    for line in lines {
                        if let Ok(w) = line {
                            let w = w.to_ascii_uppercase();
                            if w.len() == WORD_LENGTH && w.is_ascii() {
                                if builtin_acc.contains(&w) {
                                    acc_hash.insert(w);
                                }
                            } else {
                            }
                        } else {
                            break;
                        }
                    }
                }
                if let Ok(lines) = read_lines(final_file.as_str()) {
                    for line in lines {
                        if let Ok(w) = line {
                            let w = w.to_ascii_uppercase();
                            if w.len() == WORD_LENGTH && w.is_ascii() {
                                if builtin_fin.contains(&w) {
                                    fin_hash.insert(w);
                                }
                            } else {
                            }
                        } else {
                            break;
                        }
                    }
                }
                let mut fin: Vec<Word> = fin_hash.drain().collect();
                let mut acc: Vec<Word> = acc_hash.drain().collect();
                fin.sort_unstable();
                acc.sort_unstable();
                Self {
                    final_words: fin,
                    acceptable_words: acc,
                }
            }
        }
    }
}
