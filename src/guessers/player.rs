use crate::ActiveState;
use std::io::{self, Write};

use super::Guesser;

pub struct PlayerGuesser;

impl Guesser for PlayerGuesser {
    fn guess(&mut self, state: &ActiveState) -> char {
        println!("{}", state.guess);
        println!(
            "You have {} {} left | Already guessed: {}",
            state.lives,
            if state.lives == 1 { "try" } else { "tries" },
            if state.wrong.len() == 0 {
                String::from("None")
            } else {
                state
                    .wrong
                    .iter()
                    .copied()
                    .map(String::from)
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        );

        loop {
            print!("Enter a character to guess: ");
            io::stdout()
                .flush()
                .expect("could not flush text to standard output.");

            let mut buffer = String::new();

            match io::stdin().read_line(&mut buffer) {
                Ok(_) => {
                    let buffer = buffer.trim();

                    if buffer.len() != 1 {
                        println!("Please enter a single character.")
                    } else {
                        println!(); // This is done to space out the different guess attempts.
                        break buffer.chars().next().unwrap();
                    }
                }
                Err(_) => println!("Failed to read user data."),
            }
        }
    }
}
