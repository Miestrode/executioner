#![feature(total_cmp)]

use words::Words;

use crate::{game::Game, guesser::Guesser, words::WordSpace};
pub mod game;
pub mod guesser;
pub mod words;

pub fn play_game(words: Words) {
    let word = words.random_word();

    Game::new(&word).play(Guesser::new(WordSpace::new(&words)));
}
