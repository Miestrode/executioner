#![feature(total_cmp)]

use std::{collections::HashSet, ffi::OsStr, fs, io, thread, time::Duration};

use rayon::{iter::FromParallelIterator, str::ParallelString};
use words::Words;

use crate::{game::Game, guesser::Guesser, words::WordSpace};
use clap::Parser;

pub mod game;
pub mod guesser;
pub mod words;

fn path_to_word_list(path: &OsStr) -> io::Result<Words> {
    fs::read_to_string(path).map(|contents| Words {
        words: contents
            .split("\n")
            .map(String::from)
            .collect::<Vec<String>>(),
    })
}

#[derive(Parser)]
#[clap(author = "Yoav")]
#[clap(version = "0.5.0")]
#[clap(about = "A program that plays Hang-man very well.")]
#[clap(
    long_about = "Executioner is a Rust-based program that plays the game of Hang-man. It uses \
                  Information Theory to guess the \"best\" letter at each turn. Executioner can \
                  work with an arbitrary list of words, using arbitrary characters."
)]
struct Args {
    word: Option<String>,
    #[clap(short, long, parse(try_from_os_str = path_to_word_list))]
    words: Words,
}

struct GameData {
    word: String,
    words: Words,
    unique_chars: HashSet<char>,
}

impl From<Args> for GameData {
    fn from(args: Args) -> Self {
        let mut words = args.words;

        let word = match args.word {
            Some(word) => {
                if !words.words.contains(&word) {
                    words.words.push(word.clone());
                };

                word
            }
            None => words.random_word(),
        };

        Self {
            // Generally, the only characters used will be A to Z, however that isn't always the case! We have to be sure.
            unique_chars: HashSet::from_par_iter(words.words.join("").par_chars()),
            word,
            words,
        }
    }
}

fn play_game(game_data: GameData) {
    Game::new(&game_data.word).play(Guesser::new(
        WordSpace::new(&game_data.words),
        game_data.unique_chars,
    ));
}

// Returns true if the game ran successfully, otherwise returns false.
pub fn run() {
    play_game(Args::parse().into());
    thread::sleep(Duration::new(2, 0));
}
