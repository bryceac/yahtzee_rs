use rand::Rng;
use std::{ cmp::{Ordering, PartialEq, PartialOrd}, fmt};

#[derive(Copy, Clone, Eq)]
pub struct Die {
    pub number: u32,
    pub is_held: bool
}

impl Die {
    pub fn new() -> Die {
        Die {
            number: 0,
            is_held: false
        }
    }

    pub fn roll(&mut self) {
        self.number = rand::thread_rng().gen_range(1..=6);
    }
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if self.is_held {
            write!(f, "{}!", self.number)
        } else {
            write!(f, "{}", self.number)
        }
    }
}

impl PartialEq for Die {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.is_held == other.is_held
    }
}

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