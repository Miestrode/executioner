use std::path::PathBuf;

use executioner::{guessers::player::PlayerGuesser, words::Words, Game};

fn main() {
    let words =
        Words::from(PathBuf::from("../words.txt")).expect("Could not create internal word list.");

    Game::new(words.random_word(), 5).play(PlayerGuesser);
}
