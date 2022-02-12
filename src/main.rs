use std::{thread, time::Duration};

use executioner::words::Words;

fn main() {
    executioner::play_game(
        Words::from("./words.txt").expect("Could not create internal word list"),
    );
    thread::sleep(Duration::new(2, 0));
}
