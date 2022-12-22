use std::collections::HashSet;

use bitvec::{macros::internal::funty::Fundamental, prelude::*};
use rayon::iter::{ParallelDrainFull, ParallelIterator};

use crate::{
    game::{ActiveState, GuessState, Letter},
    words::WordSpace,
};

impl GuessState {
    fn unknown_indices(&mut self) -> Vec<usize> {
        self.0
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(idx, letter)| {
                if matches!(letter, Letter::Unknown) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub enum Guess<'a> {
    Char { char: char, info: f32 },
    Word(&'a str),
    Unknown,
}

pub struct Guesser<'a> {
    word_space: WordSpace<'a>,
    possible_guesses: HashSet<char>,
}

fn portion_to_info(portion: f32) -> f32 {
    if portion == 0.0 {
        0.0
    } else {
        -portion.log2()
    }
}

fn expected_info(portion: f32) -> f32 {
    portion * portion_to_info(portion)
}

impl<'a> Guesser<'a> {
    pub fn new(word_space: WordSpace<'a>, possible_guesses: HashSet<char>) -> Self {
        Self {
            word_space,
            possible_guesses,
        }
    }

    fn filter_guesses(&mut self, state: &ActiveState) {
        let drained_guesses = self.possible_guesses.par_drain();

        self.possible_guesses = drained_guesses
            .filter(|char| {
                !(state.wrong.0.contains(char) || state.guess.0.contains(&Letter::Character(*char)))
            })
            .collect();
    }

    pub fn guess(&mut self, mut state: ActiveState) -> Guess {
        self.word_space.filter_with_guess(&state);

        match self.word_space.words.len() {
            0 => Guess::Unknown,
            1 => Guess::Word(self.word_space.words[0]),
            _ => {
                self.filter_guesses(&state);
                let indices = state.guess.unknown_indices();

                let (char, info) = self
                    .possible_guesses
                    .iter()
                    .copied()
                    .map(|char| {
                        // There's always a case where you found no matches with a guess,
                        // in that case we can add the character into the "wrong guesses" set and calculate the expected information.
                        state.wrong.0.insert(char);
                        let mut info =
                            expected_info(self.word_space.matching_state_portion(&state)); // The amount of mathematical information (in bits).
                        state.wrong.0.remove(&char);

                        // This code will just update the expected information for each permutation.
                        // I use binary numbers to save the pain of writing an actual permutation function that would be 10 times slower.
                        for permutation in 1..(2u32.pow(indices.len() as u32)) {
                            let bits = permutation.view_bits::<Lsb0>();

                            indices.iter().zip(bits).for_each(|(other, bit)| {
                                state.guess.0[*other] = if bit.as_bool() {
                                    Letter::Character(char)
                                } else {
                                    Letter::Unknown
                                }
                            });

                            info += expected_info(self.word_space.matching_state_portion(&state))
                        }

                        (char, info)
                    })
                    .max_by(|(_, info), (_, other)| info.total_cmp(other))
                    .unwrap();
                Guess::Char { char, info }
            }
        }
    }
}
