use executioner::{
    game::Game,
    guesser::Guesser,
    words::{WordSpace, Words},
};

fn main() {
    let words = Words::from("./words.txt").expect("Could not create internal word list");
    let word = words.random_word();

    println!(
        "The word was {}, you guessed it with {} mistake(s)",
        word,
        Game::new(&word).play(Guesser::new(WordSpace::new(&words)))
    );
}
