use text_io::read;
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

            choice = read!();
        }

        return choice.unwrap();

    }
}