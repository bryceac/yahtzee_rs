use rand::Rng;
use std::fmt;

#[derive(Copy, Clone)]
struct Die {
    number: u32,
    is_held: bool
}

impl Die {
    fn new() -> Die {
        Die {
            number: 0,
            is_held: false
        }
    }

    fn roll(&mut self) {
        self.number = rand::thread_rng().gen_range(1..=6);
    }
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut output = String::new();

        if self.is_held {
            output = format!("{}!", self.number);
        } else {
            output = format!("{}", self.number)
        }

        write!(f, "{}", output)
    }
}