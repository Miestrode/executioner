use crossterm::{
    cursor, execute,
    style::Stylize,
    terminal::{Clear, ClearType},
};

use crate::guesser::{CharGuess, Guesser};

use std::{
    collections::HashSet,
    fmt::{self, Display, Write},
    io::{self, Read, Write as _},
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
enum GameState {
    Active(ActiveState),
    Done,
}

pub struct Game<'a> {
    word: &'a str,
    guess_state: GuessState,
    wrong: WrongGuesses,
}

impl<'a> Game<'a> {
    pub fn new(word: &'a str) -> Self {
        Self {
            guess_state: GuessState(vec![Letter::Unknown; word.len()]),
            word,
            wrong: WrongGuesses::new(),
        }
    }

    fn write_prompt(&self, suggestion: CharGuess) {
        println!(
            "{}",
            self.guess_state.to_string().bold().underlined().yellow()
        );
        println!(
            "\n{}",
            if self.wrong.0.len() == 0 {
                String::from("You have not made any mistakes.")
            } else {
                format!("You've mistakenly guessed {}.", self.wrong.to_string())
            }
        );
        println!(
            "The algorithm suggests you should guess {} since it has an expected information of \
             {} bit{}.",
            suggestion.char.yellow(),
            format!("{:.2}", suggestion.info).yellow(),
            if suggestion.info != 1.0 { "s" } else { "" }
        );
        print!("\nEnter a guess {} ", "─▶".yellow());
        io::stdout().flush().expect("Could not flush to stdout");
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
                        self.word.yellow().bold(),
                        self.wrong.0.len().to_string().yellow().bold()
                    );

                    break;
                }
            }
        }
    }

    fn get_guess(&self, suggestion: CharGuess) -> char {
        let mut stdout = io::stdout();

        self.write_prompt(suggestion);

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
                    if !guess.is_whitespace() {
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
            self.wrong.0.insert(guess);
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
                ' '
            })?;
        }

        fmt::Result::Ok(())
    }
}
