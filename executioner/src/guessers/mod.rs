use crate::ActiveState;

pub mod player;

pub trait Guess {
    fn guess(&mut self, state: &ActiveState) -> char;
}
