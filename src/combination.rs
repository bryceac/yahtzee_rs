use crate::die::Die;
use crate::pair::Pair;
use std::collections::HashSet;

pub enum Combination {
    FullHouse,
    SmallStraight,
    LargeStraight
}

impl Combination {
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

fn unique_items(dice: &[Die; 5]) -> Vec<u32> {
    let unique_set = dice.iter().map(|d| d.number).collect::<HashSet<_>>();

    return unique_set.iter().map(|n| n.clone()).collect::<Vec<_>>()
}