use crate::ActiveState;
use std::io;

use super::Guess;

pub struct PlayerGuesser;

impl Guess for PlayerGuesser {
    fn guess(&mut self, state: &ActiveState) -> char {
        println!("{}", state.guess);
        println!(
            "You have {} live(s) | Already guessed: {}",
            state.lives,
            state
                .wrong_characters
                .iter()
                .copied()
                .map(String::from)
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!("Enter a character to guess: ");

        loop {
            let mut buffer = String::new();

            match io::stdin().read_line(&mut buffer) {
                Ok(length) => {
                    if length != 1 {
                        println!("Please enter a single character.")
                    } else {
                        break buffer.chars().next().unwrap();
                    }
                }
                Err(_) => println!("Failed to read user data."),
            }
        }
    }
}
