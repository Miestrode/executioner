use std::path::PathBuf;

use executioner::{guessers::player::PlayerGuesser, words::Words, Game};

fn main() {
    let words =
        Words::from(PathBuf::from("./words.txt")).expect("Could not create internal word list.");
    let picked_word = words.random_word();

    if Game::new(picked_word, 5).play(PlayerGuesser) {
        println!("Well done! You won.")
    } else {
        println!("Unlucky! The word was {}", picked_word);
    }
}
