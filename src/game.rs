use crate::guesser::Guesser;

use std::{
    collections::HashSet,
    fmt::{self, Display, Write},
};

#[derive(Clone, Copy, PartialEq)]
pub enum Letter {
    Unknown,
    Character(char),
}

#[derive(Clone)]
pub struct GuessState(pub Vec<Letter>);

impl ActiveState {
    pub fn does_match(&self, word: &str) -> bool {
        if self.guess.0.len() != word.len() {
            false
        } else {
            for (char, letter) in word.chars().zip(self.guess.0.iter().copied()) {
                if self.wrong.contains(&char) {
                    return false;
                } else if letter != Letter::Character(char) && letter != Letter::Unknown {
                    return false;
                }
            }

            true
        }
    }
}

#[derive(Clone)]
pub struct ActiveState {
    pub guess: GuessState,
    pub wrong: HashSet<char>,
}

#[derive(Clone)]
enum GameState {
    Active(ActiveState),
    Done,
}

pub struct Game<'a> {
    word: &'a str,
    guess_state: GuessState,
    wrong: HashSet<char>,
}

impl<'a> Game<'a> {
    pub fn new(word: &'a str) -> Self {
        Self {
            guess_state: GuessState(vec![Letter::Unknown; word.len()]),
            word,
            wrong: HashSet::new(),
        }
    }

    // Returns the number of mistakes.
    pub fn play(&mut self, mut guesser: Guesser) -> usize {
        loop {
            match self.game_state() {
                GameState::Active(state) => {
                    println!("{} | Mistakes: {:?}", state.guess, self.wrong);

                    let character = guesser.guess(state);
                    self.guess_character(character);
                }
                GameState::Done => break self.wrong.len() as usize,
            }
        }
    }

    fn guess_character(&mut self, character: char) {
        if self.word.contains(character) {
            for (index, _) in self.word.match_indices(character) {
                self.guess_state.0[index] = Letter::Character(character);
            }
        } else {
            self.wrong.insert(character);
        }
    }

    fn game_state(&mut self) -> GameState {
        if !self.guess_state.0.contains(&Letter::Unknown) {
            GameState::Done
        } else {
            GameState::Active(ActiveState {
                guess: self.guess_state.clone(),
                wrong: self.wrong.clone(),
            })
        }
    }
}

impl Display for GuessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for &letter in &self.0 {
            f.write_char(if let Letter::Character(char) = letter {
                char
            } else {
                '_'
            })?;
        }

        fmt::Result::Ok(())
    }
}
