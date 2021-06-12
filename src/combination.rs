// import custom types, so that they are recognized.
use crate::die::Die;
use crate::pair::Pair;

// import HashMap, so that can can be used.
use std::collections::HashSet;

/// Combination respresents the possible combinations in a game of Yahtzee.
#[derive(Debug)]
pub enum Combination {
    FullHouse,
    SmallStraight,
    LargeStraight
}

impl Combination {
    /**
     * Determine if a given roll forms a combination.
     * The following also makes a call to Pair's pair method, to help determine if the combination forms a Full House, as a Full House is also a Three of a kind.
     */
    pub fn combination(roll: &[Die; 5]) -> Option<Combination> {
        if let Some(Pair::ThreeOfAKind) = Pair::pair(roll) {
            if unique_items(roll).len() == 2 {
                Some(Combination::FullHouse)
            } else {
                None
            }
        } else {
            match roll {
                _ if count_sequential(roll) == 5 => Some(Combination::LargeStraight),
                _ if count_sequential(roll) == 4 => Some(Combination::SmallStraight),
                _ => None
            }
        }
    }
}

// check how dice have a difference of only 1.
fn count_sequential(dice: &[Die; 5]) -> u32 {
    let mut unique_numbers = unique_items(dice);
    unique_numbers.sort();

    return unique_numbers.iter().fold(0, |sequential_numbers, number| {
        if let Some(index) = unique_numbers.iter().position(|n| n == number) {

            if let Some(last_number) = unique_numbers.iter().last() {
                if number != last_number {
                    let next_number = unique_numbers[index+1];

                    if next_number != *last_number {
                        if next_number - number == 1 {
                            sequential_numbers + 1
                        } else {
                            sequential_numbers
                        }
                    } else {
                        if next_number - number == 1 {
                            sequential_numbers + 2
                        } else {
                            sequential_numbers
                        }
                    }

                } else {
                    sequential_numbers
                }
            } else {
                sequential_numbers
            }
        } else {
            sequential_numbers
        }
    })
}

// grab only unique values from a given roll.
fn unique_items(dice: &[Die; 5]) -> Vec<u32> {
    let unique_set = dice.iter().map(|d| d.number).collect::<HashSet<_>>();

    return unique_set.iter().map(|n| n.clone()).collect::<Vec<_>>()
}