pub const WORD_LENGTH: usize = 5;
pub const KEYS_NUM: usize = 26;

/// Representing a possible status of a key.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyStatus {
    /// `'G'`, Green for exactly correct.
    Hit,
    /// `'Y'`, Yellow for being at wrong position.
    Close,
    /// `'R'`, Red for more than they are.
    Missed,
    /// `'X'`, Whether correct or not is unknown.
    Unknown,
}
pub const RIGHT_GUESS: WordStatus = WordStatus {
    data: [KeyStatus::Hit; WORD_LENGTH],
};
impl std::fmt::Display for KeyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            KeyStatus::Hit => write!(f, "{}", "G"),
            KeyStatus::Close => write!(f, "{}", "Y"),
            KeyStatus::Missed => write!(f, "{}", "R"),
            KeyStatus::Unknown => write!(f, "{}", "X"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WordStatus {
    pub data: [KeyStatus; WORD_LENGTH],
}

impl std::fmt::Display for WordStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for status in &self.data {
            write!(f, "{}", status)?;
        }
        Ok(())
    }
}

impl WordStatus {
    pub fn new() -> Self {
        Self {
            data: [KeyStatus::Unknown; WORD_LENGTH],
        }
    }
}

pub type Word = String;
