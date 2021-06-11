use rand::Rng;
use std::fmt;

#[derive(Copy, Clone)]
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