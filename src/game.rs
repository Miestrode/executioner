use bitvec::prelude::*;
use crossterm::{
    cursor, execute,
    style::Stylize,
    terminal::{Clear, ClearType},
};

use crate::{
    guesser::{Guess, Guesser},
    words::WordSpace,
};

use std::{
    collections::HashSet,
    fmt::{self, Display, Write},
    io::{self, Write as _},
    iter,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Letter {
    Unknown,
    Character(char),
}

#[derive(Clone)]
pub struct GuessState(pub Vec<Letter>);

impl GuessState {
    pub fn new(length: usize) -> Self {
        Self(iter::repeat(Letter::Unknown).take(length).collect())
    }
}

impl ActiveState {
    pub fn does_match(&self, word: &str) -> bool {
        if self.guess.0.len() != word.len() {
            false
        } else {
            let mut unique_letters = HashSet::new();

            for char in word.chars() {
                unique_letters.insert(char);
            }

            for (char, letter) in word.chars().zip(self.guess.0.iter().copied()) {
                if self.wrong.0.contains(&char) {
                    return false;
                } else if letter != Letter::Character(char) // If the letter doesn't match what's in the word, the match failed...
                // However! It could be unknown, in which case we make sure the letter is actually unique.
                    && (letter != Letter::Unknown || !unique_letters.contains(&char))
                {
                    return false;
                }
            }

            true
        }
    }
}

#[derive(Clone)]
pub struct WrongGuesses(pub HashSet<char>);

impl WrongGuesses {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
}

impl Display for WrongGuesses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert_ne!(self.0.len(), 0);
        let wrong_guesses = self
            .0
            .iter()
            .copied()
            .map(|char| char.yellow().bold().to_string())
            .collect::<Vec<_>>();

        f.write_str(&wrong_guesses[..wrong_guesses.len() - 1].as_ref().join(", "))?;

        // If the length is 1, nothing was written previously.
        if wrong_guesses.len() != 1 {
            f.write_str(" and ")?;
        }

        f.write_str(wrong_guesses.last().unwrap())?;

        fmt::Result::Ok(())
    }
}

#[derive(Clone)]
pub struct ActiveState {
    pub guess: GuessState,
    pub wrong: WrongGuesses,
}

#[derive(Clone)]
pub enum GameState {
    Active(ActiveState),
    Done,
}

pub trait Game {
    fn write_prompt<'a>(&self, char: char, info: f32) {
        execute!(io::stdout(), cursor::MoveTo(0, 0), Clear(ClearType::All))
            .expect("Failed to clear terminal");
        println!(
            "{}",
            self.guess_state().to_string().bold().underlined().yellow()
        );
        println!(
            "\n{}",
            if self.wrong().0.len() == 0 {
                String::from("You have not made any mistakes.")
            } else {
                format!("You've mistakenly guessed {}.", self.wrong().to_string())
            }
        );
        println!(
            "The algorithm suggests you should guess {} since it has an expected information of \
             {} bit{}.",
            char.yellow(),
            format!("{:.2}", info).yellow(),
            if info != 1.0 { "s" } else { "" }
        );
        print!("\nEnter a guess {} ", "─▶".yellow());
        io::stdout().flush().expect("Could not flush to stdout");
    }

    fn play(&mut self, mut guesser: Guesser) {
        let mut stdout = io::stdout();

        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))
            .expect("Failed to clear terminal");
        loop {
            let guess = guesser.guess(ActiveState {
                guess: self.guess_state().clone(),
                wrong: self.wrong().clone(),
            });

            match guess {
                Guess::Char { char, info } => {
                    self.guess_character(char, info);
                }
                Guess::Word(word) => {
                    println!(
                        "You win! The word was {}.\nDuring the game, you made {} mistake(s).",
                        word.yellow().bold(),
                        self.wrong().0.len().to_string().yellow().bold()
                    );

                    break;
                }
                Guess::Unknown => {
                    println!("This word is not in the database, and so could not be guessed.");
                    println!("{}", self.guess_state());
                    break;
                }
            }
        }
    }

    fn get_guess(&self, char: char, info: f32) -> char {
        let mut stdout = io::stdout();

        self.write_prompt(char, info);

        let cursor = cursor::position().expect("Failed to get cursor position.");

        loop {
            // Clear previous guess.
            execute!(
                stdout,
                cursor::MoveTo(cursor.0, cursor.1),
                Clear(ClearType::FromCursorDown)
            )
            .expect("Could not clear terminal");

            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Could not read line");

            if let Some(input) = buffer.trim().chars().next() {
                break input;
            }
        }
    }

    fn guess_character(&mut self, char: char, info: f32) {
        let guess = self.get_guess(char, info);

        let guess_indicies = self.get_guess_indices(guess);

        if !guess_indicies.is_empty() {
            for index in guess_indicies {
                self.mut_guess_state().0[index] = Letter::Character(guess);
            }
        } else {
            self.mut_wrong().0.insert(guess);
        }
    }

    fn game_state(&self) -> GameState {
        if !self.guess_state().0.contains(&Letter::Unknown) {
            GameState::Done
        } else {
            GameState::Active(ActiveState {
                guess: self.guess_state().clone(),
                wrong: self.wrong().clone(),
            })
        }
    }

    fn get_guess_indices(&self, guess: char) -> Vec<usize>;

    fn guess_state(&self) -> &GuessState;

    fn wrong(&self) -> &WrongGuesses;

    fn mut_guess_state(&mut self) -> &mut GuessState;

    fn mut_wrong(&mut self) -> &mut WrongGuesses;
}

pub struct FullGame<'a> {
    word: &'a str,
    guess_state: GuessState,
    wrong: WrongGuesses,
}

impl<'a> FullGame<'a> {
    pub fn new(word: &'a str) -> Self {
        Self {
            guess_state: GuessState(vec![Letter::Unknown; word.len()]),
            word,
            wrong: WrongGuesses::new(),
        }
    }
}

impl<'a> Game for FullGame<'a> {
    fn get_guess_indices(&self, guess: char) -> Vec<usize> {
        self.word
            .match_indices(guess)
            .map(|(index, _)| index)
            .collect()
    }

    fn guess_state(&self) -> &GuessState {
        &self.guess_state
    }

    fn wrong(&self) -> &WrongGuesses {
        &self.wrong
    }

    fn mut_guess_state(&mut self) -> &mut GuessState {
        &mut self.guess_state
    }

    fn mut_wrong(&mut self) -> &mut WrongGuesses {
        &mut self.wrong
    }
}

pub struct PartialGame {
    guess_state: GuessState,
    wrong: WrongGuesses,
    length: usize,
}

impl PartialGame {
    pub fn new(length: usize) -> Self {
        Self {
            guess_state: GuessState::new(length),
            wrong: WrongGuesses::new(),
            length,
        }
    }
}

impl Game for PartialGame {
    fn get_guess_indices(&self, _: char) -> Vec<usize> {
        let mut indices = vec![];

        loop {
            print!("A guess letter index in the word: ");
            io::stdout()
                .flush()
                .expect("Could not flush to standard output");

            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Could not read line");

            if buffer == "\n" {
                break indices;
            } else if let Some(index) = buffer.trim().parse().ok().and_then(|number| {
                if number < self.length && !indices.contains(&number) {
                    Some(number)
                } else {
                    None
                }
            }) {
                indices.push(index);
            }
        }
    }

    fn guess_state(&self) -> &GuessState {
        &self.guess_state
    }

    fn wrong(&self) -> &WrongGuesses {
        &self.wrong
    }

    fn mut_guess_state(&mut self) -> &mut GuessState {
        &mut self.guess_state
    }

    fn mut_wrong(&mut self) -> &mut WrongGuesses {
        &mut self.wrong
    }
}

pub struct AntagonisticGame<'a> {
    word_space: WordSpace<'a>,
    guess_state: GuessState,
    wrong: WrongGuesses,
}

impl<'a> AntagonisticGame<'a> {
    pub fn new(length: usize, word_space: WordSpace<'a>) -> Self {
        Self {
            word_space,
            guess_state: GuessState::new(length),
            wrong: WrongGuesses::new(),
        }
    }
}

impl Game for AntagonisticGame<'_> {
    fn get_guess_indices(&self, guess: char) -> Vec<usize> {
        let indices = self.guess_state.unknown_indices();

        let permutation = (0..(1 << indices.len()))
            .map(|permutation: u32| {
                let bits = permutation.view_bits::<Lsb0>();

                let mut guess_state = self.guess_state.clone();

                for (index, bit) in indices.iter().zip(bits.iter().by_ref()) {
                    if *bit {
                        guess_state.0[*index] = Letter::Character(guess);
                    }
                }
                (
                    permutation,
                    self.word_space.matching_state_count(&ActiveState {
                        guess: guess_state,
                        wrong: self.wrong.clone(),
                    }),
                )
            })
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0;

        indices
            .into_iter()
            .zip(permutation.view_bits::<Lsb0>().iter().by_refs())
            .filter_map(|(index, bit)| if *bit { Some(index) } else { None })
            .collect()
    }

    fn guess_state(&self) -> &GuessState {
        &self.guess_state
    }

    fn wrong(&self) -> &WrongGuesses {
        &self.wrong
    }

    fn mut_guess_state(&mut self) -> &mut GuessState {
        &mut self.guess_state
    }

    fn mut_wrong(&mut self) -> &mut WrongGuesses {
        &mut self.wrong
    }
}

impl Display for GuessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for &letter in &self.0 {
            f.write_char(if let Letter::Character(char) = letter {
                char
            } else {
                ' '
            })?;
        }

        fmt::Result::Ok(())
    }
}
