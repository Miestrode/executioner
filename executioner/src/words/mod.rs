use std::{fs, io, path::PathBuf};

use rand::prelude::SliceRandom;

pub struct Words {
    words: Vec<String>,
}

impl Words {
    pub fn from(path: PathBuf) -> Result<Self, io::Error> {
        fs::read_to_string(path).map(|words| Self {
            words: words.split('\n').map(String::from).collect(),
        })
    }

    pub fn random_word(&self) -> &str {
        self.words.choose(&mut rand::thread_rng()).unwrap()
    }
}
