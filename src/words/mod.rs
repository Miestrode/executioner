use std::{fs, io, path::Path};

use rand::prelude::SliceRandom;
use rayon::iter::{IntoParallelRefIterator, ParallelDrainRange, ParallelIterator};

use crate::game::ActiveState;

#[derive(Clone)]
pub struct Words {
    pub words: Vec<String>,
}

impl Words {
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        fs::read_to_string(path).map(|words| Self {
            words: words.split('\n').map(String::from).collect(),
        })
    }

    pub fn random_word(&self) -> String {
        self.words.choose(&mut rand::thread_rng()).unwrap().clone()
    }
}

pub struct WordSpace<'a> {
    pub words: Vec<&'a String>,
}

impl<'a> WordSpace<'a> {
    pub fn new(words: &'a Words) -> Self {
        WordSpace {
            words: words.words.iter().collect(),
        }
    }

    pub fn matching_state_portion(&'a self, state: &ActiveState) -> f32 {
        self.words
            .par_iter()
            .filter(|word| state.does_match(word))
            .count() as f32
            / self.words.len() as f32
    }

    pub fn filter_with_guess(&mut self, state: &ActiveState) {
        self.words = self
            .words
            .par_drain(..)
            .filter(|word| state.does_match(word))
            .collect::<Vec<_>>();
    }
}
