use std::io::{ stdin, stdout, Write };
struct Dialogue {
    message: String,
    choices: Vec<String>
}

impl Dialogue {
    fn new(message: &str, choices: Vec<String>) -> Dialogue {
        Dialogue {
            message: String::from(message),
            choices
        }
    }

    fn run(&self) -> u32 {
        let mut choice: Option<u32> = None;

        while let None = choice {
            for (index, choice) in self.choices.iter().enumerate() {
                println!("{}. {}", index+1, choice);
            }
    
            print!("{}", self.message);

            let _ = stdout().flush();

            let mut input = String::new();

            stdin().read_line(buf: &input).expect("Failed to retrieve input.");

            if let Some(selection) {
                choice = selection.parse::<u32>()
            }
        }

    }
}