mod builtin_words;
pub mod common;
mod keyboard;
mod player;
mod word_set;

use self::common::KeyStatus;
use self::common::Word;
use self::common::WordStatus;
use self::common::KEYS_NUM;
use self::common::RIGHT_GUESS;
use self::common::WORD_LENGTH;
use self::keyboard::Keyboard;
use self::player::Player;
use self::word_set::WordSet;
use std::collections::HashSet;

use rand::seq::SliceRandom;
use rand::SeedableRng;

/// One may guess at most `GUESS_MAX_NUM` times in a round.
const MAX_GUESS_NUM: usize = 6;

#[derive(Debug)]
pub enum GameLevel {
    Default,
    Hard,
}

#[derive(Debug, Clone, Copy)]
pub enum GameMode {
    /// Answer is randomly generated.
    Random,
    /// Answer is explicitly assigned
    Assigned,
}

/// Option for initializing `WordSet`.
pub enum WordsPath {
    /// Use builtin words
    Default,
    /// First `string` for file path to final words, second acceptable words.
    Customized(String, String),
}

/// Wordle game manager.
pub struct Game {
    word_set: WordSet,
    level: GameLevel,
    /// Single round in a game
    round: Round,
    used_ans: HashSet<Word>,
    player: Player,
}

pub enum GuessError {
    Invalid,
    OverTry,
}

impl Game {
    pub fn new(level: GameLevel, dictonary: WordsPath, seed: u64) -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let mut word_set = WordSet::new(dictonary);
        word_set.final_words.shuffle(&mut rng);
        Self {
            level,
            word_set,
            round: Round::new(),
            used_ans: HashSet::new(),
            player: Player::new(),
        }
    }
    fn is_valid_guess(&self, w: &Word) -> bool {
        self.word_set.is_valid_guess(w) && self.round.is_valid_guess(w, &self.level)
    }
    pub fn is_round_end(&self) -> bool {
        self.round.input.len() == MAX_GUESS_NUM
    }
    pub fn is_word(s: &String) -> bool {
        s.chars().all(|ch| ch.is_ascii_uppercase()) && s.len() == WORD_LENGTH
    }

    pub fn gen_answer_from(&mut self, d: usize) -> Word {
        self.word_set.final_words.get(d - 1).unwrap().clone()
    }
    pub fn start_new_round(&mut self, ans: Word) {
        self.round = Round::new();
        let ans_copy = ans.clone();
        self.round = Round::new();
        self.round.set_ans(ans);
        self.used_ans.insert(ans_copy);
    }
    pub fn end_cur_round(&mut self) -> (bool, Word, usize) {
        for w in self.round.input.drain(..) {
            self.player.recode_input(w);
        }
        self.player
            .evaluate(self.round.success_cnt, self.is_success());
        (
            self.is_success(),
            self.show_answer().clone(),
            self.round.success_cnt,
        )
    }
    pub fn show_most_used_words(&self) -> Vec<(Word, usize)> {
        self.player.show_most_used_words()
    }
    pub fn show_analyse(&self) -> (usize, usize, f32) {
        self.player.analyse()
    }
    pub fn guess(&mut self, guess_word: &Word) -> Result<(WordStatus, &Keyboard), GuessError> {
        if self.round.input.len() == MAX_GUESS_NUM {
            Err(GuessError::OverTry)
        } else if self.is_valid_guess(guess_word) {
            let word_copy = guess_word.clone();
            let status = self.round.take_guess(word_copy);
            Ok((status, self.show_keyboard()))
        } else {
            Err(GuessError::Invalid)
        }
    }
    fn is_success(&self) -> bool {
        self.round.is_success
    }
    fn show_answer(&mut self) -> &Word {
        &self.round.answer
    }
    pub fn show_keyboard(&self) -> &Keyboard {
        &self.round.keyboard
    }
}

/// A round in game wordle.
#[derive(Debug)]
pub struct Round {
    answer: Word,
    input: Vec<Word>,
    input_status: Vec<WordStatus>,
    is_success: bool,
    pub success_cnt: usize,
    pub keyboard: Keyboard,
    hit_restrict: [bool; WORD_LENGTH],
    pub use_restrcit: HashSet<char>,
}

impl Round {
    pub fn new() -> Self {
        Self {
            answer: "".into(),
            input: Vec::new(),
            input_status: Vec::new(),
            is_success: false,
            success_cnt: 0,
            keyboard: Keyboard::new(),
            hit_restrict: [false; WORD_LENGTH],
            use_restrcit: HashSet::new(),
        }
    }
    pub fn set_ans(&mut self, ans: Word) {
        self.answer = ans;
    }
    pub fn is_valid_guess(&self, guess: &Word, level: &GameLevel) -> bool {
        if self.input.len() == 0 {
            true
        } else {
            match level {
                GameLevel::Default => return true,
                GameLevel::Hard => {}
            }

            if self.use_restrcit.iter().all(|x| guess.contains(*x)) {
            } else {
                return false;
            }
            for (i, ch) in guess.chars().enumerate() {
                if self.hit_restrict[i] {
                    if !(ch == self.answer.chars().nth(i).unwrap()) {
                        return false;
                    }
                }
            }
            true
        }
    }

    pub fn take_guess(&mut self, guess: Word) -> WordStatus {
        let mut res = WordStatus::new();
        let mut ans_cnt: [usize; KEYS_NUM] = [0; KEYS_NUM];
        let mut guess_cnt: [usize; KEYS_NUM] = [0; KEYS_NUM];

        for ch in guess.chars() {
            ans_cnt[(ch as u8 - b'A') as usize] = self.answer.chars().filter(|x| x == &ch).count();
            guess_cnt[(ch as u8 - b'A') as usize] = guess.chars().filter(|x| x == &ch).count();
        }

        for i in 0..WORD_LENGTH {
            if guess.chars().nth(i).unwrap() == self.answer.chars().nth(i).unwrap() {
                self.keyboard
                    .update_status(guess.chars().nth(i).unwrap(), KeyStatus::Hit);
                res.data[i] = KeyStatus::Hit;
                self.hit_restrict[i] = true;
                ans_cnt[(guess.chars().nth(i).unwrap() as u8 - b'A') as usize] -= 1;
                guess_cnt[(guess.chars().nth(i).unwrap() as u8 - b'A') as usize] -= 1;
            }
        }
        for i in 0..WORD_LENGTH {
            if guess.chars().nth(i).unwrap() == self.answer.chars().nth(i).unwrap() {
            } else if ans_cnt[(guess.chars().nth(i).unwrap() as u8 - b'A') as usize] == 0 {
                self.keyboard
                    .update_status(guess.chars().nth(i).unwrap(), KeyStatus::Missed);
                res.data[i] = KeyStatus::Missed;
            } else {
                self.keyboard
                    .update_status(guess.chars().nth(i).unwrap(), KeyStatus::Close);
                res.data[i] = KeyStatus::Close;
                ans_cnt[(guess.chars().nth(i).unwrap() as u8 - b'A') as usize] -= 1;
                guess_cnt[(guess.chars().nth(i).unwrap() as u8 - b'A') as usize] -= 1;
                self.use_restrcit.insert(guess.chars().nth(i).unwrap());
            }
        }
        self.input.push(guess);

        if res == RIGHT_GUESS {
            self.is_success = true;
            self.success_cnt = if self.success_cnt == 0 {
                self.input.len()
            } else {
                self.success_cnt
            }
        }
        self.input_status.push(res);
        res
    }
}
