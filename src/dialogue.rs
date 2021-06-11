use text_io::read;
use std::{convert::TryInto, io::{stdout, Write} };
pub struct Dialogue {
    pub message: String,
    pub choices: Vec<String>
}

impl Dialogue {
    pub fn new(message: &str, choices: Vec<String>) -> Dialogue {
        Dialogue {
            message: String::from(message),
            choices
        }
    }

    pub fn run(&self) -> u32 {
        let mut choice: Option<u32> = None;

        while let None = choice {
            for (index, choice) in self.choices.iter().enumerate() {
                println!("{}. {}", index+1, choice);
            }
    
            print!("{}", self.message);
            let _ = stdout().flush();

            let input: u32 = read!();

            if input <= self.choices.len().try_into().unwrap() {
                choice = Some(input);
            } else {
                println!("")
            }
        }

        return choice.unwrap();

    }
}