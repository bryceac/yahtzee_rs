use text_io::read; // import module to make retrieve data from the user easier

// import stuff needed to convert values and to get things to show better in the CLI.
use std::{convert::TryInto, io::{stdout, Write} };

/// represents an interactive dialog for the user.
pub struct Dialogue {
    pub message: String,
    pub choices: Vec<String>
}

impl Dialogue {
    /// create dialogue object with a specified prompt and choices.
    pub fn new(message: &str, choices: Vec<String>) -> Dialogue {
        Dialogue {
            message: String::from(message),
            choices
        }
    }

    // activate the dialog and get user selection
    pub fn run(&self) -> u32 {

        // create variable to hold user input. It is an Option, so that it can be used for a loop condition.
        let mut choice: Option<u32> = None;

        // run code unit a proper choice is made. The choice is 1 higher than the index in the vector.
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
                println!("Input not valid.")
            }
        }

        return choice.unwrap();

    }
}