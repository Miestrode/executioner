use crate::ActiveState;

pub mod frequency;
pub mod player;
mod suite;

pub trait Guess {
    fn guess(&mut self, state: &ActiveState) -> char;
}
