use crate::ActiveState;

pub mod frequency;
pub mod player;
mod suite;

pub trait Guesser {
    fn guess(&mut self, state: &ActiveState) -> char;
}
