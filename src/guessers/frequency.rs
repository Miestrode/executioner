use std::collections::HashMap;

use crate::{
    words::{UniqueLetters, Words},
    ActiveState, Letter,
};

use super::Guess;

// A simple guesser that uses letter frequencies in the current word list to select a letter.
pub struct FrequencyGuesser {
    word_space: Words,
}

impl FrequencyGuesser {
    pub fn new(words: Words) -> Self {
        Self { word_space: words }
    }
}

impl Guess for FrequencyGuesser {
    fn guess(&mut self, state: &ActiveState) -> char {
        println!("{}", state.guess);
        println!(
            "Guesser has {} {} left | Already guessed: {}\n",
            state.lives,
            if state.lives == 1 { "try" } else { "tries" },
            if state.wrong_characters.len() == 0 {
                String::from("None")
            } else {
                state
                    .wrong_characters
                    .iter()
                    .copied()
                    .map(String::from)
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        );

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
