use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};

use crate::guesser::{CharGuess, Guesser};

use std::{
    collections::HashSet,
    fmt::{self, Display, Write},
    io::{self, Read, Write as _},
};

const CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

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
    pub fn play(&mut self, mut guesser: Guesser) {
        let mut stdout = io::stdout();

        loop {
            match self.game_state() {
                GameState::Active(state) => {
                    let guess = guesser.guess(state);

                    execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))
                        .expect("Failed to clear terminal");

                    self.guess_character(guess);
                }
                GameState::Done => {
                    execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))
                        .expect("Failed to clear terminal");

                    println!(
                        "You win! The word was {}.\nDuring the game, you made {} mistake(s).",
                        self.word,
                        self.wrong.len()
                    );

                    break;
                }
            }
        }
    }

    fn get_guess(&self, suggestion: CharGuess) -> char {
        let mut stdout = io::stdout();

        println!("╭─· {}", self.guess_state);
        println!(
            "· {}",
            if self.wrong.len() == 0 {
                String::from("This is your first guess, good luck!")
            } else {
                format!(
                    "You've already guessed: {}",
                    self.wrong
                        .iter()
                        .copied()
                        .map(String::from)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        );
        println!(
            "· The 🤖 thinks you should guess {} (with an expected info of {:.2} bit(s))",
            suggestion.char, suggestion.info
        );
        print!("╰─·Enter a guess·─▶ ");
        stdout.flush().expect("Could not flush to stdout");

        let cursor = cursor::position().expect("Failed to get cursor position.");

        loop {
            // Clear previous guess.
            execute!(
                stdout,
                cursor::MoveTo(cursor.0, cursor.1),
                Clear(ClearType::FromCursorDown)
            )
            .expect("Could not clear terminal");

            if let Some(guess) = io::stdin()
                .bytes()
                .next()
                .and_then(|guess| guess.ok())
                .map(|guess| guess as char)
                .and_then(|guess| {
                    if CHARS.contains(guess) {
                        Some(guess)
                    } else {
                        None
                    }
                })
            {
                break guess;
            }
        }
    }

    fn guess_character(&mut self, suggestion: CharGuess) {
        let guess = self.get_guess(suggestion);

        if self.word.contains(guess) {
            for (index, _) in self.word.match_indices(guess) {
                self.guess_state.0[index] = Letter::Character(guess);
            }
        } else {
            self.wrong.insert(guess);
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
