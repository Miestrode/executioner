pub mod guessers;
pub mod words;

use guessers::Guess;

use std::{
    collections::HashSet,
    fmt::{self, Display, Write},
};

#[derive(Clone, Copy, PartialEq)]
enum Letter {
    Unknown,
    Character(char),
}

#[derive(Clone)]
pub struct GuessState(Vec<Letter>);

impl<'a> ActiveState<'a> {
    pub fn does_match(&self, word: &str) -> bool {
        for character in word.chars() {
            if self.wrong_characters.contains(&character) {
                return false;
            }
        }

        if self.guess.0.len() != word.len() {
            false
        } else {
            self.guess
                .0
                .iter()
                .zip(word.chars())
                .all(|(guess_letter, word_letter)| match guess_letter {
                    Letter::Unknown => true,
                    &Letter::Character(guess_letter) => guess_letter == word_letter,
                })
        }
    }
}

pub struct ActiveState<'a> {
    guess: &'a GuessState,
    lives: u32,
    wrong_characters: &'a HashSet<char>,
}

enum GameState<'a> {
    Active(ActiveState<'a>),
    Done { success: bool },
}

pub struct Game<'a> {
    word: &'a str,
    guess_state: GuessState,
    wrong_characters: HashSet<char>,
    lives: u32,
}

impl<'a> Game<'a> {
    pub fn new(word: &'a str, lives: u32) -> Self {
        Self {
            guess_state: GuessState(vec![Letter::Unknown; word.len()]),
            word,
            wrong_characters: HashSet::new(),
            lives,
        }
    }

    pub fn play(&mut self, mut guesser: impl Guess) -> bool {
        loop {
            match &self.game_state() {
                GameState::Active(state) => {
                    let character = guesser.guess(&state);

                    self.guess_character(character)
                }
                &GameState::Done { success } => break success,
            }
        }
    }

    fn guess_character(&mut self, character: char) {
        if self.word.contains(character) {
            for (index, _) in self.word.match_indices(character) {
                self.guess_state.0[index] = Letter::Character(character);
            }
        } else {
            self.lives = self.lives.saturating_sub(1);
            self.wrong_characters.insert(character);
        }
    }

    fn game_state(&self) -> GameState {
        if self.lives == 0 {
            GameState::Done { success: false }
        } else if !self.guess_state.0.contains(&Letter::Unknown) {
            GameState::Done { success: true }
        } else {
            GameState::Active(ActiveState {
                lives: self.lives,
                guess: &self.guess_state,
                wrong_characters: &self.wrong_characters,
            })
        }
    }
}

impl Display for GuessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for &letter in &self.0 {
            f.write_char(if let Letter::Character(character) = letter {
                character
            } else {
                '_'
            })?;
        }

        fmt::Result::Ok(())
    }
}
