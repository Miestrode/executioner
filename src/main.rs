use std::path::PathBuf;

use executioner::{guessers::frequency::FrequencyGuesser, words::Words, Game};

fn main() {
    let words =
        Words::from(PathBuf::from("./words.txt")).expect("Could not create internal word list.");
    let picked_word = words.random_word();

    if Game::new(&picked_word, 7).play(FrequencyGuesser::new(words)) {
        println!("Well done! The word was indeed {}.", picked_word);
    } else {
        println!(
            "You lost! The word was {}, better luck next time!",
            picked_word
        );
    }
}
