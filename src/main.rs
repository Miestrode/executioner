use std::path::PathBuf;

use executioner::{guessers::frequency::FrequencyGuesser, words::Words, Game};
use yansi::{Color, Style};

fn main() {
    let words =
        Words::from(PathBuf::from("./words.txt")).expect("Could not create internal word list.");
    let picked_word = words.random_word();

    println!(
        "{}",
        Style::default()
            .paint(format!("The word is {}.\n", picked_word))
            .bold()
    );

    let won = Game::new(&picked_word, 7).play(FrequencyGuesser::new(words));
    println!();

    if won {
        println!("{}", Color::Green.paint("Success!").bold());
    } else {
        println!("{}", Color::Red.paint("Failure!").bold());
    }
}
