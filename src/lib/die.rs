use rand::Rng; // import module needed to get random numbers

// import modules necessary to display data and allow comparisons to be made
use std::{ cmp::{Ordering, PartialEq, PartialOrd}, fmt};


/// Represents a die object in a game.
#[derive(Copy, Clone, Eq)]
pub struct Die {
    /// value rolled. Defaults to 0.
    pub number: u32,

    /// specifies whether user has it in reserve. Defaults to false.
    pub is_held: bool
}

impl Die {
    /// create a new die object.
    pub fn new() -> Die {
        Die {
            number: 0,
            is_held: false
        }
    }

    /// simulate a die roll.
    pub fn roll(&mut self) {
        self.number = rand::thread_rng().gen_range(1..=6);
    }
}

// help user see information on the die, and denote whether it was held or not.
impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if self.is_held {
            write!(f, "{}!", self.number)
        } else {
            write!(f, "{}", self.number)
        }
    }
}

// implement stuff necessary equality comparisons to be made.
impl PartialEq for Die {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.is_held == other.is_held
    }
}

// implement code needed to make it easy to compare dice.
impl Ord for Die {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for Die {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}