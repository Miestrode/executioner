use std::collections::HashMap;

use yansi::Color;

use crate::{
    words::{UniqueLetters, Words},
    ActiveState, Letter,
};

use super::Guesser;

// A simple guesser that uses letter frequencies in the current word list to select a letter.
pub struct FrequencyGuesser {
    word_space: Words,
}

impl FrequencyGuesser {
    pub fn new(words: Words) -> Self {
        Self { word_space: words }
    }
}

impl Guesser for FrequencyGuesser {
    fn guess(&mut self, state: &ActiveState) -> char {
        println!("✳️ {} {}", Color::Green.paint(state.lives), state.guess);

        self.word_space.filter_with_guess(state);
        let mut frequencies = HashMap::new();

        for word in &self.word_space.words {
            for character in word.unique_letters() {
                let frequency = frequencies.entry(character).or_insert(0);
                *frequency += 1;
            }
        }

        frequencies
            .into_iter()
            .filter(|&(character, _)| !state.guess.0.contains(&Letter::Character(character)))
            .reduce(|accumulator, next| {
                if accumulator.1 > next.1 {
                    accumulator
                } else {
                    next
                }
            })
            .unwrap() // Let's for now assume in good faith that there is at least one non-empty word in the word list.
            .0
    }
}
