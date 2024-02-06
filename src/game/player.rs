use std::collections::HashMap;

use super::common::Word;

#[derive(Debug)]
pub struct Player {
    used_words: Vec<Word>,
    trys: Vec<(usize, bool)>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            used_words: Vec::new(),
            trys: Vec::new(),
        }
    }
    pub fn recode_input(&mut self, w: Word) {
        self.used_words.push(w);
    }
    pub fn evaluate(&mut self, cnt: usize, is_scuccess: bool) {
        self.trys.push((cnt, is_scuccess));
    }
    pub fn analyse(&self) -> (usize, usize, f32) {
        let tmp = self.trys.iter().filter(|k| k.1 == true).count();
        let mut rate: f32 = self.trys.iter().map(|k| k.0).sum::<usize>() as f32;
        if tmp == 0 {
            (0, self.trys.len(), 0 as f32)
        } else {
            rate = rate / (tmp as f32);
            (tmp, self.trys.len() - tmp, rate)
        }
    }
    pub fn show_most_used_words(&self) -> Vec<(Word, usize)> {
        let mut words_cnt: HashMap<Word, usize> = HashMap::new();
        for w in &self.used_words {
            *words_cnt.entry(w.clone()).or_default() += 1;
        }
        let mut sorted_words: Vec<(Word, usize)> = Vec::new();
        for (w, cnt) in words_cnt {
            sorted_words.push((w, cnt));
        }
        sorted_words.sort_by_cached_key(|(k, _)| k.clone());
        sorted_words.reverse();
        sorted_words.sort_by_cached_key(|k| (*k).1);
        sorted_words.reverse();
        sorted_words.iter().take(5).cloned().collect()
    }
}
