use super::common::KeyStatus;
use super::common::KEYS_NUM;

#[derive(Debug)]
pub struct Keyboard {
    keys: [KeyStatus; KEYS_NUM],
}

impl std::fmt::Display for Keyboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for status in &self.keys {
            write!(f, "{}", status)?;
        }
        Ok(())
    }
}

impl Keyboard {
    /// Return `true` if the target key's status is changed.
    pub fn update_status(&mut self, key: char, status: KeyStatus) -> bool {
        if status < self.keys[(key as u8 - b'A') as usize] {
            self.keys[(key as u8 - b'A') as usize] = status;
            true
        } else {
            false
        }
    }
    /// Set all keys to `KeyStatus::Unknown` by default.
    pub fn new() -> Self {
        Self {
            keys: [KeyStatus::Unknown; KEYS_NUM],
        }
    }
}
