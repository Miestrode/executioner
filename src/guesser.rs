use bitvec::{macros::internal::funty::Fundamental, prelude::*};

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

pub struct Guesser<'a> {
    word_space: WordSpace<'a>,
}

fn portion_to_info(portion: f32) -> f32 {
    if portion == 0.0 {
        0.0
    } else if portion == 1.0 {
        1.0
    } else {
        -portion.log2()
    }
}

impl<'a> Guesser<'a> {
    pub fn new(word_space: WordSpace<'a>) -> Self {
        Self { word_space }
    }

    pub fn guess(&mut self, mut state: ActiveState) -> char {
        self.word_space.filter_with_guess(&state);

        let indices = state.guess.unknown_indices();

        ('a'..='z') // TODO: Use all characters in word list! Not just the english ones!
            .filter(|char| {
                !(state.wrong.contains(char) || state.guess.0.contains(&Letter::Character(*char)))
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|char| {
                let mut info = 0.0; // The amount of mathematical information (in bits).

                //I use binary numbers to save the pain of writing an actual permutation function that would be 10 times slower.
                for permutation in 0..(2u32.pow(indices.len() as u32)) {
                    let bits = permutation.view_bits::<Lsb0>();

                    indices.iter().zip(bits).for_each(|(other, bit)| {
                        state.guess.0[*other] = if bit.as_bool() {
                            Letter::Character(char)
                        } else {
                            Letter::Unknown
                        }
                    });

                    let portion = self.word_space.matching_state_portion(&state);
                    info += portion * portion_to_info(portion);
                }

                (char, info)
            })
            .max_by(|(_, info_a), (_, info_b)| info_a.partial_cmp(info_b).unwrap())
            .unwrap()
            .0
    }
}
