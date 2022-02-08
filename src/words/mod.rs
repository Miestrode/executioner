use std::{collections::HashSet, fs, io, path::PathBuf};

use rand::prelude::SliceRandom;

use crate::ActiveState;

pub struct Words {
    pub words: Vec<String>,
}

impl Words {
    pub fn from(path: PathBuf) -> Result<Self, io::Error> {
        fs::read_to_string(path).map(|words| Self {
            words: words.split('\n').map(String::from).collect(),
        })
    }

    pub fn random_word(&self) -> String {
        self.words.choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn satisfiable_portion(&self, state: &ActiveState) -> f64 {
        self.words
            .iter()
            .filter(|&word| state.does_match(word))
            .count() as f64
            / (self.words.len() as f64)
    }

    pub fn filter_with_guess(&mut self, state: &ActiveState) {
        self.words = self
            .words
            .drain(..)
            .filter(|word| state.does_match(word))
            .collect();
    }
}

pub trait UniqueLetters {
    fn unique_letters(&self) -> HashSet<char>;
}

impl UniqueLetters for String {
    fn unique_letters(&self) -> HashSet<char> {
        let mut letters = HashSet::new();

        for character in self.chars() {
            letters.insert(character);
        }

        letters
    }
}
